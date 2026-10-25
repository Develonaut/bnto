// Required params a recipe leaves without a value. A step can't do its job
// without them, so a run's pre-flight refuses them before any step starts,
// and `bnto check` reports them.

use serde_json::{Map, Value};

use crate::metadata::ParameterDef;

/// One required param with no value, and what to tell the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingParam {
    pub param: String,
    pub message: String,
}

/// Every param in `declared` marked required that `params` leaves without a
/// value, in declared order. Left out, a param with a default has that
/// default; given as null or blank text, it has nothing.
pub fn missing_required(
    params: &Map<String, Value>,
    declared: &[ParameterDef],
) -> Vec<MissingParam> {
    declared
        .iter()
        .filter(|def| def.constraints.as_ref().is_some_and(|c| c.required))
        .filter(|def| match params.get(&def.name) {
            None => def.default.is_none(),
            Some(value) => is_blank(value),
        })
        .map(|def| MissingParam {
            param: def.name.clone(),
            message: message(def),
        })
        .collect()
}

/// True for null and for text that is empty or only whitespace.
pub fn is_blank(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(text) => text.trim().is_empty(),
        _ => false,
    }
}

fn message(def: &ParameterDef) -> String {
    if def.label.is_empty() {
        format!("param \"{}\" is required", def.name)
    } else {
        format!("param \"{}\" ({}) is required", def.name, def.label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::{Constraints, ParameterType};
    use serde_json::json;

    fn def(name: &str, required: bool, default: Option<Value>) -> ParameterDef {
        ParameterDef {
            name: name.to_string(),
            label: format!("The {name}"),
            param_type: ParameterType::String,
            default,
            constraints: Some(Constraints {
                min: None,
                max: None,
                required,
            }),
            ..ParameterDef::default()
        }
    }

    fn params(value: Value) -> Map<String, Value> {
        value.as_object().cloned().unwrap()
    }

    fn names(found: &[MissingParam]) -> Vec<&str> {
        found.iter().map(|m| m.param.as_str()).collect()
    }

    #[test]
    fn a_required_param_with_a_value_passes() {
        let found = missing_required(
            &params(json!({ "logo": "abc" })),
            &[def("logo", true, None)],
        );
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn an_empty_required_param_is_missing_and_named_with_its_label() {
        let found = missing_required(&params(json!({ "logo": "" })), &[def("logo", true, None)]);
        assert_eq!(names(&found), ["logo"]);
        assert_eq!(found[0].message, "param \"logo\" (The logo) is required");
    }

    #[test]
    fn a_blank_or_null_required_param_is_missing() {
        let declared = [def("a", true, None), def("b", true, None)];
        let found = missing_required(&params(json!({ "a": "  ", "b": null })), &declared);
        assert_eq!(names(&found), ["a", "b"]);
    }

    #[test]
    fn an_absent_required_param_with_no_default_is_missing() {
        let found = missing_required(&Map::new(), &[def("logo", true, None)]);
        assert_eq!(names(&found), ["logo"]);
    }

    #[test]
    fn an_absent_required_param_falls_back_to_its_default() {
        let declared = [def("format", true, Some(json!("png")))];
        assert!(missing_required(&Map::new(), &declared).is_empty());
    }

    #[test]
    fn an_empty_value_doesnt_fall_back_to_the_default() {
        let declared = [def("format", true, Some(json!("png")))];
        let found = missing_required(&params(json!({ "format": "" })), &declared);
        assert_eq!(names(&found), ["format"]);
    }

    #[test]
    fn optional_params_may_be_empty_or_absent() {
        let declared = [def("a", false, None), def("b", false, None)];
        assert!(missing_required(&params(json!({ "a": "" })), &declared).is_empty());
    }

    #[test]
    fn a_placeholder_counts_as_a_value() {
        let found = missing_required(
            &params(json!({ "logo": "{{fields.logo}}" })),
            &[def("logo", true, None)],
        );
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn non_text_values_count_as_values() {
        let declared = [def("n", true, None), def("o", true, None)];
        let found = missing_required(&params(json!({ "n": 0, "o": {} })), &declared);
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn a_param_without_a_label_is_named_alone() {
        let mut bare = def("logo", true, None);
        bare.label = String::new();
        let found = missing_required(&Map::new(), &[bare]);
        assert_eq!(found[0].message, "param \"logo\" is required");
    }
}
