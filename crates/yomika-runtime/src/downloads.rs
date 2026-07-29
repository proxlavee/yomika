use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use anyhow::{Context, Result};
use dashmap::{DashMap, mapref::entry::Entry};
use futures::stream::{self, StreamExt, TryStreamExt};
use hf_hub::{Cache, Repo, RepoType, api::tokio::ApiBuilder};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::header::{CONTENT_LENGTH, RANGE};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use yomika_core::events::{DownloadProgress, DownloadStatus};

use crate::runtime::{RuntimeHttpClient, RuntimeHttpConfig};

/// 10 MiB per ranged GET — same size hf-hub's `.high()` mode uses. Short enough
/// that reqwest's read_timeout catches a stalled connection quickly, and the
/// retry middleware can restart the chunk.
const CHUNK_SIZE: u64 = 10 * 1024 * 1024;

/// Avoid flooding the broadcast/SSE/UI pipeline when `bytes_stream` yields
/// small network frames. Terminal states are always emitted immediately.
const PROGRESS_EMIT_BYTES: u64 = 1024 * 1024;

/// hf-hub's internal client has no read timeout, so we cap the metadata call
/// ourselves. The response body is a single byte — a short cap is safe.
const HF_METADATA_TIMEOUT: Duration = Duration::from_secs(30);

// ---------------------------------------------------------------------------
// Downloads — unified download manager
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct Downloads {
    downloads_root: PathBuf,
    huggingface_cache: Cache,
    client: RuntimeHttpClient,
    tx: broadcast::Sender<DownloadProgress>,
    progress: Arc<MultiProgress>,
    active: Arc<DashMap<String, CancellationToken>>,
}

impl Downloads {
    pub(crate) fn new(
        downloads_root: PathBuf,
        huggingface_root: PathBuf,
        http: &RuntimeHttpConfig,
    ) -> Result<Self> {
        let client = http.build_client()?;

        Ok(Self {
            downloads_root,
            huggingface_cache: Cache::new(huggingface_root),
            client,
            tx: broadcast::channel(256).0,
            progress: Arc::new(MultiProgress::new()),
            active: Arc::new(DashMap::new()),
        })
    }

