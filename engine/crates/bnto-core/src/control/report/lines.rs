// The lines report prints: one per entry of `each`, one per file when the
// message reads `{{file}}` or `{{item.*}}`, else one. Each line is filled in
// one pass (`placeholder::fill_text`), the run's namespaces included: an
// entry line reads the loop's item, a file line the file's metadata with the
// loop's item on top.

use serde_json::Value;

use super::params::ReportParams;
use crate::control::placeholder::{self, ITEM, Missing, Scope};
use crate::processor::BatchFile;

/// Every line to print, in order, or why one can't be filled. `once` reads
/// the loop's item and the run.
pub(super) fn lines(
    params: &ReportParams,
    files: &[BatchFile],
    once: Scope<'_>,
) -> Result<Vec<String>, String> {
    let message = &params.message;
    if let Some(entries) = params.entries()? {
        return entries
            .iter()
            .enumerate()
            .map(|(i, entry)| entry_line(message, i + 1, entry, once))
            .collect();
    }
    if placeholder::reads_file(message) {
        return files
            .iter()
            .map(|file| file_line(message, file, once))
            .collect();
    }
    let line = placeholder::fill_text(message, once)
        .map_err(|m| placeholder::no_value(&m.key, None, once.item))?;
    Ok(vec![line])
}

/// The line for entry `n` (from 1): `{{entry.KEY}}` reads one of its fields,
/// `{{entry}}` is the whole entry, `{{item.KEY}}` the loop's item.
fn entry_line(message: &str, n: usize, entry: &Value, once: Scope<'_>) -> Result<String, String> {
    if once.item.is_none() && placeholder::reads(message, ITEM) {
        return Err("'message' reads {{entry}} and {{item.*}} outside a loop: \
                    a report prints a line per entry or per file, not both"
            .to_string());
    }
    let scope = Scope {
        entry: Some(entry),
        ..once
    };
    placeholder::fill_text(message, scope).map_err(|missing| match missing.namespace {
        ITEM => placeholder::no_value(&missing.key, None, once.item),
        _ => format!("entry {n} has no \"{}\"", missing.key),
    })
}

/// The line for one file: `{{file}}` is its name, `{{item.KEY}}` reads its
/// metadata, the loop's item on top.
fn file_line(message: &str, file: &BatchFile, once: Scope<'_>) -> Result<String, String> {
    let item = placeholder::file_item(&file.metadata, once.item);
    let scope = Scope {
        file: Some(&file.filename),
        item: Some(&item),
        ..once
    };
    placeholder::fill_text(message, scope).map_err(|Missing { key, .. }| {
        placeholder::no_value(&key, Some(&file.filename), Some(&item))
    })
}
