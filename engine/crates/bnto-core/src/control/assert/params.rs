// Parameter schema for assert, and typed reads of its values.

use serde_json::Value;

use crate::compare::Op;
use crate::control::placeholder::text;
use crate::metadata::{Constraints, OptionEntry, ParameterDef, ParameterType};

/// The node's params, read and checked.
#[derive(Debug, Clone)]
pub(super) struct AssertParams {
    /// The left side, as text (a number or list from a sole placeholder too).
    pub value: String,
    pub op: Op,
    /// The right side, as text; empty when not set.
    pub expected: String,
    /// Put before the reasons in the error.
    pub message: Option<String>,
}

impl AssertParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &serde_json::Map<String, Value>) -> Result<Self, String> {
        let value = params
            .get("value")
            .filter(|v| !v.is_null())
            .ok_or("'value' is required: what the assert compares")?;
        let op = params
            .get("op")
            .and_then(Value::as_str)
            .ok_or("'op' is required: equals, not_equals, after or before")?;
        Ok(Self {
            value: text(value),
            op: Op::parse(op)
                .ok_or_else(|| format!("'op' \"{op}\" is not one of {}", Op::NAMES))?,
            expected: params.get("expected").map(text).unwrap_or_default(),
            message: params.get("message").map(text).filter(|m| !m.is_empty()),
        })
    }
}

impl AssertParams {
    /// The check as a line, e.g. `"{{node.refs.missing_count}}" equals "0"`.
    pub(super) fn describe(&self) -> String {
        format!(
            "\"{}\" {} \"{}\"",
            self.value,
            self.op.name(),
            self.expected
        )
    }
}

/// All assert parameters, in the order the editor displays them.
pub(super) fn build_assert_parameters() -> Vec<ParameterDef> {
    vec![
        required(text_param(
            "value",
            "Value",
            "What to check, usually a template such as {{node.<id>.<key>}}. With {{item.<key>}} \
             or {{file}} (the file's name) it is checked once per incoming file (inside a loop, \
             over the loop's item, which {{loop.<key>}} reads as it was), and no files means \
             nothing to check.",
        )),
        required(op_param()),
        text_param("expected", "Expected", "What the value is compared to."),
        text_param(
            "message",
            "Message",
            "Put before the reasons when the check fails, so the failure says what was checked.",
        ),
    ]
}

fn text_param(name: &str, label: &str, description: &str) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        param_type: ParameterType::String,
        ..Default::default()
    }
}

/// Mark a param required: `read` refuses a step without it.
fn required(param: ParameterDef) -> ParameterDef {
    ParameterDef {
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..param
    }
}

fn op_param() -> ParameterDef {
    let option = |value: &str, label: &str| OptionEntry {
        value: value.to_string(),
        label: label.to_string(),
    };
    ParameterDef {
        name: "op".to_string(),
        label: "Comparison".to_string(),
        description: "How value is compared to expected. after and before compare RFC 3339 times."
            .to_string(),
        param_type: ParameterType::Enum {
            options: vec![
                option("equals", "Equals"),
                option("not_equals", "Does not equal"),
                option("after", "Is after (time)"),
                option("before", "Is before (time)"),
            ],
        },
        default: Some(serde_json::json!("equals")),
        ..Default::default()
    }
}
