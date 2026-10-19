// `{{item.*}}` outside an `each` group. shell-command fills item values
// itself, inside groups, so outside a loop the executor hands it every
// `{{item.KEY}}` as written; one anywhere else has nothing to fill it.

use serde_json::{Map, Value};

use super::{ITEM_OPEN, read_group};

/// The error for the first `{{item.*}}` in `params` outside an `each`
/// group, if there is one.
pub(crate) fn unfilled_item(params: &Map<String, Value>) -> Option<String> {
    let outside_groups = params.iter().flat_map(|(name, value)| match value {
        Value::Array(args) if name == "args" => args
            .iter()
            .filter(|arg| !matches!(read_group(arg), Ok(Some(_))))
            .collect(),
        other => vec![other],
    });
    let found = outside_groups.into_iter().find(|v| reads_item(v))?;
    Some(format!(
        "{} has no value: shell-command fills {{{{item.*}}}} only inside an \
         \"each\" group in 'args'",
        first_item(found)
    ))
}

/// Whether `value` (or anything in it) reads `{{item.*}}`.
fn reads_item(value: &Value) -> bool {
    value.to_string().contains(ITEM_OPEN)
}

/// The first `{{item.KEY}}` placeholder in `value`, as written.
fn first_item(value: &Value) -> String {
    let text = value.to_string();
    let start = text.find(ITEM_OPEN).unwrap_or(0);
    let end = text[start..]
        .find("}}")
        .map_or(text.len(), |e| start + e + 2);
    text[start..end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn params(value: Value) -> Map<String, Value> {
        value.as_object().unwrap().clone()
    }

    #[test]
    fn test_items_inside_groups_are_fine() {
        let p = params(json!({ "args": ["a", { "each": ["{{item.size}}"] }] }));
        assert_eq!(unfilled_item(&p), None);
    }

    #[test]
    fn test_an_item_in_a_plain_arg_is_refused() {
        let p = params(json!({ "args": ["--size={{item.size}}"] }));
        assert_eq!(
            unfilled_item(&p).unwrap(),
            "{{item.size}} has no value: shell-command fills {{item.*}} only inside \
             an \"each\" group in 'args'"
        );
    }

    #[test]
    fn test_an_item_in_another_param_is_refused() {
        let p = params(json!({ "args": [], "env": { "SIZE": "{{item.size}}" } }));
        assert!(unfilled_item(&p).unwrap().starts_with("{{item.size}}"));
    }

    #[test]
    fn test_no_items_means_nothing_to_refuse() {
        let p = params(json!({ "command": "git", "args": ["status", ["-s"]] }));
        assert_eq!(unfilled_item(&p), None);
    }
}
