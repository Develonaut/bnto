// Recipe detail screen — state for recipe configuration.
//
// The editing state machine is delegated to bnto_form::FormModel.
// This module owns recipe metadata, the "Continue" button, and
// visible_when evaluation. Loading logic lives in detail_loader.rs.
// Bridge logic (ParamEntry → Field) lives in detail_bridge.rs.

use std::collections::HashMap;

use bnto_core::metadata::{ParamCondition, ParameterType};
use bnto_form::FormModel;

/// A single editable parameter entry shown in the detail screen.
#[derive(Debug, Clone)]
pub struct ParamEntry {
    /// The node ID this param belongs to (e.g., "compress-image").
    pub node_id: String,
    /// Parameter key (e.g., "quality").
    pub name: String,
    /// Human-readable label (e.g., "Quality").
    pub label: String,
    /// Current value as a display string.
    pub value: String,
    /// The parameter type from engine metadata.
    pub param_type: ParameterType,
    /// Default value from engine metadata (display string).
    pub default: String,
    /// Human-readable description (shown when focused).
    pub description: Option<String>,
    /// Numeric constraints (min/max/required).
    pub constraints: Option<bnto_core::metadata::Constraints>,
    /// Unit suffix for display (e.g., "%", "px").
    pub suffix: Option<String>,
    /// Conditional visibility — show only when another param matches a value.
    pub visible_when: Option<ParamCondition>,
}

/// Detail screen state — recipe info + form for parameter editing.
///
/// The "Continue" button is a virtual item at form field count,
/// managed by this wrapper — not by bnto_form.
#[derive(Debug)]
pub struct DetailModel {
    /// Recipe slug (used for forward navigation).
    pub slug: String,
    /// Recipe display name.
    pub name: String,
    /// Recipe description.
    pub description: String,
    /// Original param entries (kept for visible_when evaluation + confirm).
    pub params: Vec<ParamEntry>,
    /// Form model managing all field state, focus, and editing.
    pub form: FormModel,
    /// Whether focus is on the "Continue" button below the form.
    pub on_continue: bool,
}

/// Result of confirming the detail screen — collected param overrides.
pub struct ConfigResult {
    /// Map of "node_id.param_name" → value for passing to execution.
    pub overrides: HashMap<String, String>,
}

impl DetailModel {
    /// Whether the "Continue" action at the bottom is focused.
    pub fn is_continue_focused(&self) -> bool {
        self.on_continue
    }

    /// Build a detail model from test data (no engine dependency).
    #[cfg(test)]
    pub fn from_test_data(
        slug: &str,
        name: &str,
        description: &str,
        params: Vec<ParamEntry>,
    ) -> Self {
        let fields = super::detail_bridge::params_to_fields(&params);
        let form = FormModel::new(fields);
        Self {
            slug: slug.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            params,
            form,
            on_continue: false,
        }
    }

    /// Build a detail model from a recipe slug using engine metadata.
    pub fn from_slug(slug: &str, registry: &bnto_core::registry::NodeRegistry) -> Option<Self> {
        super::detail_loader::load_detail(slug, registry)
    }

    /// Confirm the current configuration — returns param overrides.
    ///
    /// Hidden params (failing `visible_when`) are excluded from overrides.
    /// Values come from the form fields, not the original ParamEntry values.
    pub fn confirm(&self) -> ConfigResult {
        let overrides = self
            .params
            .iter()
            .enumerate()
            .filter(|(_, p)| is_param_visible(p, &self.params))
            .filter_map(|(i, p)| {
                let value = self.form.fields.get(i).map(|f| f.value.clone())?;
                let key = format!("{}.{}", p.node_id, p.name);
                Some((key, value))
            })
            .collect();
        ConfigResult { overrides }
    }
}

