// `{{<namespace>.KEY}}` placeholders a batch node (assert, report, edit-fields)
// fills itself, and the per-file `{{file}}`.
//
// The executor resolves a batch node's params once, with no item, so the
// placeholders it can't fill there (`{{item.*}}`, `{{file}}`, report's
// `{{entry.*}}`) reach the node as written. The node resolves them once per
// file or entry, against that file's name and metadata or that entry's fields.

use serde_json::{Map, Value};

/// The per-file namespace: `{{item.KEY}}` reads the incoming file's metadata.
pub(crate) const ITEM: &str = "item";

/// The per-file name token: `{{file}}` reads the incoming file's name, as
/// file-render and shell-command's `each` read it.
pub(crate) const FILE: &str = "{{file}}";

const CLOSE: &str = "}}";

/// `{{<namespace>.`, the start of every placeholder in that namespace.
fn open(namespace: &str) -> String {
    format!("{{{{{namespace}.")
}

/// Whether `text` reads any `{{<namespace>.KEY}}`.
pub(crate) fn reads(text: &str, namespace: &str) -> bool {
    text.contains(&open(namespace))
}

/// `text` with every `{{<namespace>.KEY}}` replaced from `values`, or the
/// first key `values` doesn't have.
pub(crate) fn resolve(
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

/// Whether `text` reads the file it is filled for: `{{file}}` or `{{item.*}}`.
pub(crate) fn reads_file(text: &str) -> bool {
    text.contains(FILE) || reads(text, ITEM)
}

/// `text` filled for one file: `{{file}}` with `name`, `{{item.KEY}}` from
/// `metadata`, or the first key `metadata` doesn't have. Only the template is
/// scanned, so a filled value that reads like a token is kept as written.
pub(crate) fn resolve_file(
    text_in: &str,
    name: &str,
    metadata: &Map<String, Value>,
) -> Result<String, String> {
    let pieces = text_in
        .split(FILE)
        .map(|piece| resolve(piece, ITEM, metadata))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(pieces.join(name))
}

/// A value as text: strings as written, other JSON (numbers, lists) as JSON.
pub(crate) fn text(value: &Value) -> String {
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
    fn test_reads_file_needs_the_file_token_or_an_item_key() {
        assert!(reads_file("{{file}}") && reads_file("{{item.k}}"));
        assert!(!reads_file("{{files}} {{node.a.file}}"));
    }

    #[test]
    fn test_resolve_file_fills_the_name_and_item_keys() {
        let v = values(json!({ "k": "x" }));
        let out = resolve_file("{{file}}={{item.k}} {{file}}", "a.ts", &v).unwrap();
        assert_eq!(out, "a.ts=x a.ts");
    }

    #[test]
    fn test_resolve_file_keeps_filled_values_that_read_like_tokens() {
        let v = values(json!({ "k": "{{file}}" }));
        let out = resolve_file("{{item.k}}", "{{item.k}}.ts", &v).unwrap();
        assert_eq!(out, "{{file}}");
        let out = resolve_file("{{file}}", "{{item.k}}.ts", &v).unwrap();
        assert_eq!(out, "{{item.k}}.ts");
    }

    #[test]
    fn test_resolve_file_returns_a_missing_key() {
        let v = values(json!({}));
        assert_eq!(
            resolve_file("{{file}} {{item.gone}}", "a", &v),
            Err("gone".into())
        );
    }

    #[test]
    fn test_an_unclosed_placeholder_is_left_as_written() {
        let v = values(json!({ "k": "x" }));
        assert_eq!(resolve("{{item.k", "item", &v).unwrap(), "{{item.k");
    }
}
