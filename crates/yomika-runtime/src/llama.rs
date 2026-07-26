use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::Runtime;
use crate::archive::{self, ExtractPolicy};
use crate::install::InstallState;
use crate::loader::{add_runtime_search_path, preload_library};

const LLAMA_CPP_TAG: &str = env!("LLAMA_CPP_TAG");
const RELEASE_BASE_URL: &str = "https://github.com/ggml-org/llama.cpp/releases/download";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum LlamaDistribution {
    WindowsCuda13X64,
    WindowsVulkanX64,
    LinuxVulkanX64,
    LinuxVulkanArm64,
    MacosArm64,
}

impl LlamaDistribution {
    #[allow(clippy::needless_return)]
    fn detect(_runtime: &Runtime) -> Result<Self> {
        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        return Ok(Self::windows_x64(_runtime));

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        return Ok(Self::LinuxVulkanX64);

        #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
        return Ok(Self::LinuxVulkanArm64);

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        return Ok(Self::MacosArm64);

        #[cfg(not(any(
            all(target_os = "windows", target_arch = "x86_64"),
            all(target_os = "linux", target_arch = "x86_64"),
            all(target_os = "linux", target_arch = "aarch64"),
            all(target_os = "macos", target_arch = "aarch64")
        )))]
        bail!(
            "unsupported platform: os={}, arch={}",
            std::env::consts::OS,
            std::env::consts::ARCH
        )
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    fn windows_x64(runtime: &Runtime) -> Self {
        if crate::zluda::package_enabled(runtime) {
            Self::WindowsVulkanX64
        } else if crate::cuda::llama_cuda_enabled(runtime) {
            Self::WindowsCuda13X64
        } else {
            Self::WindowsVulkanX64
        }
    }

    fn id(self) -> &'static str {
        match self {
            Self::WindowsCuda13X64 => "windows-cuda13-x64",
            Self::WindowsVulkanX64 => "windows-vulkan-x64",
            Self::LinuxVulkanX64 => "linux-vulkan-x64",
            Self::LinuxVulkanArm64 => "linux-vulkan-arm64",
            Self::MacosArm64 => "macos-arm64",
        }
    }

    fn assets(self) -> Vec<String> {
        let tag = LLAMA_CPP_TAG;
        match self {
            Self::WindowsCuda13X64 => vec![
                format!("llama-{tag}-bin-win-cuda-13.1-x64.zip"),
                "cudart-llama-bin-win-cuda-13.1-x64.zip".to_string(),
            ],
            Self::WindowsVulkanX64 => vec![format!("llama-{tag}-bin-win-vulkan-x64.zip")],
            Self::LinuxVulkanX64 => vec![format!("llama-{tag}-bin-ubuntu-vulkan-x64.tar.gz")],
            Self::LinuxVulkanArm64 => vec![format!("llama-{tag}-bin-ubuntu-vulkan-arm64.tar.gz")],
            Self::MacosArm64 => vec![format!("llama-{tag}-bin-macos-arm64.tar.gz")],
        }
    }

    fn libraries(self) -> &'static [&'static str] {
        match self {
            Self::WindowsCuda13X64 => &[
                "cudart64_13.dll",
                "cublasLt64_13.dll",
                "cublas64_13.dll",
                "libomp140.x86_64.dll",
                "ggml-base.dll",
                "ggml.dll",
                "ggml-cpu-alderlake.dll",
                "ggml-cpu-cannonlake.dll",
                "ggml-cpu-cascadelake.dll",
                "ggml-cpu-cooperlake.dll",
                "ggml-cpu-haswell.dll",
                "ggml-cpu-icelake.dll",
                "ggml-cpu-ivybridge.dll",
                "ggml-cpu-piledriver.dll",
                "ggml-cpu-sandybridge.dll",
                "ggml-cpu-sapphirerapids.dll",
                "ggml-cpu-skylakex.dll",
                "ggml-cpu-sse42.dll",
                "ggml-cpu-x64.dll",
                "ggml-cpu-zen4.dll",
                "ggml-cuda.dll",
                "ggml-rpc.dll",
                "llama.dll",
                "mtmd.dll",
            ],
            Self::WindowsVulkanX64 => &[
                "libomp140.x86_64.dll",
                "ggml-base.dll",
                "ggml.dll",
                "ggml-cpu-alderlake.dll",
                "ggml-cpu-cannonlake.dll",
                "ggml-cpu-cascadelake.dll",
                "ggml-cpu-cooperlake.dll",
                "ggml-cpu-haswell.dll",
                "ggml-cpu-icelake.dll",
                "ggml-cpu-ivybridge.dll",
                "ggml-cpu-piledriver.dll",
                "ggml-cpu-sandybridge.dll",
                "ggml-cpu-sapphirerapids.dll",
                "ggml-cpu-skylakex.dll",
                "ggml-cpu-sse42.dll",
                "ggml-cpu-x64.dll",
                "ggml-cpu-zen4.dll",
                "ggml-rpc.dll",
                "ggml-vulkan.dll",
                "llama.dll",
                "mtmd.dll",
            ],
            Self::LinuxVulkanX64 => &[
                "libggml-base.so",
                "libggml.so",
                "libggml-cpu-alderlake.so",
                "libggml-cpu-cannonlake.so",
                "libggml-cpu-cascadelake.so",
                "libggml-cpu-cooperlake.so",
                "libggml-cpu-haswell.so",
                "libggml-cpu-icelake.so",
                "libggml-cpu-ivybridge.so",
                "libggml-cpu-piledriver.so",
                "libggml-cpu-sandybridge.so",
                "libggml-cpu-sapphirerapids.so",
                "libggml-cpu-skylakex.so",
                "libggml-cpu-sse42.so",
                "libggml-cpu-x64.so",
                "libggml-cpu-zen4.so",
                "libggml-rpc.so",
                "libggml-vulkan.so",
                "libllama.so",
                "libmtmd.so",
            ],
            Self::LinuxVulkanArm64 => &[
                "libggml-base.so",
                "libggml.so",
                "libggml-cpu-armv8.0_1.so",
                "libggml-cpu-armv8.2_1.so",
                "libggml-cpu-armv8.2_2.so",
                "libggml-cpu-armv8.2_3.so",
                "libggml-cpu-armv8.6_1.so",
                "libggml-cpu-armv8.6_2.so",
                "libggml-cpu-armv9.2_1.so",
                "libggml-cpu-armv9.2_2.so",
                "libggml-rpc.so",
                "libggml-vulkan.so",
                "libllama.so",
                "libmtmd.so",
            ],
            Self::MacosArm64 => &[
                "libggml-base.dylib",
                "libggml.dylib",
                "libggml-blas.dylib",
                "libggml-cpu.dylib",
                "libggml-metal.dylib",
                "libggml-rpc.dylib",
                "libllama.dylib",
                "libmtmd.dylib",
            ],
        }
    }

    fn install_dir(self, runtime: &Runtime) -> PathBuf {
        runtime
            .root()
            .join("runtime")
            .join("llama.cpp")
            .join(LLAMA_CPP_TAG)
            .join(self.id())
    }

    fn source_id(self) -> String {
        format!("llama-{LLAMA_CPP_TAG}-{}", self.id())
    }
}

