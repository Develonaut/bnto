// Node Metadata — Self-describing processor definitions.
//
// Each processor implements `metadata()` on the `NodeProcessor` trait. The
// registry collects all metadata into a catalog exported via `node_catalog()`
// WASM function, making the engine the single source of truth for node defs.

use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

/// Parameter definitions for structural/declarative node types (input,
/// output, loop, group, parallel, transform, edit-fields).
pub mod io_container;

/// Per-category node type definitions and the aggregated registry.
pub mod node_types;

/// Parameter schema types — ParameterDef, Constraints, ParameterType, etc.
pub mod parameters;

// Re-export parameter types at this level for backward compatibility.
pub use node_types::{all_node_types, node_type_params};
pub use parameters::{
    Constraints, OptionEntry, ParamCondition, ParamConditionEntry, ParameterDef, ParameterType,
    PresetEntry,
};

// --- InputCardinality ---

/// Declares how a processor expects to receive files for smart iteration.
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(
    feature = "ts",
    ts(
        export,
        export_to = "../../../../packages/@bnto/nodes/src/generated/definitionTypes/"
    )
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum InputCardinality {
    /// Processes one file at a time.
    #[default]
    PerFile,
    /// Needs the full batch of files at once (e.g., zip, concat, merge).
    Batch,
    /// Processor generates output from its parameters — no input files.
    Source,
}

// --- NodeCategory ---

/// The broad category a node belongs to. Used for UI grouping and filtering.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum NodeCategory {
    Image,
    Spreadsheet,
    File,
    Data,
    Network,
    Control,
    System,
    Vector,
    Video,
    Io,
}

// --- NodeTypeInfo ---

/// Everything the UI needs to know about a node type, independent of any
/// specific processor/operation. The engine's authoritative type registry.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeTypeInfo {
    pub name: String,
    pub label: String,
    pub description: String,
    pub category: NodeCategory,
    pub is_container: bool,
    pub platforms: Vec<String>,
    /// Lucide icon name — consumers resolve to their own icon component.
    pub icon: String,
}

// --- Dependency ---

/// An external binary that a processor requires at runtime.
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(
    feature = "ts",
    ts(
        export,
        export_to = "../../../../packages/@bnto/nodes/src/generated/definitionTypes/"
    )
)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub binary: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    pub install_hint: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub homepage: String,
}

// --- NodeMetadata ---

