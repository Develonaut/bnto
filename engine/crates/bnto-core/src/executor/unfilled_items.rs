// Finds `{{item.KEY}}` references a resolution can't fill.
//
// A param built from a value that isn't there (a file name, a path) must not
// be used as if it were real, so the executor fails the node instead of
// resolving such a reference to empty text or leaving it as written.

use serde_json::Value;

type Params = serde_json::Map<String, Value>;

/// Keys referenced as `{{item.KEY}}` anywhere in `params` that `item` lacks
/// (every one of them when there is no item), first-seen order, no repeats.
pub(super) fn unfilled_item_keys(params: &Params, item: &Option<Params>) -> Vec<String> {
    let mut keys = Vec::new();
    for value in params.values() {
        collect_item_keys(value, &mut keys);
    }
    keys.retain(|key| item.as_ref().is_none_or(|item| !item.contains_key(key)));
    keys
}

fn collect_item_keys(value: &Value, keys: &mut Vec<String>) {
    match value {
        Value::String(s) => {
            for key in item_keys_in(s) {
                if !keys.contains(&key) {
                    keys.push(key);
                }
            }
        }
        Value::Array(values) => values.iter().for_each(|v| collect_item_keys(v, keys)),
        Value::Object(map) => map.values().for_each(|v| collect_item_keys(v, keys)),
        _ => {}
    }
}

/// The KEY of every `{{item.KEY}}` in `s`, in order.
fn item_keys_in(s: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut rest = s;
    while let Some(start) = rest.find("{{item.") {
        let after = &rest[start + "{{item.".len()..];
        let Some(end) = after.find("}}") else { break };
        keys.push(after[..end].to_string());
        rest = &after[end + 2..];
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn params(v: Value) -> Params {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn test_keys_the_item_lacks_are_listed_once_in_order() {
        let p = params(json!({
            "a": "{{item.x}}-{{item.y}}",
            "b": ["{{item.z}}", { "c": "{{item.x}}" }],
            "n": 3
        }));
        let item = Some(params(json!({ "y": "1" })));
        assert_eq!(unfilled_item_keys(&p, &item), vec!["x", "z"]);
    }

    #[test]
    fn test_every_key_is_unfilled_without_an_item() {
        let p = params(json!({ "a": "{{item.x}}" }));
        assert_eq!(unfilled_item_keys(&p, &None), vec!["x"]);
    }

    #[test]
    fn test_present_keys_and_other_namespaces_are_fine() {
        let p = params(json!({ "a": "{{item.x}} {{fields.f}} {{name}}" }));
        let item = Some(params(json!({ "x": "" })));
        assert!(
            unfilled_item_keys(&p, &item).is_empty(),
            "an empty value is a value"
        );
    }

    #[test]
    fn test_an_unclosed_placeholder_is_not_a_reference() {
        let p = params(json!({ "a": "{{item.x" }));
        assert!(unfilled_item_keys(&p, &None).is_empty());
    }
}