/// GPU backend libraries that ggml registers dynamically at runtime.
///
/// Preloading them is optional: omitting one merely leaves that backend
/// unregistered while the CPU backends carry inference. In CPU-only mode
/// they must not be touched at all — on machines whose GPU driver lacks the
/// required entry points, `LoadLibraryExW`/`dlopen` on these fails and used
/// to crash the whole app even with `--cpu`.
fn is_gpu_backend_library(library: &str) -> bool {
    const GPU_TAGS: [&str; 4] = ["vulkan", "cuda", "cublas", "metal"];
    GPU_TAGS.iter().any(|tag| library.contains(tag))
}

fn required_libraries(
    distribution: LlamaDistribution,
    wants_gpu: bool,
) -> impl Iterator<Item = &'static str> {
    distribution
        .libraries()
        .iter()
        .copied()
        .filter(move |library| wants_gpu || !is_gpu_backend_library(library))
}

fn required_libraries_present(
    distribution: LlamaDistribution,
    install_dir: &std::path::Path,
    wants_gpu: bool,
) -> bool {
    required_libraries(distribution, wants_gpu).all(|library| install_dir.join(library).exists())
}

pub(crate) fn package_enabled(runtime: &Runtime) -> bool {
    LlamaDistribution::detect(runtime).is_ok()
}

