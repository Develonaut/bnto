// Recipe vars — `{{vars.KEY}}` values supplied by the caller for a whole run.
//
// Vars are fixed for the run, so they resolve in one pass over the recipe
// before anything executes. A placeholder no var sets is reported instead of
// left in place, so a run never acts on a path like `{{vars.Drive}}/Art`.

use serde_json::Value;

const OPEN: &str = "{{vars.";
const CLOSE: &str = "}}";

/// Replace every `{{vars.KEY}}` in the recipe's strings with `lookup(KEY)`.
///
/// Keys are case-sensitive. Returns the keys `lookup` doesn't set, sorted and
/// deduplicated; when any are missing the recipe must not run.
pub fn apply_vars(
    definition: &mut Value,
    lookup: &dyn Fn(&str) -> Option<String>,
) -> Result<(), Vec<String>> {
    let mut missing = Vec::new();
    resolve_value(definition, lookup, &mut missing);
    missing.sort();
    missing.dedup();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

/// Resolve vars in every string under `value`, recording keys no var sets.
fn resolve_value(
    value: &mut Value,
    lookup: &dyn Fn(&str) -> Option<String>,
    missing: &mut Vec<String>,
) {
    match value {
        Value::String(s) if s.contains(OPEN) => *s = resolve_string(s, lookup, missing),
        Value::Array(items) => items
            .iter_mut()
            .for_each(|v| resolve_value(v, lookup, missing)),
        Value::Object(map) => map
            .values_mut()
            .for_each(|v| resolve_value(v, lookup, missing)),
        _ => {}
    }
}

/// One left-to-right pass, so a var's value is never itself resolved.
fn resolve_string(
    s: &str,
    lookup: &dyn Fn(&str) -> Option<String>,
    missing: &mut Vec<String>,
) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find(OPEN) {
        let after = &rest[start + OPEN.len()..];
        let Some(end) = after.find(CLOSE) else { break };
        let key = &after[..end];
        out.push_str(&rest[..start]);
        match lookup(key) {
            Some(val) => out.push_str(&val),
            None => {
                missing.push(key.to_string());
                out.push_str(&rest[start..start + OPEN.len() + end + CLOSE.len()]);
            }
        }
        rest = &after[end + CLOSE.len()..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;

    fn lookup_from(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: BTreeMap<String, String> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |key| map.get(key).cloned()
    }

    #[test]
    fn resolves_a_var_inside_a_longer_string() {
        let mut def =
            json!({ "nodes": [{ "parameters": { "destination": "{{vars.Drive}}/Art" } }] });
        apply_vars(&mut def, &lookup_from(&[("Drive", "D:\\Assets")])).unwrap();
        assert_eq!(
            def["nodes"][0]["parameters"]["destination"],
            "D:\\Assets/Art"
        );
    }

    #[test]
    fn resolves_vars_at_any_depth_and_in_arrays() {
        let mut def = json!({ "nodes": [{ "nodes": [{ "parameters": { "paths": ["{{vars.A}}", "x/{{vars.B}}"] } }] }] });
        apply_vars(&mut def, &lookup_from(&[("A", "1"), ("B", "2")])).unwrap();
        assert_eq!(
            def["nodes"][0]["nodes"][0]["parameters"]["paths"],
            json!(["1", "x/2"])
        );
    }

    #[test]
    fn resolves_several_vars_in_one_string() {
        let mut def = json!({ "p": "{{vars.A}}-{{vars.B}}-{{vars.A}}" });
        apply_vars(&mut def, &lookup_from(&[("A", "a"), ("B", "b")])).unwrap();
        assert_eq!(def["p"], "a-b-a");
    }

    #[test]
    fn keys_are_case_sensitive() {
        let mut def = json!({ "p": "{{vars.drive}}" });
        let missing = apply_vars(&mut def, &lookup_from(&[("Drive", "D:")])).unwrap_err();
        assert_eq!(missing, vec!["drive".to_string()]);
    }

    #[test]
    fn reports_every_missing_key_once_sorted() {
        let mut def = json!({ "a": "{{vars.Zed}}", "b": ["{{vars.Alpha}}", "{{vars.Zed}}"], "c": "{{vars.Set}}" });
        let missing = apply_vars(&mut def, &lookup_from(&[("Set", "ok")])).unwrap_err();
        assert_eq!(missing, vec!["Alpha".to_string(), "Zed".to_string()]);
    }

    #[test]
    fn a_value_holding_a_placeholder_is_not_resolved_again() {
        let mut def = json!({ "p": "{{vars.A}}" });
        apply_vars(&mut def, &lookup_from(&[("A", "{{vars.A}}")])).unwrap();
        assert_eq!(def["p"], "{{vars.A}}");
    }

    #[test]
    fn leaves_other_namespaces_and_unclosed_placeholders_alone() {
        let mut def =
            json!({ "a": "{{env.HOME}} {{ctx.date}}", "b": "{{vars.Open", "n": 3, "t": true });
        let before = def.clone();
        apply_vars(&mut def, &lookup_from(&[])).unwrap();
        assert_eq!(def, before);
    }
}
