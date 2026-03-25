// Node Metadata — Self-describing processor definitions.
//
// Each processor implements `metadata()` on the `NodeProcessor` trait. The
// registry collects all metadata into a catalog exported via `node_catalog()`
// WASM function, making the engine the single source of truth for node defs.

use serde::Serialize;

// --- ParamCondition — Conditional Visibility / Requirement Rules ---
//
// Declares when a parameter should be shown/required based on other param values.
// `Single` = one condition, `Any` = OR logic across multiple conditions.

/// A single condition entry: "when `param` has the value `equals`".
///
/// Used both standalone (in `ParamCondition::Single`) and as entries
/// in the `ParamCondition::Any` array.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParamConditionEntry {
    /// The name of the parameter to check against.
    /// Example: `"operation"` — check the value of the "operation" parameter.
    pub param: String,

    /// The value that triggers visibility/requirement.
    /// Example: `"resize"` — only show this param when operation is "resize".
    pub equals: String,
}

/// Conditional visibility/requirement rule for a parameter.
///
/// Tells the UI when to show a parameter or when to make it required.
/// Uses `#[serde(untagged)]` so Single serializes as a plain object and
/// Any serializes as an array — no type discriminator field needed.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ParamCondition {
    /// Show/require when a single parameter matches a value.
    /// Serializes as: `{"param": "operation", "equals": "resize"}`
    Single(ParamConditionEntry),

    /// Show/require when ANY of multiple conditions match (OR logic).
    /// Serializes as: `[{"param": "...", "equals": "..."}, ...]`
    Any(Vec<ParamConditionEntry>),
}

// --- InputCardinality ---

/// Declares how a processor expects to receive files for smart iteration.
/// Used by the auto-iteration executor to partition flat node sequences
/// into implicit per-file loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum InputCardinality {
    /// Processes one file at a time. Contiguous perFile nodes get wrapped
    /// in an implicit per-file loop in auto mode.
    #[default]
    PerFile,
    /// Needs the full batch of files at once (e.g., zip, concat, merge).
    /// Acts as an iteration barrier in auto mode.
    Batch,
}

// --- NodeCategory ---

/// The broad category a node belongs to. Used for UI grouping and filtering.
/// Serialized as kebab-case to match `@bnto/nodes` convention.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum NodeCategory {
    /// Image processing — compress, resize, convert formats
    Image,
    /// Spreadsheet/CSV operations — clean, rename columns
    Spreadsheet,
    /// File system operations — rename files
    File,
    /// Data transformation (future) — JSON, XML, text
    Data,
    /// Network operations (future) — HTTP requests, API calls
    Network,
    /// Control flow (future) — loops, conditionals, groups
    Control,
    /// System operations (future) — shell commands, environment
    System,
    /// Input/output nodes — file input, file output
    Io,
}

// --- ParameterType ---

/// The type of a node parameter. Determines what UI control to render.
/// Tagged with `"type"` in JSON (e.g., `{"type": "number"}`).
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ParameterType {
    /// A numeric value (integer or float). Used for quality, width, height.
    Number,
    /// A text string. Used for find/replace patterns, prefixes, suffixes.
    #[default]
    String,
    /// A true/false toggle. Used for trimWhitespace, removeEmptyRows.
    Boolean,
    /// A choice from a fixed set of options (like a dropdown/select).
    /// The `options` field lists all valid values.
    Enum {
        /// The list of valid values for this enum parameter.
        /// Example: `["jpeg", "png", "webp"]` for image format selection.
        options: Vec<std::string::String>,
    },
    /// A structured object (key-value map). Used for column rename mappings.
    Object,
}

// --- Constraints ---

/// Optional constraints on a parameter's value (min/max range, required flag).
/// Used for validation and UI hints (slider bounds, required markers).
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Constraints {
    /// Minimum allowed value (for numeric parameters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,

    /// Maximum allowed value (for numeric parameters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,

    /// Whether this parameter must be provided.
    pub required: bool,
}

// --- ParameterDef ---

/// A complete definition of one parameter a node accepts. Provides
/// everything the engine (validation) and UI (control rendering) need.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDef {
    /// The parameter's key name in config JSON (e.g., `"quality"`).
    pub name: std::string::String,

    /// Human-readable label for the UI.
    pub label: std::string::String,

    /// Description shown as tooltip or help text.
    pub description: std::string::String,

    /// Value type — determines what UI control to render.
    pub param_type: ParameterType,

    /// Default value (heterogeneous type via `serde_json::Value`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Optional validation constraints (min/max range, required flag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,

    // --- UI Metadata Fields ---
    /// Placeholder text for input controls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// Show this parameter only when another parameter matches a value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_when: Option<ParamCondition>,

    /// Require this parameter only when another parameter matches a value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_when: Option<ParamCondition>,

    /// Whether this param can be surfaced in container config panels.
    /// Defaults to `true`. Set `false` for internal wiring params.
    #[serde(default = "default_true")]
    pub surfaceable: bool,
}