/// Complete self-description of a processor. Return type of
/// `NodeProcessor::metadata()`.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeMetadata {
    pub node_type: std::string::String,
    pub name: std::string::String,
    pub description: std::string::String,
    pub category: NodeCategory,
    pub accepts: Vec<std::string::String>,
    pub platforms: Vec<std::string::String>,
    pub parameters: Vec<ParameterDef>,
    #[serde(default)]
    pub input_cardinality: InputCardinality,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<Dependency>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- InputCardinality Tests ---

    #[test]
    fn test_input_cardinality_defaults_to_per_file() {
        let cardinality = InputCardinality::default();
        assert_eq!(cardinality, InputCardinality::PerFile);
    }

    #[test]
    fn test_input_cardinality_serializes_camel_case() {
        let per_file = serde_json::to_string(&InputCardinality::PerFile).unwrap();
        assert_eq!(per_file, r#""perFile""#);

        let batch = serde_json::to_string(&InputCardinality::Batch).unwrap();
        assert_eq!(batch, r#""batch""#);

        let source = serde_json::to_string(&InputCardinality::Source).unwrap();
        assert_eq!(source, r#""source""#);
    }

    #[test]
    fn test_metadata_with_input_cardinality_round_trip() {
        let metadata = NodeMetadata {
            node_type: "image-compress".to_string(),
            name: "Compress Images".to_string(),
            description: "Reduce image file size".to_string(),
            category: NodeCategory::Image,
            accepts: vec!["image/jpeg".to_string()],
            platforms: vec!["browser".to_string()],
            parameters: vec![],
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        };
        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains(r#""inputCardinality":"perFile""#));

        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["inputCardinality"], "perFile");
    }

    #[test]
    fn test_metadata_with_batch_cardinality() {
        let metadata = NodeMetadata {
            node_type: "zip-files".to_string(),
            name: "Zip Files".to_string(),
            description: "Bundle files into a zip archive".to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["browser".to_string()],
            parameters: vec![],
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        };
        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains(r#""inputCardinality":"batch""#));
    }

    // --- Dependency Deserialization Tests ---

    #[test]
    fn test_dependency_deserializes_from_json() {
        let json = r#"{
            "binary": "yt-dlp",
            "installHint": "brew install yt-dlp",
            "homepage": "https://github.com/yt-dlp/yt-dlp"
        }"#;
        let dep: Dependency = serde_json::from_str(json).unwrap();
        assert_eq!(dep.binary, "yt-dlp");
        assert_eq!(dep.install_hint, "brew install yt-dlp");
        assert_eq!(dep.homepage, "https://github.com/yt-dlp/yt-dlp");
        assert!(dep.version.is_empty());
    }

    #[test]
    fn test_dependency_deserializes_with_version() {
        let json = r#"{
            "binary": "ffmpeg",
            "version": ">=6.0",
            "installHint": "brew install ffmpeg"
        }"#;
        let dep: Dependency = serde_json::from_str(json).unwrap();
        assert_eq!(dep.binary, "ffmpeg");
        assert_eq!(dep.version, ">=6.0");
        assert!(dep.homepage.is_empty());
    }

    #[test]
    fn test_dependency_round_trip() {
        let original = Dependency {
            binary: "yt-dlp".to_string(),
            version: ">=2024.0.0".to_string(),
            install_hint: "brew install yt-dlp".to_string(),
            homepage: "https://github.com/yt-dlp/yt-dlp".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let round_tripped: Dependency = serde_json::from_str(&json).unwrap();
        assert_eq!(original, round_tripped);
    }

    #[test]
    fn test_dependency_empty_optional_fields_omitted_in_serialization() {
        let dep = Dependency {
            binary: "curl".to_string(),
            version: String::new(),
            install_hint: "brew install curl".to_string(),
            homepage: String::new(),
        };
        let json = serde_json::to_string(&dep).unwrap();
        assert!(!json.contains("version"), "Empty version should be omitted");
        assert!(
            !json.contains("homepage"),
            "Empty homepage should be omitted"
        );
        assert!(json.contains("binary"));
        assert!(json.contains("installHint"));
    }

    // --- NodeTypeInfo Tests ---

    #[test]
    fn test_all_node_types_returns_26_entries() {
        let types = all_node_types();
        assert_eq!(types.len(), 26, "Should have exactly 26 node types");
    }

    #[test]
    fn test_all_node_types_sorted_alphabetically() {
        let types = all_node_types();
        let names: Vec<&str> = types.iter().map(|t| t.name.as_str()).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted, "Node types should be alphabetically sorted");
    }

    #[test]
    fn test_all_node_types_unique_names() {
        let types = all_node_types();
        let mut names: Vec<&str> = types.iter().map(|t| t.name.as_str()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 26, "All node type names should be unique");
    }

    #[test]
    fn test_container_types_are_group_loop_parallel() {
        let types = all_node_types();
        let mut containers: Vec<&str> = types
            .iter()
            .filter(|t| t.is_container)
            .map(|t| t.name.as_str())
            .collect();
        containers.sort();
        assert_eq!(containers, vec!["group", "loop", "parallel"]);
    }

    #[test]
    fn test_io_types_are_input_output() {
        let types = all_node_types();
        let mut io_types: Vec<&str> = types
            .iter()
            .filter(|t| t.category == NodeCategory::Io)
            .map(|t| t.name.as_str())
            .collect();
        io_types.sort();
        assert_eq!(io_types, vec!["input", "output"]);
    }

    #[test]
    fn test_server_only_types() {
        let types = all_node_types();
        let mut server_only: Vec<&str> = types
            .iter()
            .filter(|t| !t.platforms.contains(&"browser".to_string()))
            .map(|t| t.name.as_str())
            .collect();
        server_only.sort();
        assert_eq!(
            server_only,
            vec!["file-collect", "file-copy", "http-request", "shell-command"]
        );
    }

    #[test]
    fn test_node_type_info_serializes_camel_case() {
        let info = NodeTypeInfo {
            name: "image".to_string(),
            label: "Image".to_string(),
            description: "Image processing".to_string(),
            category: NodeCategory::Image,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "image".to_string(),
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains(r#""isContainer":false"#));
        assert!(!json.contains("is_container"));
    }

    // --- Serialization Tests ---

    #[test]
    fn test_category_serializes_to_kebab_case() {
        let json = serde_json::to_string(&NodeCategory::Image).unwrap();
        assert_eq!(json, r#""image""#);

        let json = serde_json::to_string(&NodeCategory::Spreadsheet).unwrap();
        assert_eq!(json, r#""spreadsheet""#);

        let json = serde_json::to_string(&NodeCategory::File).unwrap();
        assert_eq!(json, r#""file""#);

        let json = serde_json::to_string(&NodeCategory::Io).unwrap();
        assert_eq!(json, r#""io""#);

        let json = serde_json::to_string(&NodeCategory::Vector).unwrap();
        assert_eq!(json, r#""vector""#);

        let json = serde_json::to_string(&NodeCategory::Video).unwrap();
        assert_eq!(json, r#""video""#);
    }

    #[test]
    fn test_parameter_type_number_serialization() {
        let json = serde_json::to_string(&ParameterType::Number).unwrap();
        assert_eq!(json, r#"{"type":"number"}"#);
    }

    #[test]
    fn test_parameter_type_enum_serialization() {
        let param = ParameterType::Enum {
            options: vec![
                OptionEntry {
                    value: "jpeg".to_string(),
                    label: "JPEG".to_string(),
                },
                OptionEntry {
                    value: "png".to_string(),
                    label: "PNG".to_string(),
                },
                OptionEntry {
                    value: "webp".to_string(),
                    label: "WebP".to_string(),
                },
            ],
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""type":"enum""#));
        assert!(json.contains(r#""value":"jpeg""#));
        assert!(json.contains(r#""label":"JPEG""#));
        assert!(json.contains(r#""value":"webp""#));
        assert!(json.contains(r#""label":"WebP""#));
    }

    #[test]
    fn test_constraints_skips_none_fields() {
        let constraints = Constraints {
            min: Some(1.0),
            max: None,
            required: false,
        };
        let json = serde_json::to_string(&constraints).unwrap();
        assert!(json.contains(r#""min":1.0"#));
        assert!(!json.contains("max"));
        assert!(json.contains(r#""required":false"#));
    }

    #[test]
    fn test_constraints_includes_all_fields_when_present() {
        let constraints = Constraints {
            min: Some(1.0),
            max: Some(100.0),
            required: true,
        };
        let json = serde_json::to_string(&constraints).unwrap();
        assert!(json.contains(r#""min":1.0"#));
        assert!(json.contains(r#""max":100.0"#));
        assert!(json.contains(r#""required":true"#));
    }

    #[test]
    fn test_parameter_def_serializes_camel_case() {
        let param = ParameterDef {
            name: "quality".to_string(),
            label: "Quality".to_string(),
            description: "Compression quality".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::json!(80)),
            constraints: Some(Constraints {
                min: Some(1.0),
                max: Some(100.0),
                required: false,
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""paramType""#));
        assert!(!json.contains("param_type"));
    }

    #[test]
    fn test_parameter_def_skips_none_default() {
        let param = ParameterDef {
            name: "width".to_string(),
            label: "Width".to_string(),
            description: "Target width".to_string(),
            param_type: ParameterType::Number,
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(!json.contains("default"));
        assert!(!json.contains("constraints"));
        assert!(!json.contains("placeholder"));
        assert!(!json.contains("visibleWhen"));
        assert!(!json.contains("requiredWhen"));
    }

    #[test]
    fn test_parameter_def_surfaceable_defaults_to_true() {
        let param = ParameterDef {
            name: "quality".to_string(),
            label: "Quality".to_string(),
            description: "Compression quality".to_string(),
            param_type: ParameterType::Number,
            ..Default::default()
        };
        assert!(param.surfaceable, "surfaceable should default to true");
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""surfaceable":true"#));
    }

    #[test]
    fn test_parameter_def_surfaceable_false_serializes() {
        let param = ParameterDef {
            name: "items".to_string(),
            label: "Items".to_string(),
            description: "Template expression for iteration items".to_string(),
            param_type: ParameterType::String,
            surfaceable: false,
            ..Default::default()
        };
        assert!(!param.surfaceable);
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""surfaceable":false"#));
    }

    #[test]
    fn test_node_metadata_serializes_camel_case() {
        let metadata = NodeMetadata {
            node_type: "image-compress".to_string(),
            name: "Compress Images".to_string(),
            description: "Reduce image file size".to_string(),
            category: NodeCategory::Image,
            accepts: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/webp".to_string(),
            ],
            platforms: vec!["browser".to_string()],
            parameters: vec![],
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        };
        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains(r#""nodeType":"image-compress""#));
        assert!(json.contains(r#""platforms":["browser"]"#));
        assert!(!json.contains("node_type"));
    }

    #[test]
    fn test_full_metadata_round_trip() {
        let metadata = NodeMetadata {
            node_type: "image-compress".to_string(),
            name: "Compress Images".to_string(),
            description: "Reduce image file size while maintaining quality".to_string(),
            category: NodeCategory::Image,
            accepts: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/webp".to_string(),
            ],
            platforms: vec!["browser".to_string()],
            parameters: vec![ParameterDef {
                name: "quality".to_string(),
                label: "Quality".to_string(),
                description: "Compression quality (1-100)".to_string(),
                param_type: ParameterType::Number,
                default: Some(serde_json::json!(80)),
                constraints: Some(Constraints {
                    min: Some(1.0),
                    max: Some(100.0),
                    required: false,
                }),
                ..Default::default()
            }],
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        };

        let json = serde_json::to_string_pretty(&metadata).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["nodeType"], "image-compress");
        assert_eq!(parsed["category"], "image");
        assert_eq!(parsed["platforms"][0], "browser");
        assert_eq!(parsed["accepts"].as_array().unwrap().len(), 3);
        assert_eq!(parsed["parameters"].as_array().unwrap().len(), 1);
        assert_eq!(parsed["parameters"][0]["name"], "quality");
        assert_eq!(parsed["parameters"][0]["default"], 80);
    }

    // --- ParamCondition Serialization Tests ---

    #[test]
    fn test_param_condition_single_serializes_as_object() {
        let condition = ParamCondition::Single(ParamConditionEntry {
            param: "operation".to_string(),
            equals: "resize".to_string(),
        });
        let json = serde_json::to_string(&condition).unwrap();
        assert_eq!(json, r#"{"param":"operation","equals":"resize"}"#);
    }

    #[test]
    fn test_param_condition_any_serializes_as_array() {
        let condition = ParamCondition::Any(vec![
            ParamConditionEntry {
                param: "operation".to_string(),
                equals: "resize".to_string(),
            },
            ParamConditionEntry {
                param: "operation".to_string(),
                equals: "crop".to_string(),
            },
        ]);
        let json = serde_json::to_string(&condition).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_array(), "Any condition should be a JSON array");
        assert_eq!(parsed.as_array().unwrap().len(), 2);
        assert_eq!(parsed[0]["param"], "operation");
        assert_eq!(parsed[0]["equals"], "resize");
        assert_eq!(parsed[1]["equals"], "crop");
    }

    #[test]
    fn test_parameter_def_with_ui_fields_serializes_camel_case() {
        let param = ParameterDef {
            name: "width".to_string(),
            label: "Width".to_string(),
            description: "Target width in pixels".to_string(),
            param_type: ParameterType::Number,
            default: None,
            constraints: None,
            placeholder: Some("e.g. 800".to_string()),
            visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                param: "operation".to_string(),
                equals: "resize".to_string(),
            })),
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""visibleWhen""#));
        assert!(!json.contains("visible_when"));
        assert!(json.contains(r#""placeholder":"e.g. 800""#));
        assert!(!json.contains("requiredWhen"));
    }

    // --- Dependency Tests ---

    #[test]
    fn test_dependency_serializes_camel_case() {
        let dep = Dependency {
            binary: "yt-dlp".to_string(),
            version: ">=2023.01.01".to_string(),
            install_hint: "brew install yt-dlp".to_string(),
            homepage: "https://github.com/yt-dlp/yt-dlp".to_string(),
        };
        let json = serde_json::to_string(&dep).unwrap();
        assert!(json.contains(r#""binary":"yt-dlp""#));
        assert!(json.contains(r#""version":">=2023.01.01""#));
        assert!(json.contains(r#""installHint":"brew install yt-dlp""#));
        assert!(json.contains(r#""homepage":"https://github.com/yt-dlp/yt-dlp""#));
        assert!(!json.contains("install_hint"));
    }

    #[test]
    fn test_dependency_skips_empty_optional_fields() {
        let dep = Dependency {
            binary: "ffmpeg".to_string(),
            version: String::new(),
            install_hint: "brew install ffmpeg".to_string(),
            homepage: String::new(),
        };
        let json = serde_json::to_string(&dep).unwrap();
        assert!(!json.contains("version"), "empty version should be omitted");
        assert!(
            !json.contains("homepage"),
            "empty homepage should be omitted"
        );
        assert!(json.contains(r#""binary":"ffmpeg""#));
        assert!(json.contains(r#""installHint""#));
    }

    #[test]
    fn test_dependency_equality() {
        let a = Dependency {
            binary: "yt-dlp".to_string(),
            version: ">=2023.01.01".to_string(),
            install_hint: "brew install yt-dlp".to_string(),
            homepage: "https://github.com/yt-dlp/yt-dlp".to_string(),
        };
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_metadata_requires_empty_skipped_in_serialization() {
        let metadata = NodeMetadata {
            node_type: "image-compress".to_string(),
            name: "Compress".to_string(),
            description: String::new(),
            category: NodeCategory::Image,
            accepts: vec![],
            platforms: vec!["browser".to_string()],
            parameters: vec![],
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        };
        let json = serde_json::to_string(&metadata).unwrap();
        assert!(
            !json.contains("requires"),
            "empty requires should be omitted"
        );
    }

    #[test]
    fn test_metadata_requires_present_when_populated() {
        let metadata = NodeMetadata {
            node_type: "video-download".to_string(),
            name: "Download Video".to_string(),
            description: String::new(),
            category: NodeCategory::Network,
            accepts: vec![],
            platforms: vec!["server".to_string()],
            parameters: vec![],
            input_cardinality: InputCardinality::PerFile,
            requires: vec![Dependency {
                binary: "yt-dlp".to_string(),
                version: ">=2023.01.01".to_string(),
                install_hint: "brew install yt-dlp".to_string(),
                homepage: "https://github.com/yt-dlp/yt-dlp".to_string(),
            }],
        };
        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains(r#""requires""#));
        assert!(json.contains(r#""binary":"yt-dlp""#));
        assert!(json.contains(r#""version":">=2023.01.01""#));
    }

    // --- Presentation Metadata Tests ---

    #[test]
    fn test_preset_entry_serializes_value_and_label() {
        let preset = PresetEntry {
            value: serde_json::json!(80),
            label: "Balanced".to_string(),
        };
        let json = serde_json::to_string(&preset).unwrap();
        assert!(json.contains(r#""value":80"#));
        assert!(json.contains(r#""label":"Balanced""#));
    }

    #[test]
    fn test_preset_entry_accepts_heterogeneous_values() {
        let string_preset = PresetEntry {
            value: serde_json::json!("jpeg"),
            label: "JPEG".to_string(),
        };
        let json = serde_json::to_string(&string_preset).unwrap();
        assert!(json.contains(r#""value":"jpeg""#));
    }

    #[test]
    fn test_option_entry_serializes_value_and_label() {
        let option = OptionEntry {
            value: "snake".to_string(),
            label: "snake_case".to_string(),
        };
        let json = serde_json::to_string(&option).unwrap();
        assert_eq!(json, r#"{"value":"snake","label":"snake_case"}"#);
    }

    #[test]
    fn test_parameter_def_presets_round_trip() {
        let param = ParameterDef {
            name: "quality".to_string(),
            label: "Quality".to_string(),
            description: "Compression quality".to_string(),
            param_type: ParameterType::Number,
            presets: Some(vec![
                PresetEntry {
                    value: serde_json::json!(60),
                    label: "Draft".to_string(),
                },
                PresetEntry {
                    value: serde_json::json!(80),
                    label: "Balanced".to_string(),
                },
                PresetEntry {
                    value: serde_json::json!(100),
                    label: "Maximum".to_string(),
                },
            ]),
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let presets = parsed["presets"].as_array().unwrap();
        assert_eq!(presets.len(), 3);
        assert_eq!(presets[0]["value"], 60);
        assert_eq!(presets[0]["label"], "Draft");
        assert_eq!(presets[1]["label"], "Balanced");
        assert_eq!(presets[2]["value"], 100);
    }

    #[test]
    fn test_parameter_def_group_and_suffix_round_trip() {
        let param = ParameterDef {
            name: "width".to_string(),
            label: "Width".to_string(),
            description: "Target width".to_string(),
            param_type: ParameterType::Number,
            group: Some("dimensions".to_string()),
            suffix: Some("px".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""group":"dimensions""#));
        assert!(json.contains(r#""suffix":"px""#));
    }

    #[test]
    fn test_parameter_def_control_and_accept_round_trip() {
        let param = ParameterDef {
            name: "image".to_string(),
            label: "Watermark image".to_string(),
            description: "Image to overlay".to_string(),
            param_type: ParameterType::String,
            control: Some("file".to_string()),
            accept: Some(vec!["image/*".to_string()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""control":"file""#));
        assert!(json.contains(r#""accept":["image/*"]"#));
    }

    #[test]
    fn test_parameter_def_control_without_accept() {
        let param = ParameterDef {
            name: "preview".to_string(),
            label: "Preview".to_string(),
            description: "Watermark preview".to_string(),
            param_type: ParameterType::String,
            control: Some("watermarkPreview".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""control":"watermarkPreview""#));
        assert!(!json.contains("accept"));
    }

    #[test]
    fn test_parameter_def_inverted_round_trip() {
        let param = ParameterDef {
            name: "stripExif".to_string(),
            label: "Keep metadata".to_string(),
            description: "Preserve EXIF metadata".to_string(),
            param_type: ParameterType::Boolean,
            inverted: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""inverted":true"#));
    }

    #[test]
    fn test_parameter_def_new_fields_skip_none() {
        let param = ParameterDef {
            name: "quality".to_string(),
            label: "Quality".to_string(),
            description: "Compression quality".to_string(),
            param_type: ParameterType::Number,
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(!json.contains("\"group\""));
        assert!(!json.contains("\"suffix\""));
        assert!(!json.contains("\"control\""));
        assert!(!json.contains("\"accept\""));
        assert!(!json.contains("\"presets\""));
        assert!(!json.contains("\"inverted\""));
    }

    #[test]
    fn test_parameter_def_default_new_fields_are_none() {
        let param = ParameterDef::default();
        assert!(param.group.is_none());
        assert!(param.suffix.is_none());
        assert!(param.control.is_none());
        assert!(param.accept.is_none());
        assert!(param.presets.is_none());
        assert!(param.inverted.is_none());
    }

    #[test]
    fn test_parameter_def_new_fields_use_camel_case() {
        let param = ParameterDef {
            name: "image".to_string(),
            label: "Image".to_string(),
            description: "Overlay image".to_string(),
            param_type: ParameterType::String,
            control: Some("file".to_string()),
            accept: Some(vec!["image/png".to_string()]),
            group: Some("media".to_string()),
            suffix: Some("%".to_string()),
            inverted: Some(false),
            presets: Some(vec![PresetEntry {
                value: serde_json::json!(80),
                label: "Balanced".to_string(),
            }]),
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        for key in [
            "control", "accept", "group", "suffix", "inverted", "presets",
        ] {
            let needle = format!(r#""{key}""#);
            assert!(
                json.contains(&needle),
                "expected serialized param to contain {needle}; got: {json}"
            );
        }
    }
}
