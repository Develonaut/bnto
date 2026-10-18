// Parameter schema for file-sort, and typed reads of its values.
//
// `by` is one line of text, not a list, so the editor's text control and
// `bnto check` both validate it: field names separated by commas, each
// ascending unless it starts with `-`.

use bnto_core::metadata::{Constraints, OptionEntry, ParameterDef, ParameterType};
use serde_json::Value;

/// Which way one key sorts its present values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Order {
    Asc,
    Desc,
}

/// One sort key: the metadata field it reads and its direction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SortKey {
    pub field: String,
    pub order: Order,
}

/// Whether text compares as written or lowercased.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Case {
    Sensitive,
    Insensitive,
}

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct SortParams {
    /// The keys in priority order: later keys break ties in earlier ones.
    pub keys: Vec<SortKey>,
    pub case: Case,
}

impl SortParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &serde_json::Map<String, Value>) -> Result<Self, String> {
        Ok(Self {
            keys: read_by(params.get("by"))?,
            case: read_case(params.get("case"))?,
        })
    }
}

/// `by` as its keys, in order.
fn read_by(value: Option<&Value>) -> Result<Vec<SortKey>, String> {
    let text = match value {
        None | Some(Value::Null) => "",
        Some(Value::String(s)) => s.as_str(),
        Some(other) => {
            return Err(format!(
                "'by' must be text: field names separated by commas, \
                 '-' before one to sort it descending (e.g. \"source, -title\"); got {other}"
            ));
        }
    };
    if text.trim().is_empty() {
        return Err("'by' is required: the fields to sort by, e.g. \"source, -title\"".to_string());
    }
    text.split(',').map(parse_key).collect()
}

/// One comma-separated entry: `field` or `-field`.
fn parse_key(entry: &str) -> Result<SortKey, String> {
    let entry = entry.trim();
    let (order, field) = match entry.strip_prefix('-') {
        Some(rest) => (Order::Desc, rest.trim()),
        None => (Order::Asc, entry),
    };
    if field.is_empty() {
        return Err("'by' has an empty sort key: give a field name between each comma".to_string());
    }
    Ok(SortKey {
        field: field.to_string(),
        order,
    })
}

fn read_case(value: Option<&Value>) -> Result<Case, String> {
    match value.and_then(Value::as_str) {
        None | Some("sensitive") => Ok(Case::Sensitive),
        Some("insensitive") => Ok(Case::Insensitive),
        Some(other) => Err(format!(
            "'case' must be \"sensitive\" or \"insensitive\", got \"{other}\""
        )),
    }
}

/// All file-sort parameters, in the order the editor displays them.
pub(super) fn build_sort_parameters() -> Vec<ParameterDef> {
    vec![by_param(), case_param()]
}

fn by_param() -> ParameterDef {
    ParameterDef {
        name: "by".to_string(),
        label: "Sort By".to_string(),
        description: "Metadata fields to sort by, separated by commas; later fields break \
                      ties. A leading '-' sorts that field descending. Numbers sort \
                      numerically; items missing a field (or with it null or empty) go last."
            .to_string(),
        param_type: ParameterType::String,
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        placeholder: Some("source, -title".to_string()),
        ..Default::default()
    }
}

fn case_param() -> ParameterDef {
    let option = |value: &str, label: &str| OptionEntry {
        value: value.to_string(),
        label: label.to_string(),
    };
    ParameterDef {
        name: "case".to_string(),
        label: "Letter Case".to_string(),
        description: "How text compares: as written, by Unicode code point (upper case \
                      before lower), or ignoring case."
            .to_string(),
        param_type: ParameterType::Enum {
            options: vec![
                option("sensitive", "Case-sensitive"),
                option("insensitive", "Ignore case"),
            ],
        },
        default: Some(Value::String("sensitive".to_string())),
        ..Default::default()
    }
}
