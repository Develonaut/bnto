// Each file's params, with `{{item.KEY}}` filled from that file's metadata
// and the loop's item on top, and the run's namespaces (`run_placeholder`),
// as the executor fills them for a per-file node:
// a value that is one whole placeholder keeps its JSON type, anything else is
// filled in as text, in one pass, so a value that reads like a token is kept
// as written.

use bnto_core::control::placeholder::{self, Scope};
use bnto_core::processor::BatchFile;
use bnto_core::{BntoError, ProcessContext};
use serde_json::{Map, Value};

/// `params` with every `{{item.KEY}}` filled for `file`, or the error naming
/// the first key neither its metadata nor the loop's item has.
pub(super) fn filled(
    params: &Map<String, Value>,
    file: &BatchFile,
    loop_item: Option<&Map<String, Value>>,
    run: &dyn ProcessContext,
) -> Result<Map<String, Value>, BntoError> {
    let item = placeholder::file_item(&file.metadata, loop_item);
    let scope = Scope {
        item: Some(&item),
        run: Some(run),
        ..Scope::default()
    };
    let fill = |(key, value): (&String, &Value)| {
        let filled = placeholder::fill_value(value, scope)
            .map_err(|missing| unfilled(&missing.key, file, &item))?;
        Ok((key.clone(), filled))
    };
    params.iter().map(fill).collect()
}

/// A `{{item.KEY}}` the file has no value for, worded as the executor words it
/// for a per-file node (less its `Node '<id>':` prefix; the run names the step).
fn unfilled(key: &str, file: &BatchFile, item: &Map<String, Value>) -> BntoError {
    let why = placeholder::no_value(key, Some(&file.filename), Some(item));
    BntoError::InvalidInput(why)
}