pub(crate) fn package_present(runtime: &Runtime) -> Result<bool> {
    let distribution = LlamaDistribution::detect(runtime)?;
    let install_dir = distribution.install_dir(runtime);
    let source_id = distribution.source_id();
    let install = InstallState::new(&install_dir, &source_id);
    if !install.is_current() {
        return Ok(false);
    }

    Ok(required_libraries_present(
        distribution,
        &install_dir,
        runtime.wants_gpu(),
    ))
}

pub(crate) async fn package_prepare(runtime: &Runtime) -> Result<()> {
    ensure_ready(runtime).await
}

pub(crate) async fn ensure_ready(runtime: &Runtime) -> Result<()> {
    let distribution = LlamaDistribution::detect(runtime)?;
    let install_dir = distribution.install_dir(runtime);
    let source_id = distribution.source_id();
    let install = InstallState::new(&install_dir, &source_id);
    let wants_gpu = runtime.wants_gpu();

    // A CPU-only run deliberately tolerates absent GPU plugins. If the user
    // later switches GPU mode back on, the current install marker alone is
    // not enough: repair the package before attempting to preload the newly
    // required backend.
    if !install.is_current() || !required_libraries_present(distribution, &install_dir, wants_gpu) {
        install.reset()?;

        for asset in &distribution.assets() {
            let url = format!("{RELEASE_BASE_URL}/{LLAMA_CPP_TAG}/{asset}");
            let archive = runtime
                .downloads()
                .cached_download(&url, asset)
                .await
                .with_context(|| format!("failed to download `{url}`"))?;
            let kind = archive::detect_kind(asset)?;
            archive::extract(
                &archive,
                &install_dir,
                kind,
                ExtractPolicy::RuntimeLibraries,
            )?;
        }

        // GPU backend libraries are optional in CPU-only mode: users with
        // incompatible drivers may delete them to work around load failures.
        for library in required_libraries(distribution, wants_gpu) {
            if !install_dir.join(library).exists() {
                bail!(
                    "required library `{library}` missing from `{}`",
                    install_dir.display()
                );
            }
        }

        install.commit()?;
    }

    let load_dir = if wants_gpu {
        install_dir.clone()
    } else {
        cpu_runtime_view(distribution, &install_dir)?
    };
    add_runtime_search_path(&load_dir)?;
    for library in required_libraries(distribution, wants_gpu) {
        // GPU mode is explicit. Surface a broken requested backend instead
        // of silently running a large model on the CPU; CPU-only mode never
        // reaches GPU libraries in the first place.
        preload_library(&load_dir.join(library))?;
    }

    Ok(())
}

fn cpu_runtime_view(distribution: LlamaDistribution, install_dir: &Path) -> Result<PathBuf> {
    let view_dir = install_dir.join("cpu-only-v1");
    fs::create_dir_all(&view_dir)
        .with_context(|| format!("failed to create `{}`", view_dir.display()))?;

    for library in required_libraries(distribution, false) {
        let source = install_dir.join(library);
        let destination = view_dir.join(library);
        if destination.exists() {
            continue;
        }

        if let Err(link_err) = fs::hard_link(&source, &destination) {
            fs::copy(&source, &destination).with_context(|| {
                format!(
                    "failed to create CPU-only runtime library `{}` after hard-link error: {link_err}",
                    destination.display()
                )
            })?;
        }
    }

    Ok(view_dir)
}

