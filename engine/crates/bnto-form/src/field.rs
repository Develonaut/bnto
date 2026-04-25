//! Core field types for bnto-form.
//!
//! A `Field` represents one form control — text input, select dropdown,
//! confirm toggle, or number slider. Each field carries its own state,
//! value, and optional validation error.

use std::sync::Arc;

/// Callback that validates a field value. Returns `Some(error_message)` on failure.
///
/// Uses `Arc` so validators can capture parameters (e.g., `min_len(3)`)
/// while remaining `Clone + Send + Sync`.
pub type ValidatorFn = Arc<dyn Fn(&str) -> Option<String> + Send + Sync>;

/// A single form field with its display metadata, current value, and editing state.
#[derive(Clone)]
pub struct Field {
    pub id: String,
    pub label: String,
    pub kind: FieldKind,
    pub state: FieldState,
    pub value: String,
    pub default: Option<String>,
    pub description: Option<String>,
    pub error: Option<String>,
    pub validator: Option<ValidatorFn>,
    pub visible: bool,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("kind", &self.kind)
            .field("state", &self.state)
            .field("value", &self.value)
            .field("visible", &self.visible)
            .field("error", &self.error)
            .field("validator", &self.validator.as_ref().map(|_| "<fn>"))
            .finish()
    }
}

/// What type of control this field renders as.
#[derive(Debug, Clone)]
pub enum FieldKind {
    Text {
        placeholder: Option<String>,
        char_limit: Option<usize>,
    },
    Select {
        options: Vec<SelectOption>,
        filterable: bool,
    },
    Confirm {
        affirmative: String,
        negative: String,
    },
    Number {
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
        suffix: Option<String>,
        /// When true, render a visual slider bar alongside the value.
        /// Best for bounded ranges with intuitive visual mapping (e.g. percentages).
        slider: bool,
    },
}

/// A select option with separate display label and stored value.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl Field {
    /// One-line formatted value for display mode rendering.
    ///
    /// Each field kind produces a human-readable summary:
    /// - Text: the raw value (or placeholder)
    /// - Number: value + suffix (e.g., "80%")
    /// - Select: the display label (not the raw value)
    /// - Confirm: affirmative/negative label (e.g., "Yes" / "No")
    pub fn display_value(&self) -> String {
        match &self.kind {
            FieldKind::Text { .. } => self.value.clone(),
            FieldKind::Number { suffix, .. } => {
                let s = suffix.as_deref().unwrap_or("");
                format!("{}{s}", self.value)
            }
            FieldKind::Select { options, .. } => options
                .iter()
                .find(|o| o.value == self.value)
                .map(|o| o.label.clone())
                .unwrap_or_else(|| self.value.clone()),
            FieldKind::Confirm {
                affirmative,
                negative,
            } => {
                if self.value == "true" {
                    affirmative.clone()
                } else {
                    negative.clone()
                }
            }
        }
    }
}

/// Transient editing state for a field. Idle when not being edited.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldState {
    Idle,
    TextEditing {
        buffer: String,
        cursor: usize,
    },
    SelectExpanded {
        highlight: usize,
        filter: String,
        filtered_indices: Vec<usize>,
    },
    NumberEditing {
        buffer: String,
        cursor: usize,
    },
}

// --- FieldBuilder ---

/// Ergonomic builder for constructing fields.
pub struct FieldBuilder {
    id: String,
    label: Option<String>,
    kind: FieldKind,
    value: Option<String>,
    default: Option<String>,
    description: Option<String>,
    validator: Option<ValidatorFn>,
    visible: bool,
}

