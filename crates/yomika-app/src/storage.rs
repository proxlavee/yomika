//! Guarded maintenance for Yomika-owned model and temporary-download data.

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result, bail};
use strum::IntoEnumIterator;
use uuid::Uuid;
use walkdir::WalkDir;
use yomika_llm::ModelId;
use yomika_runtime::{
    MODEL_LIBRARY_MARKER, RuntimeManager, ensure_model_library_marker, hf_hub::Repo,
};

const HUGGINGFACE_DIR: &str = "huggingface";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageUsage {
    pub models_bytes: u64,
    pub temporary_bytes: u64,
    pub downloaded_local_models: usize,
}

#[derive(Debug)]
pub struct PreparedModelLocation {
    pub destination: PathBuf,
    pub copied_bytes: u64,
    source_huggingface: Option<PathBuf>,
}

pub fn usage(runtime: &RuntimeManager) -> Result<StorageUsage> {
    let huggingface = runtime.models_root().join(HUGGINGFACE_DIR);
    let download_cache = runtime.root().join("runtime").join(".downloads");
    Ok(StorageUsage {
        models_bytes: directory_size(&huggingface, false)?,
        temporary_bytes: directory_size(&download_cache, true)?
            .checked_add(partial_files_size(&huggingface)?)
            .context("temporary storage size overflow")?,
        downloaded_local_models: ModelId::iter()
            .filter(|model| model.cached_path(runtime).is_some())
            .count(),
    })
}

pub fn remove_local_model(runtime: &RuntimeManager, model_id: &str) -> Result<u64> {
    ensure_model_library(runtime.models_root())?;
    let model = ModelId::from_str(model_id)
        .map_err(|_| anyhow::anyhow!("unknown local model id: {model_id}"))?;
    let repo = Repo::model(model.repository().to_string());
    let target = runtime
        .models_root()
        .join(HUGGINGFACE_DIR)
        .join(repo.folder_name());
    remove_managed_directory(&target)
}

pub fn clear_models(runtime: &RuntimeManager) -> Result<u64> {
    ensure_model_library(runtime.models_root())?;
    let target = runtime.models_root().join(HUGGINGFACE_DIR);
    let removed = remove_managed_directory(&target)?;
    fs::create_dir_all(&target)
        .with_context(|| format!("failed to recreate `{}`", target.display()))?;
    Ok(removed)
}

pub fn clear_temporary_cache(runtime: &RuntimeManager) -> Result<u64> {
    let download_cache = runtime.root().join("runtime").join(".downloads");
    let huggingface = runtime.models_root().join(HUGGINGFACE_DIR);
    let mut removed = remove_managed_directory(&download_cache)?;
    fs::create_dir_all(&download_cache)
        .with_context(|| format!("failed to recreate `{}`", download_cache.display()))?;

    if huggingface.exists() {
        for entry in WalkDir::new(&huggingface).follow_links(false) {
            let entry = entry.with_context(|| {
                format!("failed to inspect model cache `{}`", huggingface.display())
            })?;
            let is_partial = entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.ends_with(".part"));
            if is_partial && (entry.file_type().is_file() || entry.file_type().is_symlink()) {
                let size = entry.metadata().map(|metadata| metadata.len()).unwrap_or(0);
                fs::remove_file(entry.path())
                    .with_context(|| format!("failed to remove `{}`", entry.path().display()))?;
                removed = removed
                    .checked_add(size)
                    .context("removed storage size overflow")?;
            }
        }
    }
    Ok(removed)
}

