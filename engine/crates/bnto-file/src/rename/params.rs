// Parameter schema for the rename node — pure data consumed by `metadata()`.

use bnto_core::metadata::{Constraints, OptionEntry, ParameterDef, ParameterType};

/// All rename parameters, in the order the editor displays them.
pub(super) fn build_rename_parameters() -> Vec<ParameterDef> {
    vec![
        rename_sanitize_param(),
        rename_separator_param(),
        rename_max_length_param(),
        rename_find_param(),
        rename_string_param("replace", "Replace", "Replacement text (used with Find)"),
        rename_case_param(),
        rename_prefix_param(),
        rename_suffix_param(),
        rename_pattern_param(),
        rename_counter_start_param(),
        rename_counter_pad_param(),
        rename_extension_param(),
    ]
}

fn rename_find_param() -> ParameterDef {
    rename_string_param(
        "find",
        "Find",
        "Text or regex pattern to search for in the whole relative path, folders included",
    )
}

fn rename_prefix_param() -> ParameterDef {
    rename_string_param(
        "prefix",
        "Prefix",
        "Text to prepend to the file name (not its folders)",
    )
}

fn rename_suffix_param() -> ParameterDef {
    rename_string_param(
        "suffix",
        "Suffix",
        "Text to append to the file name, before its extension",
    )
}

/// A simple string parameter for the rename node.
fn rename_string_param(name: &str, label: &str, desc: &str) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: desc.to_string(),
        ..Default::default()
    }
}

/// An enum parameter with `(value, label)` options.
fn rename_enum_param(
    name: &str,
    label: &str,
    desc: &str,
    options: &[(&str, &str)],
) -> ParameterDef {
    let options = options
        .iter()
        .map(|(value, label)| OptionEntry {
            value: value.to_string(),
            label: label.to_string(),
        })
        .collect();
    ParameterDef {
        param_type: ParameterType::Enum { options },
        ..rename_string_param(name, label, desc)
    }
}

/// A non-negative integer parameter with an integer default and optional max.
fn rename_number_param(
    name: &str,
    label: &str,
    desc: &str,
    default: u64,
    max: Option<f64>,
) -> ParameterDef {
    ParameterDef {
        param_type: ParameterType::Number,
        default: Some(serde_json::Value::Number(serde_json::Number::from(default))),
        constraints: Some(Constraints {
            min: Some(0.0),
            max,
            required: false,
        }),
        ..rename_string_param(name, label, desc)
    }
}

fn rename_case_param() -> ParameterDef {
    rename_enum_param(
        "case",
        "Case",
        "Transform the file name (not its folders) to a specific case",
        &[
            ("lower", "lowercase"),
            ("upper", "UPPERCASE"),
            ("title", "Title Case"),
        ],
    )
}

fn rename_pattern_param() -> ParameterDef {
    ParameterDef {
        placeholder: Some("{{name}}-{{counter}}.{{ext}}".to_string()),
        ..rename_string_param(
            "pattern",
            "Pattern",
            "Template for the whole relative path, folders included (supports {{name}}, {{ext}}, {{index}}, {{date}}, {{counter}}). Replaces the other name transforms; without {{name}} it drops the folders",
        )
    }
}

fn rename_counter_start_param() -> ParameterDef {
    rename_number_param(
        "counterStart",
        "Counter Start",
        "Starting number for the {{counter}} variable",
        1,
        None,
    )
}

fn rename_counter_pad_param() -> ParameterDef {
    rename_number_param(
        "counterPad",
        "Counter Padding",
        "Zero-pad width for the counter (e.g., 3 → 001, 002)",
        0,
        Some(10.0),
    )
}

fn rename_extension_param() -> ParameterDef {
    ParameterDef {
        placeholder: Some("png".to_string()),
        ..rename_string_param(
            "extension",
            "Extension",
            "Replace the file extension (without dot)",
        )
    }
}

fn rename_sanitize_param() -> ParameterDef {
    rename_enum_param(
        "sanitize",
        "Sanitize",
        "Clean the file name (not its folders) before other transforms. To fold folders into the name, replace \"/\" with \"-\" using Find in an earlier rename step",
        &[
            ("slugify", "Slugify (web-safe)"),
            ("strip", "Strip non-ASCII"),
            ("normalize", "Normalize Unicode"),
        ],
    )
}

fn rename_separator_param() -> ParameterDef {
    ParameterDef {
        default: Some(serde_json::Value::String("-".to_string())),
        ..rename_string_param(
            "separator",
            "Separator",
            "Character to replace spaces and special characters in the file name (used with Sanitize)",
        )
    }
}

fn rename_max_length_param() -> ParameterDef {
    rename_number_param(
        "maxLength",
        "Max Length",
        "Maximum file name length after sanitize, folders not counted (0 = no limit)",
        0,
        None,
    )
}
