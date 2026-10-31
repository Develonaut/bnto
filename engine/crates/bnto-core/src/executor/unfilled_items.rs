// Finds `{{item.KEY}}` and `{{loop.KEY}}` references a resolution can't fill.
//
// A param built from a value that isn't there (a file name, a path) must not
// be used as if it were real, so the executor fails the node instead of
// resolving such a reference to empty text or leaving it as written.

use serde_json::Value;

type Params = serde_json::Map<String, Value>;

const ITEM_OPEN: &str = "{{item.";
const LOOP_OPEN: &str = "{{loop.";

/// Keys referenced as `{{item.KEY}}` anywhere in `params` that `item` lacks
/// (every one of them when there is no item), first-seen order, no repeats.
pub(super) fn unfilled_item_keys(params: &Params, item: &Option<Params>) -> Vec<String> {
    unfilled_keys(params, ITEM_OPEN, item.as_ref())
}

/// Why `params` can't run with `row` as the loop's item: a `{{loop.KEY}}`
/// outside a loop, or one the row lacks. `{{loop.*}}` reads the same row for
/// every file of an iteration, so this is known before the step starts.
pub(super) fn loop_refusal(params: &Params, row: Option<&Params>) -> Option<String> {
    let key = unfilled_keys(params, LOOP_OPEN, row).into_iter().next()?;
    Some(match row {
        None => {
            format!("{{{{loop.{key}}}}} reads the loop's item, but this step is not inside a loop")
        }
        Some(row) => {
            let keys: Vec<&str> = row.keys().map(String::as_str).collect();
            format!(
                "{{{{loop.{key}}}}} has no value (the loop's item has: {})",
                keys.join(", ")
            )
        }
    })
}

fn unfilled_keys(params: &Params, open: &str, values: Option<&Params>) -> Vec<String> {
    let mut keys = Vec::new();
    for value in params.values() {
        collect_keys(value, open, &mut keys);
    }
    keys.retain(|key| values.is_none_or(|values| !values.contains_key(key)));
    keys
}

fn collect_keys(value: &Value, open: &str, keys: &mut Vec<String>) {
    match value {
        Value::String(s) => {
            for key in keys_in(s, open) {
                if !keys.contains(&key) {
                    keys.push(key);
                }
            }
        }
        Value::Array(values) => values.iter().for_each(|v| collect_keys(v, open, keys)),
        Value::Object(map) => map.values().for_each(|v| collect_keys(v, open, keys)),
        _ => {}
    }
}

/// The KEY of every `<open>KEY}}` in `s`, in order.
fn keys_in(s: &str, open: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut rest = s;
    while let Some(start) = rest.find(open) {
        let after = &rest[start + open.len()..];
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
    fn test_a_loop_key_outside_a_loop_or_missing_from_the_row_is_refused() {
        let p = params(json!({ "a": "{{loop.name}} {{item.x}}" }));
        let outside = loop_refusal(&p, None).unwrap();
        assert!(outside.contains("{{loop.name}}") && outside.contains("not inside a loop"));
        let row = params(json!({ "title": "t" }));
        let missing = loop_refusal(&p, Some(&row)).unwrap();
        assert!(missing.contains("{{loop.name}} has no value") && missing.contains("title"));
        let row = params(json!({ "name": "Ada" }));
        assert_eq!(loop_refusal(&p, Some(&row)), None);
        assert_eq!(
            loop_refusal(&params(json!({ "a": "{{item.x}}" })), None),
            None
        );
    }

    #[test]
    fn test_an_unclosed_placeholder_is_not_a_reference() {
        let p = params(json!({ "a": "{{item.x" }));
        assert!(unfilled_item_keys(&p, &None).is_empty());
    }
}
