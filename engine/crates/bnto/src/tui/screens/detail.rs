// Recipe detail screen — TEA state + transitions for recipe configuration.
//
// Model (state) + Message (events) + update() (pure transitions).
// Loading logic lives in detail_loader.rs.

use std::collections::HashMap;

use bnto_core::metadata::ParameterType;
use bnto_core::registry::NodeRegistry;

/// A single editable parameter entry shown in the detail screen.
#[derive(Debug, Clone)]
#[allow(dead_code)]
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
}

/// Detail screen state — recipe info + editable parameter list.
#[derive(Debug)]
pub struct DetailModel {
    /// Recipe slug (used for forward navigation).
    pub slug: String,
    /// Recipe display name.
    pub name: String,
    /// Recipe description.
    pub description: String,
    /// Editable parameters extracted from processor nodes.
    pub params: Vec<ParamEntry>,
    /// Which param currently has focus (index into `params`).
    pub focused: usize,
    /// Whether the focused param is being edited.
    pub editing: bool,
    /// Text buffer for the current edit.
    pub edit_buffer: String,
}

/// Messages the detail screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetailMessage {
    /// Move focus to the next parameter.
    FocusNext,
    /// Move focus to the previous parameter.
    FocusPrev,
    /// Start editing the focused parameter.
    StartEdit,
    /// User typed a character into the edit buffer.
    EditChar(char),
    /// User pressed backspace in edit mode.
    EditBackspace,
    /// Commit the edit buffer to the focused param's value.
    CommitEdit,
    /// Cancel editing — restore the previous value.
    CancelEdit,
}

/// Result of confirming the detail screen — collected param overrides.
#[allow(dead_code)]
pub struct ConfigResult {
    /// Map of "node_id.param_name" → value for passing to execution.
    pub overrides: HashMap<String, String>,
}

impl DetailModel {
    /// Build a detail model from test data (no engine dependency).
    #[cfg(test)]
    pub fn from_test_data(
        slug: &str,
        name: &str,
        description: &str,
        params: Vec<ParamEntry>,
    ) -> Self {
        Self {
            slug: slug.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            params,
            focused: 0,
            editing: false,
            edit_buffer: String::new(),
        }
    }

    /// Build a detail model from a recipe slug using engine metadata.
    pub fn from_slug(slug: &str, registry: &NodeRegistry) -> Option<Self> {
        super::detail_loader::load_detail(slug, registry)
    }

    /// Confirm the current configuration — returns param overrides.
    #[allow(dead_code)]
    pub fn confirm(&self) -> ConfigResult {
        let overrides = self
            .params
            .iter()
            .map(|p| {
                let key = format!("{}.{}", p.node_id, p.name);
                (key, p.value.clone())
            })
            .collect();
        ConfigResult { overrides }
    }
}

