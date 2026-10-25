// The check that runs before any copy: a copy that would replace a file
// another file of the batch is read from refuses the whole batch.

use std::path::{Path, PathBuf};

use bnto_core::BntoError;
use bnto_core::processor::{FileData, NodeInput};

use super::PerFile;
use crate::copy::{conflict_mode, dest_path};
use crate::copy_newer;
use crate::readers::readers_of;
use crate::relocate;

/// What to do instead, after the refusal's listing.
const HINT: &str = "Copy into another folder, or use conflict \"skip\" or \"rename\".";

/// Refuse the batch, before any copy, when a copy would replace a file
/// another file of the batch is read from. A dry run is refused the same.
pub(crate) fn refuse_replacing_read_files(files: &[PerFile]) -> Result<(), BntoError> {
    let inputs: Vec<&NodeInput> = files.iter().map(|f| &f.input).collect();
    let conflicts = conflicts(&inputs);
    if conflicts.is_empty() {
        return Ok(());
    }
    let listing = relocate::listing(&conflicts, ("conflict", "conflicts"));
    Err(BntoError::InvalidInput(format!(
        "File copy refused, nothing copied: {listing}\n  {HINT}"
    )))
}

/// One line per copy that would replace a file another file is read from.
fn conflicts(inputs: &[&NodeInput]) -> Vec<String> {
    let dests: Vec<PathBuf> = inputs.iter().map(|input| dest_path(input)).collect();
    let replaced: Vec<Option<&Path>> = inputs
        .iter()
        .zip(&dests)
        .map(|(input, dest)| replaces(input, dest).then_some(dest.as_path()))
        .collect();
    let sources: Vec<&FileData> = inputs.iter().map(|input| &input.data).collect();
    let line = |(i, readers): (usize, &Vec<usize>)| {
        let reader = inputs[*readers.first()?];
        Some(conflict(inputs[i], &dests[i], reader))
    };
    readers_of(&sources, &replaced)
        .iter()
        .enumerate()
        .filter_map(line)
        .collect()
}

/// Whether copying `input` to `dest` replaces the file there: `overwrite`
/// onto another file, or `newer` onto an older one. A file already at its
/// destination is left as is (ISS-36), and `skip` and `rename` never replace.
fn replaces(input: &NodeInput, dest: &Path) -> bool {
    if !dest.is_file() || input.data.is_at(dest) {
        return false;
    }
    match conflict_mode(&input.params) {
        "overwrite" => true,
        "newer" => copy_newer::replaces(input, dest),
        _ => false,
    }
}

/// One refusal line: the destination, what would be copied over it, and the
/// file read from it.
fn conflict(input: &NodeInput, dest: &Path, reader: &NodeInput) -> String {
    let from = match &input.data {
        FileData::Path(src) => src.display().to_string(),
        _ => input.filename.clone(),
    };
    format!(
        "{}: copying {from} over it would replace the file {} is read from",
        dest.display(),
        reader.filename
    )
}
