// Bridge between engine ParamEntry metadata and bnto-form fields.
//
// Converts ParamEntry (engine-owned parameter definitions) into
// bnto_form::Field instances. Also handles visible_when re-evaluation
// and provides a FormTheme adapter wrapping the TUI Theme.

use bnto_core::metadata::ParameterType;
use bnto_form::{Field, FieldState, FormModel};
use ratatui::style::Style;

use super::detail::{ParamEntry, is_param_visible};
use crate::tui::theme::Theme;

/// Convert a ParamEntry into a bnto_form::Field.
///
/// Maps engine parameter types to form field kinds:
/// - Boolean → Confirm (Yes/No toggle)
/// - Enum → Select (with options, filterable if >5)
/// - Number with constraints → Number (with range/suffix)
/// - Number without constraints → Text (free-form entry)
/// - String → Text
pub fn param_to_field(param: &ParamEntry) -> Field {
    match &param.param_type {
        ParameterType::Boolean => bnto_form::confirm(&param.name)
            .label(&param.label)
            .value(&param.value)
            .default(Some(&param.default))
            .description(param.description.as_deref())
            .build(),

        ParameterType::Enum { options } => {
            let opts: Vec<(&str, &str)> = options
                .iter()
                .map(|o| (o.value.as_str(), o.label.as_str()))
                .collect();
            let filterable = options.len() > 5;
            let mut builder = bnto_form::select(&param.name, &opts)
                .label(&param.label)
                .value(&param.value)
                .default(Some(&param.default))
                .description(param.description.as_deref());
            if filterable {
                builder = builder.filterable();
            }
            builder.build()
        }

        ParameterType::Number => {
            if let Some(ref constraints) = param.constraints {
                let mut builder = bnto_form::number(&param.name)
                    .label(&param.label)
                    .value(&param.value)
                    .default(Some(&param.default))
                    .description(param.description.as_deref());
                if let (Some(min), Some(max)) = (constraints.min, constraints.max) {
                    builder = builder.range(min, max);
                }
                if let Some(ref suffix) = param.suffix {
                    builder = builder.suffix(suffix);
                }
                if param.control.as_deref() == Some("slider") {
                    builder = builder.slider(true);
                }
                builder.build()
            } else {
                // No constraints — fall back to text input for free-form entry.
                bnto_form::text(&param.name)
                    .label(&param.label)
                    .value(&param.value)
                    .default(Some(&param.default))
                    .description(param.description.as_deref())
                    .build()
            }
        }

        _ => bnto_form::text(&param.name)
            .label(&param.label)
            .value(&param.value)
            .default(Some(&param.default))
            .description(param.description.as_deref())
            .build(),
    }
}

/// Convert all ParamEntries into form fields, applying initial visibility.
pub fn params_to_fields(params: &[ParamEntry]) -> Vec<Field> {
    params
        .iter()
        .map(|p| {
            let mut field = param_to_field(p);
            field.visible = is_param_visible(p, params);
            field
        })
        .collect()
}

/// Re-evaluate field visibility after a form value changes.
///
/// Reads current values from the form's fields, maps them back to
/// the ParamEntry list to evaluate visible_when conditions, then
/// updates each field's `visible` flag.
pub fn update_visibility(form: &mut FormModel, params: &[ParamEntry]) {
    // Build a temporary params list with updated values from the form.
    let updated_params: Vec<ParamEntry> = params
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let value = form
                .fields
                .get(i)
                .map(|f| f.value.clone())
                .unwrap_or_else(|| p.value.clone());
            ParamEntry { value, ..p.clone() }
        })
        .collect();

    for (i, param) in params.iter().enumerate() {
        if let Some(field) = form.fields.get_mut(i) {
            field.visible = is_param_visible(
                &ParamEntry {
                    visible_when: param.visible_when.clone(),
                    ..updated_params[i].clone()
                },
                &updated_params,
            );
        }
    }
}

/// Whether the form is currently in an editing state (any field not Idle).
pub fn is_form_editing(form: &FormModel) -> bool {
    form.focused_field()
        .is_some_and(|f| !matches!(f.state, FieldState::Idle))
}

/// Theme adapter — wraps the TUI Theme to implement bnto_form::FormTheme.
pub struct BntoFormTheme<'a>(pub &'a Theme);

