// Parameters for the `input` node: how data enters a recipe.

use super::helpers::{enum_of, min, param, visible_when, visible_when_any};
use crate::metadata::{ParamCondition, ParameterDef, ParameterType};

/// Canonical file extensions offered by the input extension picker.
const EXTENSIONS: [&str; 15] = [
    ".jpg", ".jpeg", ".png", ".webp", ".gif", ".svg", ".csv", ".json", ".txt", ".pdf", ".xml",
    ".zip", ".mp3", ".mp4", ".wav",
];

/// Parameters for the `input` node, in schema order.
pub(super) fn input_params() -> Vec<ParameterDef> {
    vec![
        mode(),
        accept(),
        extensions(),
        label(),
        multiple(),
        max_file_size(),
        max_files(),
        placeholder(),
    ]
}

fn when_file_upload() -> Option<ParamCondition> {
    Some(visible_when("mode", "file-upload"))
}

fn mode() -> ParameterDef {
    let modes = [
        ("file-upload", "File Upload"),
        ("text", "Text"),
        ("url", "URL"),
        ("none", "No Input"),
    ];
    let description = "How data is provided to the recipe.";
    ParameterDef {
        default: Some(serde_json::json!("file-upload")),
        control: Some("select".to_string()),
        ..param("mode", "Mode", description, enum_of(&modes))
    }
}

fn accept() -> ParameterDef {
    let description =
        "MIME types accepted (e.g., \"image/jpeg\", \"image/png\"). Derived from extensions.";
    ParameterDef {
        control: Some("tagPicker".to_string()),
        visible_when: when_file_upload(),
        ..param(
            "accept",
            "Accepted MIME Types",
            description,
            ParameterType::String,
        )
    }
}

fn extensions() -> ParameterDef {
    let options: Vec<(&str, &str)> = EXTENSIONS.iter().map(|e| (*e, *e)).collect();
    let description = "File extensions accepted (e.g., \".jpg\", \".png\").";
    ParameterDef {
        control: Some("tagPicker".to_string()),
        visible_when: when_file_upload(),
        ..param(
            "extensions",
            "File Extensions",
            description,
            enum_of(&options),
        )
    }
}

fn label() -> ParameterDef {
    let description = "Human-readable label for the input control.";
    ParameterDef {
        placeholder: Some("JPEG, PNG, or WebP images".to_string()),
        visible_when: when_file_upload(),
        ..param("label", "Label", description, ParameterType::String)
    }
}

fn multiple() -> ParameterDef {
    let description = "Whether multiple files or items are accepted.";
    ParameterDef {
        default: Some(serde_json::json!(true)),
        control: Some("switch".to_string()),
        visible_when: when_file_upload(),
        ..param("multiple", "Multiple", description, ParameterType::Boolean)
    }
}

fn max_file_size() -> ParameterDef {
    let description = "Maximum file size in bytes. 0 = no limit.";
    ParameterDef {
        default: Some(serde_json::json!(0)),
        constraints: Some(min(0.0, false)),
        visible_when: when_file_upload(),
        ..param(
            "maxFileSize",
            "Max File Size",
            description,
            ParameterType::Number,
        )
    }
}

fn max_files() -> ParameterDef {
    let description = "Maximum number of files. 0 = no limit.";
    ParameterDef {
        default: Some(serde_json::json!(0)),
        constraints: Some(min(0.0, false)),
        visible_when: when_file_upload(),
        ..param("maxFiles", "Max Files", description, ParameterType::Number)
    }
}

fn placeholder() -> ParameterDef {
    let description = "Placeholder text for text or URL input.";
    ParameterDef {
        visible_when: Some(visible_when_any(&[("mode", "text"), ("mode", "url")])),
        ..param(
            "placeholder",
            "Placeholder",
            description,
            ParameterType::String,
        )
    }
}