    pub fn client(&self) -> RuntimeHttpClient {
        Arc::clone(&self.client)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DownloadProgress> {
        self.tx.subscribe()
    }

    /// Request cancellation of an active transfer. The transfer owns final
    /// status emission and partial-file cleanup, so callers must not evict its
    /// registry row themselves.
    pub fn cancel(&self, id: &str) -> bool {
        let Some(token) = self.active.get(id) else {
            return false;
        };
        token.cancel();
        true
    }

    pub fn is_active(&self, id: &str) -> bool {
        self.active.contains_key(id)
    }

    pub fn has_active(&self) -> bool {
        !self.active.is_empty()
    }

    /// Download a HuggingFace model file, using the local cache first.
    ///
    /// hf-hub resolves URL + metadata + cache layout; the byte transfer runs
    /// on our retry-configured client so a stalled chunk is retried by the
    /// middleware instead of hanging the future.
    pub async fn huggingface_model(&self, repo: &str, filename: &str) -> Result<PathBuf> {
        self.huggingface_model_with_id(filename, repo, filename)
            .await
    }

    /// Download a Hugging Face model with a stable UI-facing operation id.
    pub async fn huggingface_model_with_id(
        &self,
        id: &str,
        repo: &str,
        filename: &str,
    ) -> Result<PathBuf> {
        let cache_repo = self
            .huggingface_cache
            .repo(Repo::new(repo.to_string(), RepoType::Model));

        if let Some(path) = cache_repo.get(filename) {
            return Ok(path);
        }

        let (cancellation, active_guard) = self.register(id)?;
        let reporter = self.begin(id, filename);
        reporter.start(None);

        let result: Result<PathBuf> = async {
            let api = ApiBuilder::from_cache(self.huggingface_cache.clone())
                .with_progress(false)
                .with_user_agent("yomika", env!("CARGO_PKG_VERSION"))
                .build()
                .context("failed to build HF Hub API")?;
            let repo_handle = api.model(repo.to_string());
            let url = repo_handle.url(filename);

            let metadata = tokio::select! {
                () = cancellation.cancelled() => return Err(cancelled_error()),
                result = tokio::time::timeout(HF_METADATA_TIMEOUT, api.metadata(&url)) => {
                    result
                        .map_err(|_| anyhow::anyhow!("HF metadata request timed out for `{repo}/{filename}`"))?
                        .with_context(|| format!("failed to fetch HF metadata for `{repo}/{filename}`"))?
                }
            };

            let blob_path = cache_repo.blob_path(metadata.etag());
            if let Some(parent) = blob_path.parent() {
                tokio::fs::create_dir_all(parent).await.with_context(|| {
                    format!("failed to create HF blob directory `{}`", parent.display())
                })?;
            }

            if !blob_path.exists() {
                self.ranged_download(
                    &url,
                    &blob_path,
                    &reporter,
                    Some(metadata.size() as u64),
                    &cancellation,
                )
                .await
                .with_context(|| format!("failed to download HF model file `{repo}/{filename}`"))?;
            }

            if cancellation.is_cancelled() {
                return Err(cancelled_error());
            }

            let pointer_dir = cache_repo.pointer_path(metadata.commit_hash());
            let pointer_path = pointer_dir.join(filename);
            if let Some(parent) = pointer_path.parent() {
                tokio::fs::create_dir_all(parent).await.ok();
            }
            if !pointer_path.exists() {
                #[cfg(target_os = "windows")]
                let link_result = std::os::windows::fs::symlink_file(&blob_path, &pointer_path);
                #[cfg(target_family = "unix")]
                let link_result = std::os::unix::fs::symlink(&blob_path, &pointer_path);
                if link_result.is_err() {
                    tokio::fs::rename(&blob_path, &pointer_path)
                        .await
                        .with_context(|| {
                            format!(
                                "failed to place HF cache pointer `{}`",
                                pointer_path.display()
                            )
                        })?;
                }
            }
            cache_repo
                .create_ref(metadata.commit_hash())
                .context("failed to create HF cache ref")?;

            Ok(if pointer_path.exists() {
                pointer_path
            } else {
                blob_path
            })
        }
        .await;

        drop(active_guard);
        match result {
            Ok(path) => {
                reporter.finish();
                Ok(path)
            }
            Err(error) if is_cancelled(&error) => {
                reporter.cancel();
                Err(error)
            }
            Err(error) => {
                reporter.fail(&error);
                Err(error)
            }
        }
    }

    /// Download a file to the downloads cache, returning the cached path.
    pub(crate) async fn cached_download(&self, url: &str, file_name: &str) -> Result<PathBuf> {
        let destination = self.downloads_root.join(file_name);
        if destination.exists() {
            return Ok(destination);
        }

        if let Some(parent) = destination.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .with_context(|| format!("failed to create `{}`", parent.display()))?;
        }

        let (cancellation, active_guard) = self.register(file_name)?;
        let reporter = self.begin(file_name, file_name);
        reporter.start(None);
        let result = self
            .ranged_download(url, &destination, &reporter, None, &cancellation)
            .await;
        drop(active_guard);
        match result {
            Ok(()) => reporter.finish(),
            Err(error) if is_cancelled(&error) => {
                reporter.cancel();
                return Err(error);
            }
            Err(error) => {
                reporter.fail(&error);
                return Err(error);
            }
        }
        Ok(destination)
    }

