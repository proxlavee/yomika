//! `.ymk` archive = zip of `.ymkproj/` minus `cache/` and `.lock`.
//!
//! Blobs are already compressed (webp/jpg/webp-sprite), so they go in as
//! `Stored`. Text/metadata files (`project.toml`, `scene.bin`, `history.log`)
//! use `Deflated`.

use std::fs::File;
use std::io::{Cursor, Seek, Write};

use anyhow::{Context, Result};
use atomicwrites::{AtomicFile, OverwriteBehavior};
use camino::{Utf8Path, Utf8PathBuf};
use walkdir::WalkDir;
use zip::{CompressionMethod, ZipArchive, ZipWriter, write::SimpleFileOptions};

const SKIP_DIRS: &[&str] = &["cache", ".lock"];

/// Pack `project_dir` (`.ymkproj/`) into `out_ymk` as a `.ymk` archive.
pub fn export_ymk(project_dir: &Utf8Path, out_ymk: &Utf8Path) -> Result<()> {
    let project_dir_std = project_dir.as_std_path().to_path_buf();
    let out_std = out_ymk.as_std_path().to_path_buf();

    AtomicFile::new(out_std, OverwriteBehavior::AllowOverwrite)
        .write(move |f| -> Result<()> {
            write_ymk_zip(&project_dir_std, f)?;
            Ok(())
        })
        .map_err(|e| match e {
            atomicwrites::Error::Internal(io) => anyhow::Error::new(io),
            atomicwrites::Error::User(e) => e,
        })?;
    Ok(())
}

/// Pack `project_dir` into an in-memory `.ymk` zip. Used by the HTTP export
/// route that streams bytes to the client instead of writing to disk.
pub fn export_ymk_bytes(project_dir: &Utf8Path) -> Result<Vec<u8>> {
    let project_dir_std = project_dir.as_std_path().to_path_buf();
    let mut cursor = Cursor::new(Vec::new());
    write_ymk_zip(&project_dir_std, &mut cursor)?;
    Ok(cursor.into_inner())
}

fn write_ymk_zip<W: Write + Seek>(project_dir_std: &std::path::Path, w: W) -> Result<()> {
    let mut zip = ZipWriter::new(w);
    for entry in WalkDir::new(project_dir_std)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path == project_dir_std {
            continue;
        }
        let rel = path
            .strip_prefix(project_dir_std)
            .expect("walkdir starts at root")
            .to_path_buf();
        if should_skip(&rel) {
            continue;
        }
        let rel_str = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        if entry.file_type().is_dir() {
            zip.add_directory(&rel_str, SimpleFileOptions::default())?;
            continue;
        }
        let method = if rel_str.starts_with("blobs/") {
            CompressionMethod::Stored
        } else {
            CompressionMethod::Deflated
        };
        zip.start_file(
            &rel_str,
            SimpleFileOptions::default().compression_method(method),
        )?;
        let mut src = File::open(path)?;
        std::io::copy(&mut src, &mut zip)?;
    }
    zip.finish()?;
    Ok(())
}

/// Read bytes of a `.ymk` archive and extract into `project_dir`. Symmetrical
/// with `export_ymk_bytes`: used by the HTTP `/projects/import` route.
pub fn import_ymk_bytes(bytes: &[u8], project_dir: &Utf8Path) -> Result<Utf8PathBuf> {
    if project_dir.exists() {
        anyhow::bail!("destination already exists: {project_dir}");
    }
    std::fs::create_dir_all(project_dir.as_std_path())?;
    let extraction = (|| -> Result<()> {
        let mut archive = ZipArchive::new(Cursor::new(bytes)).context("open zip archive")?;
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            let Some(enclosed) = entry.enclosed_name() else {
                continue;
            };
            let rel = Utf8PathBuf::from_path_buf(enclosed.to_path_buf())
                .map_err(|p| anyhow::anyhow!("archive entry not UTF-8: {}", p.display()))?;
            let target = project_dir.join(&rel);
            if entry.is_dir() {
                std::fs::create_dir_all(target.as_std_path())?;
                continue;
            }
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent.as_std_path())?;
            }
            let mut out = File::options()
                .write(true)
                .create_new(true)
                .open(target.as_std_path())
                .with_context(|| format!("create {target}"))?;
            std::io::copy(&mut entry, &mut out)
                .with_context(|| format!("extract archive entry to {target}"))?;
        }
        Ok(())
    })();

    if let Err(error) = extraction {
        if let Err(cleanup_error) = std::fs::remove_dir_all(project_dir.as_std_path()) {
            return Err(error.context(format!(
                "also failed to remove partial import {project_dir}: {cleanup_error}"
            )));
        }
        return Err(error);
    }
    Ok(project_dir.to_path_buf())
}

/// Unpack `ymk_path` into `project_dir`. `project_dir` must not exist yet.
pub fn import_ymk(ymk_path: &Utf8Path, project_dir: &Utf8Path) -> Result<Utf8PathBuf> {
    let bytes = std::fs::read(ymk_path.as_std_path())
        .with_context(|| format!("read archive {ymk_path}"))?;
    import_ymk_bytes(&bytes, project_dir)
}

/// Pack `(filename, bytes)` pairs into a `Deflated` zip in memory. Used by
/// the HTTP export route when a format produces multiple files (per-page PSD,
/// per-page PNG). Filenames are used verbatim — caller decides structure.
pub fn zip_files_to_bytes(files: &[(String, Vec<u8>)]) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut zip = ZipWriter::new(&mut cursor);
        for (name, bytes) in files {
            zip.start_file(
                name,
                SimpleFileOptions::default().compression_method(CompressionMethod::Deflated),
            )?;
            zip.write_all(bytes)?;
        }
        zip.finish()?;
    }
    Ok(cursor.into_inner())
}

fn should_skip(rel: &std::path::Path) -> bool {
    rel.components()
        .any(|c| SKIP_DIRS.contains(&c.as_os_str().to_string_lossy().as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8PathBuf;
    use tempfile::tempdir;

    #[test]
    fn export_then_import_round_trips_files() {
        let tmp = tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(tmp.path().to_path_buf()).unwrap();

        // Build a fake project.
        let proj = root.join("proj.ymkproj");
        std::fs::create_dir_all(proj.join("blobs/ab").as_std_path()).unwrap();
        std::fs::create_dir_all(proj.join("cache").as_std_path()).unwrap();
        std::fs::write(proj.join("project.toml").as_std_path(), b"name = \"x\"\n").unwrap();
        std::fs::write(proj.join("blobs/ab/cdef").as_std_path(), b"blob bytes").unwrap();
        std::fs::write(proj.join("cache/thumb.webp").as_std_path(), b"cached").unwrap();

        let ymk = root.join("out.ymk");
        export_ymk(&proj, &ymk).unwrap();

        let restored = root.join("restored.ymkproj");
        import_ymk(&ymk, &restored).unwrap();
        assert!(restored.join("project.toml").exists());
        assert!(restored.join("blobs/ab/cdef").exists());
        assert!(
            !restored.join("cache/thumb.webp").exists(),
            "cache excluded"
        );
    }

    #[test]
    fn failed_import_removes_partial_destination() {
        let tmp = tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(tmp.path().to_path_buf()).unwrap();
        let restored = root.join("broken.ymkproj");

        let error = import_ymk_bytes(b"not a zip archive", &restored).unwrap_err();

        assert!(error.to_string().contains("open zip archive"));
        assert!(
            !restored.exists(),
            "partial import directory must be removed"
        );
    }
}
