// The sources a file-move or file-delete batch would take away that a later
// step of the run still reads: another loop iteration's file, or a branch's
// that runs later (`ProcessContext::held_elsewhere`, ISS-82). Such a batch is
// refused before it starts, as file-copy refuses to replace a held file.

use std::path::Path;

use bnto_core::ProcessContext;

/// One line per source a later step still reads, saying it can't be `verb`
/// ("moved", "deleted") and which file reads it.
pub(crate) fn held_sources<'a>(
    sources: impl IntoIterator<Item = &'a Path>,
    verb: &str,
    ctx: &dyn ProcessContext,
) -> Vec<String> {
    sources
        .into_iter()
        .filter_map(|src| {
            let reader = ctx.held_elsewhere(src)?;
            Some(format!(
                "{}: a later step still reads it as {reader}, so it can't be {verb}",
                src.display()
            ))
        })
        .collect()
}

/// What to do instead, after a refusal's listing.
pub(crate) const HINT: &str =
    "Give each file once, or copy it (file-copy) where a later step still needs it.";
