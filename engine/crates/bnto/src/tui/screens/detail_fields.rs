// Recipe field → detail param conversion.
//
// Converts `FieldDef` declarations from recipe definitions into `ParamEntry`
// values for the TUI detail screen. Extracted from `detail_loader.rs` to keep
// files under 250 production lines.

use bnto_core::FieldDef;
use bnto_core::metadata::{Constraints, OptionEntry, ParameterType};

use super::detail::ParamEntry;

/// Convert node-level FieldDef declarations into ParamEntry list.
///
/// Sorted by `order` (lower first), then alphabetically by key.
/// `node_id` is attached to each entry so the TUI can route overrides
/// back to the correct node via `nodeId:key=value` format.
pub(super) fn fields_to_params(
    fields: &std::collections::BTreeMap<String, FieldDef>,
    node_id: &str,
) -> Vec<ParamEntry> {
    let mut entries: Vec<(String, &FieldDef)> =
        fields.iter().map(|(k, v)| (k.clone(), v)).collect();
    entries.sort_by(|a, b| a.1.order().cmp(&b.1.order()).then_with(|| a.0.cmp(&b.0)));

    entries
        .into_iter()
        .map(|(name, def)| field_def_to_param(&name, def, node_id))
        .collect()
}

/// Convert a single FieldDef into a ParamEntry for the detail screen.
fn field_def_to_param(name: &str, def: &FieldDef, node_id: &str) -> ParamEntry {
    match def {
        FieldDef::String {
            label,
            description,
            default,
            placeholder,
            ..
        } => ParamEntry {
            node_id: node_id.to_string(),
            name: name.to_string(),
            label: label.clone(),
            value: default.clone().unwrap_or_default(),
            param_type: ParameterType::String,
            default: default.clone().unwrap_or_default(),
            description: description.clone().or_else(|| placeholder.clone()),
            constraints: None,
            suffix: None,
            control: None,
            visible_when: None,
        },
        FieldDef::Number {
            label,
            description,
            default,
            min,
            max,
            suffix,
            ..
        } => ParamEntry {
            node_id: node_id.to_string(),
            name: name.to_string(),
            label: label.clone(),
            value: default.map(|n| n.to_string()).unwrap_or_default(),
            param_type: ParameterType::Number,
            default: default.map(|n| n.to_string()).unwrap_or_default(),
            description: description.clone(),
            constraints: if min.is_some() || max.is_some() {
                Some(Constraints {
                    min: *min,
                    max: *max,
                    required: false,
                })
            } else {
                None
            },
            suffix: suffix.clone(),
            control: None,
            visible_when: None,
        },
        FieldDef::Boolean {
            label,
            description,
            default,
            ..
        } => ParamEntry {
            node_id: node_id.to_string(),
            name: name.to_string(),
            label: label.clone(),
            value: default.map(|b| b.to_string()).unwrap_or_default(),
            param_type: ParameterType::Boolean,
            default: default.map(|b| b.to_string()).unwrap_or_default(),
            description: description.clone(),
            constraints: None,
            suffix: None,
            control: None,
            visible_when: None,
        },
        FieldDef::Enum {
            label,
            options,
            description,
            default,
            ..
        } => ParamEntry {
            node_id: node_id.to_string(),
            name: name.to_string(),
            label: label.clone(),
            value: default.clone().unwrap_or_default(),
            param_type: ParameterType::Enum {
                options: options
                    .iter()
                    .map(|o| OptionEntry {
                        value: o.value.clone(),
                        label: o.label.clone(),
                    })
                    .collect(),
            },
            default: default.clone().unwrap_or_default(),
            description: description.clone(),
            constraints: None,
            suffix: None,
            control: None,
            visible_when: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn enum_field_produces_correct_options() {
        let mut fields = BTreeMap::new();
        fields.insert(
            "format".to_string(),
            FieldDef::Enum {
                label: "Output Format".to_string(),
                options: vec![
                    bnto_core::FieldOption {
                        value: "mp4".to_string(),
                        label: "MP4".to_string(),
                    },
                    bnto_core::FieldOption {
                        value: "webm".to_string(),
                        label: "WebM".to_string(),
                    },
                ],
                description: Some("Container format".to_string()),
                default: Some("mp4".to_string()),
                order: Some(1),
            },
        );
        let params = fields_to_params(&fields, "test-node");
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].label, "Output Format");
        assert_eq!(params[0].value, "mp4");
        match &params[0].param_type {
            ParameterType::Enum { options } => {
                assert_eq!(options.len(), 2);
                assert_eq!(options[0].value, "mp4");
                assert_eq!(options[1].label, "WebM");
            }
            _ => panic!("Expected Enum param type"),
        }
    }

    #[test]
    fn number_field_includes_constraints() {
        let mut fields = BTreeMap::new();
        fields.insert(
            "quality".to_string(),
            FieldDef::Number {
                label: "Quality".to_string(),
                description: None,
                default: Some(80.0),
                min: Some(1.0),
                max: Some(100.0),
                step: None,
                suffix: Some("%".to_string()),
                order: None,
            },
        );
        let params = fields_to_params(&fields, "test-node");
        assert_eq!(params[0].value, "80");
        assert_eq!(params[0].suffix.as_deref(), Some("%"));
        let c = params[0].constraints.as_ref().unwrap();
        assert_eq!(c.min, Some(1.0));
        assert_eq!(c.max, Some(100.0));
    }

    #[test]
    fn boolean_field_maps_correctly() {
        let mut fields = BTreeMap::new();
        fields.insert(
            "verbose".to_string(),
            FieldDef::Boolean {
                label: "Verbose".to_string(),
                description: Some("Enable verbose output".to_string()),
                default: Some(true),
                order: None,
            },
        );
        let params = fields_to_params(&fields, "test-node");
        assert_eq!(params[0].value, "true");
        assert_eq!(params[0].param_type, ParameterType::Boolean);
    }

    #[test]
    fn string_field_uses_placeholder_as_description() {
        let mut fields = BTreeMap::new();
        fields.insert(
            "name".to_string(),
            FieldDef::String {
                label: "Name".to_string(),
                description: None,
                default: None,
                placeholder: Some("Enter name...".to_string()),
                order: None,
            },
        );
        let params = fields_to_params(&fields, "test-node");
        assert_eq!(params[0].description.as_deref(), Some("Enter name..."));
    }

    #[test]
    fn fields_sorted_by_order_then_alpha() {
        let mut fields = BTreeMap::new();
        fields.insert(
            "beta".to_string(),
            FieldDef::String {
                label: "Beta".to_string(),
                description: None,
                default: None,
                placeholder: None,
                order: Some(2),
            },
        );
        fields.insert(
            "alpha".to_string(),
            FieldDef::String {
                label: "Alpha".to_string(),
                description: None,
                default: None,
                placeholder: None,
                order: Some(1),
            },
        );
        fields.insert(
            "gamma".to_string(),
            FieldDef::String {
                label: "Gamma".to_string(),
                description: None,
                default: None,
                placeholder: None,
                order: Some(2),
            },
        );
        let params = fields_to_params(&fields, "test-node");
        assert_eq!(params[0].name, "alpha");
        assert_eq!(params[1].name, "beta");
        assert_eq!(params[2].name, "gamma");
    }
}
