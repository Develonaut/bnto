// Field template resolution — substitutes `{fields.*}` placeholders in node params.
//
// Called before passing params to a processor so templates like
// `"{fields.format}"` become concrete values like `"mp4"`.

use std::collections::BTreeMap;

use serde_json::Value;

/// Substitute `{fields.*}` placeholders in all string values within params.
///
/// Walks the params map and replaces occurrences of `{fields.X}` with the
/// corresponding value from `field_values`. Non-string values pass through
/// unchanged. Supports substitution inside array elements and nested strings
/// containing multiple placeholders.
pub fn resolve_fields(
    params: &serde_json::Map<String, Value>,
    field_values: &BTreeMap<String, Value>,
) -> serde_json::Map<String, Value> {
    params
        .iter()
        .map(|(k, v)| (k.clone(), resolve_value(v, field_values)))
        .collect()
}

/// Resolve a single JSON value, recursing into arrays and objects.
fn resolve_value(value: &Value, field_values: &BTreeMap<String, Value>) -> Value {
    match value {
        Value::String(s) => resolve_string(s, field_values),
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| resolve_value(v, field_values)).collect())
        }
        Value::Object(map) => Value::Object(resolve_fields(map, field_values)),
        other => other.clone(),
    }
}

/// Resolve `{fields.*}` placeholders in a string.
///
/// If the entire string is a single `{fields.X}` placeholder and the value
/// is a non-string type (number, boolean), returns the typed value directly
/// to preserve JSON types. Otherwise, performs string interpolation.
fn resolve_string(s: &str, field_values: &BTreeMap<String, Value>) -> Value {
    // Fast path: no placeholders.
    if !s.contains("{fields.") {
        return Value::String(s.to_string());
    }

    // Check if the entire string is exactly one placeholder — preserve type.
    if let Some(key) = extract_sole_placeholder(s) {
        if let Some(value) = field_values.get(key) {
            return value.clone();
        }
        // Key not found — leave placeholder as-is.
        return Value::String(s.to_string());
    }

    // Multiple or partial placeholders — string interpolation.
    let mut result = s.to_string();
    for (key, value) in field_values {
        let placeholder = format!("{{fields.{key}}}");
        if result.contains(&placeholder) {
            let replacement = value_to_string(value);
            result = result.replace(&placeholder, &replacement);
        }
    }
    Value::String(result)
}

/// If the string is exactly `{fields.X}`, return the key `X`.
fn extract_sole_placeholder(s: &str) -> Option<&str> {
    let trimmed = s.trim();
    if trimmed.starts_with("{fields.") && trimmed.ends_with('}') {
        let inner = &trimmed[8..trimmed.len() - 1];
        // Only a sole placeholder if there's no nested braces.
        if !inner.contains('{') && !inner.contains('}') {
            return Some(inner);
        }
    }
    None
}

