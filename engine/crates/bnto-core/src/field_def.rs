// Node-level field declarations — user-facing controls for nodes.
//
// Fields provide an interface layer between what users see and what
// processors consume. A node declares typed fields (string, number,
// boolean, enum) that render as form controls in the TUI/editor.
// Field values are substituted into `{{fields.*}}` templates in the
// node's parameters at execution time.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "ts")]
use ts_rs::TS;

/// Ordered map of field name → definition.
pub type FieldDefs = BTreeMap<String, FieldDef>;

/// A single user-facing field — declared in recipe JSON, rendered by TUI/editor.
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(
    feature = "ts",
    ts(
        export,
        export_to = "../../../../packages/@bnto/nodes/src/generated/definitionTypes/"
    )
)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum FieldDef {
    #[serde(rename = "string")]
    String {
        label: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        default: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        placeholder: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        order: Option<u32>,
    },
    #[serde(rename = "number")]
    Number {
        label: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        default: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        min: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        max: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        step: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        suffix: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        order: Option<u32>,
    },
    #[serde(rename = "boolean")]
    Boolean {
        label: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        default: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        order: Option<u32>,
    },
    #[serde(rename = "enum")]
    Enum {
        label: String,
        options: Vec<FieldOption>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        default: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "ts", ts(optional))]
        order: Option<u32>,
    },
}

/// A single option in an enum field.
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(
    feature = "ts",
    ts(
        export,
        export_to = "../../../../packages/@bnto/nodes/src/generated/definitionTypes/"
    )
)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldOption {
    pub value: String,
    pub label: String,
}

impl FieldDef {
    /// The display order for this field (lower = first).
    pub fn order(&self) -> u32 {
        match self {
            Self::String { order, .. }
            | Self::Number { order, .. }
            | Self::Boolean { order, .. }
            | Self::Enum { order, .. } => order.unwrap_or(u32::MAX),
        }
    }

