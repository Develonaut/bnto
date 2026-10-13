// Shorthand constructors shared by the IO / container param builders.

use crate::metadata::{
    Constraints, OptionEntry, ParamCondition, ParamConditionEntry, ParameterDef, ParameterType,
};

/// A parameter with the four always-present fields set; callers fill the
/// rest with struct-update syntax (`..param(...)`).
pub(super) fn param(
    name: &str,
    label: &str,
    description: &str,
    param_type: ParameterType,
) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        param_type,
        ..Default::default()
    }
}

/// An `Enum` param type built from `(value, label)` pairs.
pub(super) fn enum_of(options: &[(&str, &str)]) -> ParameterType {
    ParameterType::Enum {
        options: options
            .iter()
            .map(|(value, label)| OptionEntry {
                value: (*value).to_string(),
                label: (*label).to_string(),
            })
            .collect(),
    }
}

/// `visibleWhen: { param, equals }` — single-condition shorthand.
pub(super) fn visible_when(param: &str, equals: &str) -> ParamCondition {
    ParamCondition::Single(entry(param, equals))
}

/// `visibleWhen: [{param, equals}, ...]` — OR logic shorthand.
pub(super) fn visible_when_any(conditions: &[(&str, &str)]) -> ParamCondition {
    ParamCondition::Any(conditions.iter().map(|(p, e)| entry(p, e)).collect())
}

fn entry(param: &str, equals: &str) -> ParamConditionEntry {
    ParamConditionEntry {
        param: param.to_string(),
        equals: equals.to_string(),
    }
}

/// Constraints with only the `required` flag set.
pub(super) fn required(required: bool) -> Constraints {
    Constraints {
        min: None,
        max: None,
        required,
    }
}

/// Constraints with a lower bound.
pub(super) fn min(min: f64, required: bool) -> Constraints {
    Constraints {
        min: Some(min),
        max: None,
        required,
    }
}