impl bnto_form::FormTheme for BntoFormTheme<'_> {
    fn text(&self) -> Style {
        self.0.text()
    }

    fn selected(&self) -> Style {
        self.0.selected()
    }

    fn muted(&self) -> Style {
        self.0.muted()
    }

    fn error(&self) -> Style {
        Style::default().fg(ratatui::style::Color::Red)
    }

    fn border(&self) -> Style {
        self.0.muted()
    }

    fn heading(&self) -> Style {
        self.0.heading()
    }
}

/// Extract SelectOption values from a field for comparison (test helper).
#[cfg(test)]
fn select_options(field: &Field) -> Vec<bnto_form::SelectOption> {
    match &field.kind {
        bnto_form::FieldKind::Select { options, .. } => options.clone(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::metadata::{Constraints, OptionEntry, ParamCondition, ParamConditionEntry};
    use bnto_form::FieldKind;

    fn string_param(name: &str, value: &str) -> ParamEntry {
        ParamEntry {
            node_id: "n".into(),
            name: name.into(),
            label: format!("{name} Label"),
            value: value.into(),
            param_type: ParameterType::String,
            default: value.into(),
            description: Some(format!("{name} desc")),
            constraints: None,
            suffix: None,
            control: None,
            visible_when: None,
        }
    }

    fn bool_param(name: &str, value: &str) -> ParamEntry {
        ParamEntry {
            param_type: ParameterType::Boolean,
            ..string_param(name, value)
        }
    }

    fn enum_param(name: &str, value: &str, opts: &[(&str, &str)]) -> ParamEntry {
        ParamEntry {
            param_type: ParameterType::Enum {
                options: opts
                    .iter()
                    .map(|(v, l)| OptionEntry {
                        value: v.to_string(),
                        label: l.to_string(),
                    })
                    .collect(),
            },
            ..string_param(name, value)
        }
    }

    fn number_param(
        name: &str,
        value: &str,
        min: f64,
        max: f64,
        suffix: Option<&str>,
    ) -> ParamEntry {
        ParamEntry {
            param_type: ParameterType::Number,
            constraints: Some(Constraints {
                min: Some(min),
                max: Some(max),
                required: false,
            }),
            suffix: suffix.map(|s| s.to_string()),
            control: None,
            ..string_param(name, value)
        }
    }

    fn number_param_no_constraints(name: &str, value: &str) -> ParamEntry {
        ParamEntry {
            param_type: ParameterType::Number,
            constraints: None,
            ..string_param(name, value)
        }
    }

    // --- param_to_field: String ---

    #[test]
    fn string_param_becomes_text_field() {
        let field = param_to_field(&string_param("query", "hello"));
        assert_eq!(field.id, "query");
        assert_eq!(field.label, "query Label");
        assert_eq!(field.value, "hello");
        assert_eq!(field.default.as_deref(), Some("hello"));
        assert_eq!(field.description.as_deref(), Some("query desc"));
        assert!(matches!(field.kind, FieldKind::Text { .. }));
    }

    // --- param_to_field: Boolean ---

    #[test]
    fn boolean_param_becomes_confirm_field() {
        let field = param_to_field(&bool_param("strip", "true"));
        assert_eq!(field.id, "strip");
        assert_eq!(field.value, "true");
        assert!(matches!(field.kind, FieldKind::Confirm { .. }));
    }

    // --- param_to_field: Enum ---

    #[test]
    fn enum_param_becomes_select_field() {
        let field = param_to_field(&enum_param(
            "format",
            "jpeg",
            &[("jpeg", "JPEG"), ("png", "PNG"), ("webp", "WebP")],
        ));
        assert_eq!(field.id, "format");
        assert_eq!(field.value, "jpeg");
        let opts = select_options(&field);
        assert_eq!(opts.len(), 3);
        assert_eq!(opts[0].value, "jpeg");
        assert_eq!(opts[0].label, "JPEG");
    }

    #[test]
    fn enum_with_many_options_is_filterable() {
        let many_opts: Vec<(&str, &str)> = (0..6)
            .map(|i| {
                // Leak strings for test lifetime
                let v: &str = Box::leak(format!("v{i}").into_boxed_str());
                let l: &str = Box::leak(format!("L{i}").into_boxed_str());
                (v, l)
            })
            .collect();
        let field = param_to_field(&enum_param("many", "v0", &many_opts));
        match &field.kind {
            FieldKind::Select { filterable, .. } => assert!(filterable),
            _ => panic!("expected Select"),
        }
    }

    #[test]
    fn enum_with_few_options_not_filterable() {
        let field = param_to_field(&enum_param("fmt", "a", &[("a", "A"), ("b", "B")]));
        match &field.kind {
            FieldKind::Select { filterable, .. } => assert!(!filterable),
            _ => panic!("expected Select"),
        }
    }

    // --- param_to_field: Number ---

    #[test]
    fn number_with_constraints_becomes_number_field() {
        let field = param_to_field(&number_param("quality", "80", 1.0, 100.0, Some("%")));
        assert_eq!(field.id, "quality");
        assert_eq!(field.value, "80");
        match &field.kind {
            FieldKind::Number {
                min, max, suffix, ..
            } => {
                assert_eq!(*min, Some(1.0));
                assert_eq!(*max, Some(100.0));
                assert_eq!(suffix.as_deref(), Some("%"));
            }
            _ => panic!("expected Number"),
        }
    }

    #[test]
    fn number_with_slider_control_enables_slider() {
        let mut param = number_param("quality", "80", 1.0, 100.0, Some("%"));
        param.control = Some("slider".into());
        let field = param_to_field(&param);
        match &field.kind {
            FieldKind::Number { slider, .. } => assert!(*slider, "slider should be enabled"),
            _ => panic!("expected Number"),
        }
    }

    #[test]
    fn number_without_slider_control_disables_slider() {
        let field = param_to_field(&number_param("quality", "80", 1.0, 100.0, Some("%")));
        match &field.kind {
            FieldKind::Number { slider, .. } => assert!(!*slider, "slider should be disabled"),
            _ => panic!("expected Number"),
        }
    }

    #[test]
    fn number_without_constraints_becomes_text_field() {
        let field = param_to_field(&number_param_no_constraints("count", "10"));
        assert!(
            matches!(field.kind, FieldKind::Text { .. }),
            "unconstrained Number should fall back to Text"
        );
    }

    // --- visibility ---

    #[test]
    fn update_visibility_hides_conditional_params() {
        let params = vec![
            string_param("mode", "compress"),
            ParamEntry {
                visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                    param: "mode".into(),
                    equals: "resize".into(),
                })),
                ..string_param("width", "800")
            },
        ];
        let fields = params_to_fields(&params);
        let mut form = FormModel::new(fields);
        update_visibility(&mut form, &params);
        assert!(
            !form.fields[1].visible,
            "width should be hidden when mode=compress"
        );
    }

    #[test]
    fn update_visibility_shows_matching_params() {
        let params = vec![
            string_param("mode", "resize"),
            ParamEntry {
                visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                    param: "mode".into(),
                    equals: "resize".into(),
                })),
                ..string_param("width", "800")
            },
        ];
        let fields = params_to_fields(&params);
        let mut form = FormModel::new(fields);
        update_visibility(&mut form, &params);
        assert!(
            form.fields[1].visible,
            "width should be visible when mode=resize"
        );
    }

    #[test]
    fn update_visibility_uses_form_values_not_original() {
        let params = vec![
            string_param("mode", "compress"),
            ParamEntry {
                visible_when: Some(ParamCondition::Single(ParamConditionEntry {
                    param: "mode".into(),
                    equals: "resize".into(),
                })),
                ..string_param("width", "800")
            },
        ];
        let fields = params_to_fields(&params);
        let mut form = FormModel::new(fields);
        // Simulate form changing mode to "resize".
        form.fields[0].value = "resize".into();
        update_visibility(&mut form, &params);
        assert!(
            form.fields[1].visible,
            "should use form value, not original param value"
        );
    }

    // --- is_form_editing ---

    #[test]
    fn is_form_editing_false_when_idle() {
        let form = FormModel::new(vec![bnto_form::text("a").build()]);
        assert!(!is_form_editing(&form));
    }

    #[test]
    fn is_form_editing_true_when_text_editing() {
        let mut form = FormModel::new(vec![bnto_form::text("a").build()]);
        form.fields[0].state = FieldState::TextEditing {
            buffer: String::new(),
            cursor: 0,
        };
        assert!(is_form_editing(&form));
    }

    // --- theme bridge ---

    #[test]
    fn theme_bridge_maps_styles() {
        use crate::tui::theme::ThemeVariant;
        use bnto_form::FormTheme;

        let theme = Theme::from_variant(ThemeVariant::Tokyo);
        let bridge = BntoFormTheme(&theme);
        // Just verify the trait methods don't panic and return valid styles.
        let _ = bridge.text();
        let _ = bridge.selected();
        let _ = bridge.muted();
        let _ = bridge.error();
        let _ = bridge.border();
        let _ = bridge.heading();
    }
}
