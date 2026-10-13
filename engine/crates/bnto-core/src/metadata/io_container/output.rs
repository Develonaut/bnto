// Parameters for the `output` node: how results leave a recipe.

use super::helpers::{enum_of, param, visible_when};
use crate::metadata::{ParamCondition, ParameterDef, ParameterType};

/// Parameters for the `output` node, in schema order.
pub(super) fn output_params() -> Vec<ParameterDef> {
    vec![
        mode(),
        directory(),
        filename(),
        zip(),
        label(),
        auto_download(),
    ]
}

fn when_write() -> Option<ParamCondition> {
    Some(visible_when("mode", "write"))
}

fn mode() -> ParameterDef {
    let modes = [
        ("write", "Write"),
        ("overwrite", "Overwrite"),
        ("message", "Message"),
        ("none", "None"),
    ];
    let description = "How results are delivered to the user.";
    ParameterDef {
        default: Some(serde_json::json!("write")),
        control: Some("select".to_string()),
        ..param("mode", "Mode", description, enum_of(&modes))
    }
}

fn directory() -> ParameterDef {
    let description = "Output directory path. Supports {{ctx.date}}, {{ctx.timestamp}} templates.";
    ParameterDef {
        default: Some(serde_json::json!("")),
        placeholder: Some("{{ctx.date}}-bulk-download".to_string()),
        visible_when: when_write(),
        ..param(
            "directory",
            "Output Directory",
            description,
            ParameterType::String,
        )
    }
}

fn filename() -> ParameterDef {
    let description = "Filename template for output files.";
    ParameterDef {
        placeholder: Some("compressed-{{name}}".to_string()),
        visible_when: when_write(),
        ..param(
            "filename",
            "Filename Template",
            description,
            ParameterType::String,
        )
    }
}

fn zip() -> ParameterDef {
    let description = "Auto-zip when there are multiple output files.";
    ParameterDef {
        default: Some(serde_json::json!(true)),
        control: Some("switch".to_string()),
        visible_when: when_write(),
        ..param("zip", "ZIP Multiple", description, ParameterType::Boolean)
    }
}

fn label() -> ParameterDef {
    let description = "Label for the download button or display section.";
    ParameterDef {
        placeholder: Some("Compressed Images".to_string()),
        ..param("label", "Label", description, ParameterType::String)
    }
}

fn auto_download() -> ParameterDef {
    let description = "Automatically download results on completion.";
    ParameterDef {
        default: Some(serde_json::json!(true)),
        control: Some("switch".to_string()),
        visible_when: when_write(),
        ..param(
            "autoDownload",
            "Auto-Download",
            description,
            ParameterType::Boolean,
        )
    }
}
