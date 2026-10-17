// `{{item.KEY}}` in assert's params, resolved against each incoming file.
//
// The executor resolves a batch node's params once, with no item, so
// `{{item.*}}` placeholders reach this node unresolved. Assert resolves them
// itself, once per file, against that file's metadata.

use serde_json::{Map, Value};

use super::params::text;

const OPEN: &str = "{{item.";
const CLOSE: &str = "}}";

/// Whether `text` reads the per-file item.
pub(super) fn reads_item(text: &str) -> bool {
    text.contains(OPEN)
}

/// `text` with every `{{item.KEY}}` replaced from `item`, or the first key
/// the file's metadata doesn't have.
pub(super) fn resolve(text_in: &str, item: &Map<String, Value>) -> Result<String, String> {
    let mut out = String::with_capacity(text_in.len());
    let mut rest = text_in;
    while let Some(start) = rest.find(OPEN) {
        let after = &rest[start + OPEN.len()..];
        let Some(end) = after.find(CLOSE) else { break };
        let key = &after[..end];
        let value = item.get(key).ok_or_else(|| key.to_string())?;
        out.push_str(&rest[..start]);
        out.push_str(&text(value));
        rest = &after[end + CLOSE.len()..];
    }
    out.push_str(rest);
    Ok(out)
}
