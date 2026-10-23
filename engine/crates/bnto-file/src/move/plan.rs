// Turns the files passing through into the moves file-move makes.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use bnto_core::processor::{BatchFile, FileData};

use super::root::Root;
use crate::relocate::{Move, check_platform_name, listing};

/// One move per file, in input order (`src == dest` when a file is already
/// in place), and each file's name as a `/`-joined path under the root.
pub(super) struct Planned {
    pub moves: Vec<Move>,
    pub names: Vec<String>,
}

/// Plan every move. Fails on in-memory data, or lists every unusable name,
/// missing source, repeated source, bad folder and other-drive file, before
/// anything moves.
#[expect(clippy::too_many_lines)]
pub(super) fn plan(files: &[BatchFile], root: &mut Root) -> Result<Planned, String> {
    let mut planned = Planned {
        moves: Vec::with_capacity(files.len()),
        names: Vec::with_capacity(files.len()),
    };
    let mut problems = Vec::new();
    let mut seen = HashSet::new();
    for file in files {
        let src = source_path(&file.data, &file.filename)?;
        match plan_one(src, &file.filename, root, &mut seen) {
            Ok((m, name)) => {
                planned.moves.push(m);
                planned.names.push(name);
            }
            Err(why) if !problems.contains(&why) => problems.push(why),
            Err(_) => {}
        }
    }
    if problems.is_empty() {
        return Ok(planned);
    }
    Err(refused(&listing(&problems, ("problem", "problems"))))
}

fn plan_one(
    src: &Path,
    name: &str,
    root: &mut Root,
    seen: &mut HashSet<PathBuf>,
) -> Result<(Move, String), String> {
    let segments = segments(name)?;
    let src = canonical_source(src)?;
    if !seen.insert(src.clone()) {
        return Err(format!("{} is in the run more than once", src.display()));
    }
    root.check_volume(&src)?;
    let mut dest = root.path.clone();
    dest.extend(&segments);
    root.check_folder(&dest)?;
    Ok((Move { src, dest }, segments.join("/")))
}

fn source_path<'a>(data: &'a FileData, name: &str) -> Result<&'a Path, String> {
    match data {
        FileData::Path(src) => Ok(src),
        FileData::Bytes(_) => Err(refused(&format!(
            "{name} is in memory, not a file on disk; file-move runs on the CLI only"
        ))),
    }
}

/// The parent canonicalized and the file's own name kept, so a symlink is
/// moved as the link, not as the file it points to.
fn canonical_source(src: &Path) -> Result<PathBuf, String> {
    let missing = || format!("{} doesn't exist", src.display());
    std::fs::symlink_metadata(src).map_err(|_| missing())?;
    let leaf = src.file_name().ok_or_else(missing)?;
    let folder = src.parent().filter(|p| !p.as_os_str().is_empty());
    let folder = std::fs::canonicalize(folder.unwrap_or(Path::new("."))).map_err(|_| missing())?;
    Ok(folder.join(leaf))
}

/// A name's path segments under the root. `/` and `\` both separate on every
/// OS, so one table works everywhere; empty and `.` segments drop out.
#[expect(clippy::too_many_lines)]
fn segments(name: &str) -> Result<Vec<&str>, String> {
    if name.starts_with(['/', '\\']) || has_drive(name) || Path::new(name).is_absolute() {
        return Err(format!(
            "\"{name}\" is an absolute path; names must be relative to the destination"
        ));
    }
    let segments: Vec<&str> = name
        .split(['/', '\\'])
        .filter(|s| !s.is_empty() && *s != ".")
        .collect();
    if segments.is_empty() {
        return Err(format!("\"{name}\" has no file name"));
    }
    if segments.contains(&"..") {
        return Err(format!(
            "\"{name}\" climbs out of the destination with \"..\""
        ));
    }
    for segment in &segments {
        check_platform_name(segment).map_err(|why| format!("\"{name}\": \"{segment}\" {why}"))?;
    }
    Ok(segments)
}

/// A Windows drive letter (`D:`, `D:\x`), refused on every OS so a table
/// means the same thing wherever it runs.
fn has_drive(name: &str) -> bool {
    let bytes = name.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

pub(super) fn refused(why: &str) -> String {
    format!("File move refused, nothing moved: {why}")
}