    /// The default value as a JSON Value, or Null if none.
    pub fn default_value(&self) -> serde_json::Value {
        match self {
            Self::String { default, .. } => default
                .as_ref()
                .map(|s| serde_json::Value::String(s.clone()))
                .unwrap_or(serde_json::Value::Null),
            Self::Number { default, .. } => default
                .and_then(serde_json::Number::from_f64)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            Self::Boolean { default, .. } => default
                .map(serde_json::Value::Bool)
                .unwrap_or(serde_json::Value::Null),
            Self::Enum { default, .. } => default
                .as_ref()
                .map(|s| serde_json::Value::String(s.clone()))
                .unwrap_or(serde_json::Value::Null),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_field_round_trips() {
        let json = r#"{"type":"string","label":"Name","default":"hello","placeholder":"Enter..."}"#;
        let field: FieldDef = serde_json::from_str(json).unwrap();
        let FieldDef::String {
            label,
            default,
            placeholder,
            ..
        } = &field
        else {
            panic!("expected String variant");
        };
        assert_eq!(label, "Name");
        assert_eq!(default.as_deref(), Some("hello"));
        assert_eq!(placeholder.as_deref(), Some("Enter..."));
        let serialized = serde_json::to_string(&field).unwrap();
        let round_tripped: FieldDef = serde_json::from_str(&serialized).unwrap();
        assert_eq!(field, round_tripped);
    }

    #[test]
    fn number_field_round_trips() {
        let json = r#"{"type":"number","label":"Quality","default":80,"min":1,"max":100,"step":1,"suffix":"%"}"#;
        let field: FieldDef = serde_json::from_str(json).unwrap();
        let FieldDef::Number {
            label,
            default,
            min,
            max,
            step,
            suffix,
            ..
        } = &field
        else {
            panic!("expected Number variant");
        };
        assert_eq!(label, "Quality");
        assert_eq!(*default, Some(80.0));
        assert_eq!(*min, Some(1.0));
        assert_eq!(*max, Some(100.0));
        assert_eq!(*step, Some(1.0));
        assert_eq!(suffix.as_deref(), Some("%"));
        let serialized = serde_json::to_string(&field).unwrap();
        let round_tripped: FieldDef = serde_json::from_str(&serialized).unwrap();
        assert_eq!(field, round_tripped);
    }

    #[test]
    fn boolean_field_round_trips() {
        let json = r#"{"type":"boolean","label":"Strip Metadata","default":true}"#;
        let field: FieldDef = serde_json::from_str(json).unwrap();
        let FieldDef::Boolean { label, default, .. } = &field else {
            panic!("expected Boolean variant");
        };
        assert_eq!(label, "Strip Metadata");
        assert_eq!(*default, Some(true));
        let serialized = serde_json::to_string(&field).unwrap();
        let round_tripped: FieldDef = serde_json::from_str(&serialized).unwrap();
        assert_eq!(field, round_tripped);
    }

    #[test]
    fn enum_field_round_trips() {
        let json = r#"{
            "type": "enum",
            "label": "Format",
            "options": [
                {"value": "mp4", "label": "MP4"},
                {"value": "webm", "label": "WebM"}
            ],
            "default": "mp4"
        }"#;
        let field: FieldDef = serde_json::from_str(json).unwrap();
        let FieldDef::Enum {
            label,
            options,
            default,
            ..
        } = &field
        else {
            panic!("expected Enum variant");
        };
        assert_eq!(label, "Format");
        assert_eq!(options.len(), 2);
        assert_eq!(options[0].value, "mp4");
        assert_eq!(options[0].label, "MP4");
        assert_eq!(default.as_deref(), Some("mp4"));
        let serialized = serde_json::to_string(&field).unwrap();
        let round_tripped: FieldDef = serde_json::from_str(&serialized).unwrap();
        assert_eq!(field, round_tripped);
    }

    #[test]
    fn tagged_union_discriminator() {
        let json = r#"{"type":"enum","label":"X","options":[]}"#;
        let field: FieldDef = serde_json::from_str(json).unwrap();
        assert!(matches!(field, FieldDef::Enum { .. }));

        let json = r#"{"type":"string","label":"X"}"#;
        let field: FieldDef = serde_json::from_str(json).unwrap();
        assert!(matches!(field, FieldDef::String { .. }));
    }

    #[test]
    fn field_defs_map_round_trips() {
        let json = r#"{
            "format": {"type":"enum","label":"Format","options":[{"value":"mp4","label":"MP4"}],"default":"mp4","order":1},
            "quality": {"type":"number","label":"Quality","default":80,"min":1,"max":100,"order":2}
        }"#;
        let fields: FieldDefs = serde_json::from_str(json).unwrap();
        assert_eq!(fields.len(), 2);
        assert!(matches!(fields["format"], FieldDef::Enum { .. }));
        assert!(matches!(fields["quality"], FieldDef::Number { .. }));
    }

    #[test]
    fn order_defaults_to_max() {
        let field = FieldDef::String {
            label: "X".into(),
            description: None,
            default: None,
            placeholder: None,
            order: None,
        };
        assert_eq!(field.order(), u32::MAX);
    }

    #[test]
    fn order_returns_explicit_value() {
        let field = FieldDef::Enum {
            label: "X".into(),
            options: vec![],
            description: None,
            default: None,
            order: Some(3),
        };
        assert_eq!(field.order(), 3);
    }

    #[test]
    fn default_value_for_each_variant() {
        let s = FieldDef::String {
            label: "X".into(),
            description: None,
            default: Some("hello".into()),
            placeholder: None,
            order: None,
        };
        assert_eq!(s.default_value(), serde_json::json!("hello"));

        let n = FieldDef::Number {
            label: "X".into(),
            description: None,
            default: Some(42.0),
            min: None,
            max: None,
            step: None,
            suffix: None,
            order: None,
        };
        assert_eq!(n.default_value(), serde_json::json!(42.0));

        let b = FieldDef::Boolean {
            label: "X".into(),
            description: None,
            default: Some(false),
            order: None,
        };
        assert_eq!(b.default_value(), serde_json::json!(false));

        let no_default = FieldDef::String {
            label: "X".into(),
            description: None,
            default: None,
            placeholder: None,
            order: None,
        };
        assert!(no_default.default_value().is_null());
    }

    #[test]
    fn optional_fields_omitted_on_serialization() {
        let field = FieldDef::String {
            label: "Name".into(),
            description: None,
            default: None,
            placeholder: None,
            order: None,
        };
        let json = serde_json::to_string(&field).unwrap();
        assert!(!json.contains("description"));
        assert!(!json.contains("default"));
        assert!(!json.contains("placeholder"));
        assert!(!json.contains("order"));
    }
}