/// Convert a JSON value to a string for interpolation.
fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Collect field values from field definitions and user overrides.
///
/// User overrides take precedence over defaults. Returns a map of
/// field name → resolved value.
pub fn collect_field_values(
    field_defs: &BTreeMap<String, crate::field_def::FieldDef>,
    overrides: &BTreeMap<String, Value>,
) -> BTreeMap<String, Value> {
    let mut values = BTreeMap::new();
    for (name, def) in field_defs {
        if let Some(override_val) = overrides.get(name) {
            values.insert(name.clone(), override_val.clone());
        } else {
            let default = def.default_value();
            if !default.is_null() {
                values.insert(name.clone(), default);
            }
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_params(pairs: &[(&str, Value)]) -> serde_json::Map<String, Value> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    fn make_fields(pairs: &[(&str, Value)]) -> BTreeMap<String, Value> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn simple_string_substitution() {
        let params = make_params(&[("format", json!("{fields.format}"))]);
        let fields = make_fields(&[("format", json!("mp4"))]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["format"], json!("mp4"));
    }

    #[test]
    fn substitution_inside_array() {
        let params = make_params(&[("args", json!(["--format", "{fields.format}", "-o", "out"]))]);
        let fields = make_fields(&[("format", json!("webm"))]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["args"], json!(["--format", "webm", "-o", "out"]));
    }

    #[test]
    fn multiple_placeholders_in_one_string() {
        let params = make_params(&[(
            "selector",
            json!("vcodec:{fields.videoCodec},acodec:{fields.audioCodec}"),
        )]);
        let fields = make_fields(&[("videoCodec", json!("h264")), ("audioCodec", json!("m4a"))]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["selector"], json!("vcodec:h264,acodec:m4a"));
    }

    #[test]
    fn non_string_values_pass_through() {
        let params = make_params(&[
            ("quality", json!(80)),
            ("enabled", json!(true)),
            ("nothing", json!(null)),
        ]);
        let fields = make_fields(&[]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["quality"], json!(80));
        assert_eq!(resolved["enabled"], json!(true));
        assert_eq!(resolved["nothing"], json!(null));
    }

    #[test]
    fn missing_field_leaves_placeholder() {
        let params = make_params(&[("x", json!("{fields.missing}"))]);
        let fields = make_fields(&[]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["x"], json!("{fields.missing}"));
    }

    #[test]
    fn sole_placeholder_preserves_number_type() {
        let params = make_params(&[("quality", json!("{fields.quality}"))]);
        let fields = make_fields(&[("quality", json!(80))]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["quality"], json!(80));
        assert!(resolved["quality"].is_number());
    }

    #[test]
    fn sole_placeholder_preserves_boolean_type() {
        let params = make_params(&[("strip", json!("{fields.strip}"))]);
        let fields = make_fields(&[("strip", json!(true))]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["strip"], json!(true));
        assert!(resolved["strip"].is_boolean());
    }

    #[test]
    fn number_field_stringified_in_interpolation() {
        let params = make_params(&[("label", json!("Quality: {fields.quality}%"))]);
        let fields = make_fields(&[("quality", json!(80))]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["label"], json!("Quality: 80%"));
    }

    #[test]
    fn no_placeholder_passes_through() {
        let params = make_params(&[("command", json!("yt-dlp"))]);
        let fields = make_fields(&[("format", json!("mp4"))]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["command"], json!("yt-dlp"));
    }

    #[test]
    fn collect_field_values_override_beats_default() {
        let mut defs = BTreeMap::new();
        defs.insert(
            "format".into(),
            crate::field_def::FieldDef::Enum {
                label: "Format".into(),
                options: vec![],
                description: None,
                default: Some("mp4".into()),
                order: None,
            },
        );
        let mut overrides = BTreeMap::new();
        overrides.insert("format".into(), json!("webm"));

        let values = collect_field_values(&defs, &overrides);
        assert_eq!(values["format"], json!("webm"));
    }

    #[test]
    fn collect_field_values_default_used_when_no_override() {
        let mut defs = BTreeMap::new();
        defs.insert(
            "format".into(),
            crate::field_def::FieldDef::Enum {
                label: "Format".into(),
                options: vec![],
                description: None,
                default: Some("mp4".into()),
                order: None,
            },
        );
        let overrides = BTreeMap::new();

        let values = collect_field_values(&defs, &overrides);
        assert_eq!(values["format"], json!("mp4"));
    }

    #[test]
    fn collect_field_values_no_default_no_override() {
        let mut defs = BTreeMap::new();
        defs.insert(
            "name".into(),
            crate::field_def::FieldDef::String {
                label: "Name".into(),
                description: None,
                default: None,
                placeholder: None,
                order: None,
            },
        );
        let overrides = BTreeMap::new();

        let values = collect_field_values(&defs, &overrides);
        assert!(!values.contains_key("name"));
    }

    #[test]
    fn nested_object_substitution() {
        let params = make_params(&[("config", json!({"nested": "{fields.x}"}))]);
        let fields = make_fields(&[("x", json!("resolved"))]);
        let resolved = resolve_fields(&params, &fields);
        assert_eq!(resolved["config"]["nested"], json!("resolved"));
    }
}
