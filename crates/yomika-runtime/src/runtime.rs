use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use tokio::sync::broadcast;

use crate::downloads::Downloads;
use crate::packages::PackageCatalog;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub const MODEL_LIBRARY_MARKER: &str = ".yomika-model-library";

pub type RuntimeHttpClient = Arc<ClientWithMiddleware>;

/// Create the ownership marker for a model-library root, or validate the
/// existing marker without following links outside the managed directory.
pub fn ensure_model_library_marker(root: &Path) -> Result<()> {
    let marker = root.join(MODEL_LIBRARY_MARKER);
    match std::fs::symlink_metadata(&marker) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            anyhow::bail!(
                "refusing symlinked model-library marker `{}`",
                marker.display()
            )
        }
        Ok(metadata) if !metadata.is_file() => {
            anyhow::bail!("model-library marker is not a file: `{}`", marker.display())
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            std::fs::write(&marker, b"Yomika managed model library\n")
                .with_context(|| format!("failed to create `{}`", marker.display()))
        }
        Err(error) => {
            Err(error).with_context(|| format!("failed to inspect `{}`", marker.display()))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputePolicy {
    PreferGpu,
    CpuOnly,
}

#[derive(Debug, Clone)]
pub struct RuntimeHttpConfig {
    pub connect_timeout_secs: u64,
    pub read_timeout_secs: u64,
    pub max_retries: u32,
}

impl Default for RuntimeHttpConfig {
    fn default() -> Self {
        Self {
            connect_timeout_secs: 20,
            read_timeout_secs: 300,
            max_retries: 3,
        }
    }
}

impl RuntimeHttpConfig {
    pub fn build_client(&self) -> Result<RuntimeHttpClient> {
        let base = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .connect_timeout(Duration::from_secs(self.connect_timeout_secs))
            .read_timeout(Duration::from_secs(self.read_timeout_secs))
            .build()?;
        Ok(Arc::new(
            ClientBuilder::new(base)
                .with(RetryTransientMiddleware::new_with_policy(
                    ExponentialBackoff::builder().build_with_max_retries(self.max_retries),
                ))
                .build(),
        ))
    }
}

// FIXME: move this function to a more appropriate place, e.g. a `config` module
pub fn default_app_data_root() -> Utf8PathBuf {
    // Env-override (tests, CI, portable installs).
    if let Ok(v) = std::env::var("YOMIKA_DATA_ROOT")
        && !v.is_empty()
    {
        return Utf8PathBuf::from(v);
    }

    if let Ok(exe) = std::env::current_exe()
        && let Some(root) = exe.parent()
        && root.join("config.toml").is_file()
    {
        return Utf8PathBuf::from_path_buf(root.to_path_buf())
            .unwrap_or_else(|path| Utf8PathBuf::from(path.to_string_lossy().into_owned()));
    }

    let root = dirs::data_local_dir()
        .or_else(dirs::data_dir)
        .unwrap_or_else(std::env::temp_dir)
        .join("Yomika");
    Utf8PathBuf::from_path_buf(root)
        .unwrap_or_else(|path| Utf8PathBuf::from(path.to_string_lossy().into_owned()))
}

#[derive(Clone)]
pub struct Runtime {
    inner: Arc<RuntimeInner>,
}

struct RuntimeInner {
    root: PathBuf,
    models_root: PathBuf,
    compute: ComputePolicy,
    downloads: Downloads,
    packages: PackageCatalog,
}

impl Runtime {
    pub fn new(root: impl Into<PathBuf>, compute: ComputePolicy) -> Result<Self> {
        Self::new_with_http(root, compute, RuntimeHttpConfig::default())
    }

    pub fn new_with_http(
        root: impl Into<PathBuf>,
        compute: ComputePolicy,
        http: RuntimeHttpConfig,
    ) -> Result<Self> {
        let root = root.into();
        let models_root = root.join("models");
        Self::new_with_storage(root, models_root, compute, http)
    }

    /// Build a runtime with a separate managed model-library directory.
    /// Existing callers keep the conventional `{root}/models` layout through
    /// [`Runtime::new_with_http`].
    pub fn new_with_storage(
        root: impl Into<PathBuf>,
        models_root: impl Into<PathBuf>,
        compute: ComputePolicy,
        http: RuntimeHttpConfig,
    ) -> Result<Self> {
        let root = root.into();
        let models_root = models_root.into();
        let downloads = Downloads::new(
            root.join("runtime").join(".downloads"),
            models_root.join("huggingface"),
            &http,
        )?;

        Ok(Self {
            inner: Arc::new(RuntimeInner {
                root,
                models_root,
                compute,
                downloads,
                packages: PackageCatalog::discover(),
            }),
        })
    }

    pub fn root(&self) -> &Path {
        &self.inner.root
    }

    pub fn models_root(&self) -> &Path {
        &self.inner.models_root
    }

    pub fn wants_gpu(&self) -> bool {
        matches!(self.inner.compute, ComputePolicy::PreferGpu)
    }

    pub fn http_client(&self) -> RuntimeHttpClient {
        self.inner.downloads.client()
    }

    pub fn subscribe_downloads(&self) -> broadcast::Receiver<yomika_core::DownloadProgress> {
        self.inner.downloads.subscribe()
    }

    pub fn downloads(&self) -> Downloads {
        self.inner.downloads.clone()
    }

    pub async fn prepare(&self) -> Result<()> {
        let dirs = [
            self.root().join("runtime"),
            self.root().join("runtime").join(".downloads"),
            self.models_root().to_path_buf(),
            self.models_root().join("huggingface"),
        ];
        for dir in dirs {
            std::fs::create_dir_all(&dir)
                .with_context(|| format!("failed to create `{}`", dir.display()))?;
        }
        ensure_model_library_marker(self.models_root())?;
        self.inner.packages.prepare_bootstrap(self).await
    }

    pub fn llama_directory(&self) -> Result<PathBuf> {
        crate::llama::runtime_dir(self)
    }
}

pub type RuntimeManager = Runtime;

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;

    use super::*;

    #[cfg(target_family = "unix")]
    #[test]
    fn model_library_marker_never_follows_a_symlink() -> Result<()> {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir()?;
        let external = root.path().join("external.txt");
        let models = root.path().join("models");
        fs::create_dir_all(&models)?;
        fs::write(&external, b"keep")?;
        symlink(&external, models.join(MODEL_LIBRARY_MARKER))?;

        let error = ensure_model_library_marker(&models).unwrap_err();
        assert!(error.to_string().contains("symlinked"));
        assert_eq!(fs::read(&external)?, b"keep");
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn prepares_llama_runtime_into_configured_root() -> Result<()> {
        let tempdir = tempfile::tempdir()?;
        let runtime = Runtime::new(tempdir.path(), ComputePolicy::CpuOnly)?;
        runtime.prepare().await?;
        assert!(runtime.llama_directory()?.exists());
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn repeated_basename_loads_succeed_after_prepare() -> Result<()> {
        let tempdir = tempfile::tempdir()?;
        let runtime = Runtime::new(tempdir.path(), ComputePolicy::CpuOnly)?;
        runtime.prepare().await?;
        let dir = runtime.llama_directory()?;

        let lib_name = fs::read_dir(&dir)?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                name.contains("llama").then_some(name)
            })
            .next()
            .ok_or_else(|| anyhow::anyhow!("no llama library found"))?;

        let _first = crate::load_library_by_name(&lib_name)?;
        let _second = crate::load_library_by_name(&lib_name)?;
        Ok(())
    }
}