/// Serde default for `surfaceable` field during deserialization.
#[allow(dead_code)]
fn default_true() -> bool {
    true
}

/// Manual Default because `surfaceable` must default to `true`, not `false`.
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
            visible_when: None,
            required_when: None,
            surfaceable: true,
        }
    }
}

// --- NodeTypeInfo — Node-type-level metadata (all 15 types) ---
//
// Separate from NodeMetadata because NodeMetadata describes a PROCESSOR
// (e.g., "image-compress") while NodeTypeInfo describes a NODE TYPE
// (e.g., "image-compress") — one per node type.
// Includes types the engine doesn't have processors for yet (http-request,
// shell-command). Codegen generates TS `NODE_TYPE_INFO` from this.

/// Everything the UI needs to know about a node type, independent of any
/// specific processor/operation. The engine's authoritative type registry.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeTypeInfo {
    /// Type name as used in `.bnto.json` (e.g., `"image-compress"`, `"file-rename"`).
    pub name: String,
    /// Human-readable display label.
    pub label: String,
    /// One-sentence description.
    pub description: String,
    /// Category for UI grouping/filtering.
    pub category: NodeCategory,
    /// Whether this node can contain child nodes.
    pub is_container: bool,
    /// Platforms this type runs on (e.g., `["browser"]`, `["server"]`).
    pub platforms: Vec<String>,
    /// Lucide icon name — consumers resolve to their own icon component.
    pub icon: String,
}

/// Constructs a `NodeTypeInfo` from positional fields. Reduces per-entry
/// boilerplate in `all_node_types()` — keeps the table scannable.
macro_rules! node_type {
    ($name:expr, $label:expr, $desc:expr, $cat:expr, $container:expr, $platform:expr, $icon:expr) => {
        NodeTypeInfo {
            name: $name.to_string(),
            label: $label.to_string(),
            description: $desc.to_string(),
            category: $cat,
            is_container: $container,
            platforms: vec![$platform.to_string()],
            icon: $icon.to_string(),
        }
    };
}

/// Return metadata for all 15 registered node types.
///
/// Single source of truth for the engine's node type registry.
/// Composed from per-category helpers, then sorted alphabetically for stable output.
pub fn all_node_types() -> Vec<NodeTypeInfo> {
    let mut types = Vec::with_capacity(15);
    types.extend(control_node_types());
    types.extend(data_node_types());
    types.extend(file_node_types());
    types.extend(image_node_types());
    types.extend(io_node_types());
    types.extend(network_node_types());
    types.extend(spreadsheet_node_types());
    types.extend(system_node_types());
    types.sort_by(|a, b| a.name.cmp(&b.name));
    types
}

fn control_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "group",
            "Group",
            "Container for child nodes. Orchestrates sequential or parallel execution.",
            NodeCategory::Control,
            true,
            "browser",
            "box"
        ),
        node_type!(
            "loop",
            "Loop",
            "Iterate over arrays (forEach), repeat N times, or loop while condition.",
            NodeCategory::Control,
            true,
            "browser",
            "repeat"
        ),
        node_type!(
            "parallel",
            "Parallel",
            "Execute tasks concurrently with configurable worker pool and error strategy.",
            NodeCategory::Control,
            true,
            "browser",
            "git-fork"
        ),
    ]
}

fn data_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "edit-fields",
            "Edit Fields",
            "Set field values from static values or template expressions.",
            NodeCategory::Data,
            false,
            "browser",
            "pen-line"
        ),
        node_type!(
            "transform",
            "Transform",
            "Transform data using expressions (single value) or field mappings.",
            NodeCategory::Data,
            false,
            "browser",
            "arrow-left-right"
        ),
    ]
}

fn file_node_types() -> Vec<NodeTypeInfo> {
    vec![node_type!(
        "file-rename",
        "Rename Files",
        "Transform filenames using patterns, find/replace, and case rules.",
        NodeCategory::File,
        false,
        "browser",
        "folder-open"
    )]
}

fn image_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "image-compress",
            "Compress Images",
            "Reduce image file size while maintaining quality.",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "image-convert",
            "Convert Image Format",
            "Convert images between JPEG, PNG, and WebP formats.",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "image-resize",
            "Resize Images",
            "Change image dimensions while maintaining quality.",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
    ]
}

