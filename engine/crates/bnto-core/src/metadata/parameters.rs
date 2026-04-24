// Parameter schema types — definitions for node parameters.
//
// These types describe what parameters a node accepts: their names,
// types, constraints, UI hints, and conditional visibility rules.
// Used by both the engine (validation) and the UI (control rendering).

use serde::Serialize;

// --- ParamCondition — Conditional Visibility / Requirement Rules ---

/// A single condition entry: "when `param` has the value `equals`".
///
/// Used both standalone (in `ParamCondition::Single`) and as entries
/// in the `ParamCondition::Any` array.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParamConditionEntry {
    /// The name of the parameter to check against.
    pub param: String,
    /// The value that triggers visibility/requirement.
    pub equals: String,
}

/// Conditional visibility/requirement rule for a parameter.
///
/// Uses `#[serde(untagged)]` so Single serializes as a plain object and
/// Any serializes as an array — no type discriminator field needed.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ParamCondition {
    /// Show/require when a single parameter matches a value.
    Single(ParamConditionEntry),
    /// Show/require when ANY of multiple conditions match (OR logic).
    Any(Vec<ParamConditionEntry>),
}

// --- OptionEntry — labeled enum choice ---

/// A single choice inside an `Enum` parameter. Carries a machine value
/// and a human-readable label.
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
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ParameterType {
    Number,
    #[default]
    String,
    Boolean,
    /// A choice from a fixed set of options (like a dropdown/select).
    Enum {
        options: Vec<OptionEntry>,
    },
    Object,
    /// A file upload parameter (base64-encoded).
    File {
        accept: Vec<std::string::String>,
    },
}

// --- Constraints ---

/// Optional constraints on a parameter's value (min/max range, required flag).
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Constraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    pub required: bool,
}

// --- PresetEntry — quick-pick preset for sliders/numerics ---

/// A labeled preset value offered next to a parameter's control.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PresetEntry {
    pub value: serde_json::Value,
    pub label: String,
}

// --- ParameterDef ---

/// A complete definition of one parameter a node accepts. Provides
/// everything the engine (validation) and UI (control rendering) need.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDef {
    pub name: std::string::String,
    pub label: std::string::String,
    pub description: std::string::String,
    /// Value type — determines what UI control to render.
    pub param_type: ParameterType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,

    // --- UI Metadata Fields ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_when: Option<ParamCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_when: Option<ParamCondition>,
    /// Whether this param can be surfaced in container config panels.
    #[serde(default = "default_true")]
    pub surfaceable: bool,

    // --- Presentation Fields (engine-owned schema) ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Override control identifier (e.g., `"slider"`, `"select"`, `"file"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<Vec<PresetEntry>>,
    /// Flip the semantic of a boolean control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverted: Option<bool>,
}

/// Serde default for `surfaceable` field during deserialization.
#[allow(dead_code)]
fn default_true() -> bool {
    true
}

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
