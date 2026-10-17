// `{{<namespace>.KEY}}` placeholders a control node fills itself.
//
// The executor resolves a batch node's params once, with no item, so the
// placeholders it can't fill there (`{{item.*}}`, report's `{{entry.*}}`)
// reach the node as written. The node resolves them once per file or entry,
// against that file's metadata or that entry's fields.

use serde_json::{Map, Value};

/// The per-file namespace: `{{item.KEY}}` reads the incoming file's metadata.
pub(super) const ITEM: &str = "item";

const CLOSE: &str = "}}";

/// `{{<namespace>.`, the start of every placeholder in that namespace.
fn open(namespace: &str) -> String {
    format!("{{{{{namespace}.")
}

/// Whether `text` reads any `{{<namespace>.KEY}}`.
pub(super) fn reads(text: &str, namespace: &str) -> bool {
    text.contains(&open(namespace))
}

/// `text` with every `{{<namespace>.KEY}}` replaced from `values`, or the
/// first key `values` doesn't have.
pub(super) fn resolve(
    text_in: &str,
    namespace: &str,
    values: &Map<String, Value>,
) -> Result<String, String> {
    let open = open(namespace);
    let mut out = String::with_capacity(text_in.len());
    let mut rest = text_in;
    while let Some(start) = rest.find(&open) {
        let after = &rest[start + open.len()..];
        let Some(end) = after.find(CLOSE) else { break };
        let key = &after[..end];
        let value = values.get(key).ok_or_else(|| key.to_string())?;
        out.push_str(&rest[..start]);
        out.push_str(&text(value));
        rest = &after[end + CLOSE.len()..];
    }
    out.push_str(rest);
    Ok(out)
}

/// A value as text: strings as written, other JSON (numbers, lists) as JSON.
pub(super) fn text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn values(v: Value) -> Map<String, Value> {
        v.as_object().cloned().unwrap()
    }

    #[test]
    fn test_resolves_only_the_named_namespace() {
        let v = values(json!({ "k": "x", "n": 2 }));
        let out = resolve("{{entry.k}}-{{entry.n}} {{item.k}}", "entry", &v).unwrap();
        assert_eq!(out, "x-2 {{item.k}}");
    }

    #[test]
    fn test_a_missing_key_is_returned() {
        let v = values(json!({}));
        assert_eq!(resolve("a {{item.gone}}", "item", &v), Err("gone".into()));
    }

    #[test]
    fn test_reads_needs_the_namespace_and_a_dot() {
        assert!(reads("x {{item.k}}", "item"));
        assert!(!reads("x {{items.k}} {{item}}", "item"));
    }

    #[test]
    fn test_an_unclosed_placeholder_is_left_as_written() {
        let v = values(json!({ "k": "x" }));
        assert_eq!(resolve("{{item.k", "item", &v).unwrap(), "{{item.k");
    }
}