/// Prepare a new model-library root. When `move_existing` is true, only the
/// Yomika-owned Hugging Face cache is copied; unrelated destination files are
/// preserved. The source remains intact until configuration persistence has
/// succeeded and [`finish_model_location`] is called.
pub fn prepare_model_location(
    runtime: &RuntimeManager,
    destination: &Path,
    move_existing: bool,
) -> Result<PreparedModelLocation> {
    ensure_model_library(runtime.models_root())?;
    ensure_safe_root(destination)?;
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create `{}`", destination.display()))?;
    ensure_not_symlink(destination)?;

    let source = fs::canonicalize(runtime.models_root()).with_context(|| {
        format!(
            "failed to resolve current model library `{}`",
            runtime.models_root().display()
        )
    })?;
    let destination = fs::canonicalize(destination).with_context(|| {
        format!(
            "failed to resolve destination model library `{}`",
            destination.display()
        )
    })?;
    if source == destination || source.starts_with(&destination) || destination.starts_with(&source)
    {
        bail!("model-library paths must be different and cannot contain one another");
    }

    let destination_huggingface = destination.join(HUGGINGFACE_DIR);
    ensure_not_symlink_if_present(&destination_huggingface)?;
    if !move_existing {
        fs::create_dir_all(&destination_huggingface)
            .with_context(|| format!("failed to create `{}`", destination_huggingface.display()))?;
        write_library_marker(&destination)?;
        return Ok(PreparedModelLocation {
            destination,
            copied_bytes: 0,
            source_huggingface: None,
        });
    }

    if destination_huggingface.exists()
        && fs::read_dir(&destination_huggingface)
            .with_context(|| format!("failed to inspect `{}`", destination_huggingface.display()))?
            .next()
            .is_some()
    {
        bail!("destination already contains a Hugging Face model cache");
    }

    let source_huggingface = source.join(HUGGINGFACE_DIR);
    if !source_huggingface.exists() {
        fs::create_dir_all(&destination_huggingface)
            .with_context(|| format!("failed to create `{}`", destination_huggingface.display()))?;
        write_library_marker(&destination)?;
        return Ok(PreparedModelLocation {
            destination,
            copied_bytes: 0,
            source_huggingface: None,
        });
    }
    ensure_not_symlink(&source_huggingface)?;

    let staging = destination.join(format!(".yomika-model-migration-{}", Uuid::new_v4()));
    let mut staging_guard = StagingGuard::new(staging.clone());
    let copied_bytes = copy_tree_materialized(&source_huggingface, &staging)?;
    let staged_bytes = directory_size(&staging, true)?;
    if copied_bytes != staged_bytes {
        bail!(
            "model migration validation failed: copied {copied_bytes} bytes but found {staged_bytes}"
        );
    }

    if destination_huggingface.exists() {
        fs::remove_dir(&destination_huggingface).with_context(|| {
            format!(
                "failed to replace empty destination `{}`",
                destination_huggingface.display()
            )
        })?;
    }
    fs::rename(&staging, &destination_huggingface).with_context(|| {
        format!(
            "failed to activate migrated model library `{}`",
            destination_huggingface.display()
        )
    })?;
    staging_guard.disarm();
    write_library_marker(&destination)?;

    Ok(PreparedModelLocation {
        destination,
        copied_bytes,
        source_huggingface: Some(source_huggingface),
    })
}

/// Remove the old managed cache after the new location is persisted. Failure
/// is safe: both copies remain, and the caller can report that cleanup did not
/// finish.
pub fn finish_model_location(prepared: &PreparedModelLocation) -> Result<bool> {
    let Some(source) = prepared.source_huggingface.as_deref() else {
        return Ok(true);
    };
    remove_managed_directory(source)?;
    fs::create_dir_all(source)
        .with_context(|| format!("failed to recreate `{}`", source.display()))?;
    Ok(true)
}

fn ensure_model_library(root: &Path) -> Result<()> {
    ensure_safe_root(root)?;
    ensure_not_symlink(root)?;
    let marker = root.join(MODEL_LIBRARY_MARKER);
    let metadata = fs::symlink_metadata(&marker).with_context(|| {
        format!(
            "model library marker is missing at `{}`; restart Yomika before maintenance",
            marker.display()
        )
    })?;
    if !metadata.file_type().is_file() {
        bail!("invalid model library marker `{}`", marker.display());
    }
    Ok(())
}

fn write_library_marker(root: &Path) -> Result<()> {
    ensure_model_library_marker(root)
}

fn ensure_safe_root(path: &Path) -> Result<()> {
    if !path.is_absolute() {
        bail!("model-library path must be absolute");
    }
    if path.parent().is_none() {
        bail!("a filesystem root cannot be used as the model library");
    }
    Ok(())
}

fn ensure_not_symlink(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("failed to inspect `{}`", path.display()))?;
    if metadata.file_type().is_symlink() {
        bail!(
            "refusing to manage symlinked directory `{}`",
            path.display()
        );
    }
    if !metadata.is_dir() {
        bail!("expected a directory at `{}`", path.display());
    }
    Ok(())
}

fn ensure_not_symlink_if_present(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            bail!(
                "refusing to manage symlinked directory `{}`",
                path.display()
            )
        }
        Ok(metadata) if !metadata.is_dir() => {
            bail!("expected a directory at `{}`", path.display())
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("failed to inspect `{}`", path.display())),
    }
}

