// Fills file-render's own tokens: `{{item.KEY}}` and `{{file}}` per item,
// `{{count}}` anywhere.
//
// The node opts in with `resolves_items()`, so outside a loop the executor
// hands `{{item.*}}` over as written; it's filled here from each item's
// metadata. `bnto_core::control::placeholder` is the general resolver; this
// one adds escaping of filled values and the node's own `{{file}}` and
// `{{count}}`.

use bnto_core::processor::BatchFile;
use serde_json::Value;

use super::escape::Escape;

const OPEN: &str = "{{";
const ITEM_OPEN: &str = "{{item.";
const CLOSE: &str = "}}";
const FILE_TOKEN: &str = "{{file}}";
const COUNT_TOKEN: &str = "{{count}}";

/// The first token in `text` that reads an item, if any.
pub(super) fn per_item_token(text: &str) -> Option<&'static str> {
    if text.contains(ITEM_OPEN) {
        Some("{{item.*}}")
    } else if text.contains(FILE_TOKEN) {
        Some(FILE_TOKEN)
    } else {
        None
    }
}

/// `text` with `{{count}}` replaced by the number of items.
pub(super) fn fill_count(text: &str, count: usize) -> String {
    text.replace(COUNT_TOKEN, &count.to_string())
}

/// `row` filled for one item, its values escaped, or the first `{{item.KEY}}`
/// key the item lacks. Only the template is scanned, so a value that reads
/// like a token is written as it is.
pub(super) fn fill_row(row: &str, item: &BatchFile, escape: Escape) -> Result<String, String> {
    let mut out = String::with_capacity(row.len());
    let mut rest = row;
    while let Some(start) = rest.find(OPEN) {
        out.push_str(&rest[..start]);
        let (piece, after) = token(&rest[start..], item, escape)?;
        out.push_str(&piece);
        rest = after;
    }
    out.push_str(rest);
    Ok(out)
}

/// The text for the token `at` starts with, and what follows it. A `{{`
/// that isn't this node's token is kept as written.
fn token<'a>(at: &'a str, item: &BatchFile, escape: Escape) -> Result<(String, &'a str), String> {
    if let Some(after) = at.strip_prefix(FILE_TOKEN) {
        return Ok((escape.apply(&item.filename), after));
    }
    let key_and_rest = at
        .strip_prefix(ITEM_OPEN)
        .and_then(|after| after.split_once(CLOSE));
    match key_and_rest {
        Some((key, after)) => {
            let value = item.metadata.get(key).ok_or_else(|| key.to_string())?;
            Ok((escape.apply(&value_text(value)), after))
        }
        None => Ok((OPEN.to_string(), &at[OPEN.len()..])),
    }
}

/// A metadata value as text: strings as written, null empty, other JSON
/// (numbers, booleans, lists) as JSON.
fn value_text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}
