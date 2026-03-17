// Node metadata — self-describing processor definitions.
// Each processor declares its parameters, MIME types, category, and platform
// support. Powers the `node_catalog()` WASM export.

use serde::Serialize;

// =============================================================================
// ParamCondition — Conditional Visibility / Requirement Rules
// =============================================================================
//
// Some parameters only apply in certain contexts (e.g., "width" only matters
// when operation is "resize"). The engine declares these conditions as metadata
// so the UI can show/hide parameters dynamically.

/// A single condition entry: "when `param` has the value `equals`".
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParamConditionEntry {
    pub param: String,
    pub equals: String,
}

/// Conditional visibility/requirement rule for a parameter.
///
/// Uses `#[serde(untagged)]` so Single serializes as a plain object
/// and Any serializes as an array — no type discriminator field.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ParamCondition {
    /// Show/require when a single parameter matches a value.
    Single(ParamConditionEntry),
    /// Show/require when ANY of multiple conditions match (OR logic).
    Any(Vec<ParamConditionEntry>),
}

// =============================================================================
// NodeCategory
// =============================================================================

/// Broad category for UI grouping and filtering.
/// Serializes to kebab-case to match `@bnto/nodes` conventions.
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
    Io,
}

// =============================================================================
// ParameterType
// =============================================================================

/// The type of a node parameter. Determines what UI control to render.
/// Serializes with an internally tagged `"type"` field.
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ParameterType {
    Number,
    #[default]
    String,
    Boolean,
    Enum {
        options: Vec<std::string::String>,
    },
    Object,
}

// =============================================================================
// Constraints
// =============================================================================

/// Validation constraints on a parameter's value.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Constraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    pub required: bool,
}

// =============================================================================
// ParameterDef
// =============================================================================

/// Complete definition of one parameter a node accepts.
/// Drives both engine validation and UI control rendering.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDef {
    /// Key name as it appears in config JSON (e.g., "quality").
    pub name: std::string::String,

    /// Human-readable label for the UI.
    pub label: std::string::String,

    /// Description shown as tooltip or help text.
    pub description: std::string::String,

    /// Value type — determines the UI control.
    pub param_type: ParameterType,

    /// Default value when user doesn't provide one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,

    /// Placeholder text for string/number inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// Hidden parameters are engine wiring fields the user never sees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    /// Show this parameter only when another parameter has a specific value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_when: Option<ParamCondition>,

    /// Required only when another parameter has a specific value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_when: Option<ParamCondition>,

    /// Whether this parameter can be surfaced in container config panels.
    /// Defaults to `true` — only set `false` for internal wiring parameters.
    #[serde(default = "default_true")]
    pub surfaceable: bool,
}

/// Serde default helper — `#[serde(default = "...")]` requires a named function.
#[allow(dead_code)]
fn default_true() -> bool {
    true
}

/// Manual Default because `surfaceable` must default to `true`
/// (derived Default would give `false`).
impl Default for ParameterDef {
    fn default() -> Self {
        Self {
            name: String::default(),
            label: String::default(),
            description: String::default(),
            param_type: ParameterType::default(),
            default: None,
            constraints: None,
            placeholder: None,
            hidden: None,
            visible_when: None,
            required_when: None,
            surfaceable: true,
        }
    }
}

// =============================================================================
// NodeTypeInfo — Node-type-level metadata (all 12 types)
// =============================================================================
//
// Separate from NodeMetadata: NodeMetadata describes a PROCESSOR (one operation
// like "image:compress"). NodeTypeInfo describes a NODE TYPE (like "image") —
// the umbrella that may have multiple processors.
//
// Adding a new node type: add it to `all_node_types()`, run
// `task wasm:build` -> `task nodes:generate`, done.

/// Everything the UI needs to know about a node type — independent of any
/// specific processor/operation. Codegen generates TS `NODE_TYPE_INFO` from this.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeTypeInfo {
    /// Node type name as used in `.bnto.json` (e.g., "image", "file-system").
    pub name: String,
    pub label: String,
    pub description: String,
    pub category: NodeCategory,
    pub is_container: bool,
    /// Derived from processor registration. Types without processors yet
    /// (http-request, shell-command) list their expected platforms.
    pub platforms: Vec<String>,
    /// Lucide icon name — consumers resolve to their own icon component.
    pub icon: String,
}

