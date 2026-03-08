// =============================================================================
// Node Metadata — Self-Describing Processor Definitions
// =============================================================================
//
// WHAT IS THIS FILE?
// This defines the types that let each node processor describe ITSELF — what
// it's called, what category it belongs to, what parameters it accepts, what
// file types it can handle, and whether it runs in the browser.
//
// WHY DO WE NEED THIS?
// Currently, node metadata lives in TWO uncoordinated places:
//   - `@bnto/nodes` (TypeScript) — defines names, labels, categories, schemas
//   - Rust processors — implicitly know their parameters but don't declare them
//
// By adding a `metadata()` method to every processor, the engine becomes the
// single source of truth for node definitions. The TypeScript side can validate
// against the engine's output, and eventually consume it directly.
//
// HOW IT WORKS:
// Each processor implements `metadata()` on the `NodeProcessor` trait. The
// registry collects all metadata into a "catalog" — a JSON-serializable list
// of every processor's self-description. The `node_catalog()` WASM function
// exports this catalog to JavaScript.

use serde::Serialize;

// =============================================================================
// NodeCategory — What kind of node is this?
// =============================================================================

/// The broad category a node belongs to. Used for grouping in the UI
/// (e.g., "Image" tools, "Spreadsheet" tools) and for filtering.
///
/// RUST CONCEPT: `#[derive(...)]`
/// `derive` automatically generates implementations of common traits:
///   - `Debug` — lets you print the value with `{:?}` (for logging)
///   - `Clone` — lets you make copies with `.clone()`
///   - `Serialize` — lets serde convert this to JSON
///   - `PartialEq` — lets you compare with `==`
///
/// RUST CONCEPT: `#[serde(rename_all = "kebab-case")]`
/// When serialized to JSON, variant names are converted to kebab-case:
///   `Image` → `"image"`, `Spreadsheet` → `"spreadsheet"`, `FileSystem` → `"file-system"`
/// This matches the naming convention used in `@bnto/nodes`.
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

// =============================================================================
// ParameterType — What kind of value does a parameter accept?
// =============================================================================

/// The type of a node parameter. This tells the UI what kind of input
/// control to render (number slider, text field, checkbox, dropdown).
///
/// RUST CONCEPT: `#[serde(tag = "type", rename_all = "camelCase")]`
/// `tag = "type"` means the JSON output includes a `"type"` field that
/// identifies the variant. `rename_all = "camelCase"` converts variant
/// names to camelCase in JSON.
///
/// Examples of serialized output:
///   `ParameterType::Number`              → `{"type": "number"}`
///   `ParameterType::Enum { options: .. }` → `{"type": "enum", "options": ["jpeg", "png"]}`
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ParameterType {
    /// A numeric value (integer or float). Used for quality, width, height.
    Number,
    /// A text string. Used for find/replace patterns, prefixes, suffixes.
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

// =============================================================================
// Constraints — Validation rules for a parameter
// =============================================================================

/// Optional constraints on a parameter's value. These are used for
/// validation (rejecting out-of-range values) and for UI hints (setting
/// slider min/max, marking required fields).
///
/// RUST CONCEPT: `#[serde(skip_serializing_if = "Option::is_none")]`
/// When serializing to JSON, fields that are `None` are omitted entirely.
/// So `Constraints { min: Some(1.0), max: None, required: false }` becomes
/// `{"min": 1.0, "required": false}` — no `"max"` key at all. This keeps
/// the JSON output clean and compact.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Constraints {
    /// The minimum allowed value (for numeric parameters).
    /// Example: `min: 1.0` for image quality (can't be less than 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,

    /// The maximum allowed value (for numeric parameters).
    /// Example: `max: 100.0` for image quality (can't exceed 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,

    /// Whether this parameter MUST be provided (can't be omitted).
    /// Most parameters have defaults and are optional (`required: false`).
    pub required: bool,
}

// =============================================================================
// ParameterDef — A single parameter's full definition
// =============================================================================

