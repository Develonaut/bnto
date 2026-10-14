// Turns a pipeline result into the renames overwrite mode would make.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use bnto_core::PipelineResult;
use bnto_core::processor::FileData;

/// One rename, `src` to `dest` in the same folder.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Move {
    pub src: PathBuf,
    pub dest: PathBuf,
}

/// The renames a result asks for, and how many files already have their name.
pub(super) struct Plan {
    pub moves: Vec<Move>,
    pub unchanged: usize,
}

/// Plan every rename. Fails on in-memory data, an unusable name, or a source
/// listed twice, before anything is renamed.
pub(super) fn plan(result: &PipelineResult) -> Result<Plan, String> {
    let mut planned = Plan {
        moves: Vec::new(),
        unchanged: 0,
    };
    let mut seen = HashSet::new();
    for file in &result.files {
        let m = plan_one(&file.data, &file.name, &mut seen)?;
        if m.dest == m.src {
            planned.unchanged += 1;
        } else {
            planned.moves.push(m);
        }
    }
    Ok(planned)
}

/// One file's rename (`src == dest` when its name doesn't change).
fn plan_one(data: &FileData, name: &str, seen: &mut HashSet<PathBuf>) -> Result<Move, String> {
    let src = source_path(data)?;
    if !seen.insert(src.clone()) {
        let why = format!("{} is in the run more than once", src.display());
        return Err(refused(&why));
    }
    let dest = src.parent().unwrap_or(Path::new(".")).join(leaf(name)?);
    Ok(Move { src, dest })
}

fn source_path(data: &FileData) -> Result<PathBuf, String> {
    match data {
        FileData::Path(src) => Ok(src.clone()),
        FileData::Bytes(_) => Err("Overwrite mode requires FileData::Path (CLI-only). \
             Cannot overwrite in-memory data."
            .to_string()),
    }
}

/// The last segment of a pipeline name, checked to be a plain filename.
fn leaf(name: &str) -> Result<&str, String> {
    let leaf = Path::new(name)
        .file_name()
        .and_then(|n| n.to_str())
        .filter(|leaf| !leaf.contains(['/', '\\']))
        .ok_or_else(|| refused(&format!("\"{name}\" has no usable file name")))?;
    check_platform_name(leaf).map_err(|why| refused(&format!("\"{leaf}\" {why}")))?;
    Ok(leaf)
}

pub(super) fn refused(why: &str) -> String {
    format!("Overwrite refused, nothing renamed: {why}")
}

/// Names Windows can't hold, checked up front: folder inputs are canonical
/// `\\?\` paths, which skip Win32 name normalization, so these would be
/// created literally or fail halfway through the run.
#[cfg(windows)]
fn check_platform_name(leaf: &str) -> Result<(), &'static str> {
    const RESERVED: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    if leaf.ends_with(['.', ' ']) {
        return Err("ends with a dot or space, which Windows drops");
    }
    if leaf
        .chars()
        .any(|c| c.is_control() || "<>:\"|?*".contains(c))
    {
        return Err("holds a character Windows doesn't allow in file names");
    }
    let stem = leaf.split('.').next().unwrap_or(leaf);
    if RESERVED.iter().any(|r| r.eq_ignore_ascii_case(stem)) {
        return Err("is a name Windows reserves for a device");
    }
    Ok(())
}

#[cfg(not(windows))]
fn check_platform_name(_leaf: &str) -> Result<(), &'static str> {
    Ok(())
}
