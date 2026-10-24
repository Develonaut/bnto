// Turns a pipeline result into the renames overwrite mode would make.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use bnto_core::PipelineResult;
use bnto_core::processor::FileData;
use bnto_file::relocate::{Move, check_platform_name};

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
    let dest = dest_beside(&src, name)?;
    Ok(Move { src, dest })
}

/// Where overwrite mode renames a file to: its pipeline name's last segment,
/// in the source file's own folder. `None` for data or a name it would refuse.
pub(crate) fn destination(data: &FileData, name: &str) -> Option<PathBuf> {
    dest_beside(&source_path(data).ok()?, name).ok()
}

fn dest_beside(src: &Path, name: &str) -> Result<PathBuf, String> {
    Ok(src.parent().unwrap_or(Path::new(".")).join(leaf(name)?))
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
