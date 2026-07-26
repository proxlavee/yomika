//! Linear undo/redo history + append-only durable op log.
//!
//! Two concerns, deliberately separated:
//!   1. **Durability log** — `history.log`: each applied op fsynced before ack
//!      so a crash loses at most the op currently being written.
//!   2. **Undo/redo stacks** — in-memory only; Cmd+Z within a session.
//!
//! Undo/redo are themselves logged ops: when the user undoes, we apply the
//! inverse and append it to the log as a normal op. Replay on open always
//! produces the post-undo state. No special entry type.

use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use yomika_core::{Op, PageId, Scene};

/// Default cap for the in-memory undo stack. The log on disk is not capped —
/// it's compacted on snapshot.
const DEFAULT_UNDO_LIMIT: usize = 500;

// ---------------------------------------------------------------------------
// Log frames
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct LogFrame {
    epoch: u64,
    op: Op,
}

// ---------------------------------------------------------------------------
// History
// ---------------------------------------------------------------------------

pub struct History {
    log_path: PathBuf,
    log: BufWriter<File>,
    epoch: u64,
    undo_stack: VecDeque<Op>,
    redo_stack: Vec<Op>,
    limit: usize,
}

impl History {
    /// Open the log at `path`, creating it if missing. Caller is expected to
    /// have already replayed any existing frames (see `Self::replay`).
    pub fn open(path: impl Into<PathBuf>, epoch: u64) -> Result<Self> {
        let log_path = path.into();
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&log_path)
            .with_context(|| format!("open history log {}", log_path.display()))?;
        Ok(Self {
            log_path,
            log: BufWriter::new(file),
            epoch,
            undo_stack: VecDeque::new(),
            redo_stack: Vec::new(),
            limit: DEFAULT_UNDO_LIMIT,
        })
    }

    /// Override the in-memory undo-stack cap.
    pub fn with_undo_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    /// Apply an op to the scene, fsync a frame to disk, push to the undo stack.
    pub fn apply(&mut self, scene: &mut Scene, mut op: Op) -> Result<u64> {
        op.apply(scene).context("apply op to scene")?;
        self.epoch += 1;
        self.write_frame(&op)?;
        self.push_undo(op);
        self.redo_stack.clear();
        Ok(self.epoch)
    }

    /// Apply an auto-render only when it still belongs to the exact edit that
    /// scheduled it, then combine both into one in-memory undo entry.
    ///
    /// Rendering runs asynchronously. An undo, page switch, or newer edit may
    /// advance history while the renderer is working; applying that stale
    /// output would both overwrite newer pixels and create a misleading undo
    /// step. `None` means the expected edit is no longer current, so the
    /// caller must discard the render output.
    pub fn apply_auto_render(
        &mut self,
        scene: &mut Scene,
        mut op: Op,
        expected_epoch: u64,
        page: PageId,
    ) -> Result<Option<u64>> {
        let Some(previous) = self.undo_stack.back() else {
            return Ok(None);
        };
        if self.epoch != expected_epoch
            || !op_touches_page(previous, page)
            || !op_touches_page(&op, page)
        {
            return Ok(None);
        }

        op.apply(scene).context("apply auto-render op to scene")?;
        self.epoch += 1;
        self.write_frame(&op)?;

        let previous = self
            .undo_stack
            .pop_back()
            .expect("undo entry checked immediately above");
        let label = op_label(&previous);
        self.push_undo(Op::Batch {
            ops: vec![previous, op],
            label,
        });
        self.redo_stack.clear();
        Ok(Some(self.epoch))
    }

    /// Undo the most recent op. Applies its inverse, records the inverse in
    /// the log, and moves the original onto the redo stack. Returns the new
    /// epoch + the inverse op that was just applied (so the RPC layer can
    /// broadcast it for clients to patch their mirrors without refetching).
    pub fn undo(&mut self, scene: &mut Scene) -> Result<Option<(u64, Op)>> {
        let Some(original) = self.undo_stack.pop_back() else {
            return Ok(None);
        };
        let mut inverse = original.inverse();
        inverse.apply(scene).context("apply inverse op")?;
        self.epoch += 1;
        self.write_frame(&inverse)?;
        let inverse_out = inverse.clone();
        self.redo_stack.push(original);
        Ok(Some((self.epoch, inverse_out)))
    }

    /// Re-apply the most recent undo. Symmetric with `undo`. Returns the new
    /// epoch + the op that was just re-applied.
    pub fn redo(&mut self, scene: &mut Scene) -> Result<Option<(u64, Op)>> {
        let Some(mut op) = self.redo_stack.pop() else {
            return Ok(None);
        };
        op.apply(scene).context("re-apply op")?;
        self.epoch += 1;
        self.write_frame(&op)?;
        let applied = op.clone();
        self.push_undo(op);
        Ok(Some((self.epoch, applied)))
    }

    /// Truncate the log after a snapshot has been committed.
    /// Caller must have already fsynced the snapshot file.
    pub fn truncate_log(&mut self) -> Result<()> {
        self.log.flush()?;
        self.log.get_ref().sync_all()?;
        // Reopen to truncate; BufWriter's underlying file handle is append-only.
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(&self.log_path)
            .with_context(|| format!("truncate history log {}", self.log_path.display()))?;
        file.sync_all()?;
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&self.log_path)?;
        self.log = BufWriter::new(file);
        Ok(())
    }

    // --- internals ---------------------------------------------------------

    fn write_frame(&mut self, op: &Op) -> Result<()> {
        let frame = LogFrame {
            epoch: self.epoch,
            op: op.clone(),
        };
        let bytes = postcard::to_allocvec(&frame).context("encode log frame")?;
        let len = u32::try_from(bytes.len()).context("log frame too large")?;
        self.log.write_all(&len.to_le_bytes())?;
        self.log.write_all(&bytes)?;
        self.log.flush()?;
        self.log.get_ref().sync_data()?;
        Ok(())
    }

    fn push_undo(&mut self, op: Op) {
        self.undo_stack.push_back(op);
        while self.undo_stack.len() > self.limit {
            self.undo_stack.pop_front();
        }
    }
}

