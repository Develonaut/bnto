// Parameter schema for summarize, and typed reads of its values.
//
// Both params are one line of text, field names separated by commas, like
// file-sort's `by`, so the editor's text control and `bnto check` both
// validate them.

use serde_json::{Map, Value};

use crate::metadata::{ParameterDef, ParameterType};

/// The field each group reports its item count in.
pub(super) const COUNT_FIELD: &str = "count";

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct SummarizeParams {
    /// Fields whose values together name an item's group; none puts every
    /// item in one group.
    pub group_by: Vec<String>,
    /// Numeric fields each group totals, as `sum_<field>`.
    pub sum: Vec<String>,
}

impl SummarizeParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &Map<String, Value>) -> Result<Self, String> {
        let read = Self {
            group_by: read_fields(params.get("groupBy"), "groupBy")?,
            sum: read_fields(params.get("sum"), "sum")?,
        };
        read.check_collisions()?;
        Ok(read)
    }

    /// Refuse a group field the node's own output fields would overwrite.
    fn check_collisions(&self) -> Result<(), String> {
        let written: Vec<String> = std::iter::once(COUNT_FIELD.to_string())
            .chain(self.sum.iter().map(|f| sum_field(f)))
            .collect();
        match self.group_by.iter().find(|f| written.contains(f)) {
            Some(f) => Err(format!(
                "'groupBy' field \"{f}\" collides with the \"{f}\" field summarize writes; \
                 rename it first (edit-fields can)"
            )),
            None => Ok(()),
        }
    }
}

/// The output field a summed `field` totals into.
pub(super) fn sum_field(field: &str) -> String {
    format!("sum_{field}")
}

/// A comma-separated field list; absent, null or blank is no fields.
fn read_fields(value: Option<&Value>, param: &str) -> Result<Vec<String>, String> {
    let text = match value {
        None | Some(Value::Null) => "",
        Some(Value::String(s)) => s.as_str(),
        Some(other) => {
            return Err(format!(
                "'{param}' must be text: field names separated by commas \
                 (e.g. \"type, year\"); got {other}"
            ));
        }
    };
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    text.split(',').map(|f| read_field(f, param)).collect()
}

fn read_field(entry: &str, param: &str) -> Result<String, String> {
    let field = entry.trim();
    if field.is_empty() {
        return Err(format!(
            "'{param}' has an empty field: give a field name between each comma"
        ));
    }
    Ok(field.to_string())
}

/// All summarize parameters, in the order the editor displays them.
pub(super) fn build_summarize_parameters() -> Vec<ParameterDef> {
    vec![group_by_param(), sum_param()]
}

fn group_by_param() -> ParameterDef {
    ParameterDef {
        name: "groupBy".to_string(),
        label: "Group By".to_string(),
        description: "Metadata fields to group items by, separated by commas; items whose \
                      values print the same share a group. Groups keep the order their \
                      first item arrived in. Items missing a field (or with it null or \
                      empty) group together, with that field null. Leave empty to \
                      summarize every item as one group."
            .to_string(),
        param_type: ParameterType::String,
        placeholder: Some("type, year".to_string()),
        ..Default::default()
    }
}

fn sum_param() -> ParameterDef {
    ParameterDef {
        name: "sum".to_string(),
        label: "Sum".to_string(),
        description: "Numeric metadata fields to total per group, separated by commas; each \
                      total is written as sum_<field>. Numbers written as text count; blank \
                      values are skipped; any other value fails the run, naming the item."
            .to_string(),
        param_type: ParameterType::String,
        placeholder: Some("file_size".to_string()),
        ..Default::default()
    }
}