/// A complete definition of one parameter a node accepts. This tells both
/// the engine (for validation) and the UI (for rendering controls) everything
/// they need to know about the parameter.
///
/// RUST CONCEPT: `#[serde(rename_all = "camelCase")]`
/// Rust uses `snake_case` for field names (`param_type`), but JavaScript
/// uses `camelCase` (`paramType`). This attribute automatically converts
/// field names when serializing to JSON: `param_type` → `"paramType"`.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDef {
    /// The parameter's key name, as it appears in the config JSON.
    /// Example: `"quality"`, `"trimWhitespace"`, `"format"`
    pub name: std::string::String,

    /// A human-readable label for the UI.
    /// Example: `"Quality"`, `"Trim Whitespace"`, `"Output Format"`
    pub label: std::string::String,

    /// A longer description explaining what this parameter does.
    /// Shown as a tooltip or help text in the UI.
    pub description: std::string::String,

    /// The type of value this parameter accepts (number, string, boolean, etc.).
    /// Determines what kind of UI control to render.
    pub param_type: ParameterType,

    /// The default value for this parameter, if any.
    /// When the user doesn't provide a value, this is what the processor uses.
    ///
    /// RUST CONCEPT: `serde_json::Value`
    /// This is Rust's equivalent of JavaScript's `any` for JSON values.
    /// It can hold numbers, strings, booleans, arrays, objects, or null.
    /// We use it here because different parameters have different default
    /// types (80 for quality, true for trimWhitespace, "jpeg" for format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Optional validation constraints (min/max range, required flag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
}

// =============================================================================
// NodeTypeInfo — Node-type-level metadata (all 12 types)
// =============================================================================
//
// WHY IS THIS SEPARATE FROM NodeMetadata?
// NodeMetadata describes a PROCESSOR (one specific operation, like "image:compress").
// NodeTypeInfo describes a NODE TYPE (like "image") — the umbrella that may have
// multiple processors underneath it.
//
// The TypeScript side needs to know about ALL 12 node types (including ones the
// engine doesn't have processors for yet, like "http-request" and "shell-command").
// This struct is the engine's definition of every node type — its label, icon,
// category, whether it's a container, and what platforms it can run on.
//
// The codegen script reads this from the catalog snapshot and generates the
// TypeScript `NODE_TYPE_INFO` map, so adding a new node type is:
//   1. Add it to `all_node_types()` below
//   2. Run `task wasm:build` → `task nodes:generate`
//   3. Done — TypeScript picks it up automatically

/// Everything the UI needs to know about a node type — independent of any
/// specific processor/operation.
///
/// This is the engine's authoritative definition of each node type.
/// The codegen script generates the TypeScript `NODE_TYPE_INFO` from this.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeTypeInfo {
    /// The node type name as used in `.bnto.json` definitions.
    /// e.g., `"image"`, `"spreadsheet"`, `"file-system"`, `"input"`
    pub name: String,

    /// Human-readable display label.
    /// e.g., `"Image"`, `"Spreadsheet"`, `"File System"`, `"Input"`
    pub label: String,

    /// One-sentence description of what the node type does.
    pub description: String,

    /// Category for UI grouping/filtering.
    pub category: NodeCategory,

    /// Whether this node can contain child nodes (group, loop, parallel).
    pub is_container: bool,

    /// Platforms this node type can run on.
    /// Derived from processor registration — if any processor for this type
    /// runs on "browser", the node type has "browser" in its platforms.
    /// For types without processors yet (http-request, shell-command), this
    /// is set to the expected platforms when implemented.
    pub platforms: Vec<String>,

    /// Lucide icon name for visual consumers.
    /// Pure string metadata — consumers resolve to their own icon component.
    /// e.g., `"image"` → ImageIcon, `"table"` → TableIcon
    pub icon: String,
}

/// Return metadata for all 12 registered node types.
///
/// This is the engine's single source of truth for what node types exist,
/// what they're called, what category they belong to, and where they run.
/// The `node_catalog()` WASM export includes this in the catalog snapshot,
/// and the codegen script generates TypeScript's `NODE_TYPE_INFO` from it.
///
/// Node types are listed in alphabetical order by name for stable output.
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
            description: "File operations: rename, copy, move, delete, mkdir, exists, list.".to_string(),
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
            description: "Image processing: resize, convert formats, compress, composite, batch.".to_string(),
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
            description: "Read and write CSV or Excel files.".to_string(),
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
// NodeMetadata — The complete self-description of a processor
// =============================================================================

