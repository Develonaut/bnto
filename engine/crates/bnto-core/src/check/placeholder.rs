// Placeholder form check — each `{{` must close, and a `{{namespace.` must
// name a namespace the engine resolves. Values are never looked up.

use serde_json::Value;

/// Namespaces a `{{namespace.key}}` placeholder may name: the executor's
/// (`executor/template.rs`), run vars (`vars.rs`) and the report node's
/// per-entry `entry` (`control/report`).
const NAMESPACES: [&str; 8] = [
    "fields", "env", "ctx", "node", "item", "loop", "vars", "entry",
];

const OPEN: &str = "{{";
const CLOSE: &str = "}}";

/// Problems with the placeholders in every string under `value`.
pub(super) fn value_problems(value: &Value) -> Vec<String> {
    match value {
        Value::String(s) => string_problems(s),
        Value::Array(items) => items.iter().flat_map(value_problems).collect(),
        Value::Object(map) => map.values().flat_map(value_problems).collect(),
        _ => Vec::new(),
    }
}

/// Scan `s` left to right, one placeholder at a time.
fn string_problems(s: &str) -> Vec<String> {
    scan(s)
        .into_iter()
        .filter_map(|found| match found {
            Placeholder::Closed(inner) => namespace_problem(inner),
            Placeholder::Unclosed(text) => Some(unclosed(text)),
        })
        .collect()
}

/// A problem for each `{{` in `s` that never closes. A fill keeps such a
/// `{{` as text (`interpolate`), so the output folder refuses it (ISS-79).
pub(crate) fn unclosed_problems(s: &str) -> Vec<String> {
    scan(s)
        .into_iter()
        .filter_map(|found| match found {
            Placeholder::Unclosed(text) => Some(unclosed(text)),
            Placeholder::Closed(_) => None,
        })
        .collect()
}

/// One `{{` in a string: what's inside its braces, or the text after an
/// unclosed one.
enum Placeholder<'a> {
    Closed(&'a str),
    Unclosed(&'a str),
}

/// Every placeholder in `s`, in order.
fn scan(s: &str) -> Vec<Placeholder<'_>> {
    let mut found = Vec::new();
    let mut rest = s;
    while let Some(start) = rest.find(OPEN) {
        let (placeholder, remaining) = scan_placeholder(&rest[start + OPEN.len()..]);
        found.push(placeholder);
        rest = remaining;
    }
    found
}

/// Read one placeholder from just after its `{{`, and the text left to scan.
fn scan_placeholder(after: &str) -> (Placeholder<'_>, &str) {
    let next_open = after.find(OPEN);
    match after.find(CLOSE) {
        Some(end) if next_open.is_none_or(|open| open > end) => (
            Placeholder::Closed(&after[..end]),
            &after[end + CLOSE.len()..],
        ),
        // No close, or another placeholder opens first: this one never closes.
        _ => {
            let stop = next_open.unwrap_or(after.len());
            (Placeholder::Unclosed(&after[..stop]), &after[stop..])
        }
    }
}

/// Longest stretch of an unclosed placeholder's text a message quotes.
const QUOTE_CHARS: usize = 40;

fn unclosed(text: &str) -> String {
    let quoted: String = text.trim_end().chars().take(QUOTE_CHARS).collect();
    format!("unclosed placeholder \"{OPEN}{quoted}\"")
}

/// A `{{ns.key}}` whose `ns` the engine doesn't resolve. A placeholder with
/// no dot is a node's own template (file-rename's `{{name}}`), and one with
/// nothing before the dot isn't the engine's (a shell tool's `{{.Field}}`).
fn namespace_problem(inner: &str) -> Option<String> {
    let (namespace, _) = inner.split_once('.')?;
    if namespace.trim().is_empty() || NAMESPACES.contains(&namespace) {
        return None;
    }
    Some(format!(
        "unknown placeholder namespace \"{namespace}\" in \"{OPEN}{inner}{CLOSE}\" (known: {})",
        NAMESPACES.join(", ")
    ))
}