/// Pure state transition for the detail screen.
pub fn update(model: DetailModel, msg: DetailMessage) -> DetailModel {
    match msg {
        DetailMessage::FocusNext => {
            if model.editing || model.params.is_empty() {
                return model;
            }
            let next = if model.focused + 1 >= model.params.len() {
                0
            } else {
                model.focused + 1
            };
            DetailModel {
                focused: next,
                ..model
            }
        }
        DetailMessage::FocusPrev => {
            if model.editing || model.params.is_empty() {
                return model;
            }
            let prev = if model.focused == 0 {
                model.params.len().saturating_sub(1)
            } else {
                model.focused - 1
            };
            DetailModel {
                focused: prev,
                ..model
            }
        }
        DetailMessage::StartEdit => {
            if model.params.is_empty() {
                return model;
            }
            let buffer = model.params[model.focused].value.clone();
            DetailModel {
                editing: true,
                edit_buffer: buffer,
                ..model
            }
        }
        DetailMessage::EditChar(ch) => {
            if !model.editing {
                return model;
            }
            let mut buf = model.edit_buffer;
            buf.push(ch);
            DetailModel {
                edit_buffer: buf,
                ..model
            }
        }
        DetailMessage::EditBackspace => {
            if !model.editing {
                return model;
            }
            let mut buf = model.edit_buffer;
            buf.pop();
            DetailModel {
                edit_buffer: buf,
                ..model
            }
        }
        DetailMessage::CommitEdit => {
            if !model.editing {
                return model;
            }
            let mut params = model.params;
            params[model.focused].value = model.edit_buffer.clone();
            DetailModel {
                params,
                editing: false,
                edit_buffer: String::new(),
                ..model
            }
        }
        DetailMessage::CancelEdit => DetailModel {
            editing: false,
            edit_buffer: String::new(),
            ..model
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Test helpers ---

    fn sample_params() -> Vec<ParamEntry> {
        vec![
            ParamEntry {
                node_id: "compress-image".into(),
                name: "quality".into(),
                label: "Quality".into(),
                value: "80".into(),
                param_type: ParameterType::Number,
                default: "80".into(),
            },
            ParamEntry {
                node_id: "compress-image".into(),
                name: "format".into(),
                label: "Output Format".into(),
                value: "jpeg".into(),
                param_type: ParameterType::Enum {
                    options: vec!["jpeg".into(), "png".into(), "webp".into()],
                },
                default: "jpeg".into(),
            },
            ParamEntry {
                node_id: "compress-image".into(),
                name: "strip_metadata".into(),
                label: "Strip Metadata".into(),
                value: "true".into(),
                param_type: ParameterType::Boolean,
                default: "true".into(),
            },
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
    fn initial_focus_is_zero_and_not_editing() {
        let m = detail();
        assert_eq!(m.focused, 0);
        assert!(!m.editing);
        assert_eq!(m.edit_buffer, "");
    }

    // --- Focus navigation ---

    #[test]
    fn focus_next_advances_by_one() {
        let m = detail();
        let m = update(m, DetailMessage::FocusNext);
        assert_eq!(m.focused, 1);
    }

    #[test]
    fn focus_next_wraps_at_end() {
        let mut m = detail();
        m.focused = 2; // last param
        let m = update(m, DetailMessage::FocusNext);
        assert_eq!(m.focused, 0);
    }

    #[test]
    fn focus_prev_moves_back_by_one() {
        let mut m = detail();
        m.focused = 2;
        let m = update(m, DetailMessage::FocusPrev);
        assert_eq!(m.focused, 1);
    }

    #[test]
    fn focus_prev_wraps_at_start() {
        let m = detail(); // focused = 0
        let m = update(m, DetailMessage::FocusPrev);
        assert_eq!(m.focused, 2);
    }

    #[test]
    fn focus_movement_on_empty_params_is_safe() {
        let m = DetailModel::from_test_data("s", "n", "d", vec![]);
        let m = update(m, DetailMessage::FocusNext);
        assert_eq!(m.focused, 0);
        let m = update(m, DetailMessage::FocusPrev);
        assert_eq!(m.focused, 0);
    }

    #[test]
    fn focus_locked_while_editing() {
        let mut m = detail();
        m.editing = true;
        m.edit_buffer = "90".into();
        let m = update(m, DetailMessage::FocusNext);
        assert_eq!(m.focused, 0, "focus should not move while editing");
        let m = update(m, DetailMessage::FocusPrev);
        assert_eq!(m.focused, 0, "focus should not move while editing");
    }

    // --- Editing workflow ---

    #[test]
    fn start_edit_copies_value_to_buffer() {
        let m = detail();
        let m = update(m, DetailMessage::StartEdit);
        assert!(m.editing);
        assert_eq!(m.edit_buffer, "80");
    }

    #[test]
    fn edit_char_appends_to_buffer() {
        let mut m = detail();
        m.editing = true;
        m.edit_buffer = "8".into();
        let m = update(m, DetailMessage::EditChar('5'));
        assert_eq!(m.edit_buffer, "85");
    }

    #[test]
    fn edit_backspace_removes_last_char() {
        let mut m = detail();
        m.editing = true;
        m.edit_buffer = "85".into();
        let m = update(m, DetailMessage::EditBackspace);
        assert_eq!(m.edit_buffer, "8");
    }

    #[test]
    fn edit_backspace_on_empty_buffer_is_safe() {
        let mut m = detail();
        m.editing = true;
        m.edit_buffer = String::new();
        let m = update(m, DetailMessage::EditBackspace);
        assert_eq!(m.edit_buffer, "");
    }

    #[test]
    fn commit_edit_writes_buffer_to_param_value() {
        let m = detail();
        let m = update(m, DetailMessage::StartEdit);
        let m = update(m, DetailMessage::EditChar('9'));
        let m = update(m, DetailMessage::EditChar('0'));
        let m = update(m, DetailMessage::CommitEdit);
        assert!(!m.editing);
        assert_eq!(m.params[0].value, "8090", "buffer was '80' + '9' + '0'");
        assert_eq!(m.edit_buffer, "");
    }

    #[test]
    fn cancel_edit_restores_original_value() {
        let m = detail();
        let m = update(m, DetailMessage::StartEdit);
        let m = update(m, DetailMessage::EditChar('9'));
        let m = update(m, DetailMessage::EditChar('9'));
        let m = update(m, DetailMessage::CancelEdit);
        assert!(!m.editing);
        assert_eq!(m.params[0].value, "80", "original value preserved");
        assert_eq!(m.edit_buffer, "");
    }

    #[test]
    fn start_edit_on_empty_params_is_safe() {
        let m = DetailModel::from_test_data("s", "n", "d", vec![]);
        let m = update(m, DetailMessage::StartEdit);
        assert!(!m.editing, "should not enter edit mode with no params");
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
    fn confirm_reflects_edited_values() {
        let m = detail();
        let m = update(m, DetailMessage::StartEdit);
        // Clear the buffer and type "60"
        let mut m = update(m, DetailMessage::EditBackspace);
        m = update(m, DetailMessage::EditBackspace);
        m = update(m, DetailMessage::EditChar('6'));
        m = update(m, DetailMessage::EditChar('0'));
        let m = update(m, DetailMessage::CommitEdit);
        let result = m.confirm();
        assert_eq!(
            result.overrides.get("compress-image.quality"),
            Some(&"60".to_string())
        );
    }

    // --- Edit char/backspace ignored when not editing ---

    #[test]
    fn edit_char_ignored_when_not_editing() {
        let m = detail();
        let m = update(m, DetailMessage::EditChar('x'));
        assert_eq!(m.edit_buffer, "");
        assert!(!m.editing);
    }

    #[test]
    fn edit_backspace_ignored_when_not_editing() {
        let m = detail();
        let m = update(m, DetailMessage::EditBackspace);
        assert_eq!(m.edit_buffer, "");
    }

    #[test]
    fn commit_ignored_when_not_editing() {
        let m = detail();
        let m = update(m, DetailMessage::CommitEdit);
        assert!(!m.editing);
        assert_eq!(m.params[0].value, "80", "value unchanged");
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
        // The compress recipe has a quality param.
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
        // No param should come from an "input" or "output" node.
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
        // Label comes from engine metadata, not empty.
        assert!(
            !quality.label.is_empty(),
            "label should come from engine metadata"
        );
    }

    #[test]
    fn from_slug_multi_node_recipe_extracts_all_processor_params() {
        let registry = bnto_engine::create_registry();
        // resize-images has width/height/mode params.
        let m =
            DetailModel::from_slug("resize-images", &registry).expect("resize-images should exist");
        assert!(
            m.params.len() >= 2,
            "resize recipe should have multiple params, got {}",
            m.params.len()
        );
    }
}
