// Parameters for the `loop` container: forEach / times / while iteration.

use super::helpers::{enum_of, min, param, required, visible_when};
use crate::metadata::{ParameterDef, ParameterType};

/// Parameters for the `loop` node, in schema order.
pub(super) fn loop_params() -> Vec<ParameterDef> {
    vec![mode(), items(), count(), condition(), break_condition()]
}

fn mode() -> ParameterDef {
    let modes = [
        ("forEach", "For Each"),
        ("times", "N Times"),
        ("while", "While"),
    ];
    let description = "How the loop iterates: over items, N times, or while a condition holds.";
    ParameterDef {
        constraints: Some(required(true)),
        control: Some("select".to_string()),
        ..param("mode", "Mode", description, enum_of(&modes))
    }
}

fn items() -> ParameterDef {
    let description = "Optional template expression for the item source. The Rust engine iterates over the incoming file batch by default — this is only needed for custom data sources.";
    ParameterDef {
        placeholder: Some("{{index . \"list-files\" \"files\"}}".to_string()),
        visible_when: Some(visible_when("mode", "forEach")),
        ..param("items", "Items", description, ParameterType::String)
    }
}

fn count() -> ParameterDef {
    let description = "Number of times to repeat.";
    ParameterDef {
        constraints: Some(min(0.0, false)),
        visible_when: Some(visible_when("mode", "times")),
        required_when: Some(visible_when("mode", "times")),
        ..param("count", "Count", description, ParameterType::Number)
    }
}

fn condition() -> ParameterDef {
    let description = "Expr expression that must evaluate to true to continue looping.";
    ParameterDef {
        placeholder: Some("counter < 10".to_string()),
        visible_when: Some(visible_when("mode", "while")),
        required_when: Some(visible_when("mode", "while")),
        ..param("condition", "Condition", description, ParameterType::String)
    }
}

fn break_condition() -> ParameterDef {
    let description = "Optional expr expression — breaks out of the loop early when true.";
    ParameterDef {
        placeholder: Some("item.status == 'done'".to_string()),
        ..param(
            "breakCondition",
            "Break Condition",
            description,
            ParameterType::String,
        )
    }
}
