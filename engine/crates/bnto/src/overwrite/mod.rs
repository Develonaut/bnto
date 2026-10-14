// `output` overwrite mode: rename each source file in place to its pipeline name.
//
// Every rename is planned before any runs. A plan that would put two files on
// one name, or onto a file that isn't moving away, is refused whole, so a
// mapping table with a mistake can't silently replace a file. Accepted plans
// run in an order that keeps every file (a→b, b→c moves b first; a swap goes
// through a temporary name), and each rename re-checks its target by file
// identity, not by name, so case-sensitive filesystems are safe too.

mod conflicts;
mod execute;
mod plan;

use bnto_core::PipelineResult;

/// Rename every result file in place; on a dry run, only check the plan.
/// Returns how many files were (or would be) renamed in place, no-ops included.
pub fn overwrite_in_place(result: &PipelineResult, dry_run: bool) -> Result<usize, String> {
    let planned = plan::plan(result)?;
    conflicts::refuse_conflicts(&planned.moves)?;
    let count = planned.moves.len() + planned.unchanged;
    if !dry_run {
        execute::run_in_order(planned.moves)?;
    }
    Ok(count)
}

/// Case-folded path, so `A.wav` and `a.wav` count as one name, as they are on
/// Windows and macOS. Only ever used to find candidates; identity is decided
/// by `same_file`.
fn key(path: &std::path::Path) -> String {
    path.to_string_lossy().to_lowercase()
}

/// True when `a` and `b` are the same file on disk (false if either is missing).
fn same_file(a: &std::path::Path, b: &std::path::Path) -> bool {
    same_file::is_same_file(a, b).unwrap_or(false)
}

/// True when anything (file, folder, link) sits at `path`.
fn occupied(path: &std::path::Path) -> bool {
    std::fs::symlink_metadata(path).is_ok()
}

#[cfg(test)]
mod tests;