    /// Stream a URL to `destination` as a set of ranged GETs running up to
    /// `chunk_parallelism()` in flight (defaults to the host's CPU core count).
    /// The temp file is pre-allocated to the full size so each worker can
    /// seek-and-write its range independently. Transient failures surface as
    /// `Err`; the retry middleware on `self.client` retries at the request
    /// level, and when retries are exhausted the whole download fails cleanly.
    async fn ranged_download(
        &self,
        url: &str,
        destination: &Path,
        reporter: &TransferReporter,
        total_hint: Option<u64>,
        cancellation: &CancellationToken,
    ) -> Result<()> {
        let total = match total_hint {
            Some(t) => t,
            None => self.probe_content_length(url, cancellation).await?,
        };
        reporter.start(Some(total));

        if cancellation.is_cancelled() {
            return Err(cancelled_error());
        }

        let temp = part_path(destination)?;
        tokio::fs::remove_file(&temp).await.ok();
        let mut partial_guard = PartialFileGuard::new(temp.clone());
        {
            let file = tokio::fs::File::create(&temp)
                .await
                .with_context(|| format!("failed to create `{}`", temp.display()))?;
            file.set_len(total)
                .await
                .with_context(|| format!("failed to preallocate `{}`", temp.display()))?;
        }

        let mut chunks = Vec::new();
        let mut start: u64 = 0;
        while start < total {
            let stop = (start + CHUNK_SIZE).min(total) - 1;
            chunks.push((start, stop));
            start = stop + 1;
        }

        let temp_ref: &Path = &temp;
        let cancellation = cancellation.clone();
        let write_result: Result<()> = stream::iter(chunks)
            .map(|(start, stop)| {
                let cancellation = cancellation.clone();
                async move {
                    if cancellation.is_cancelled() {
                        return Err(cancelled_error());
                    }
                    let range = format!("bytes={start}-{stop}");
                    let response = tokio::select! {
                        () = cancellation.cancelled() => return Err(cancelled_error()),
                        result = self.client.get(url).header(RANGE, &range).send() => {
                            result
                                .with_context(|| format!("failed to fetch range {range} of `{url}`"))?
                                .error_for_status()
                                .with_context(|| format!("fetch failed for range {range} of `{url}`"))?
                        }
                    };
                    let mut file = tokio::fs::OpenOptions::new()
                        .write(true)
                        .open(temp_ref)
                        .await
                        .with_context(|| format!("failed to open `{}`", temp_ref.display()))?;
                    file.seek(std::io::SeekFrom::Start(start))
                        .await
                        .with_context(|| format!("failed to seek in `{}`", temp_ref.display()))?;
                    let mut received = 0_u64;
                    let mut body = response.bytes_stream();
                    loop {
                        let next = tokio::select! {
                            () = cancellation.cancelled() => return Err(cancelled_error()),
                            next = body.next() => next,
                        };
                        let Some(chunk) = next else {
                            break;
                        };
                        let chunk = chunk
                            .with_context(|| format!("failed to read range {range} of `{url}`"))?;
                        file.write_all(&chunk)
                            .await
                            .with_context(|| format!("failed to write `{}`", temp_ref.display()))?;
                        received += chunk.len() as u64;
                        reporter.advance(chunk.len());
                    }
                    file.flush()
                        .await
                        .with_context(|| format!("failed to flush `{}`", temp_ref.display()))?;
                    let expected = stop - start + 1;
                    if received != expected {
                        anyhow::bail!(
                            "range {range} returned {received} bytes instead of {expected} for `{url}`"
                        );
                    }
                    Ok::<_, anyhow::Error>(())
                }
            })
            .buffer_unordered(num_cpus::get())
            .try_collect()
            .await;

        if let Err(err) = write_result {
            tokio::fs::remove_file(&temp).await.ok();
            return Err(err);
        }

        if cancellation.is_cancelled() {
            tokio::fs::remove_file(&temp).await.ok();
            return Err(cancelled_error());
        }

        tokio::fs::remove_file(destination).await.ok();
        tokio::fs::rename(&temp, destination)
            .await
            .with_context(|| {
                format!(
                    "failed to rename `{}` → `{}`",
                    temp.display(),
                    destination.display()
                )
            })?;
        partial_guard.disarm();
        Ok(())
    }

    async fn probe_content_length(
        &self,
        url: &str,
        cancellation: &CancellationToken,
    ) -> Result<u64> {
        let response = tokio::select! {
            () = cancellation.cancelled() => return Err(cancelled_error()),
            result = self.client.head(url).send() => {
                result
                    .with_context(|| format!("failed to HEAD `{url}`"))?
                    .error_for_status()
                    .with_context(|| format!("HEAD failed for `{url}`"))?
            }
        };

        let content_length = response
            .headers()
            .get(CONTENT_LENGTH)
            .ok_or_else(|| anyhow::anyhow!("missing Content-Length for `{url}`"))?
            .to_str()
            .context("invalid Content-Length header")?;
        content_length
            .trim()
            .parse::<u64>()
            .with_context(|| format!("invalid Content-Length `{content_length}` for `{url}`"))
    }

    fn begin(&self, id: &str, label: &str) -> TransferReporter {
        let bar = self.progress.add(ProgressBar::new_spinner());
        bar.enable_steady_tick(Duration::from_millis(120));
        bar.set_style(
            ProgressStyle::with_template(
                "{msg} [{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} ({eta})",
            )
            .expect("progress style"),
        );
        bar.set_message(label.to_string());
        TransferReporter::new(self.tx.clone(), bar, id, label)
    }

    fn register(&self, id: &str) -> Result<(CancellationToken, ActiveDownloadGuard)> {
        let id = id.to_string();
        let token = CancellationToken::new();
        match self.active.entry(id.clone()) {
            Entry::Occupied(_) => anyhow::bail!("download `{id}` is already running"),
            Entry::Vacant(entry) => {
                entry.insert(token.clone());
            }
        }
        Ok((
            token,
            ActiveDownloadGuard {
                id,
                active: Arc::clone(&self.active),
            },
        ))
    }
}

struct ActiveDownloadGuard {
    id: String,
    active: Arc<DashMap<String, CancellationToken>>,
}

impl Drop for ActiveDownloadGuard {
    fn drop(&mut self) {
        self.active.remove(&self.id);
    }
}

struct PartialFileGuard {
    path: PathBuf,
    armed: bool,
}

