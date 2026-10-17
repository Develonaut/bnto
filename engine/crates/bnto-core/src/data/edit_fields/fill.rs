// The values edit-fields sets on one item, every `{{item.KEY}}` filled from
// the fields that item arrived with.
//
// Outside a loop the executor hands `{{item.*}}` to this node as written
// (`resolves_items`), so it fills them here, once per file. A key the item
// lacks is a recipe mistake: it fails the run instead of becoming "".

use serde_json::{Map, Value};

use crate::control::placeholder::{self, ITEM};

/// `values` with every `{{item.KEY}}` filled from `item`, or why one can't
/// be: the field being set, the key, and the fields `item` does have.
pub(super) fn fill_values(
    values: &Map<String, Value>,
    item: &Map<String, Value>,
) -> Result<Map<String, Value>, String> {
    values
        .iter()
        .map(|(field, value)| {
            let filled = fill(value, item).map_err(|key| unknown(field, &key, item))?;
            Ok((field.clone(), filled))
        })
        .collect()
}

fn unknown(field: &str, key: &str, item: &Map<String, Value>) -> String {
    let has = if item.is_empty() {
        "it has no fields".to_string()
    } else {
        let keys: Vec<&str> = item.keys().map(String::as_str).collect();
        format!("it has: {}", keys.join(", "))
    };
    format!("\"{field}\" reads {{{{item.{key}}}}}, but the item has no field \"{key}\" ({has})")
}

/// One value filled, recursing into lists and maps; `Err` is the first key
/// `item` lacks.
fn fill(value: &Value, item: &Map<String, Value>) -> Result<Value, String> {
    match value {
        Value::String(text) => fill_text(text, item),
        Value::Array(values) => values.iter().map(|v| fill(v, item)).collect(),
        Value::Object(map) => map
            .iter()
            .map(|(k, v)| Ok((k.clone(), fill(v, item)?)))
            .collect::<Result<Map<_, _>, _>>()
            .map(Value::Object),
        other => Ok(other.clone()),
    }
}

/// A sole `{{item.KEY}}` is the field itself, type and all (as the executor
/// resolves a sole placeholder); any other text stays text.
fn fill_text(text: &str, item: &Map<String, Value>) -> Result<Value, String> {
    if let Some(key) = sole_item_key(text) {
        return item.get(key).cloned().ok_or_else(|| key.to_string());
    }
    placeholder::resolve(text, ITEM, item).map(Value::String)
}

/// KEY when `text` (trimmed) is exactly one `{{item.KEY}}`.
fn sole_item_key(text: &str) -> Option<&str> {
    let key = text.trim().strip_prefix("{{item.")?.strip_suffix("}}")?;
    (!key.contains('{') && !key.contains('}')).then_some(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sole_item_key_needs_exactly_one_placeholder() {
        assert_eq!(sole_item_key(" {{item.a.b}} "), Some("a.b"));
        assert_eq!(sole_item_key("{{item.a}}{{item.b}}"), None);
        assert_eq!(sole_item_key("x {{item.a}}"), None);
        assert_eq!(sole_item_key("{{fields.a}}"), None);
    }
}
