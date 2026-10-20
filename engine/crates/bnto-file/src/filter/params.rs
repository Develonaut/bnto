// file-filter parameter schema.

use bnto_core::metadata::{Constraints, OptionEntry, ParameterDef, ParameterType};

pub(super) fn build_filter_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "extensions".to_string(),
            label: "Extensions".to_string(),
            description: "Comma-separated list of file extensions to keep (e.g., \"jpg,png,svg\"). Leave empty to allow all extensions.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("jpg,png,svg".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "name_pattern".to_string(),
            label: "Name Pattern".to_string(),
            description: "Glob pattern to match against filenames (e.g., \"photo*\", \"*.backup.*\"). Uses glob syntax with * and ? wildcards.".to_string(),
            param_type: ParameterType::String,
            placeholder: Some("photo*".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "pattern_mode".to_string(),
            label: "Pattern Mode".to_string(),
            description: "How to interpret the name pattern.".to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    OptionEntry {
                        value: "glob".to_string(),
                        label: "Glob (*, ?)".to_string(),
                    },
                    OptionEntry {
                        value: "regex".to_string(),
                        label: "Regex".to_string(),
                    },
                ],
            },
            default: Some(serde_json::json!("glob")),
            ..Default::default()
        },
        ParameterDef {
            name: "match".to_string(),
            label: "Match Against".to_string(),
            description: "What the name pattern is tested against: the file's name, or its whole relative path (folders separated by /).".to_string(),
            param_type: ParameterType::Enum {
                options: vec![
                    OptionEntry {
                        value: "name".to_string(),
                        label: "File name".to_string(),
                    },
                    OptionEntry {
                        value: "path".to_string(),
                        label: "Relative path".to_string(),
                    },
                ],
            },
            default: Some(serde_json::json!("name")),
            ..Default::default()
        },
        ParameterDef {
            name: "keep".to_string(),
            label: "Keep Matches".to_string(),
            description: "When on, files matching every criterion are kept and the rest dropped. When off, matching files are dropped and the rest kept.".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            ..Default::default()
        },
        ParameterDef {
            name: "min_size".to_string(),
            label: "Minimum Size".to_string(),
            description: "Minimum file size in bytes. Files smaller than this are dropped.".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::json!(0)),
            constraints: Some(Constraints {
                min: Some(0.0),
                max: None,
                required: false,
            }),
            suffix: Some("bytes".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "max_size".to_string(),
            label: "Maximum Size".to_string(),
            description: "Maximum file size in bytes. Files larger than this are dropped. 0 means no limit.".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::json!(0)),
            constraints: Some(Constraints {
                min: Some(0.0),
                max: None,
                required: false,
            }),
            suffix: Some("bytes".to_string()),
            ..Default::default()
        },
    ]
}