fn op_label(op: &Op) -> String {
    match op {
        Op::RemoveNode { .. } => "Delete block",
        Op::UpdateNode { .. } => "Update block",
        Op::AddNode { .. } => "Add block",
        Op::RemovePage { .. } => "Delete page",
        Op::AddPage { .. } => "Add page",
        Op::UpdatePage { .. } => "Update page",
        Op::ReorderPages { .. } => "Reorder pages",
        Op::ReorderNodes { .. } => "Reorder blocks",
        Op::UpdateProjectMeta { .. } => "Update project metadata",
        Op::Batch { label, .. } => label,
    }
    .to_string()
}

fn op_touches_page(op: &Op, page: PageId) -> bool {
    match op {
        Op::AddPage { page: added, .. } => added.id == page,
        Op::RemovePage { id, .. } | Op::UpdatePage { id, .. } => *id == page,
        Op::ReorderPages { order, .. } => order.contains(&page),
        Op::AddNode { page: target, .. }
        | Op::RemoveNode { page: target, .. }
        | Op::UpdateNode { page: target, .. }
        | Op::ReorderNodes { page: target, .. } => *target == page,
        Op::Batch { ops, .. } => ops.iter().any(|child| op_touches_page(child, page)),
        Op::UpdateProjectMeta { .. } => false,
    }
}

// ---------------------------------------------------------------------------
// Replay — called once on project open, before a `History` is constructed.
// ---------------------------------------------------------------------------

