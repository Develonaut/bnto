// Ergonomic builder API for constructing form fields.

use std::sync::Arc;

use crate::field::{Field, FieldKind, FieldState, SelectOption, ValidatorFn};

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
        match &mut self.kind {
            FieldKind::Text { placeholder: p, .. } | FieldKind::TextArea { placeholder: p, .. } => {
                *p = Some(placeholder.to_string());
            }
            _ => {}
        }
        self
    }

    pub fn char_limit(mut self, limit: usize) -> Self {
        match &mut self.kind {
            FieldKind::Text { char_limit: c, .. } | FieldKind::TextArea { char_limit: c, .. } => {
                *c = Some(limit);
            }
            _ => {}
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

    /// Set allowed file extensions for a FilePath field.
    pub fn extensions(mut self, exts: &[&str]) -> Self {
        if let FieldKind::FilePath {
            extensions: ref mut e,
            ..
        } = self.kind
        {
            *e = exts.iter().map(|s| s.to_string()).collect();
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

/// Start building a file path browser field.
pub fn file_path(id: &str) -> FieldBuilder {
    FieldBuilder::new(
        id,
        FieldKind::FilePath {
            extensions: Vec::new(),
            start_home: true,
        },
    )
}

/// Start building a multi-line text area field.
pub fn textarea(id: &str) -> FieldBuilder {
    FieldBuilder::new(
        id,
        FieldKind::TextArea {
            placeholder: None,
            char_limit: None,
        },
    )
}

/// Start building a read-only note field for informational text.
pub fn note(id: &str, content: &str) -> FieldBuilder {
    FieldBuilder::new(
        id,
        FieldKind::Note {
            content: content.to_string(),
        },
    )
}