/// Everything a consumer needs to know about a node processor — what it does,
/// what files it accepts, what parameters it takes, and where it can run.
/// This is the return type of `NodeProcessor::metadata()`.
///
/// The `node_type` + `operation` pair forms the compound key used by the
/// registry for dispatch (e.g., `"image:compress"`, `"spreadsheet:clean"`).
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeMetadata {
    /// The node type group (e.g., `"image"`, `"spreadsheet"`, `"file-system"`).
    /// Combined with `operation` to form the registry dispatch key.
    pub node_type: std::string::String,

    /// The specific operation within the node type (e.g., `"compress"`, `"clean"`).
    /// Combined with `node_type` to form the registry dispatch key.
    pub operation: std::string::String,

    /// A human-readable name for the processor (e.g., `"Compress Images"`).
    /// Shown in the UI as the node's title.
    pub name: std::string::String,

    /// A description of what this processor does.
    /// Shown in the UI as the node's subtitle or tooltip.
    pub description: std::string::String,

    /// The category this processor belongs to (for UI grouping/filtering).
    pub category: NodeCategory,

    /// MIME types this processor can handle (e.g., `["image/jpeg", "image/png"]`).
    /// The UI uses this to filter which files the user can drop on this node.
    /// An empty list means "accepts any file type" (like rename-files).
    pub accepts: Vec<std::string::String>,

    /// The platforms this processor can run on.
    ///
    /// Current platform values:
    ///   - `"browser"` — runs in the browser via WASM (free tier)
    ///   - `"server"` — runs on a server via Cloud API (Pro tier, M4)
    ///   - `"desktop"` — runs natively on desktop via Tauri (M3)
    ///
    /// All 6 current processors support `["browser"]`. Future processors
    /// (like AI-powered nodes) might only support `["server"]`. Some nodes
    /// may support multiple platforms (e.g., image compress runs everywhere).
    ///
    /// This replaces the old `browserCapable: bool` field with a more
    /// flexible list that can grow as bnto adds new execution targets.
    pub platforms: Vec<std::string::String>,

    /// The parameters this processor accepts, with types, defaults, and constraints.
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
        // The engine defines all 12 node types.
        let types = all_node_types();
        assert_eq!(types.len(), 12, "Should have exactly 12 node types");
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
        assert_eq!(names.len(), 12, "All node type names should be unique");
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
            default: None,
            constraints: None,
        };
        let json = serde_json::to_string(&param).unwrap();
        assert!(!json.contains("default"));
        assert!(!json.contains("constraints"));
    }

    #[test]
    fn test_node_metadata_serializes_camel_case() {
        // NodeMetadata fields should be camelCase in JSON.
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
        // Should use camelCase field names.
        assert!(json.contains(r#""nodeType":"image""#));
        assert!(json.contains(r#""platforms":["browser"]"#));
        assert!(!json.contains("node_type"));
    }

    #[test]
    fn test_full_metadata_round_trip() {
        // Build a complete NodeMetadata and verify it serializes to valid JSON
        // that can be parsed back.
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
            }],
        };

        // Serialize to JSON string.
        let json = serde_json::to_string_pretty(&metadata).unwrap();

        // Parse back to a generic JSON Value (round-trip test).
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify key fields are present and correct.
        assert_eq!(parsed["nodeType"], "image");
        assert_eq!(parsed["operation"], "compress");
        assert_eq!(parsed["category"], "image");
        assert_eq!(parsed["platforms"][0], "browser");
        assert_eq!(parsed["accepts"].as_array().unwrap().len(), 3);
        assert_eq!(parsed["parameters"].as_array().unwrap().len(), 1);
        assert_eq!(parsed["parameters"][0]["name"], "quality");
        assert_eq!(parsed["parameters"][0]["default"], 80);
    }
}
