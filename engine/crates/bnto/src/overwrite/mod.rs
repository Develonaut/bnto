// `output` overwrite mode: rename each source file in place to its pipeline name.
//
// Every rename is planned before any runs, then checked and run by
// `bnto_file::relocate` (shared with the `file-move` node): a plan that would
// replace a file is refused whole, and an accepted one runs in an order that
// keeps every file.

mod plan;

pub(crate) use plan::destination;

use bnto_core::PipelineResult;
use bnto_file::relocate;

/// Rename every result file in place; on a dry run, only check the plan.
/// Returns how many files were (or would be) renamed in place, no-ops included.
pub fn overwrite_in_place(result: &PipelineResult, dry_run: bool) -> Result<usize, String> {
    let planned = plan::plan(result)?;
    let conflicts = relocate::conflicts(&planned.moves);
    if !conflicts.is_empty() {
        let nouns = ("conflict", "conflicts");
        return Err(plan::refused(&relocate::listing(&conflicts, nouns)));
    }
    let count = planned.moves.len() + planned.unchanged;
    if !dry_run {
        relocate::run_in_order(planned.moves)?;
    }
    Ok(count)
}

#[cfg(test)]
mod tests;