impl PartialFileGuard {
    fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for PartialFileGuard {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

#[derive(Debug)]
struct DownloadCancelled;

impl std::fmt::Display for DownloadCancelled {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("download cancelled")
    }
}

impl std::error::Error for DownloadCancelled {}

fn cancelled_error() -> anyhow::Error {
    DownloadCancelled.into()
}

fn is_cancelled(error: &anyhow::Error) -> bool {
    error
        .chain()
        .any(|source| source.downcast_ref::<DownloadCancelled>().is_some())
}

// ---------------------------------------------------------------------------
// Transfer progress reporter
// ---------------------------------------------------------------------------

const UNKNOWN_TOTAL: u64 = u64::MAX;

#[derive(Clone)]
struct TransferReporter {
    tx: broadcast::Sender<DownloadProgress>,
    bar: ProgressBar,
    id: Arc<str>,
    filename: Arc<str>,
    downloaded: Arc<AtomicU64>,
    last_emitted: Arc<AtomicU64>,
    total: Arc<AtomicU64>,
}

impl TransferReporter {
    fn new(
        tx: broadcast::Sender<DownloadProgress>,
        bar: ProgressBar,
        id: &str,
        label: &str,
    ) -> Self {
        Self {
            tx,
            bar,
            id: Arc::<str>::from(id),
            filename: Arc::<str>::from(label),
            downloaded: Arc::new(AtomicU64::new(0)),
            last_emitted: Arc::new(AtomicU64::new(0)),
            total: Arc::new(AtomicU64::new(UNKNOWN_TOTAL)),
        }
    }

    fn start(&self, total: Option<u64>) {
        self.total
            .store(total.unwrap_or(UNKNOWN_TOTAL), Ordering::Relaxed);
        self.downloaded.store(0, Ordering::Relaxed);
        self.last_emitted.store(0, Ordering::Relaxed);
        self.bar.set_length(total.unwrap_or(0));
        self.bar.set_position(0);
        self.emit(DownloadStatus::Started);
    }

    fn advance(&self, delta: usize) {
        let downloaded = self.downloaded.fetch_add(delta as u64, Ordering::Relaxed) + delta as u64;
        self.bar.inc(delta as u64);
        let last_emitted = self.last_emitted.load(Ordering::Relaxed);
        if downloaded.saturating_sub(last_emitted) >= PROGRESS_EMIT_BYTES
            && self
                .last_emitted
                .compare_exchange(
                    last_emitted,
                    downloaded,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
        {
            self.emit(DownloadStatus::Downloading);
        }
    }

    fn finish(&self) {
        self.bar.finish_and_clear();
        self.emit(DownloadStatus::Completed);
    }

    fn cancel(&self) {
        self.bar.finish_and_clear();
        self.emit(DownloadStatus::Cancelled);
    }

    fn fail(&self, error: &anyhow::Error) {
        self.bar.finish_and_clear();
        self.emit(DownloadStatus::Failed {
            reason: error.to_string(),
        });
    }

    fn emit(&self, status: DownloadStatus) {
        let total = self.total.load(Ordering::Relaxed);
        let _ = self.tx.send(DownloadProgress {
            id: self.id.to_string(),
            filename: self.filename.to_string(),
            downloaded: self.downloaded.load(Ordering::Relaxed),
            total: (total != UNKNOWN_TOTAL).then_some(total),
            status,
        });
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn part_path(destination: &Path) -> Result<PathBuf> {
    let file_name = destination.file_name().ok_or_else(|| {
        anyhow::anyhow!(
            "destination `{}` does not have a filename",
            destination.display()
        )
    })?;
    Ok(destination.with_file_name(format!("{}.part", file_name.to_string_lossy())))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use indicatif::ProgressBar;
    use tokio::sync::broadcast::error::TryRecvError;
    use yomika_core::DownloadStatus;

    use super::{PROGRESS_EMIT_BYTES, TransferReporter, part_path};

    #[test]
    fn partial_download_path_appends_suffix() {
        let part = part_path(Path::new("/tmp/models/config.json")).unwrap();
        assert_eq!(part, Path::new("/tmp/models/config.json.part"));
    }

    #[test]
    fn progress_events_are_throttled_but_terminal_states_are_immediate() {
        let (tx, mut rx) = tokio::sync::broadcast::channel(8);
        let reporter = TransferReporter::new(tx, ProgressBar::hidden(), "model", "model.gguf");
        reporter.start(Some(PROGRESS_EMIT_BYTES * 2));
        assert_eq!(rx.try_recv().unwrap().status, DownloadStatus::Started);

        reporter.advance((PROGRESS_EMIT_BYTES / 2) as usize);
        assert!(matches!(rx.try_recv(), Err(TryRecvError::Empty)));

        reporter.advance((PROGRESS_EMIT_BYTES / 2) as usize);
        assert_eq!(rx.try_recv().unwrap().status, DownloadStatus::Downloading);

        reporter.finish();
        assert_eq!(rx.try_recv().unwrap().status, DownloadStatus::Completed);
    }
}
