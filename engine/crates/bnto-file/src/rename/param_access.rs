// Typed accessors for the rename node's JSON params map.

/// Extract a string param from the JSON map. Handles both string and number values.
pub(super) fn get_string_param(
    params: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<String> {
    params.get(key).and_then(|v| match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        _ => None,
    })
}

/// Extract an integer param from the JSON map.
pub(super) fn get_int_param(
    params: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<usize> {
    params.get(key).and_then(|v| match v {
        serde_json::Value::Number(n) => n.as_u64().map(|n| n as usize),
        serde_json::Value::String(s) => s.parse().ok(),
        _ => None,
    })
}