fn io_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "input",
            "Input",
            "Declares how data enters the recipe.",
            NodeCategory::Io,
            false,
            "browser",
            "file-up"
        ),
        node_type!(
            "output",
            "Output",
            "Declares how results are delivered.",
            NodeCategory::Io,
            false,
            "browser",
            "download"
        ),
    ]
}

fn network_node_types() -> Vec<NodeTypeInfo> {
    vec![node_type!(
        "http-request",
        "HTTP Request",
        "Make HTTP requests to APIs (GET, POST, PUT, DELETE, etc.).",
        NodeCategory::Network,
        false,
        "server",
        "globe"
    )]
}

fn spreadsheet_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "spreadsheet-clean",
            "Clean CSV",
            "Remove empty rows, trim whitespace, and deduplicate CSV data.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "sheet"
        ),
        node_type!(
            "spreadsheet-rename",
            "Rename CSV Columns",
            "Rename column headers in a CSV file.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "sheet"
        ),
    ]
}

fn system_node_types() -> Vec<NodeTypeInfo> {
    vec![node_type!(
        "shell-command",
        "Shell Command",
        "Execute shell commands with stall detection, retry, and streaming output.",
        NodeCategory::System,
        false,
        "server",
        "terminal"
    )]
}

// --- NodeMetadata ---

