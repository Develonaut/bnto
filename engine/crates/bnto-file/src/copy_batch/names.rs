// The check that runs before any copy: a file's name is its path under the
// destination, so one that would leave it (absolute, a drive, or "..", which
// a `file-render` filename or a `file-rename` can build), one with no file
// name (only `.` segments: the destination itself), or one with a part this
// OS can't hold as a name (`CON`, a trailing dot on Windows) refuses the
// batch, naming every such file (ISS-128).

use bnto_core::BntoError;

use super::PerFile;
use crate::relocate::{self, name_refusal};

/// Refuse the batch, before any copy, when a file's name can't be copied
/// under the destination folder (`name_refusal`). A dry run is refused the
/// same.
pub(crate) fn refuse_bad_names(files: &[PerFile]) -> Result<(), BntoError> {
    let bad: Vec<String> = files
        .iter()
        .filter_map(|f| name_refusal(&f.input.filename, "the destination folder"))
        .collect();
    if bad.is_empty() {
        return Ok(());
    }
    let listing = relocate::listing(&bad, ("name", "names"));
    Err(BntoError::InvalidInput(format!(
        "File copy refused, this step copied nothing: {listing}\n  \
         A file's name is its path under the destination."
    )))
}
