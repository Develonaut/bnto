// The lines report prints: one per entry of `each`, one per file when the
// message reads `{{file}}` or `{{item.*}}`, else one.

use serde_json::{Map, Value};

use super::params::{ENTRY, ReportParams, WHOLE_ENTRY};
use crate::control::placeholder::{self, text};
use crate::processor::BatchFile;

/// Every line to print, in order, or why one can't be filled.
pub(super) fn lines(params: &ReportParams, files: &[BatchFile]) -> Result<Vec<String>, String> {
    let message = &params.message;
    if let Some(entries) = params.entries()? {
        return entries
            .iter()
            .enumerate()
            .map(|(i, entry)| entry_line(message, i + 1, entry))
            .collect();
    }
    if placeholder::reads_file(message) {
        return files.iter().map(|file| file_line(message, file)).collect();
    }
    Ok(vec![message.clone()])
}

/// The line for entry `n` (from 1): `{{entry.KEY}}` reads one of its fields,
/// `{{entry}}` is the whole entry.
fn entry_line(message: &str, n: usize, entry: &Value) -> Result<String, String> {
    let no_fields = Map::new();
    let fields = entry.as_object().unwrap_or(&no_fields);
    let line = placeholder::resolve(message, ENTRY, fields)
        .map_err(|key| format!("entry {n} has no \"{key}\""))?;
    Ok(line.replace(WHOLE_ENTRY, &text(entry)))
}

/// The line for one file: `{{file}}` is its name, `{{item.KEY}}` reads its
/// metadata.
fn file_line(message: &str, file: &BatchFile) -> Result<String, String> {
    placeholder::resolve_file(message, &file.filename, &file.metadata)
        .map_err(|key| format!("{} has no item \"{key}\"", file.filename))
}
