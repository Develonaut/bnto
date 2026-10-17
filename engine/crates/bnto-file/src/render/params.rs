// Parameter schema for file-render, and typed reads of its values.

use bnto_core::metadata::{Constraints, OptionEntry, ParameterDef, ParameterType};
use serde_json::Value;

use super::escape::Escape;
use super::fill::per_item_token;

const DEFAULT_SEPARATOR: &str = "\n";

/// The node's params, read and checked.
#[derive(Debug)]
pub(super) struct RenderParams {
    pub header: String,
    pub row: String,
    pub footer: String,
    pub separator: String,
    pub filename: String,
    pub escape: Escape,
}

type Params = serde_json::Map<String, Value>;

impl RenderParams {
    /// Read the params, or the reason they're invalid.
    pub(super) fn read(params: &Params) -> Result<Self, String> {
        let row = text(params, "row").ok_or("'row' is required: the block written per item")?;
        let filename =
            text(params, "filename").ok_or("'filename' is required: the file to write")?;
        let read = Self {
            header: text(params, "header").unwrap_or_default(),
            footer: text(params, "footer").unwrap_or_default(),
            separator: separator(params),
            escape: read_escape(params)?,
            row,
            filename,
        };
        read.refuse_per_item_tokens()?;
        Ok(read)
    }

    /// Header, footer and filename are written once, so they can't read an item.
    fn refuse_per_item_tokens(&self) -> Result<(), String> {
        let once = [
            ("header", &self.header),
            ("footer", &self.footer),
            ("filename", &self.filename),
        ];
        for (name, value) in once {
            if let Some(token) = per_item_token(value) {
                return Err(format!(
                    "'{name}' is written once, so it can't read {token}; use it in 'row'"
                ));
            }
        }
        Ok(())
    }
}

/// A text param; empty counts as not set.
fn text(params: &Params, key: &str) -> Option<String> {
    params
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// `separator` as written (empty joins rows directly); a newline when not set.
fn separator(params: &Params) -> String {
    params
        .get("separator")
        .and_then(Value::as_str)
        .unwrap_or(DEFAULT_SEPARATOR)
        .to_string()
}

fn read_escape(params: &Params) -> Result<Escape, String> {
    match text(params, "escape") {
        None => Ok(Escape::None),
        Some(name) => Escape::parse(&name)
            .ok_or_else(|| format!("'escape' \"{name}\" is not one of {}", Escape::NAMES)),
    }
}

/// All file-render parameters, in the order the editor displays them.
pub(super) fn build_render_parameters() -> Vec<ParameterDef> {
    vec![
        block_param("header", "Header", HEADER_DESCRIPTION, false),
        block_param("row", "Row", ROW_DESCRIPTION, true),
        block_param("footer", "Footer", FOOTER_DESCRIPTION, false),
        separator_param(),
        filename_param(),
        escape_param(),
    ]
}

const HEADER_DESCRIPTION: &str = "Written once before the rows. Reads {{count}} (the number \
     of items) and run templates such as {{vars.X}} or {{node.<id>.<key>}}.";

const ROW_DESCRIPTION: &str = "Written once per item. {{item.KEY}} is that item's metadata \
     value KEY (a key it lacks fails the run, naming the item); {{file}} is its file name.";

const FOOTER_DESCRIPTION: &str = "Written once after the rows. Reads what the header reads.";

fn block_param(name: &str, label: &str, description: &str, required: bool) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        control: Some("textarea".to_string()),
        constraints: required.then_some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

fn separator_param() -> ParameterDef {
    ParameterDef {
        name: "separator".to_string(),
        label: "Separator".to_string(),
        description: "Put between rows. Header, rows and footer are each on their own \
                      line, and the file ends with a newline."
            .to_string(),
        default: Some(serde_json::json!(DEFAULT_SEPARATOR)),
        ..Default::default()
    }
}

fn filename_param() -> ParameterDef {
    ParameterDef {
        name: "filename".to_string(),
        label: "File Name".to_string(),
        description: "Name of the file written, e.g. report.md (.md is text/markdown, \
                      anything else text/plain). Reads run templates, not {{item.*}}."
            .to_string(),
        placeholder: Some("report.md".to_string()),
        constraints: Some(Constraints {
            min: None,
            max: None,
            required: true,
        }),
        ..Default::default()
    }
}

fn escape_param() -> ParameterDef {
    let option = |value: &str, label: &str| OptionEntry {
        value: value.to_string(),
        label: label.to_string(),
    };
    ParameterDef {
        name: "escape".to_string(),
        label: "Escape Values".to_string(),
        description: "How values put into a row are escaped; the template text never is. \
                      markdown puts a backslash before \\ ` * _ [ ] < > | and turns line \
                      breaks into <br>, so a value can't break a table cell."
            .to_string(),
        param_type: ParameterType::Enum {
            options: vec![option("none", "None"), option("markdown", "Markdown")],
        },
        default: Some(serde_json::json!("none")),
        ..Default::default()
    }
}
