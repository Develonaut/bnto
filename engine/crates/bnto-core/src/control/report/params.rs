// Parameter schema for report, and typed reads of its values.

use serde_json::Value;

use crate::control::placeholder::{self, FILE, text};
use crate::metadata::{Constraints, ParameterDef, ParameterType};

/// The per-entry namespace: `{{entry.KEY}}` reads a field of `each`'s entry.
pub(super) const ENTRY: &str = "entry";
/// The whole entry, for a list of plain values (text, numbers).
pub(super) const WHOLE_ENTRY: &str = "{{entry}}";

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct ReportParams {
    /// The line to print, its `{{file}}` / `{{item.*}}` / `{{entry.*}}` still
    /// to fill.
    pub message: String,
    /// What to print a line per entry of; `None` when not set.
    pub each: Option<Value>,
}

impl ReportParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &serde_json::Map<String, Value>) -> Result<Self, String> {
        let message = params
            .get("message")
            .map(text)
            .filter(|m| !m.is_empty())
            .ok_or("'message' is required: the line to print")?;
        let each = params.get("each").filter(|v| !v.is_null()).cloned();
        check_each(each.as_ref())?;
        check_namespaces(&message, each.is_some())?;
        Ok(Self { message, each })
    }

    /// `each`'s entries, `None` when it isn't set; refused when it didn't
    /// resolve to a list.
    pub(super) fn entries(&self) -> Result<Option<&[Value]>, String> {
        match &self.each {
            None => Ok(None),
            Some(Value::Array(entries)) => Ok(Some(entries)),
            Some(other) => Err(not_a_list(other)),
        }
    }
}

/// `each` is a list, or text holding a placeholder that may resolve to one.
fn check_each(each: Option<&Value>) -> Result<(), String> {
    match each {
        None | Some(Value::Array(_)) => Ok(()),
        Some(Value::String(s)) if s.contains("{{") => Ok(()),
        Some(other) => Err(not_a_list(other)),
    }
}

fn not_a_list(value: &Value) -> String {
    format!("'each' must resolve to a list, got {value}")
}

/// `{{entry}}` needs `each`, and a line is per entry or per file, not both:
/// an entry line can't read `{{file}}`. It can read `{{item.*}}` inside a
/// loop, where that is the loop's item (`lines` refuses it outside one).
fn check_namespaces(message: &str, has_each: bool) -> Result<(), String> {
    let entry = placeholder::reads(message, ENTRY) || message.contains(WHOLE_ENTRY);
    if entry && !has_each {
        return Err("'message' reads {{entry}} but 'each' is not set: \
                    set 'each' to the list to print a line per entry of"
            .to_string());
    }
    if entry && message.contains(FILE) {
        return Err("'message' reads {{entry}} and {{file}}: \
                    a report prints a line per entry or per file, not both"
            .to_string());
    }
    Ok(())
}

/// All report parameters, in the order the editor displays them.
pub(super) fn build_report_parameters() -> Vec<ParameterDef> {
    vec![message_param(), each_param()]
}

fn message_param() -> ParameterDef {
    ParameterDef {
        name: "message".to_string(),
        label: "Message".to_string(),
        description: "The line to print, usually a template. {{file}} (the file's name) or \
                      {{item.<key>}} prints it once per file (inside a loop, over the loop's \
                      item, which {{loop.<key>}} reads as it was), {{entry.<key>}} (or \
                      {{entry}}) once per entry of each, where {{item.<key>}} is the loop's \
                      item."
            .to_string(),
        param_type: ParameterType::String,
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

fn each_param() -> ParameterDef {
    ParameterDef {
        name: "each".to_string(),
        label: "For Each".to_string(),
        description: "A list to print the message once per entry of, in order, usually a \
                      {{node.<id>.<key>}} template, or inside a loop the loop's {{item.<key>}}."
            .to_string(),
        param_type: ParameterType::Object,
        placeholder: Some("{{node.<id>.<key>}}".to_string()),
        ..Default::default()
    }
}
