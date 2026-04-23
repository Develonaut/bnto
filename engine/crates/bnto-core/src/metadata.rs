// Node Metadata — Self-describing processor definitions.
//
// Each processor implements `metadata()` on the `NodeProcessor` trait. The
// registry collects all metadata into a catalog exported via `node_catalog()`
// WASM function, making the engine the single source of truth for node defs.

use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

/// Parameter definitions for the engine-defined non-processor node types
/// (input, output, loop, group, parallel, transform, edit-fields). Processor
/// node types carry their params on `NodeMetadata::parameters`; these
/// structural / declarative types need a parallel home.
pub mod io_container;

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
    /// Processes one file at a time. Contiguous perFile nodes get wrapped
    /// in an implicit per-file loop in auto mode.
    #[default]
    PerFile,
    /// Needs the full batch of files at once (e.g., zip, concat, merge).
    /// Acts as an iteration barrier in auto mode.
    Batch,
    /// Processor generates output from its parameters — no input files.
    /// Runs exactly once, ignoring the file pipeline.
    Source,
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
    /// Vector graphics operations — SVG rasterization, optimization
    Vector,
    /// Video operations — download, transcode (CLI/desktop only)
    Video,
    /// Input/output nodes — file input, file output
    Io,
}

// --- OptionEntry — labeled enum choice ---

/// A single choice inside an `Enum` parameter. Carries a machine value
/// (used as the config JSON value) and a human-readable label (shown in
/// UI selects). Replaces the earlier bare `Vec<String>` shape so the
/// engine can describe proper `{value, label}` select options.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OptionEntry {
    /// The value stored in config JSON (e.g., `"jpeg"`).
    pub value: String,
    /// The label shown in the UI (e.g., `"JPEG"`).
    pub label: String,
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
    /// Each option carries both a machine value and a human-readable label.
    Enum {
        /// The list of valid `{value, label}` entries for this parameter.
        /// Example: `[{value:"jpeg", label:"JPEG"}, ...]` for format selection.
        options: Vec<OptionEntry>,
    },
    /// A structured object (key-value map). Used for column rename mappings.
    Object,
    /// A file upload parameter (base64-encoded). Used for overlay images, etc.
    /// The `accept` field lists allowed MIME types for the file picker.
    File {
        /// Accepted MIME types (e.g., `["image/png", "image/jpeg"]`).
        accept: Vec<std::string::String>,
    },
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

// --- PresetEntry — quick-pick preset for sliders/numerics ---

/// A labeled preset value offered next to a parameter's control. Lets the UI
/// render quick-pick chips (e.g., `Draft | Balanced | Maximum` for quality).
/// `value` is heterogeneous (serde_json::Value) because presets are not
/// limited to numbers — string presets are valid for text/enum parameters.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PresetEntry {
    /// The value applied when the preset is chosen (e.g., `80` or `"jpeg"`).
    pub value: serde_json::Value,
    /// The human-readable label for the preset (e.g., `"Balanced"`).
    pub label: String,
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

    // --- Presentation Fields (engine-owned schema) ---
    //
    // These six optional fields let the engine describe how a parameter
    // should be presented, so `@bnto/nodes` can collapse to a barrel over
    // generated code instead of hand-writing overlays per processor.
    /// Visual group label — parameters sharing a group render together
    /// under a common heading (e.g., `"dimensions"` groups width + height).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Unit suffix shown next to numeric inputs (e.g., `"%"`, `"px"`, `"ms"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// Override control identifier. Platform-agnostic string that consumers
    /// map to their widget: `"slider"`, `"select"`, `"switch"`, `"file"`,
    /// `"textarea"`, `"positionGrid"`, `"watermarkPreview"`, `"tagPicker"`,
    /// `"keyValue"`. When unset, consumers derive a default from `param_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,

    /// Accepted MIME types when this parameter backs a file picker control.
    /// Populated alongside `control: "file"`; unused for other controls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<Vec<String>>,

    /// Quick-pick presets rendered next to the control (e.g., quality chips).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<Vec<PresetEntry>>,

    /// Flip the semantic of a boolean control so `true` renders as "off"
    /// and `false` as "on" (e.g., `keepOriginal` vs `removeOriginal`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverted: Option<bool>,
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
            group: None,
            suffix: None,
            control: None,
            accept: None,
            presets: None,
            inverted: None,
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

/// Return metadata for all 22 registered node types.
///
/// Single source of truth for the engine's node type registry.
/// Composed from per-category helpers, then sorted alphabetically for stable output.
pub fn all_node_types() -> Vec<NodeTypeInfo> {
    let mut types = Vec::with_capacity(22);
    types.extend(control_node_types());
    types.extend(data_node_types());
    types.extend(file_node_types());
    types.extend(image_node_types());
    types.extend(io_node_types());
    types.extend(network_node_types());
    types.extend(spreadsheet_node_types());
    types.extend(system_node_types());
    types.extend(vector_node_types());
    types.extend(video_node_types());
    types.sort_by(|a, b| a.name.cmp(&b.name));
    types
}

