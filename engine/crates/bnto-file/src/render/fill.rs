// Fills file-render's own tokens in one pass over the recipe's template:
// `{{item.KEY}}` and `{{file}}` per item, `{{count}}` anywhere.
//
// The node opts in with `resolves_items()`, so the executor hands `{{item.*}}`
// over as written, in a loop too. A row reads each item's metadata with the
// loop's item on top, its values escaped; the header, footer and filename are
// written once, so they read only the loop's item (`fill_once`). Filling is
// `bnto_core::control::placeholder`'s, so a value that reads like a token is
// written as it is.

use bnto_core::control::placeholder::{self, Scope};
use bnto_core::processor::BatchFile;
use serde_json::{Map, Value};

use super::escape::Escape;

const ITEM_OPEN: &str = "{{item.";
const FILE_TOKEN: &str = "{{file}}";
const COUNT: &str = "count";

/// The first token in `text` that reads a file: `{{file}}`, or `{{item.*}}`
/// unless `in_loop`, where that is the loop's item.
pub(super) fn per_item_token(text: &str, in_loop: bool) -> Option<&'static str> {
    if text.contains(FILE_TOKEN) {
        Some(FILE_TOKEN)
    } else if !in_loop && text.contains(ITEM_OPEN) {
        Some("{{item.*}}")
    } else {
        None
    }
}

/// `row` filled for one item, its values escaped, or the first `{{item.KEY}}`
/// key `values` (the item's metadata, the loop's item on top) lacks.
pub(super) fn fill_row(
    row: &str,
    item: &BatchFile,
    values: &Map<String, Value>,
    count: usize,
    escape: Escape,
) -> Result<String, String> {
    let scope = Scope {
        file: Some(&item.filename),
        item: Some(values),
        ..Scope::default()
    };
    placeholder::fill_with(row, |inner| {
        if inner == COUNT {
            return Ok(Some(count.to_string()));
        }
        let text = scope.text(inner).map_err(|missing| missing.key)?;
        Ok(text.map(|t| escape.apply(&t)))
    })
}

/// A template written once (header, footer, filename) filled from the loop's
/// item, and `{{count}}` when `count` is given, or the key the loop's item
/// lacks. Values go in as they are, as the executor filled them before.
pub(super) fn fill_once(
    text: &str,
    loop_item: Option<&Map<String, Value>>,
    count: Option<usize>,
) -> Result<String, String> {
    let scope = Scope {
        item: loop_item,
        ..Scope::default()
    };
    placeholder::fill_with(text, |inner| match count {
        Some(count) if inner == COUNT => Ok(Some(count.to_string())),
        _ => scope.text(inner).map_err(|missing| missing.key),
    })
}
