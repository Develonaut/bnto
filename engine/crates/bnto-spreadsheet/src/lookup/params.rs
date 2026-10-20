// Parameter schema for spreadsheet-lookup, and typed reads of its values.

use bnto_core::metadata::{Constraints, OptionEntry, ParameterDef, ParameterType};

/// What part of a file's pipeline name is compared with the key column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MatchOn {
    /// The whole name, relative folders included (`Pack/sub/a.wav`).
    Path,
    /// The name after the last `/` (`a.wav`).
    Filename,
}

/// What happens to a file whose key has no row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Unmatched {
    /// Pass it on with `unmatched: 1`.
    Keep,
    /// Leave it out of the output.
    Drop,
    /// Fail the run, naming the file.
    Fail,
}

/// The node's params, read and checked.
pub(super) struct LookupParams {
    pub table: String,
    pub key_column: String,
    pub match_on: MatchOn,
    pub unmatched: Unmatched,
}

type Params = serde_json::Map<String, serde_json::Value>;

impl LookupParams {
    /// Read the params, or every reason they're invalid.
    pub(super) fn read(params: &Params) -> Result<Self, Vec<String>> {
        let table = require_table(params);
        let match_on = parse_match_on(text(params, "matchOn"));
        let unmatched = parse_unmatched(text(params, "unmatched"));
        match (table, match_on, unmatched) {
            (Ok(table), Ok(match_on), Ok(unmatched)) => Ok(Self {
                table,
                key_column: text(params, "keyColumn")
                    .unwrap_or(DEFAULT_KEY_COLUMN)
                    .to_string(),
                match_on,
                unmatched,
            }),
            (table, match_on, unmatched) => Err([table.err(), match_on.err(), unmatched.err()]
                .into_iter()
                .flatten()
                .collect()),
        }
    }
}

fn require_table(params: &Params) -> Result<String, String> {
    text(params, "table")
        .map(str::to_string)
        .ok_or_else(|| "'table' is required: the path of the CSV to look rows up in".to_string())
}

/// A non-empty string param.
fn text<'a>(params: &'a Params, key: &str) -> Option<&'a str> {
    params
        .get(key)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
}

const DEFAULT_KEY_COLUMN: &str = "name";

fn parse_match_on(value: Option<&str>) -> Result<MatchOn, String> {
    match value {
        None | Some("path") => Ok(MatchOn::Path),
        Some("filename") => Ok(MatchOn::Filename),
        Some(other) => Err(format!(
            "'matchOn' must be \"path\" or \"filename\", got \"{other}\""
        )),
    }
}

fn parse_unmatched(value: Option<&str>) -> Result<Unmatched, String> {
    match value {
        None | Some("keep") => Ok(Unmatched::Keep),
        Some("drop") => Ok(Unmatched::Drop),
        Some("fail") => Ok(Unmatched::Fail),
        Some(other) => Err(format!(
            "'unmatched' must be \"keep\", \"drop\" or \"fail\", got \"{other}\""
        )),
    }
}

// --- Schema ---

/// All lookup parameters, in the order the editor displays them.
pub(super) fn build_lookup_parameters() -> Vec<ParameterDef> {
    vec![
        table_param(),
        key_column_param(),
        enum_param(
            "matchOn",
            "Match On",
            "Which part of the file's name must equal the key exactly",
            &[
                ("path", "Path (relative folders included)"),
                ("filename", "Filename only"),
            ],
        ),
        enum_param(
            "unmatched",
            "Unmatched Files",
            "What happens to a file with no row: keep it (flagged unmatched: 1), drop it, or fail the run",
            &[
                ("keep", "Keep and flag"),
                ("drop", "Drop"),
                ("fail", "Fail the run"),
            ],
        ),
    ]
}

fn table_param() -> ParameterDef {
    ParameterDef {
        name: "table".to_string(),
        label: "Table".to_string(),
        description: "Path of the CSV (with a header row) to look each file up in".to_string(),
        placeholder: Some("mapping.csv".to_string()),
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

fn key_column_param() -> ParameterDef {
    ParameterDef {
        name: "keyColumn".to_string(),
        label: "Key Column".to_string(),
        description: "Table column matched against each file; a key may appear on one row only"
            .to_string(),
        default: Some(serde_json::json!(DEFAULT_KEY_COLUMN)),
        ..Default::default()
    }
}

/// An enum param whose first option is its default.
fn enum_param(name: &str, label: &str, desc: &str, options: &[(&str, &str)]) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: desc.to_string(),
        param_type: ParameterType::Enum {
            options: options
                .iter()
                .map(|(value, label)| OptionEntry {
                    value: value.to_string(),
                    label: label.to_string(),
                })
                .collect(),
        },
        default: Some(serde_json::json!(options[0].0)),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn read(v: serde_json::Value) -> Result<LookupParams, Vec<String>> {
        LookupParams::read(v.as_object().unwrap())
    }

    #[test]
    fn test_defaults() {
        let p = read(json!({ "table": "t.csv" })).ok().unwrap();
        assert_eq!(p.key_column, "name");
        assert_eq!(p.match_on, MatchOn::Path);
        assert_eq!(p.unmatched, Unmatched::Keep);
    }

    #[test]
    fn test_table_is_required() {
        let errors = read(json!({ "table": "" })).err().unwrap();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("'table'"));
    }

    #[test]
    fn test_every_bad_value_is_reported() {
        let errors = read(json!({ "matchOn": "stem", "unmatched": "skip" }))
            .err()
            .unwrap();
        assert_eq!(errors.len(), 3, "{errors:?}");
    }

    #[test]
    fn test_schema_defaults_match_the_reader() {
        let defs = build_lookup_parameters();
        let default = |n: &str| defs.iter().find(|d| d.name == n).unwrap().default.clone();
        assert_eq!(default("keyColumn"), Some(json!("name")));
        assert_eq!(default("matchOn"), Some(json!("path")));
        assert_eq!(default("unmatched"), Some(json!("keep")));
    }
}