/// Replay each frame in `log_path` with epoch greater than `start_epoch`
/// against `scene`. Returns the final epoch seen.
pub fn replay(log_path: &Path, start_epoch: u64, scene: &mut Scene) -> Result<u64> {
    if !log_path.exists() {
        return Ok(start_epoch);
    }
    let file =
        File::open(log_path).with_context(|| format!("open history log {}", log_path.display()))?;
    let mut reader = BufReader::new(file);
    let mut epoch = start_epoch;
    loop {
        let mut len_buf = [0u8; 4];
        match reader.read_exact(&mut len_buf) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(anyhow::Error::new(e).context("read log frame length")),
        }
        let len = u32::from_le_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        match reader.read_exact(&mut buf) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                // Truncated frame (likely crash mid-write) — stop cleanly.
                tracing::warn!(
                    path = %log_path.display(),
                    expected_len = len,
                    "truncated trailing frame in history log; discarding"
                );
                break;
            }
            Err(e) => return Err(anyhow::Error::new(e).context("read log frame body")),
        }
        let frame: LogFrame = match postcard::from_bytes(&buf) {
            Ok(frame) => frame,
            Err(err) => {
                tracing::warn!(
                    path = %log_path.display(),
                    error = %err,
                    "undecodable frame in history log; stopping replay"
                );
                break;
            }
        };
        if frame.epoch > epoch {
            let mut op = frame.op;
            op.apply(scene).context("replay op")?;
            epoch = frame.epoch;
        }
    }
    // Seek to end so subsequent appends go after the last valid frame.
    let _ = reader.seek(SeekFrom::End(0));
    Ok(epoch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn make_temp_history() -> (History, std::path::PathBuf) {
        let mut temp_path = std::env::temp_dir();
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        temp_path.push(format!("test_history_{}.log", unique_id));
        let history = History::open(&temp_path, 0).unwrap();
        (history, temp_path)
    }

    #[test]
    fn ordinary_renderer_apply_remains_a_separate_undo_entry() {
        let (mut history, path) = make_temp_history();
        let mut scene = Scene::default();

        history
            .apply(
                &mut scene,
                Op::Batch {
                    ops: vec![],
                    label: "Update block".to_string(),
                },
            )
            .unwrap();
        history
            .apply(
                &mut scene,
                Op::Batch {
                    ops: vec![],
                    label: "yomika-renderer: manual run".to_string(),
                },
            )
            .unwrap();

        assert_eq!(history.undo_stack.len(), 2);
        drop(history);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn auto_render_rejects_stale_epoch_or_unrelated_page() {
        use yomika_core::{Page, PagePatch};

        let (mut history, path) = make_temp_history();
        let mut scene = Scene::default();
        let page_a = Page::new("a", 800, 1200);
        let page_a_id = page_a.id;
        let page_b = Page::new("b", 800, 1200);
        let page_b_id = page_b.id;
        Op::AddPage {
            page: page_a,
            at: 0,
        }
        .apply(&mut scene)
        .unwrap();
        Op::AddPage {
            page: page_b,
            at: 1,
        }
        .apply(&mut scene)
        .unwrap();

        let edit_epoch = history
            .apply(
                &mut scene,
                Op::UpdatePage {
                    id: page_a_id,
                    patch: PagePatch {
                        name: Some("edited".to_string()),
                        ..Default::default()
                    },
                    prev: PagePatch::default(),
                },
            )
            .unwrap();
        let render = Op::Batch {
            ops: vec![Op::UpdatePage {
                id: page_a_id,
                patch: PagePatch {
                    width: Some(900),
                    ..Default::default()
                },
                prev: PagePatch::default(),
            }],
            label: format!("yomika-renderer: page {page_a_id}"),
        };

        assert!(
            history
                .apply_auto_render(&mut scene, render.clone(), edit_epoch - 1, page_a_id)
                .unwrap()
                .is_none()
        );
        assert!(
            history
                .apply_auto_render(&mut scene, render, edit_epoch, page_b_id)
                .unwrap()
                .is_none()
        );
        assert_eq!(history.epoch(), edit_epoch);
        assert_eq!(scene.page(page_a_id).unwrap().width, 800);
        assert_eq!(history.undo_stack.len(), 1);

        drop(history);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_single_undo_reverts_edit_and_auto_render_together() {
        use yomika_core::{
            BlobRef, Node, NodeDataPatch, NodeId, NodeKind, NodePatch, Page, TextData,
            TextDataPatch, TextStyle, Transform,
        };

        let (mut history, path) = make_temp_history();
        let mut scene = Scene::default();

        // Setup: one page with one text node (applied directly, off-history).
        let page = Page::new("p1", 800, 1200);
        let page_id = page.id;
        Op::AddPage { page, at: 0 }.apply(&mut scene).unwrap();
        let node = Node {
            id: NodeId::new(),
            transform: Transform::default(),
            visible: true,
            kind: NodeKind::Text(TextData::default()),
        };
        let node_id = node.id;
        Op::AddNode {
            page: page_id,
            node,
            at: 0,
        }
        .apply(&mut scene)
        .unwrap();

        // 1. User edit: manual font size override, like the font-size box emits.
        let user_edit = Op::UpdateNode {
            page: page_id,
            id: node_id,
            patch: NodePatch {
                data: Some(NodeDataPatch::Text(TextDataPatch {
                    style: Some(Some(TextStyle {
                        font_size: Some(8.0),
                        ..Default::default()
                    })),
                    ..Default::default()
                })),
                ..Default::default()
            },
            prev: NodePatch::default(),
        };
        let edit_epoch = history.apply(&mut scene, user_edit).unwrap();

        // 2. The debounced auto-render lands right after (renderer engine op).
        let render_op = Op::Batch {
            ops: vec![Op::UpdateNode {
                page: page_id,
                id: node_id,
                patch: NodePatch {
                    data: Some(NodeDataPatch::Text(TextDataPatch {
                        sprite: Some(Some(BlobRef::new("rendered-v1"))),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
                prev: NodePatch::default(),
            }],
            label: format!("yomika-renderer: page {}", page_id),
        };
        assert!(
            history
                .apply_auto_render(&mut scene, render_op, edit_epoch, page_id)
                .unwrap()
                .is_some()
        );

        // The edit and its auto-render are a single undo entry…
        assert_eq!(history.undo_stack.len(), 1);
        let node = scene.node(page_id, node_id).unwrap();
        let NodeKind::Text(text) = &node.kind else {
            panic!("expected text node")
        };
        assert_eq!(text.style.as_ref().and_then(|s| s.font_size), Some(8.0));
        assert_eq!(text.sprite.as_ref().map(|b| b.hash()), Some("rendered-v1"));

        // …so ONE Ctrl+Z reverts both — no invisible render op left on top.
        let undone = history.undo(&mut scene).unwrap();
        assert!(undone.is_some());
        assert!(history.undo_stack.is_empty());

        let node = scene.node(page_id, node_id).unwrap();
        let NodeKind::Text(text) = &node.kind else {
            panic!("expected text node")
        };
        assert!(
            text.style.as_ref().and_then(|s| s.font_size).is_none(),
            "font-size override must be reverted by the same undo"
        );
        assert!(
            text.sprite.is_none(),
            "auto-render output must be reverted by the same undo"
        );

        drop(history);
        let _ = std::fs::remove_file(path);
    }
}