/// Check whether a param should be visible based on its `visible_when` condition.
///
/// Evaluates against current values of all params. Returns true if:
/// - No condition (`visible_when` is None)
/// - Single condition matches
/// - Any condition in an OR list matches
pub fn is_param_visible(param: &ParamEntry, all_params: &[ParamEntry]) -> bool {
    let Some(condition) = &param.visible_when else {
        return true;
    };
    match condition {
        ParamCondition::Single(entry) => all_params
            .iter()
            .any(|p| p.name == entry.param && p.value == entry.equals),
        ParamCondition::Any(entries) => entries.iter().any(|entry| {
            all_params
                .iter()
                .any(|p| p.name == entry.param && p.value == entry.equals)
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::metadata::{Constraints, OptionEntry, ParamConditionEntry};

    // --- Test helpers ---

    fn test_param(
        node_id: &str,
        name: &str,
        label: &str,
        value: &str,
        param_type: ParameterType,
        default: &str,
    ) -> ParamEntry {
        ParamEntry {
            node_id: node_id.into(),
            name: name.into(),
            label: label.into(),
            value: value.into(),
            param_type,
            default: default.into(),
            description: None,
            constraints: None,
            suffix: None,
            visible_when: None,
        }
    }

    fn sample_params() -> Vec<ParamEntry> {
        let quality = {
            let mut p = test_param(
                "compress-image",
                "quality",
                "Quality",
                "80",
                ParameterType::Number,
                "80",
            );
            p.constraints = Some(Constraints {
                min: Some(1.0),
                max: Some(100.0),
                required: false,
            });
            p.suffix = Some("%".into());
            p.description = Some("Compression quality (1=smallest, 100=best)".into());
            p
        };

        vec![
            quality,
            test_param(
                "compress-image",
                "format",
                "Output Format",
                "jpeg",
                ParameterType::Enum {
                    options: vec![
                        OptionEntry {
                            value: "jpeg".into(),
                            label: "JPEG".into(),
                        },
                        OptionEntry {
                            value: "png".into(),
                            label: "PNG".into(),
                        },
                        OptionEntry {
                            value: "webp".into(),
                            label: "WebP".into(),
                        },
                    ],
                },
                "jpeg",
            ),
            test_param(
                "compress-image",
                "strip_metadata",
                "Strip Metadata",
                "true",
                ParameterType::Boolean,
                "true",
            ),
        ]
    }

    fn detail() -> DetailModel {
        DetailModel::from_test_data(
            "compress-images",
            "Compress Images",
            "Accepts image files and compresses each one.",
            sample_params(),
        )
    }

    // --- Initial state ---

    #[test]
    fn initial_state_has_recipe_info() {
        let m = detail();
        assert_eq!(m.slug, "compress-images");
        assert_eq!(m.name, "Compress Images");
        assert_eq!(
            m.description,
            "Accepts image files and compresses each one."
        );
    }

    #[test]
    fn initial_state_has_params_with_defaults() {
        let m = detail();
        assert_eq!(m.params.len(), 3);
        assert_eq!(m.params[0].name, "quality");
        assert_eq!(m.params[0].value, "80");
        assert_eq!(m.params[1].name, "format");
        assert_eq!(m.params[1].value, "jpeg");
    }

    #[test]
    fn initial_focus_is_zero_and_not_on_continue() {
        let m = detail();
        assert_eq!(m.form.focused, 0);
        assert!(!m.on_continue);
    }

    // --- Form fields match params ---

    #[test]
    fn form_has_same_field_count_as_params() {
        let m = detail();
        assert_eq!(m.form.fields.len(), m.params.len());
    }

    #[test]
    fn form_field_values_match_param_values() {
        let m = detail();
        assert_eq!(m.form.fields[0].value, "80");
        assert_eq!(m.form.fields[1].value, "jpeg");
        assert_eq!(m.form.fields[2].value, "true");
    }

    // --- Focus navigation via FormModel ---

    #[test]
    fn focus_next_advances_via_form() {
        let mut m = detail();
        m.form = bnto_form::update(m.form, bnto_form::FormMessage::FocusNext);
        assert_eq!(m.form.focused, 1);
    }

    // --- Confirm ---

    #[test]
    fn confirm_returns_all_param_overrides() {
        let m = detail();
        let result = m.confirm();
        assert_eq!(result.overrides.len(), 3);
        assert_eq!(
            result.overrides.get("compress-image.quality"),
            Some(&"80".to_string())
        );
        assert_eq!(
            result.overrides.get("compress-image.format"),
            Some(&"jpeg".to_string())
        );
    }

    #[test]
    fn confirm_reads_form_values_not_params() {
        let mut m = detail();
        // Change value via form field directly.
        m.form.fields[0].value = "60".into();
        let result = m.confirm();
        assert_eq!(
            result.overrides.get("compress-image.quality"),
            Some(&"60".to_string())
        );
    }

    #[test]
    fn confirm_omits_hidden_params() {
        let params = vec![
            conditional_param("mode", "compress", None),
            conditional_param("width", "800", Some(single_condition("mode", "resize"))),
            conditional_param("format", "jpeg", None),
        ];
        let m = DetailModel::from_test_data("s", "n", "d", params);
        let result = m.confirm();
        assert_eq!(result.overrides.len(), 2, "hidden param excluded");
        assert!(!result.overrides.contains_key("n.width"));
        assert!(result.overrides.contains_key("n.mode"));
        assert!(result.overrides.contains_key("n.format"));
    }

    #[test]
    fn confirm_includes_visible_conditional_params() {
        let params = vec![
            conditional_param("mode", "resize", None),
            conditional_param("width", "800", Some(single_condition("mode", "resize"))),
        ];
        let m = DetailModel::from_test_data("s", "n", "d", params);
        let result = m.confirm();
        assert_eq!(result.overrides.len(), 2, "matching condition → included");
        assert!(result.overrides.contains_key("n.width"));
    }

    // --- is_continue_focused ---

    #[test]
    fn is_continue_focused_when_on_continue() {
        let mut m = detail();
        m.on_continue = true;
        assert!(m.is_continue_focused());
    }

    #[test]
    fn is_continue_focused_false_on_param() {
        let m = detail();
        assert!(!m.is_continue_focused());
    }

    // --- Integration: loads real recipe from engine ---

    #[test]
    fn from_slug_loads_compress_images() {
        let registry = bnto_engine::create_registry();
        let m = DetailModel::from_slug("compress-images", &registry)
            .expect("compress-images should exist");
        assert_eq!(m.slug, "compress-images");
        assert_eq!(m.name, "Compress Images");
        assert!(!m.params.is_empty(), "should have at least one param");
        let quality = m.params.iter().find(|p| p.name == "quality");
        assert!(quality.is_some(), "should have a quality param");
        assert_eq!(quality.unwrap().value, "80");
    }

    #[test]
    fn from_slug_returns_none_for_unknown() {
        let registry = bnto_engine::create_registry();
        assert!(DetailModel::from_slug("nonexistent-recipe", &registry).is_none());
    }

    #[test]
    fn from_slug_skips_input_output_nodes() {
        let registry = bnto_engine::create_registry();
        let m = DetailModel::from_slug("compress-images", &registry).unwrap();
        for param in &m.params {
            assert_ne!(
                param.node_id, "input",
                "input node params should be skipped"
            );
            assert_ne!(
                param.node_id, "output",
                "output node params should be skipped"
            );
        }
    }

    #[test]
    fn from_slug_populates_param_labels_from_metadata() {
        let registry = bnto_engine::create_registry();
        let m = DetailModel::from_slug("compress-images", &registry).unwrap();
        let quality = m.params.iter().find(|p| p.name == "quality").unwrap();
        assert!(
            !quality.label.is_empty(),
            "label should come from engine metadata"
        );
    }

    #[test]
    fn from_slug_multi_node_recipe_extracts_all_processor_params() {
        let registry = bnto_engine::create_registry();
        let m =
            DetailModel::from_slug("resize-images", &registry).expect("resize-images should exist");
        assert!(
            m.params.len() >= 2,
            "resize recipe should have multiple params, got {}",
            m.params.len()
        );
    }

    // --- Integration: form fields wired from engine ---

    #[test]
    fn from_slug_creates_form_fields() {
        let registry = bnto_engine::create_registry();
        let m = DetailModel::from_slug("compress-images", &registry).unwrap();
        assert_eq!(
            m.form.fields.len(),
            m.params.len(),
            "form should have same field count as params"
        );
    }

    // --- Integration: metadata enrichment from engine ---

    #[test]
    fn from_slug_carries_constraints() {
        let registry = bnto_engine::create_registry();
        let m = DetailModel::from_slug("compress-images", &registry).unwrap();
        let quality = m.params.iter().find(|p| p.name == "quality").unwrap();
        assert!(
            quality.constraints.is_some(),
            "quality should have constraints from engine"
        );
        let c = quality.constraints.as_ref().unwrap();
        assert!(c.min.is_some(), "quality should have min constraint");
        assert!(c.max.is_some(), "quality should have max constraint");
    }

    #[test]
    fn from_slug_carries_description() {
        let registry = bnto_engine::create_registry();
        let m = DetailModel::from_slug("compress-images", &registry).unwrap();
        let quality = m.params.iter().find(|p| p.name == "quality").unwrap();
        assert!(
            quality.description.is_some(),
            "quality should have description from engine"
        );
    }

    // --- Conditional visibility (visible_when) ---

    fn conditional_param(
        name: &str,
        value: &str,
        visible_when: Option<ParamCondition>,
    ) -> ParamEntry {
        ParamEntry {
            node_id: "n".into(),
            name: name.into(),
            label: name.into(),
            value: value.into(),
            param_type: ParameterType::String,
            default: value.into(),
            description: None,
            constraints: None,
            suffix: None,
            visible_when,
        }
    }

    fn single_condition(param: &str, equals: &str) -> ParamCondition {
        ParamCondition::Single(ParamConditionEntry {
            param: param.into(),
            equals: equals.into(),
        })
    }

    fn any_condition(pairs: &[(&str, &str)]) -> ParamCondition {
        ParamCondition::Any(
            pairs
                .iter()
                .map(|(p, e)| ParamConditionEntry {
                    param: (*p).into(),
                    equals: (*e).into(),
                })
                .collect(),
        )
    }

    #[test]
    fn is_param_visible_true_when_no_condition() {
        let params = vec![conditional_param("mode", "resize", None)];
        assert!(is_param_visible(&params[0], &params));
    }

    #[test]
    fn is_param_visible_single_matches() {
        let params = vec![
            conditional_param("mode", "resize", None),
            conditional_param("width", "800", Some(single_condition("mode", "resize"))),
        ];
        assert!(is_param_visible(&params[1], &params));
    }

    #[test]
    fn is_param_visible_single_no_match() {
        let params = vec![
            conditional_param("mode", "compress", None),
            conditional_param("width", "800", Some(single_condition("mode", "resize"))),
        ];
        assert!(!is_param_visible(&params[1], &params));
    }

    #[test]
    fn is_param_visible_any_matches() {
        let params = vec![
            conditional_param("mode", "text", None),
            conditional_param(
                "prompt",
                "hello",
                Some(any_condition(&[("mode", "text"), ("mode", "url")])),
            ),
        ];
        assert!(is_param_visible(&params[1], &params));
    }

    #[test]
    fn is_param_visible_any_none_match() {
        let params = vec![
            conditional_param("mode", "file-upload", None),
            conditional_param(
                "prompt",
                "hello",
                Some(any_condition(&[("mode", "text"), ("mode", "url")])),
            ),
        ];
        assert!(!is_param_visible(&params[1], &params));
    }

    #[test]
    fn is_param_visible_unknown_param_defaults_hidden() {
        let params = vec![conditional_param(
            "width",
            "800",
            Some(single_condition("nonexistent", "resize")),
        )];
        assert!(!is_param_visible(&params[0], &params));
    }
}
