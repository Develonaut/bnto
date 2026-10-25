// Each file's params, with `{{item.KEY}}` filled from that file's metadata,
// as the executor fills them for a per-file node: a value that is one whole
// placeholder keeps its JSON type, anything else is filled in as text.

use bnto_core::BntoError;
use bnto_core::control::placeholder::{self, ITEM};
use bnto_core::processor::BatchFile;
use serde_json::{Map, Value};

/// `params` with every `{{item.KEY}}` filled from `file`'s metadata, or the
/// error naming the first key the file lacks.
pub(super) fn filled(
    params: &Map<String, Value>,
    file: &BatchFile,
) -> Result<Map<String, Value>, BntoError> {
    let fill = |(key, value): (&String, &Value)| Ok((key.clone(), fill_value(value, file)?));
    params.iter().map(fill).collect()
}

fn fill_value(value: &Value, file: &BatchFile) -> Result<Value, BntoError> {
    let Value::String(text) = value else {
        return Ok(value.clone());
    };
    if let Some(key) = placeholder::sole_key(text, ITEM) {
        return file
            .metadata
            .get(key)
            .cloned()
            .ok_or_else(|| unfilled(key, file));
    }
    if !placeholder::reads(text, ITEM) {
        return Ok(value.clone());
    }
    placeholder::resolve(text, ITEM, &file.metadata)
        .map(Value::String)
        .map_err(|key| unfilled(&key, file))
}

/// A `{{item.KEY}}` the file has no value for, worded as the executor words it
/// for a per-file node (less its `Node '<id>':` prefix; the run names the step).
fn unfilled(key: &str, file: &BatchFile) -> BntoError {
    let why = placeholder::no_value(key, Some(&file.filename), Some(&file.metadata));
    BntoError::InvalidInput(why)
}