/// Complete self-description of a processor. Return type of
/// `NodeProcessor::metadata()`. The `node_type` is the direct dispatch
/// key (e.g., `"image-compress"`).
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeMetadata {
    /// Per-operation node type (e.g., `"image-compress"`, `"spreadsheet-clean"`).
    pub node_type: std::string::String,
    /// Human-readable processor name.
    pub name: std::string::String,
    /// Description of what this processor does.
    pub description: std::string::String,
    /// Category for UI grouping/filtering.
    pub category: NodeCategory,
    /// Accepted MIME types. Empty means "any file type".
    pub accepts: Vec<std::string::String>,
    /// Platforms this processor runs on (`"browser"`, `"server"`, `"desktop"`).
    pub platforms: Vec<std::string::String>,
    /// Parameters with types, defaults, and constraints.
    pub parameters: Vec<ParameterDef>,
    /// How this processor expects to receive files: one at a time or as a batch.
    /// Defaults to `PerFile`. Used by the auto-iteration executor.
    #[serde(default)]
    pub input_cardinality: InputCardinality,
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
        };
        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains(r#""inputCardinality":"batch""#));
    }

    // --- NodeTypeInfo Tests ---

    #[test]
    fn test_all_node_types_returns_15_entries() {
        // The engine defines all 15 node types.
        let types = all_node_types();
        assert_eq!(types.len(), 15, "Should have exactly 15 node types");
    }

    #[test]
    fn test_all_node_types_sorted_alphabetically() {
        // Entries should be sorted by name for deterministic output.
        let types = all_node_types();
        let names: Vec<&str> = types.iter().map(|t| t.name.as_str()).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted, "Node types should be alphabetically sorted");
    }

    #[test]
    fn test_all_node_types_unique_names() {
        // Every node type name should be unique.
        let types = all_node_types();
        let mut names: Vec<&str> = types.iter().map(|t| t.name.as_str()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 15, "All node type names should be unique");
    }

    #[test]
    fn test_container_types_are_group_loop_parallel() {
        // Only group, loop, and parallel should be containers.
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
        // Only input and output should have the Io category.
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
        // http-request and shell-command should only have "server" platform.
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
        // NodeTypeInfo should serialize with camelCase keys.
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
        // isContainer should be camelCase in JSON
        assert!(json.contains(r#""isContainer":false"#));
        assert!(!json.contains("is_container"));
    }

    // --- Serialization Tests ---
    // These verify that our types serialize to the expected JSON format,
    // with camelCase keys, skip_serializing_if working, etc.

    #[test]
    fn test_category_serializes_to_kebab_case() {
        // NodeCategory variants should serialize as kebab-case strings.
        let json = serde_json::to_string(&NodeCategory::Image).unwrap();
        assert_eq!(json, r#""image""#);

        let json = serde_json::to_string(&NodeCategory::Spreadsheet).unwrap();
        assert_eq!(json, r#""spreadsheet""#);

        let json = serde_json::to_string(&NodeCategory::File).unwrap();
        assert_eq!(json, r#""file""#);

        let json = serde_json::to_string(&NodeCategory::Io).unwrap();
        assert_eq!(json, r#""io""#);
    }

    #[test]
    fn test_parameter_type_number_serialization() {
        // Number type serializes with a "type" tag.
        let json = serde_json::to_string(&ParameterType::Number).unwrap();
        assert_eq!(json, r#"{"type":"number"}"#);
    }

    #[test]
    fn test_parameter_type_enum_serialization() {
        // Enum type includes the options list.
        let param = ParameterType::Enum {
            options: vec!["jpeg".to_string(), "png".to_string(), "webp".to_string()],
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""type":"enum""#));
        assert!(json.contains(r#""options":["jpeg","png","webp"]"#));
    }

    #[test]
    fn test_constraints_skips_none_fields() {
        // Fields that are None should be omitted from the JSON output.
        let constraints = Constraints {
            min: Some(1.0),
            max: None,
            required: false,
        };
        let json = serde_json::to_string(&constraints).unwrap();
        // Should have "min" but NOT "max".
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
        // ParameterDef fields should be camelCase in JSON.
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
        // Should use "paramType" not "param_type".
        assert!(json.contains(r#""paramType""#));
        assert!(!json.contains("param_type"));
    }

    #[test]
    fn test_parameter_def_skips_none_default() {
        // When default is None, it should be omitted from JSON.
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
        // UI metadata fields should also be omitted when None.
        assert!(!json.contains("placeholder"));
        assert!(!json.contains("visibleWhen"));
        assert!(!json.contains("requiredWhen"));
    }

    #[test]
    fn test_parameter_def_surfaceable_defaults_to_true() {
        // The `surfaceable` field should default to `true` — most params are
        // user-facing controls that should appear in surfaced container views.
        let param = ParameterDef {
            name: "quality".to_string(),
            label: "Quality".to_string(),
            description: "Compression quality".to_string(),
            param_type: ParameterType::Number,
            ..Default::default()
        };
        // Default::default() should give surfaceable = true.
        assert!(param.surfaceable, "surfaceable should default to true");
        // And it should serialize with the field present.
        let json = serde_json::to_string(&param).unwrap();
        assert!(json.contains(r#""surfaceable":true"#));
    }

    #[test]
    fn test_parameter_def_surfaceable_false_serializes() {
        // Internal wiring params (like loop `items`) should be explicitly
        // marked `surfaceable: false` so the editor doesn't surface them.
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
        // NodeMetadata fields should be camelCase in JSON.
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
        };
        let json = serde_json::to_string(&metadata).unwrap();
        // Should use camelCase field names.
        assert!(json.contains(r#""nodeType":"image-compress""#));
        assert!(json.contains(r#""platforms":["browser"]"#));
        assert!(!json.contains("node_type"));
    }

    #[test]
    fn test_full_metadata_round_trip() {
        // Build a complete NodeMetadata and verify it serializes to valid JSON
        // that can be parsed back.
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
        };

        // Serialize to JSON string.
        let json = serde_json::to_string_pretty(&metadata).unwrap();

        // Parse back to a generic JSON Value (round-trip test).
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify key fields are present and correct.
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
        // A Single condition should serialize as a flat JSON object
        // with "param" and "equals" keys (camelCase).
        let condition = ParamCondition::Single(ParamConditionEntry {
            param: "operation".to_string(),
            equals: "resize".to_string(),
        });
        let json = serde_json::to_string(&condition).unwrap();
        // Should be a flat object, not wrapped in a type tag.
        assert_eq!(json, r#"{"param":"operation","equals":"resize"}"#);
    }

    #[test]
    fn test_param_condition_any_serializes_as_array() {
        // An Any condition should serialize as a JSON array of condition objects.
        // This represents OR logic: show when ANY condition matches.
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
        // Should be an array of objects.
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_array(), "Any condition should be a JSON array");
        assert_eq!(parsed.as_array().unwrap().len(), 2);
        assert_eq!(parsed[0]["param"], "operation");
        assert_eq!(parsed[0]["equals"], "resize");
        assert_eq!(parsed[1]["equals"], "crop");
    }

    #[test]
    fn test_parameter_def_with_ui_fields_serializes_camel_case() {
        // When UI metadata fields are set, they should appear in JSON
        // with camelCase keys (visibleWhen, not visible_when).
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
        // "visibleWhen" should be camelCase (not "visible_when").
        assert!(json.contains(r#""visibleWhen""#));
        assert!(!json.contains("visible_when"));
        // "placeholder" should be present.
        assert!(json.contains(r#""placeholder":"e.g. 800""#));
        // "requiredWhen" should be omitted (it's None).
        assert!(!json.contains("requiredWhen"));
    }
}
