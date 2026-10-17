// Placeholder form check — each `{{` must close, and a `{{namespace.` must
// name a namespace the engine resolves. Values are never looked up.

use serde_json::Value;

/// Namespaces a `{{namespace.key}}` placeholder may name: the executor's
/// (`executor/template.rs`) plus run vars (`vars.rs`).
const NAMESPACES: [&str; 6] = ["fields", "env", "ctx", "node", "item", "vars"];

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
    let mut problems = Vec::new();
    let mut rest = s;
    while let Some(start) = rest.find(OPEN) {
        let (problem, remaining) = scan_placeholder(&rest[start + OPEN.len()..]);
        problems.extend(problem);
        rest = remaining;
    }
    problems
}

/// Read one placeholder from just after its `{{`: its problem, if any, and
/// the text left to scan.
fn scan_placeholder(after: &str) -> (Option<String>, &str) {
    let next_open = after.find(OPEN);
    match after.find(CLOSE) {
        Some(end) if next_open.is_none_or(|open| open > end) => (
            namespace_problem(&after[..end]),
            &after[end + CLOSE.len()..],
        ),
        // No close, or another placeholder opens first: this one never closes.
        _ => {
            let stop = next_open.unwrap_or(after.len());
            (Some(unclosed(&after[..stop])), &after[stop..])
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