/// Return metadata for all 12 registered node types (alphabetical order).
pub fn all_node_types() -> Vec<NodeTypeInfo> {
    vec![
        NodeTypeInfo {
            name: "edit-fields".to_string(),
            label: "Edit Fields".to_string(),
            description: "Set field values from static values or template expressions.".to_string(),
            category: NodeCategory::Data,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "pen-line".to_string(),
        },
        NodeTypeInfo {
            name: "file-system".to_string(),
            label: "File System".to_string(),
            description: "File operations: rename files with find/replace, case transforms, and patterns.".to_string(),
            category: NodeCategory::File,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "folder-open".to_string(),
        },
        NodeTypeInfo {
            name: "group".to_string(),
            label: "Group".to_string(),
            description: "Container for child nodes. Orchestrates sequential or parallel execution.".to_string(),
            category: NodeCategory::Control,
            is_container: true,
            platforms: vec!["browser".to_string()],
            icon: "box".to_string(),
        },
        NodeTypeInfo {
            name: "http-request".to_string(),
            label: "HTTP Request".to_string(),
            description: "Make HTTP requests to APIs (GET, POST, PUT, DELETE, etc.).".to_string(),
            category: NodeCategory::Network,
            is_container: false,
            platforms: vec!["server".to_string()],
            icon: "globe".to_string(),
        },
        NodeTypeInfo {
            name: "image".to_string(),
            label: "Image".to_string(),
            description: "Image processing: compress, resize, and convert formats.".to_string(),
            category: NodeCategory::Image,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "image".to_string(),
        },
        NodeTypeInfo {
            name: "input".to_string(),
            label: "Input".to_string(),
            description: "Declares how data enters the recipe. Read by the environment to render the appropriate input widget.".to_string(),
            category: NodeCategory::Io,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "file-up".to_string(),
        },
        NodeTypeInfo {
            name: "loop".to_string(),
            label: "Loop".to_string(),
            description: "Iterate over arrays (forEach), repeat N times, or loop while condition.".to_string(),
            category: NodeCategory::Control,
            is_container: true,
            platforms: vec!["browser".to_string()],
            icon: "repeat".to_string(),
        },
        NodeTypeInfo {
            name: "output".to_string(),
            label: "Output".to_string(),
            description: "Declares how results are delivered. Read by the environment to render the appropriate output widget.".to_string(),
            category: NodeCategory::Io,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "download".to_string(),
        },
        NodeTypeInfo {
            name: "parallel".to_string(),
            label: "Parallel".to_string(),
            description: "Execute tasks concurrently with configurable worker pool and error strategy.".to_string(),
            category: NodeCategory::Control,
            is_container: true,
            platforms: vec!["browser".to_string()],
            icon: "git-fork".to_string(),
        },
        NodeTypeInfo {
            name: "shell-command".to_string(),
            label: "Shell Command".to_string(),
            description: "Execute shell commands with stall detection, retry, and streaming output.".to_string(),
            category: NodeCategory::System,
            is_container: false,
            platforms: vec!["server".to_string()],
            icon: "terminal".to_string(),
        },
        NodeTypeInfo {
            name: "spreadsheet".to_string(),
            label: "Spreadsheet".to_string(),
            description: "Spreadsheet operations: clean data and rename columns.".to_string(),
            category: NodeCategory::Spreadsheet,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "sheet".to_string(),
        },
        NodeTypeInfo {
            name: "transform".to_string(),
            label: "Transform".to_string(),
            description: "Transform data using expressions (single value) or field mappings.".to_string(),
            category: NodeCategory::Data,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "arrow-left-right".to_string(),
        },
    ]
}

// =============================================================================
// NodeMetadata
// =============================================================================

/// Complete self-description of a processor. The `node_type` + `operation` pair
/// forms the compound registry key (e.g., "image:compress", "spreadsheet:clean").
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeMetadata {
    pub node_type: std::string::String,
    pub operation: std::string::String,
    pub name: std::string::String,
    pub description: std::string::String,
    pub category: NodeCategory,
    /// MIME types this processor handles. Empty = accepts any file type.
    pub accepts: Vec<std::string::String>,
    /// Platforms: "browser" (WASM, free), "server" (Cloud, Pro), "desktop" (Tauri).
    pub platforms: Vec<std::string::String>,
    pub parameters: Vec<ParameterDef>,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- NodeTypeInfo Tests ---

    #[test]
    fn test_all_node_types_returns_12_entries() {
        let types = all_node_types();
        assert_eq!(types.len(), 12, "Should have exactly 12 node types");
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
        assert_eq!(names.len(), 12, "All node type names should be unique");
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
        assert_eq!(server_only, vec!["http-request", "shell-command"]);
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
        assert_eq!(
            serde_json::to_string(&NodeCategory::Image).unwrap(),
            r#""image""#
        );
        assert_eq!(
            serde_json::to_string(&NodeCategory::Spreadsheet).unwrap(),
            r#""spreadsheet""#
        );
        assert_eq!(
            serde_json::to_string(&NodeCategory::File).unwrap(),
            r#""file""#
        );
        assert_eq!(serde_json::to_string(&NodeCategory::Io).unwrap(), r#""io""#);
    }

    #[test]
    fn test_parameter_type_number_serialization() {
        let json = serde_json::to_string(&ParameterType::Number).unwrap();
        assert_eq!(json, r#"{"type":"number"}"#);
    }

    #[test]
    fn test_parameter_type_enum_serialization() {
        let param = ParameterType::Enum {
            options: vec!["jpeg".to_string(), "png".to_string(), "webp".to_string()],
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""type":"enum""#));
        assert!(json.contains(r#""options":["jpeg","png","webp"]"#));
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
        assert!(!json.contains("hidden"));
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
        // Internal wiring params should be explicitly marked non-surfaceable.
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
            node_type: "image".to_string(),
            operation: "compress".to_string(),
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
        };
        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains(r#""nodeType":"image""#));
        assert!(json.contains(r#""platforms":["browser"]"#));
        assert!(!json.contains("node_type"));
    }

    #[test]
    fn test_full_metadata_round_trip() {
        // Verify serialize -> parse -> check fields round trip.
        let metadata = NodeMetadata {
            node_type: "image".to_string(),
            operation: "compress".to_string(),
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
        };

        let json = serde_json::to_string_pretty(&metadata).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["nodeType"], "image");
        assert_eq!(parsed["operation"], "compress");
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
        assert!(!json.contains("hidden"));
        assert!(!json.contains("requiredWhen"));
    }

    #[test]
    fn test_parameter_def_hidden_field_serialization() {
        let param = ParameterDef {
            name: "inputPath".to_string(),
            label: "Input Path".to_string(),
            description: "Internal path template".to_string(),
            param_type: ParameterType::String,
            default: None,
            constraints: None,
            hidden: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""hidden":true"#));
    }
}
