// Parameter schema for spreadsheet-read, and typed reads of its values.

use bnto_core::metadata::{Constraints, OptionEntry, ParameterDef, ParameterType};

const DEFAULT_MAX_ROWS: u64 = 100_000;

type Params = serde_json::Map<String, serde_json::Value>;

/// How a file's content is parsed into rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Format {
    Csv,
    Json,
}

impl Format {
    /// Extension of the items read from a file of this format.
    pub(super) fn extension(self) -> &'static str {
        match self {
            Format::Csv => "csv",
            Format::Json => "json",
        }
    }

    pub(super) fn mime_type(self) -> &'static str {
        match self {
            Format::Csv => "text/csv",
            Format::Json => "application/json",
        }
    }
}

/// The node's params, read with their defaults.
pub(super) struct ReadConfig {
    pub has_headers: bool,
    pub delimiter: String,
    pub max_rows: u64,
    /// `None` is `auto`: chosen per file by [`ReadConfig::format_for`].
    pub format: Option<Format>,
}

impl ReadConfig {
    /// Read the params, or why `format` is invalid.
    pub(super) fn from_params(params: &Params) -> Result<Self, String> {
        let text = |key: &str, default| params.get(key).and_then(|v| v.as_str()).unwrap_or(default);
        Ok(Self {
            has_headers: params
                .get("hasHeaders")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            delimiter: text("delimiter", "comma").to_string(),
            max_rows: params
                .get("maxRows")
                .and_then(|v| v.as_u64())
                .unwrap_or(DEFAULT_MAX_ROWS),
            format: parse_format(text("format", "auto"))?,
        })
    }

    /// The set format, or under `auto` JSON for a `.json` name or a JSON MIME
    /// type and CSV otherwise. Content isn't sniffed: a CSV may start with `[`.
    pub(super) fn format_for(&self, filename: &str, mime_type: Option<&str>) -> Format {
        let json_name = bnto_core::file_extension(filename).eq_ignore_ascii_case("json");
        let json_mime = mime_type.is_some_and(|m| m.starts_with("application/json"));
        match self.format {
            Some(format) => format,
            None if json_name || json_mime => Format::Json,
            None => Format::Csv,
        }
    }
}

fn parse_format(value: &str) -> Result<Option<Format>, String> {
    match value {
        "auto" => Ok(None),
        "csv" => Ok(Some(Format::Csv)),
        "json" => Ok(Some(Format::Json)),
        other => Err(format!(
            "'format' must be \"auto\", \"csv\" or \"json\", got \"{other}\""
        )),
    }
}

/// What's wrong with `params` that the schema can't express.
pub(super) fn validate_params(params: &Params) -> Vec<String> {
    ReadConfig::from_params(params).err().into_iter().collect()
}

// --- Schema ---

/// All read parameters, in the order the editor displays them.
#[expect(clippy::too_many_lines)]
pub(super) fn build_read_parameters() -> Vec<ParameterDef> {
    vec![
        enum_param(
            "format",
            "Format",
            "How the file is read: auto picks JSON for a .json file and CSV otherwise",
            &[("auto", "Auto"), ("csv", "CSV"), ("json", "JSON array")],
        ),
        has_headers_param(),
        enum_param(
            "delimiter",
            "Delimiter",
            "Column separator character (CSV only)",
            &[
                ("comma", "Comma"),
                ("semicolon", "Semicolon"),
                ("tab", "Tab"),
                ("pipe", "Pipe"),
            ],
        ),
        max_rows_param(),
    ]
}

fn has_headers_param() -> ParameterDef {
    ParameterDef {
        name: "hasHeaders".to_string(),
        label: "Has Headers".to_string(),
        description: "First row contains column headers (CSV only)".to_string(),
        param_type: ParameterType::Boolean,
        default: Some(serde_json::json!(true)),
        ..Default::default()
    }
}

fn max_rows_param() -> ParameterDef {
    ParameterDef {
        name: "maxRows".to_string(),
        label: "Max Rows".to_string(),
        description:
            "Maximum rows (CSV rows or JSON array elements) to process. Error if the file has more."
                .to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::json!(DEFAULT_MAX_ROWS)),
        constraints: Some(Constraints {
            min: Some(1.0),
            max: Some(10_000_000.0),
            required: false,
        }),
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