fn remove_managed_directory(path: &Path) -> Result<u64> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            bail!(
                "refusing to remove symlinked directory `{}`",
                path.display()
            )
        }
        Ok(metadata) if !metadata.is_dir() => {
            bail!("refusing to remove non-directory `{}`", path.display())
        }
        Ok(_) => {
            let bytes = directory_size(path, true)?;
            fs::remove_dir_all(path)
                .with_context(|| format!("failed to remove `{}`", path.display()))?;
            Ok(bytes)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(0),
        Err(error) => Err(error).with_context(|| format!("failed to inspect `{}`", path.display())),
    }
}

fn directory_size(root: &Path, include_partials: bool) -> Result<u64> {
    if !root.exists() {
        return Ok(0);
    }
    let mut bytes = 0_u64;
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry.with_context(|| format!("failed to inspect `{}`", root.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        if !include_partials
            && entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.ends_with(".part"))
        {
            continue;
        }
        bytes = bytes
            .checked_add(entry.metadata()?.len())
            .context("storage size overflow")?;
    }
    Ok(bytes)
}

fn partial_files_size(root: &Path) -> Result<u64> {
    if !root.exists() {
        return Ok(0);
    }
    let mut bytes = 0_u64;
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry.with_context(|| format!("failed to inspect `{}`", root.display()))?;
        if entry.file_type().is_file()
            && entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.ends_with(".part"))
        {
            bytes = bytes
                .checked_add(entry.metadata()?.len())
                .context("temporary storage size overflow")?;
        }
    }
    Ok(bytes)
}

fn copy_tree_materialized(source: &Path, destination: &Path) -> Result<u64> {
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create `{}`", destination.display()))?;
    let source = fs::canonicalize(source)
        .with_context(|| format!("failed to resolve `{}`", source.display()))?;
    let mut copied_bytes = 0_u64;

    for entry in WalkDir::new(&source).follow_links(false).min_depth(1) {
        let entry = entry.with_context(|| format!("failed to inspect `{}`", source.display()))?;
        let relative = entry.path().strip_prefix(&source)?;
        let target = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)
                .with_context(|| format!("failed to create `{}`", target.display()))?;
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create `{}`", parent.display()))?;
        }

        let copy_source = if entry.file_type().is_symlink() {
            let resolved = fs::canonicalize(entry.path()).with_context(|| {
                format!("failed to resolve cache link `{}`", entry.path().display())
            })?;
            if !resolved.starts_with(&source) || !resolved.is_file() {
                bail!(
                    "refusing model-cache link outside the managed source: `{}`",
                    entry.path().display()
                );
            }
            let relocated = destination.join(resolved.strip_prefix(&source)?);
            if create_file_symlink(&relocated, &target).is_ok() {
                continue;
            }
            resolved
        } else if entry.file_type().is_file() {
            entry.path().to_path_buf()
        } else {
            bail!("unsupported cache entry `{}`", entry.path().display());
        };

        let copied = fs::copy(&copy_source, &target).with_context(|| {
            format!(
                "failed to copy `{}` to `{}`",
                copy_source.display(),
                target.display()
            )
        })?;
        copied_bytes = copied_bytes
            .checked_add(copied)
            .context("copied storage size overflow")?;
    }
    Ok(copied_bytes)
}

#[cfg(target_os = "windows")]
fn create_file_symlink(source: &Path, destination: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(source, destination)
}

#[cfg(target_family = "unix")]
fn create_file_symlink(source: &Path, destination: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(source, destination)
}

struct StagingGuard {
    path: PathBuf,
    armed: bool,
}

impl StagingGuard {
    fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for StagingGuard {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_tree_preserves_contents_and_validates_size() -> Result<()> {
        let source = tempfile::tempdir()?;
        let destination_root = tempfile::tempdir()?;
        fs::create_dir_all(source.path().join("nested"))?;
        fs::write(source.path().join("nested/model.bin"), b"model")?;
        let destination = destination_root.path().join("copy");

        let copied = copy_tree_materialized(source.path(), &destination)?;

        assert_eq!(copied, 5);
        assert_eq!(directory_size(&destination, true)?, 5);
        assert_eq!(fs::read(destination.join("nested/model.bin"))?, b"model");
        Ok(())
    }

    #[test]
    fn model_size_excludes_partial_files() -> Result<()> {
        let root = tempfile::tempdir()?;
        fs::write(root.path().join("ready.bin"), b"ready")?;
        fs::write(root.path().join("pending.bin.part"), b"partial")?;

        assert_eq!(directory_size(root.path(), false)?, 5);
        assert_eq!(partial_files_size(root.path())?, 7);
        Ok(())
    }
}
