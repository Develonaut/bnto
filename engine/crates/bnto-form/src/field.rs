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
    FilePath {
        extensions: Vec<String>,
        start_home: bool,
    },
    TextArea {
        placeholder: Option<String>,
        char_limit: Option<usize>,
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
            FieldKind::FilePath { .. } => {
                if self.value.is_empty() {
                    String::new()
                } else {
                    abbreviate_home(&self.value)
                }
            }
            FieldKind::TextArea { .. } => {
                if self.value.is_empty() {
                    return String::new();
                }
                let lines: Vec<&str> = self.value.lines().collect();
                let first = lines.first().unwrap_or(&"");
                if lines.len() > 1 {
                    format!("{first} ({} lines)", lines.len())
                } else {
                    first.to_string()
                }
            }
        }
    }
}

/// Replace the user's home directory prefix with `~` for compact display.
pub fn abbreviate_home(path: &str) -> String {
    if let Ok(home) = std::env::var("HOME")
        && path.starts_with(&home)
    {
        format!("~{}", &path[home.len()..])
    } else {
        path.to_string()
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
    FilePathBrowsing {
        current_dir: std::path::PathBuf,
        entries: Vec<crate::file_entry::FileEntry>,
        cursor: usize,
        show_hidden: bool,
        viewport_offset: usize,
        viewport_height: usize,
        nav_history: crate::file_entry::NavHistory,
    },
    TextAreaEditing {
        buffer: String,
        cursor: usize,
        line: usize,
        scroll_offset: usize,
    },
}

pub use crate::field_builder::{FieldBuilder, confirm, file_path, number, select, text, textarea};

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

    // --- FilePath tests ---

    #[test]
    fn test_field_builder_file_path() {
        let field = file_path("input").label("Input File").build();
        assert_eq!(field.id, "input");
        assert!(matches!(field.kind, FieldKind::FilePath { .. }));
    }

    #[test]
    fn test_field_builder_file_path_defaults() {
        let field = file_path("f").build();
        match &field.kind {
            FieldKind::FilePath {
                extensions,
                start_home,
            } => {
                assert!(extensions.is_empty());
                assert!(*start_home);
            }
            _ => panic!("expected FilePath kind"),
        }
    }

    #[test]
    fn test_field_builder_file_path_with_extensions() {
        let field = file_path("f").extensions(&["jpg", "png"]).build();
        match &field.kind {
            FieldKind::FilePath { extensions, .. } => {
                assert_eq!(extensions, &["jpg", "png"]);
            }
            _ => panic!("expected FilePath kind"),
        }
    }

    #[test]
    fn test_display_value_file_path_empty() {
        let field = file_path("f").build();
        assert_eq!(field.display_value(), "");
    }

    #[test]
    fn test_display_value_file_path_abbreviates_home() {
        let home = std::env::var("HOME").unwrap_or_default();
        if home.is_empty() {
            return;
        }
        let path = format!("{home}/Documents/file.txt");
        let field = file_path("f").value(&path).build();
        assert_eq!(field.display_value(), "~/Documents/file.txt");
    }

    // --- TextArea tests ---

    #[test]
    fn test_field_builder_textarea() {
        let field = textarea("notes").label("Notes").build();
        assert_eq!(field.id, "notes");
        assert!(matches!(field.kind, FieldKind::TextArea { .. }));
    }

    #[test]
    fn test_field_builder_textarea_defaults() {
        let field = textarea("t").build();
        match &field.kind {
            FieldKind::TextArea {
                placeholder,
                char_limit,
            } => {
                assert!(placeholder.is_none());
                assert!(char_limit.is_none());
            }
            _ => panic!("expected TextArea kind"),
        }
    }

    #[test]
    fn test_field_builder_textarea_with_options() {
        let field = textarea("t")
            .placeholder("Enter text")
            .char_limit(500)
            .build();
        match &field.kind {
            FieldKind::TextArea {
                placeholder,
                char_limit,
            } => {
                assert_eq!(placeholder.as_deref(), Some("Enter text"));
                assert_eq!(*char_limit, Some(500));
            }
            _ => panic!("expected TextArea kind"),
        }
    }

    #[test]
    fn test_display_value_textarea_empty() {
        let field = textarea("t").build();
        assert_eq!(field.display_value(), "");
    }

    #[test]
    fn test_display_value_textarea_single_line() {
        let field = textarea("t").value("hello world").build();
        assert_eq!(field.display_value(), "hello world");
    }

    #[test]
    fn test_display_value_textarea_multi_line() {
        let field = textarea("t").value("line1\nline2\nline3").build();
        assert_eq!(field.display_value(), "line1 (3 lines)");
    }
}
