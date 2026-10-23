// Where an incoming item's file sits, checked against the base folder.
//
// Folders in the item's path are resolved (links and `..` included) before
// the check, so neither can reach outside the base. The file's own name is
// kept, so a link is deleted as the link, never as the file it points to,
// and the file a link points to must be under the base too.

use std::io::ErrorKind;
use std::path::{Component, Path, PathBuf};

use bnto_core::processor::{BatchFile, FileData};

/// What deleting one item means.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Target {
    /// A file to delete, at this resolved path.
    File(PathBuf),
    /// Nothing is there: skipped and counted.
    Missing,
}

/// The base folder as a canonical path; it must exist and be a folder.
pub(super) fn base_root(base: &str) -> Result<PathBuf, String> {
    let path = std::fs::canonicalize(base)
        .map_err(|e| format!("base folder {base} can't be resolved: {e}"))?;
    if !path.is_dir() {
        return Err(format!("base folder {base} is not a folder"));
    }
    Ok(path)
}

/// The item's target under `root`, or why it is refused. `base` is the
/// param as written, for the message.
#[expect(clippy::too_many_lines)]
pub(super) fn check(file: &BatchFile, root: &Path, base: &str) -> Result<Target, String> {
    let name = &file.filename;
    let FileData::Path(path) = &file.data else {
        return Err(format!(
            "{name} is in memory, not a file on disk; file-delete only deletes files on disk"
        ));
    };
    let outside = || {
        format!(
            "{name} ({}) is outside the base folder {base}",
            path.display()
        )
    };
    let location = locate(path).ok_or_else(outside)?;
    if !location.starts_with(root) {
        return Err(outside());
    }
    let meta = match std::fs::symlink_metadata(&location) {
        Ok(meta) => meta,
        Err(e) if is_absent(&e) => return Ok(Target::Missing),
        Err(e) => return Err(format!("{name} can't be read: {e}")),
    };
    if meta.file_type().is_symlink() {
        let pointee = std::fs::canonicalize(&location)
            .map_err(|e| format!("{name} is a link that can't be resolved: {e}"))?;
        if !pointee.starts_with(root) {
            return Err(format!("{} (a link)", outside()));
        }
    }
    if std::fs::metadata(&location).is_ok_and(|m| m.is_dir()) {
        return Err(format!(
            "{name} ({}) is a folder; file-delete only deletes files",
            path.display()
        ));
    }
    Ok(Target::File(location))
}

fn is_absent(e: &std::io::Error) -> bool {
    matches!(e.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory)
}

/// `path` with its folders resolved and its own name kept. `None` when it
/// names no file (it ends in `..` or is a root).
fn locate(path: &Path) -> Option<PathBuf> {
    let leaf = path.file_name()?;
    let folder = path.parent().filter(|p| !p.as_os_str().is_empty());
    Some(resolve_folder(folder.unwrap_or(Path::new(".")))?.join(leaf))
}

/// The deepest part of `folder` that exists, canonicalized, with the rest
/// (which can't hold links, as it doesn't exist) applied by name.
fn resolve_folder(folder: &Path) -> Option<PathBuf> {
    let parts: Vec<Component> = folder.components().collect();
    (0..=parts.len()).rev().find_map(|split| {
        let existing: PathBuf = parts[..split].iter().collect();
        let existing = if split == 0 {
            PathBuf::from(".")
        } else {
            existing
        };
        let mut resolved = std::fs::canonicalize(existing).ok()?;
        for part in &parts[split..] {
            match part {
                Component::Normal(name) => resolved.push(name),
                Component::ParentDir => {
                    resolved.pop();
                }
                _ => {}
            }
        }
        Some(resolved)
    })
}