impl FieldBuilder {
    fn new(id: &str, kind: FieldKind) -> Self {
        Self {
            id: id.to_string(),
            label: None,
            kind,
            value: None,
            default: None,
            description: None,
            validator: None,
            visible: true,
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }

    pub fn default(mut self, default: Option<&str>) -> Self {
        self.default = default.map(|s| s.to_string());
        self
    }

    pub fn description(mut self, desc: Option<&str>) -> Self {
        self.description = desc.map(|s| s.to_string());
        self
    }

    pub fn validator(mut self, v: ValidatorFn) -> Self {
        self.validator = Some(v);
        self
    }

    /// Shorthand for `.validator(Arc::new(not_empty))` — rejects empty values.
    pub fn required(self) -> Self {
        self.validator(Arc::new(crate::validators::not_empty))
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        if let FieldKind::Text {
            placeholder: ref mut p,
            ..
        } = self.kind
        {
            *p = Some(placeholder.to_string());
        }
        self
    }

    pub fn char_limit(mut self, limit: usize) -> Self {
        if let FieldKind::Text {
            char_limit: ref mut c,
            ..
        } = self.kind
        {
            *c = Some(limit);
        }
        self
    }

    pub fn filterable(mut self) -> Self {
        if let FieldKind::Select {
            filterable: ref mut f,
            ..
        } = self.kind
        {
            *f = true;
        }
        self
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        if let FieldKind::Number {
            min: ref mut mi,
            max: ref mut ma,
            ..
        } = self.kind
        {
            *mi = Some(min);
            *ma = Some(max);
        }
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        if let FieldKind::Number {
            step: ref mut s, ..
        } = self.kind
        {
            *s = Some(step);
        }
        self
    }

    pub fn suffix(mut self, suffix: &str) -> Self {
        if let FieldKind::Number {
            suffix: ref mut su, ..
        } = self.kind
        {
            *su = Some(suffix.to_string());
        }
        self
    }

    /// Enable a visual slider bar for this number field.
    pub fn slider(mut self, enabled: bool) -> Self {
        if let FieldKind::Number {
            slider: ref mut s, ..
        } = self.kind
        {
            *s = enabled;
        }
        self
    }

    pub fn build(self) -> Field {
        let label = self.label.unwrap_or_else(|| self.id.clone());
        let value = self.value.unwrap_or_default();

        Field {
            id: self.id,
            label,
            kind: self.kind,
            state: FieldState::Idle,
            value,
            default: self.default,
            description: self.description,
            error: None,
            validator: self.validator,
            visible: self.visible,
        }
    }
}

// --- Top-level builder functions ---

/// Start building a text input field.
pub fn text(id: &str) -> FieldBuilder {
    FieldBuilder::new(
        id,
        FieldKind::Text {
            placeholder: None,
            char_limit: None,
        },
    )
}

/// Start building a select field.
pub fn select(id: &str, options: &[(&str, &str)]) -> FieldBuilder {
    let opts = options
        .iter()
        .map(|(value, label)| SelectOption {
            value: value.to_string(),
            label: label.to_string(),
        })
        .collect();
    FieldBuilder::new(
        id,
        FieldKind::Select {
            options: opts,
            filterable: false,
        },
    )
}

/// Start building a confirm (yes/no) field.
pub fn confirm(id: &str) -> FieldBuilder {
    FieldBuilder::new(
        id,
        FieldKind::Confirm {
            affirmative: "Yes".to_string(),
            negative: "No".to_string(),
        },
    )
}

/// Start building a number field.
pub fn number(id: &str) -> FieldBuilder {
    FieldBuilder::new(
        id,
        FieldKind::Number {
            min: None,
            max: None,
            step: None,
            suffix: None,
            slider: false,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_builder_text() {
        let field = text("name").label("Recipe Name").build();
        assert_eq!(field.id, "name");
        assert_eq!(field.label, "Recipe Name");
        assert!(matches!(field.kind, FieldKind::Text { .. }));
    }

    #[test]
    fn test_field_builder_text_with_placeholder() {
        let field = text("name")
            .placeholder("Enter name")
            .char_limit(50)
            .build();
        match &field.kind {
            FieldKind::Text {
                placeholder,
                char_limit,
            } => {
                assert_eq!(placeholder.as_deref(), Some("Enter name"));
                assert_eq!(*char_limit, Some(50));
            }
            _ => panic!("expected Text kind"),
        }
    }

    #[test]
    fn test_field_builder_select() {
        let field = select("format", &[("jpeg", "JPEG"), ("png", "PNG")])
            .label("Format")
            .build();
        match &field.kind {
            FieldKind::Select {
                options,
                filterable,
            } => {
                assert_eq!(options.len(), 2);
                assert_eq!(options[0].value, "jpeg");
                assert_eq!(options[0].label, "JPEG");
                assert!(!filterable);
            }
            _ => panic!("expected Select kind"),
        }
    }

    #[test]
    fn test_field_builder_select_filterable() {
        let field = select("format", &[("a", "A")]).filterable().build();
        match &field.kind {
            FieldKind::Select { filterable, .. } => assert!(filterable),
            _ => panic!("expected Select kind"),
        }
    }

    #[test]
    fn test_field_builder_confirm() {
        let field = confirm("overwrite").label("Overwrite?").build();
        match &field.kind {
            FieldKind::Confirm {
                affirmative,
                negative,
            } => {
                assert_eq!(affirmative, "Yes");
                assert_eq!(negative, "No");
            }
            _ => panic!("expected Confirm kind"),
        }
    }

    #[test]
    fn test_field_builder_number() {
        let field = number("quality")
            .range(1.0, 100.0)
            .step(5.0)
            .suffix("%")
            .build();
        match &field.kind {
            FieldKind::Number {
                min,
                max,
                step,
                suffix,
                slider,
            } => {
                assert_eq!(*min, Some(1.0));
                assert_eq!(*max, Some(100.0));
                assert_eq!(*step, Some(5.0));
                assert_eq!(suffix.as_deref(), Some("%"));
                assert!(!slider, "slider defaults to false");
            }
            _ => panic!("expected Number kind"),
        }
    }

    #[test]
    fn test_field_builder_defaults() {
        let field = text("x").build();
        assert_eq!(field.label, "x"); // falls back to id
        assert_eq!(field.value, "");
        assert!(field.default.is_none());
        assert!(field.description.is_none());
        assert!(field.error.is_none());
        assert!(field.validator.is_none());
    }

    #[test]
    fn test_field_state_idle_by_default() {
        let field = text("x").build();
        assert_eq!(field.state, FieldState::Idle);
    }

    #[test]
    fn test_field_visible_by_default() {
        let field = text("x").build();
        assert!(field.visible);
    }

    #[test]
    fn test_field_builder_with_value_and_default() {
        let field = text("x")
            .value("current")
            .default(Some("fallback"))
            .description(Some("A description"))
            .build();
        assert_eq!(field.value, "current");
        assert_eq!(field.default.as_deref(), Some("fallback"));
        assert_eq!(field.description.as_deref(), Some("A description"));
    }

    #[test]
    fn test_field_builder_hidden() {
        let field = text("x").visible(false).build();
        assert!(!field.visible);
    }

    // --- display_value tests ---

    #[test]
    fn test_display_value_text() {
        let field = text("x").value("hello").build();
        assert_eq!(field.display_value(), "hello");
    }

    #[test]
    fn test_display_value_number_with_suffix() {
        let field = number("q").suffix("%").value("80").build();
        assert_eq!(field.display_value(), "80%");
    }

    #[test]
    fn test_display_value_number_no_suffix() {
        let field = number("q").value("42").build();
        assert_eq!(field.display_value(), "42");
    }

    #[test]
    fn test_display_value_select_shows_label() {
        let field = select("fmt", &[("jpeg", "JPEG"), ("png", "PNG")])
            .value("jpeg")
            .build();
        assert_eq!(field.display_value(), "JPEG");
    }

    #[test]
    fn test_display_value_select_unknown_value() {
        let field = select("fmt", &[("a", "Alpha")]).value("unknown").build();
        assert_eq!(field.display_value(), "unknown");
    }

    #[test]
    fn test_display_value_confirm_true() {
        let field = confirm("ok").value("true").build();
        assert_eq!(field.display_value(), "Yes");
    }

    #[test]
    fn test_display_value_confirm_false() {
        let field = confirm("ok").value("false").build();
        assert_eq!(field.display_value(), "No");
    }
}