pub(crate) fn runtime_dir(runtime: &Runtime) -> Result<PathBuf> {
    let distribution = LlamaDistribution::detect(runtime)?;
    let install_dir = distribution.install_dir(runtime);
    if runtime.wants_gpu() {
        Ok(install_dir)
    } else {
        // llama.cpp's backend registry scans every plugin in the directory it
        // receives. Point CPU-only initialization at a filtered hard-link
        // view so GPU DLLs/shared objects cannot be probed indirectly.
        cpu_runtime_view(distribution, &install_dir)
    }
}

crate::declare_native_package!(
    id: "runtime:llama",
    bootstrap: true,
    order: 20,
    enabled: crate::llama::package_enabled,
    present: crate::llama::package_present,
    prepare: crate::llama::package_prepare,
);

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    fn touch(path: &Path) {
        fs::write(path, b"ok").unwrap();
    }

    #[test]
    fn detect_returns_a_variant_for_current_platform() {
        let runtime = Runtime::new("/tmp/yomika-runtime", crate::ComputePolicy::PreferGpu).unwrap();
        let distribution = LlamaDistribution::detect(&runtime).unwrap();
        assert!(!distribution.id().is_empty());
        assert!(!distribution.assets().is_empty());
        assert!(!distribution.libraries().is_empty());
    }

    #[test]
    fn install_dir_includes_tag_and_id() {
        let runtime = Runtime::new("/tmp/yomika-runtime", crate::ComputePolicy::CpuOnly).unwrap();
        let dir = LlamaDistribution::WindowsVulkanX64.install_dir(&runtime);
        assert!(
            dir.ends_with(
                std::path::Path::new("llama.cpp")
                    .join(LLAMA_CPP_TAG)
                    .join("windows-vulkan-x64")
            )
        );
    }

    #[test]
    fn preload_order_matches_libraries() {
        let tempdir = tempfile::tempdir().unwrap();
        let root = tempdir.path();
        let runtime = LlamaDistribution::WindowsCuda13X64;

        for library in runtime.libraries() {
            touch(&root.join(library));
        }

        let paths: Vec<PathBuf> = runtime
            .libraries()
            .iter()
            .map(|library| root.join(library))
            .collect();
        assert!(paths.iter().all(|path| path.exists()));
        assert_eq!(paths.len(), runtime.libraries().len());
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    #[test]
    fn windows_runtime_prefers_vulkan_when_zluda_is_enabled() {
        let runtime = Runtime::new("/tmp/yomika-runtime", crate::ComputePolicy::PreferGpu).unwrap();
        if crate::zluda::package_enabled(&runtime) {
            assert_eq!(
                LlamaDistribution::detect(&runtime).unwrap(),
                LlamaDistribution::WindowsVulkanX64
            );
        }
    }

    #[test]
    fn gpu_backend_libraries_cover_every_accelerated_preload() {
        // Every Vulkan/CUDA/Metal artifact across distributions must be
        // recognized, so CPU-only mode never attempts to load them.
        for lib in [
            "ggml-vulkan.dll",
            "libggml-vulkan.so",
            "ggml-cuda.dll",
            "cudart64_13.dll",
            "cublas64_13.dll",
            "cublasLt64_13.dll",
            "libggml-metal.dylib",
        ] {
            assert!(
                is_gpu_backend_library(lib),
                "{lib} must be treated as a GPU backend"
            );
        }
    }

    #[test]
    fn cpu_and_core_libraries_are_never_treated_as_gpu() {
        for lib in [
            "ggml-base.dll",
            "ggml.dll",
            "ggml-cpu-x64.dll",
            "ggml-cpu-haswell.dll",
            "libggml-base.so",
            "libggml.so",
            "libggml-cpu-zen4.so",
            "libggml-cpu-armv8.6_1.so",
            "libggml.dylib",
            "libggml-cpu.dylib",
            "libggml-blas.dylib",
            "ggml-rpc.dll",
            "libggml-rpc.so",
            "libggml-rpc.dylib",
            "llama.dll",
            "libllama.so",
            "libllama.dylib",
            "mtmd.dll",
            "libmtmd.so",
            "libmtmd.dylib",
            "libomp140.x86_64.dll",
        ] {
            assert!(
                !is_gpu_backend_library(lib),
                "{lib} must stay preloaded in CPU-only mode"
            );
        }
    }

    #[test]
    fn every_distribution_library_is_classified() {
        // Guard against future backend libraries slipping past the CPU-only
        // filter: every preloaded file must be either a GPU backend or a
        // recognized core/CPU library.
        let distributions = [
            LlamaDistribution::WindowsCuda13X64,
            LlamaDistribution::WindowsVulkanX64,
            LlamaDistribution::LinuxVulkanX64,
            LlamaDistribution::LinuxVulkanArm64,
            LlamaDistribution::MacosArm64,
        ];
        const CORE_PATTERNS: [&str; 9] = [
            "cpu", "base", "ggml.", "llama", "mtmd", "rpc", "blas", "libomp", "ggml",
        ];
        for dist in distributions {
            for lib in dist.libraries() {
                assert!(
                    is_gpu_backend_library(lib)
                        || CORE_PATTERNS.iter().any(|pat| lib.contains(pat)),
                    "unclassified library `{lib}` in {}: extend is_gpu_backend_library or the core list",
                    dist.id()
                );
            }
        }
    }

    #[test]
    fn cpu_only_mode_keeps_cpu_backends_for_every_distribution() {
        // The CPU-only preload set must always retain a ggml CPU backend and
        // the core ggml/llama libraries.
        let distributions = [
            LlamaDistribution::WindowsCuda13X64,
            LlamaDistribution::WindowsVulkanX64,
            LlamaDistribution::LinuxVulkanX64,
            LlamaDistribution::LinuxVulkanArm64,
            LlamaDistribution::MacosArm64,
        ];
        for dist in distributions {
            let kept: Vec<_> = dist
                .libraries()
                .iter()
                .filter(|lib| !is_gpu_backend_library(lib))
                .collect();
            assert!(
                kept.iter().any(|lib| lib.contains("cpu")),
                "{}: CPU-only mode would keep no CPU backend",
                dist.id()
            );
            assert!(
                kept.iter().any(|lib| lib.contains("llama")),
                "{}: CPU-only mode would keep no llama library",
                dist.id()
            );
        }
    }

    #[test]
    fn required_library_set_tracks_compute_mode() {
        let distribution = LlamaDistribution::WindowsVulkanX64;
        let cpu_only: Vec<_> = required_libraries(distribution, false).collect();
        let gpu: Vec<_> = required_libraries(distribution, true).collect();

        assert!(!cpu_only.contains(&"ggml-vulkan.dll"));
        assert!(cpu_only.contains(&"ggml-cpu-x64.dll"));
        assert!(gpu.contains(&"ggml-vulkan.dll"));
        assert_eq!(gpu.as_slice(), distribution.libraries());
    }

    #[test]
    fn missing_gpu_plugin_only_invalidates_gpu_mode() {
        let tempdir = tempfile::tempdir().unwrap();
        let distribution = LlamaDistribution::WindowsVulkanX64;
        for library in distribution.libraries() {
            if *library != "ggml-vulkan.dll" {
                touch(&tempdir.path().join(library));
            }
        }

        assert!(required_libraries_present(
            distribution,
            tempdir.path(),
            false
        ));
        assert!(!required_libraries_present(
            distribution,
            tempdir.path(),
            true
        ));
    }

    #[test]
    fn cpu_runtime_view_excludes_gpu_backends() {
        let tempdir = tempfile::tempdir().unwrap();
        let distribution = LlamaDistribution::WindowsVulkanX64;
        for library in distribution.libraries() {
            touch(&tempdir.path().join(library));
        }

        let view = cpu_runtime_view(distribution, tempdir.path()).unwrap();
        assert!(view.join("ggml-cpu-x64.dll").exists());
        assert!(view.join("ggml.dll").exists());
        assert!(!view.join("ggml-vulkan.dll").exists());
        assert!(fs::read_dir(view).unwrap().all(|entry| {
            let name = entry.unwrap().file_name();
            !is_gpu_backend_library(&name.to_string_lossy())
        }));
    }
}
