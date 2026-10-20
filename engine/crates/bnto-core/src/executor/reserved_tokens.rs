// SEC-27: a processor that fills tokens of its own after the executor (shell-
// command's {{output_dir}}, {{url}}, each groups' {{path}}) can't tell which
// text came from the recipe and which from a value. So a value the executor
// fills in may not carry one of those tokens: filling never adds or removes
// them, so any token that appears more often after filling came from data.

use std::collections::HashMap;

use serde_json::Value;

use crate::errors::BntoError;

type Params = serde_json::Map<String, Value>;

/// Refuse when filling put one of `reserved` into `resolved` that `raw`
/// didn't have. An entry ending in `.` (`{{item.`) matches every token it
/// starts, each counted on its own.
pub(super) fn check_reserved(
    reserved: &[&str],
    raw: &Params,
    resolved: &Params,
    node_id: &str,
) -> Result<(), BntoError> {
    if reserved.is_empty() {
        return Ok(());
    }
    let before = counts(reserved, raw);
    for (token, after) in counts(reserved, resolved) {
        if after > before.get(&token).copied().unwrap_or(0) {
            return Err(BntoError::InvalidInput(format!(
                "Node '{node_id}': a value filled into its params contains '{token}', which \
                 this step fills itself, so it can't be told apart from the recipe's own"
            )));
        }
    }
    Ok(())
}

/// How often each reserved token appears in every string of `params`.
fn counts(reserved: &[&str], params: &Params) -> HashMap<String, usize> {
    let mut found = HashMap::new();
    params
        .values()
        .for_each(|v| count_in(v, reserved, &mut found));
    found
}

fn count_in(value: &Value, reserved: &[&str], found: &mut HashMap<String, usize>) {
    match value {
        Value::String(s) => count_tokens(s, reserved, found),
        Value::Array(items) => items.iter().for_each(|v| count_in(v, reserved, found)),
        Value::Object(map) => map.values().for_each(|v| count_in(v, reserved, found)),
        _ => {}
    }
}

fn count_tokens(s: &str, reserved: &[&str], found: &mut HashMap<String, usize>) {
    let mut rest = s;
    while let Some(start) = rest.find("{{") {
        let after = &rest[start..];
        let Some(end) = after.find("}}") else { return };
        let token = &after[..end + 2];
        if reserved
            .iter()
            .any(|r| token == *r || (r.ends_with('.') && token.starts_with(r)))
        {
            *found.entry(token.to_string()).or_default() += 1;
        }
        rest = &after[2..];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const RESERVED: &[&str] = &["{{output_dir}}", "{{url}}", "{{path}}", "{{item."];

    fn params(v: Value) -> Params {
        v.as_object().unwrap().clone()
    }

    fn check(raw: Value, resolved: Value) -> Result<(), BntoError> {
        check_reserved(RESERVED, &params(raw), &params(resolved), "n")
    }

    #[test]
    fn the_recipes_own_tokens_pass_through() {
        let raw =
            json!({ "args": ["-o", "{{output_dir}}/{{fields.name}}", { "each": ["{{path}}"] }] });
        let resolved = json!({ "args": ["-o", "{{output_dir}}/clip", { "each": ["{{path}}"] }] });
        assert!(check(raw, resolved).is_ok());
    }

    #[test]
    fn a_value_carrying_a_reserved_token_is_refused() {
        for value in ["{{output_dir}}", "x{{url}}", "{{path}}", "{{item.secret}}"] {
            let raw = json!({ "args": ["--title={{fields.t}}"] });
            let resolved = json!({ "args": [format!("--title={value}")] });
            let err = check(raw, resolved).unwrap_err().to_string();
            assert!(err.contains("fills itself"), "{value}: {err}");
        }
    }

    #[test]
    fn item_tokens_are_counted_one_by_one() {
        // A loop filled the recipe's {{item.x}}; a value brought in {{item.y}}.
        let raw = json!({ "args": ["{{item.x}}", "{{fields.v}}"] });
        let resolved = json!({ "args": ["1", "{{item.y}}"] });
        assert!(check(raw, resolved).is_err());
    }

    #[test]
    fn values_without_reserved_tokens_pass() {
        let raw = json!({ "args": ["{{fields.v}}"] });
        let resolved = json!({ "args": ["{{ not a token }} {{fields.other}}"] });
        assert!(check(raw, resolved).is_ok());
    }

    #[test]
    fn processors_without_reserved_tokens_skip_the_check() {
        let raw = params(json!({ "a": "{{fields.v}}" }));
        let resolved = params(json!({ "a": "{{output_dir}}" }));
        assert!(check_reserved(&[], &raw, &resolved, "n").is_ok());
    }
}