/// Parameter definitions for a node type, when the engine knows them
/// independently of a `NodeProcessor` registration.
///
/// Processor node types carry their params on `NodeMetadata::parameters`
/// (collected from the registry). Structural / declarative node types
/// (input, output, loop, group, parallel, transform, edit-fields) declare
/// their params in `metadata::io_container`, which this accessor exposes
/// through a single lookup by type name.
///
/// Returns `None` for unknown types and for processor types (use the
/// registry's `NodeMetadata::parameters` for those).
pub fn node_type_params(type_name: &str) -> Option<Vec<ParameterDef>> {
    io_container::io_container_param_defs()
        .get(type_name)
        .cloned()
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
        node_type!(
            "image-strip-exif",
            "Strip EXIF",
            "Remove all EXIF metadata from images (GPS, camera info, timestamps).",
            NodeCategory::Image,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "image-overlay",
            "Overlay Image",
            "Overlay an image onto source images at a configurable position, size, and opacity.",
            NodeCategory::Image,
            false,
            "browser",
            "stamp"
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
            "spreadsheet-convert",
            "CSV to JSON",
            "Convert CSV data to JSON format with configurable delimiters.",
            NodeCategory::Spreadsheet,
            false,
            "browser",
            "sheet"
        ),
        node_type!(
            "spreadsheet-merge",
            "Merge CSV",
            "Combine multiple CSV files into one with header reconciliation and deduplication.",
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

fn vector_node_types() -> Vec<NodeTypeInfo> {
    vec![
        node_type!(
            "vector-rasterize",
            "SVG to Image",
            "Convert SVG files to raster images (PNG, JPEG, WebP).",
            NodeCategory::Vector,
            false,
            "browser",
            "image"
        ),
        node_type!(
            "vector-optimize",
            "Optimize SVG",
            "Remove editor metadata, comments, and unnecessary elements from SVG files.",
            NodeCategory::Vector,
            false,
            "browser",
            "file-minus-2"
        ),
    ]
}

fn video_node_types() -> Vec<NodeTypeInfo> {
    vec![node_type!(
        "video-download",
        "Download Video",
        "Download video from URLs using yt-dlp (CLI/desktop only).",
        NodeCategory::Video,
        false,
        "server",
        "video"
    )]
}

// --- Dependency ---

/// An external binary that a processor requires at runtime.
///
/// Pure-Rust processors (image, csv, file) have no dependencies.
/// Processors wrapping CLI tools (yt-dlp, ffmpeg) declare their
/// requirements here. The dependency checker verifies these before
/// pipeline execution; `bnto doctor` reports missing deps with install hints.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    /// Binary name to look up on PATH (e.g., `"yt-dlp"`, `"ffmpeg"`).
    pub binary: String,
    /// Semver version constraint (e.g., `">=2023.0.0"`). Empty = any version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Human-readable install instructions (e.g., `"brew install yt-dlp"`).
    pub install_hint: String,
    /// Homepage URL for the tool.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub homepage: String,
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
    /// External binary dependencies. Empty for browser-only processors.
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
        // version defaults to empty when absent
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
        // homepage defaults to empty when absent
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
    fn test_all_node_types_returns_22_entries() {
        // The engine defines all 22 node types.
        let types = all_node_types();
        assert_eq!(types.len(), 22, "Should have exactly 22 node types");
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
        assert_eq!(names.len(), 22, "All node type names should be unique");
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
        assert_eq!(
            server_only,
            vec!["http-request", "shell-command", "video-download"]
        );
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

        let json = serde_json::to_string(&NodeCategory::Vector).unwrap();
        assert_eq!(json, r#""vector""#);

        let json = serde_json::to_string(&NodeCategory::Video).unwrap();
        assert_eq!(json, r#""video""#);
    }

    #[test]
    fn test_parameter_type_number_serialization() {
        // Number type serializes with a "type" tag.
        let json = serde_json::to_string(&ParameterType::Number).unwrap();
        assert_eq!(json, r#"{"type":"number"}"#);
    }

    #[test]
    fn test_parameter_type_enum_serialization() {
        // Enum options carry both a machine value and a human-readable label.
        // This replaces the earlier `Vec<String>` shape — the UI needs labels
        // so engine-backed node types can render proper select option text.
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
            requires: vec![],
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
            requires: vec![],
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
        // Must NOT contain snake_case keys.
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

    // --- PR 1: Presentation Metadata Tests ---
    //
    // These cover the six new `ParameterDef` fields (group, suffix, control,
    // accept, presets, inverted) plus the `PresetEntry` and `OptionEntry`
    // structs. Together they let the engine ship the full UI contract —
    // `@bnto/nodes` no longer hand-writes presentation overlays.

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
        // Presets hold `serde_json::Value` so they work for numbers, strings,
        // and booleans — a quality slider uses numeric presets while a format
        // select would use string presets.
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
        // `group` co-locates related params (e.g., width+height under
        // "dimensions"); `suffix` annotates the value unit in the UI.
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
        // `control` overrides the default UI widget (e.g., "file" for a
        // file picker); `accept` narrows MIME types when control = "file".
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
        // Not every `control` needs `accept` — "watermarkPreview" is a
        // synthetic preview field with no param binding.
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
        // `inverted` lets a boolean switch display the negation of the
        // stored value (e.g., store `strip_exif: true` but show as
        // "Keep metadata: off").
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
        // New fields are single-word so the camelCase rename is a no-op,
        // but we assert the serialized shape explicitly so downstream
        // TypeScript consumers can rely on it.
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
