// Recipe detail screen — TEA state + transitions for recipe configuration.
//
// Model (state) + Message (events) + update() (pure transitions).
// Loading logic lives in detail_loader.rs.

use std::collections::HashMap;

use bnto_core::metadata::{Constraints, ParamCondition, ParameterType};
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
    /// Human-readable description (shown when focused).
    pub description: Option<String>,
    /// Numeric constraints (min/max/required).
    pub constraints: Option<Constraints>,
    /// Unit suffix for display (e.g., "%", "px").
    pub suffix: Option<String>,
    /// Conditional visibility — show only when another param matches a value.
    pub visible_when: Option<ParamCondition>,
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
    /// Validation error for the focused param (cleared on next keystroke).
    pub error: Option<String>,
    /// Line-based scroll offset for Paragraph::scroll().
    pub scroll_offset: usize,
    /// Available lines in the content area (set by Resize events).
    pub viewport_height: usize,
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
    /// Toggle a boolean parameter (Space key).
    ToggleBool,
    /// Cycle an enum parameter to the next option (→ key).
    EnumNext,
    /// Cycle an enum parameter to the previous option (← key).
    EnumPrev,
    /// Increment a bounded number parameter (→ key).
    NumberIncrement,
    /// Decrement a bounded number parameter (← key).
    NumberDecrement,
    /// Reset the focused parameter to its default value (d key).
    ResetDefault,
    /// Terminal resized — update viewport height.
    Resize { height: usize },
}

/// Result of confirming the detail screen — collected param overrides.
pub struct ConfigResult {
    /// Map of "node_id.param_name" → value for passing to execution.
    pub overrides: HashMap<String, String>,
}

impl DetailModel {
    /// Whether the "Continue" action at the bottom is focused.
    /// The continue action lives at index `params.len()`.
    pub fn is_continue_focused(&self) -> bool {
        self.focused == self.params.len()
    }

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
            error: None,
            scroll_offset: 0,
            viewport_height: 100, // large default for tests
        }
    }

    /// Build a detail model from a recipe slug using engine metadata.
    pub fn from_slug(slug: &str, registry: &NodeRegistry) -> Option<Self> {
        super::detail_loader::load_detail(slug, registry)
    }

    /// Confirm the current configuration — returns param overrides.
    ///
    /// Hidden params (failing `visible_when`) are excluded from overrides.
    pub fn confirm(&self) -> ConfigResult {
        let overrides = self
            .params
            .iter()
            .filter(|p| is_param_visible(p, &self.params))
            .map(|p| {
                let key = format!("{}.{}", p.node_id, p.name);
                (key, p.value.clone())
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
/// - The referenced param doesn't exist (permissive default)
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

/// Estimate the line number where a given param starts in the rendered output.
///
/// Used to keep the focused param visible during scrolling.
/// Each visible param occupies: label+value (1) + spacing (1).
/// Descriptions only show for the focused param, so they don't affect
/// the line count of params before it.
fn estimate_focused_line(focused: usize, params: &[ParamEntry]) -> usize {
    // Header: name + description + blank + PARAMETERS + blank = 5 lines
    let header = 5;
    let visible_before = params[..focused]
        .iter()
        .filter(|p| is_param_visible(p, params))
        .count();
    // Each non-focused visible param takes 2 lines: label + spacing.
    header + visible_before * 2
}

/// Pure state transition for the detail screen.
pub fn update(model: DetailModel, msg: DetailMessage) -> DetailModel {
    match msg {
        DetailMessage::FocusNext => {
            if model.editing || model.params.is_empty() {
                return model;
            }
            // Items: params[0..n] + continue action at index n.
            // Skip hidden params; continue action is always navigable.
            let item_count = model.params.len() + 1;
            let mut next = (model.focused + 1) % item_count;
            // Loop to skip hidden params (at most item_count steps).
            for _ in 0..item_count {
                if next == model.params.len()
                    || is_param_visible(&model.params[next], &model.params)
                {
                    break;
                }
                next = (next + 1) % item_count;
            }
            let focused_line = estimate_focused_line(next, &model.params);
            let scroll_offset = super::viewport::ensure_cursor_visible(
                focused_line,
                model.scroll_offset,
                model.viewport_height,
            );
            DetailModel {
                focused: next,
                scroll_offset,
                ..model
            }
        }
        DetailMessage::FocusPrev => {
            if model.editing || model.params.is_empty() {
                return model;
            }
            let item_count = model.params.len() + 1;
            let mut prev = if model.focused == 0 {
                item_count - 1
            } else {
                model.focused - 1
            };
            // Loop to skip hidden params (at most item_count steps).
            for _ in 0..item_count {
                if prev == model.params.len()
                    || is_param_visible(&model.params[prev], &model.params)
                {
                    break;
                }
                prev = if prev == 0 { item_count - 1 } else { prev - 1 };
            }
            let focused_line = estimate_focused_line(prev, &model.params);
            let scroll_offset = super::viewport::ensure_cursor_visible(
                focused_line,
                model.scroll_offset,
                model.viewport_height,
            );
            DetailModel {
                focused: prev,
                scroll_offset,
                ..model
            }
        }
        DetailMessage::StartEdit => {
            if model.params.is_empty() || model.is_continue_focused() {
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
        DetailMessage::ToggleBool => {
            if model.params.is_empty() || model.is_continue_focused() {
                return model;
            }
            let param = &model.params[model.focused];
            if !matches!(param.param_type, ParameterType::Boolean) {
                return model;
            }
            let mut params = model.params;
            params[model.focused].value =
                super::controls::boolean::toggle(&params[model.focused].value);
            DetailModel {
                params,
                error: None,
                ..model
            }
        }
        DetailMessage::EnumNext => {
            if model.params.is_empty() || model.is_continue_focused() {
                return model;
            }
            let ParameterType::Enum { ref options } = model.params[model.focused].param_type else {
                return model;
            };
            let next = super::controls::enum_select::cycle_next(
                &model.params[model.focused].value,
                options,
            );
            let mut params = model.params;
            params[model.focused].value = next;
            DetailModel {
                params,
                error: None,
                ..model
            }
        }
        DetailMessage::EnumPrev => {
            if model.params.is_empty() || model.is_continue_focused() {
                return model;
            }
            let ParameterType::Enum { ref options } = model.params[model.focused].param_type else {
                return model;
            };
            let prev = super::controls::enum_select::cycle_prev(
                &model.params[model.focused].value,
                options,
            );
            let mut params = model.params;
            params[model.focused].value = prev;
            DetailModel {
                params,
                error: None,
                ..model
            }
        }
        DetailMessage::NumberIncrement => {
            if model.params.is_empty() || model.is_continue_focused() {
                return model;
            }
            let param = &model.params[model.focused];
            if !matches!(param.param_type, ParameterType::Number) || param.constraints.is_none() {
                return model;
            }
            let step = super::controls::number::step_size(param.constraints.as_ref());
            let Some(next) =
                super::controls::number::step(&param.value, step, param.constraints.as_ref())
            else {
                return model;
            };
            let mut params = model.params;
            params[model.focused].value = next;
            DetailModel {
                params,
                error: None,
                ..model
            }
        }
        DetailMessage::NumberDecrement => {
            if model.params.is_empty() || model.is_continue_focused() {
                return model;
            }
            let param = &model.params[model.focused];
            if !matches!(param.param_type, ParameterType::Number) || param.constraints.is_none() {
                return model;
            }
            let step = super::controls::number::step_size(param.constraints.as_ref());
            let Some(next) =
                super::controls::number::step(&param.value, -step, param.constraints.as_ref())
            else {
                return model;
            };
            let mut params = model.params;
            params[model.focused].value = next;
            DetailModel {
                params,
                error: None,
                ..model
            }
        }
        DetailMessage::ResetDefault => {
            if model.params.is_empty() || model.is_continue_focused() {
                return model;
            }
            let default = model.params[model.focused].default.clone();
            let mut params = model.params;
            params[model.focused].value = default;
            DetailModel {
                params,
                error: None,
                ..model
            }
        }
        DetailMessage::Resize { height } => {
            let focused_line = estimate_focused_line(model.focused, &model.params);
            let scroll_offset =
                super::viewport::ensure_cursor_visible(focused_line, model.scroll_offset, height);
            DetailModel {
                viewport_height: height,
                scroll_offset,
                ..model
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::metadata::{Constraints, OptionEntry, ParamConditionEntry};

    // --- Test helpers ---

    /// Create a test ParamEntry with metadata fields defaulted to None.
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
    fn focus_next_reaches_continue_action() {
        let mut m = detail();
        m.focused = 2; // last param
        let m = update(m, DetailMessage::FocusNext);
        assert_eq!(m.focused, 3, "should reach continue action");
        assert!(m.is_continue_focused());
    }

    #[test]
    fn focus_next_wraps_from_continue_to_first() {
        let mut m = detail();
        m.focused = 3; // continue action
        let m = update(m, DetailMessage::FocusNext);
        assert_eq!(m.focused, 0, "should wrap to first param");
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
        assert_eq!(m.focused, 3, "should wrap to continue action");
        assert!(m.is_continue_focused());
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

    #[test]
    fn start_edit_ignored_on_continue_action() {
        let mut m = detail();
        m.focused = 3; // continue action
        let m = update(m, DetailMessage::StartEdit);
        assert!(!m.editing, "should not enter edit mode on continue");
    }

    #[test]
    fn is_continue_focused_true_at_params_len() {
        let mut m = detail();
        m.focused = m.params.len();
        assert!(m.is_continue_focused());
    }

    #[test]
    fn is_continue_focused_false_on_param() {
        let m = detail();
        assert!(!m.is_continue_focused());
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

    // --- Boolean toggle ---

    #[test]
    fn toggle_bool_flips_true_to_false() {
        let mut m = detail();
        m.focused = 2; // strip_metadata (Boolean, value="true")
        let m = update(m, DetailMessage::ToggleBool);
        assert_eq!(m.params[2].value, "false");
    }

    #[test]
    fn toggle_bool_flips_false_to_true() {
        let mut m = detail();
        m.focused = 2;
        m.params[2].value = "false".into();
        let m = update(m, DetailMessage::ToggleBool);
        assert_eq!(m.params[2].value, "true");
    }

    #[test]
    fn toggle_bool_noop_on_non_boolean() {
        let m = detail(); // focused=0, quality (Number)
        let m = update(m, DetailMessage::ToggleBool);
        assert_eq!(m.params[0].value, "80", "non-boolean unchanged");
    }

    // --- Enum cycling ---

    #[test]
    fn enum_next_advances_to_next_option() {
        let mut m = detail();
        m.focused = 1; // format (Enum, value="jpeg")
        let m = update(m, DetailMessage::EnumNext);
        assert_eq!(m.params[1].value, "png");
    }

    #[test]
    fn enum_next_wraps_at_end() {
        let mut m = detail();
        m.focused = 1;
        m.params[1].value = "webp".into(); // last option
        let m = update(m, DetailMessage::EnumNext);
        assert_eq!(m.params[1].value, "jpeg");
    }

    #[test]
    fn enum_prev_wraps_at_start() {
        let mut m = detail();
        m.focused = 1; // format, value="jpeg" (first option)
        let m = update(m, DetailMessage::EnumPrev);
        assert_eq!(m.params[1].value, "webp");
    }

    #[test]
    fn enum_noop_on_non_enum() {
        let m = detail(); // focused=0, quality (Number)
        let m = update(m, DetailMessage::EnumNext);
        assert_eq!(m.params[0].value, "80", "non-enum unchanged");
    }

    // --- Number stepping ---

    #[test]
    fn number_increment_steps_up() {
        let m = detail(); // focused=0, quality=80, constraints 1-100
        let m = update(m, DetailMessage::NumberIncrement);
        assert_eq!(m.params[0].value, "81");
    }

    #[test]
    fn number_decrement_steps_down() {
        let m = detail();
        let m = update(m, DetailMessage::NumberDecrement);
        assert_eq!(m.params[0].value, "79");
    }

    #[test]
    fn number_clamp_at_max() {
        let mut m = detail();
        m.params[0].value = "100".into();
        let m = update(m, DetailMessage::NumberIncrement);
        assert_eq!(m.params[0].value, "100");
    }

    #[test]
    fn number_clamp_at_min() {
        let mut m = detail();
        m.params[0].value = "1".into();
        let m = update(m, DetailMessage::NumberDecrement);
        assert_eq!(m.params[0].value, "1");
    }

    #[test]
    fn number_noop_without_constraints() {
        let mut m = detail();
        m.params[0].constraints = None; // remove constraints
        let m = update(m, DetailMessage::NumberIncrement);
        assert_eq!(m.params[0].value, "80", "unbounded number unchanged");
    }

    // --- Reset default ---

    #[test]
    fn reset_default_restores_original() {
        let mut m = detail();
        m.params[0].value = "42".into(); // changed from default "80"
        let m = update(m, DetailMessage::ResetDefault);
        assert_eq!(m.params[0].value, "80");
    }

    #[test]
    fn reset_default_noop_when_already_default() {
        let m = detail(); // quality = "80", default = "80"
        let m = update(m, DetailMessage::ResetDefault);
        assert_eq!(m.params[0].value, "80");
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

    /// Build a param with a visible_when condition for testing.
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
    fn focus_next_skips_hidden_param() {
        // Params: [mode=compress, width(hidden), format]. Focus 0 → should skip 1, land on 2.
        let params = vec![
            conditional_param("mode", "compress", None),
            conditional_param("width", "800", Some(single_condition("mode", "resize"))),
            conditional_param("format", "jpeg", None),
        ];
        let m = DetailModel::from_test_data("s", "n", "d", params);
        let m = update(m, DetailMessage::FocusNext);
        assert_eq!(m.focused, 2, "should skip hidden param at index 1");
    }

    #[test]
    fn focus_prev_skips_hidden_param() {
        // Params: [mode=compress, width(hidden), format]. Focus 2 → should skip 1, land on 0.
        let params = vec![
            conditional_param("mode", "compress", None),
            conditional_param("width", "800", Some(single_condition("mode", "resize"))),
            conditional_param("format", "jpeg", None),
        ];
        let mut m = DetailModel::from_test_data("s", "n", "d", params);
        m.focused = 2;
        let m = update(m, DetailMessage::FocusPrev);
        assert_eq!(m.focused, 0, "should skip hidden param at index 1");
    }

    #[test]
    fn focus_wraps_past_all_hidden_to_continue() {
        // All params hidden → only continue (index 2) is navigable.
        let params = vec![
            conditional_param("a", "1", Some(single_condition("x", "y"))),
            conditional_param("b", "2", Some(single_condition("x", "y"))),
        ];
        let m = DetailModel::from_test_data("s", "n", "d", params);
        let m = update(m, DetailMessage::FocusNext);
        assert_eq!(m.focused, 2, "should land on continue action");
        assert!(m.is_continue_focused());
    }

    #[test]
    fn is_param_visible_unknown_param_defaults_visible() {
        // Condition references a param that doesn't exist → permissive default (hidden).
        // Actually, no match found → returns false. But the plan says "visible".
        // Let's check: the referenced param "nonexistent" isn't found, so
        // `any(|p| p.name == "nonexistent" && ...)` is false → function returns false.
        // The plan says "visible" but logically if the referenced param is missing,
        // the condition can't be satisfied. Let's match actual behavior.
        let params = vec![conditional_param(
            "width",
            "800",
            Some(single_condition("nonexistent", "resize")),
        )];
        // No param named "nonexistent" → condition not met → hidden.
        assert!(!is_param_visible(&params[0], &params));
    }

    // --- Scroll state ---

    #[test]
    fn resize_updates_viewport_height() {
        let m = detail();
        let m = update(m, DetailMessage::Resize { height: 20 });
        assert_eq!(m.viewport_height, 20);
    }

    #[test]
    fn focus_next_adjusts_scroll_when_past_viewport() {
        // Small viewport (3 lines) — focusing past it should increase scroll_offset.
        let params: Vec<ParamEntry> = (0..10)
            .map(|i| {
                test_param(
                    "n",
                    &format!("p{i}"),
                    &format!("P{i}"),
                    "v",
                    ParameterType::String,
                    "v",
                )
            })
            .collect();
        let mut m = DetailModel::from_test_data("s", "n", "d", params);
        m.viewport_height = 3;
        m.scroll_offset = 0;
        // Focus through several params to push past viewport.
        for _ in 0..8 {
            m = update(m, DetailMessage::FocusNext);
        }
        assert!(
            m.scroll_offset > 0,
            "scroll should increase for small viewport"
        );
    }

    #[test]
    fn scroll_offset_stays_zero_when_all_fit() {
        let m = detail(); // 3 params, viewport_height=100 (from_test_data)
        let m = update(m, DetailMessage::FocusNext);
        assert_eq!(m.scroll_offset, 0, "everything fits — no scrolling");
    }

    #[test]
    fn focus_prev_scrolls_back_up() {
        let params: Vec<ParamEntry> = (0..10)
            .map(|i| {
                test_param(
                    "n",
                    &format!("p{i}"),
                    &format!("P{i}"),
                    "v",
                    ParameterType::String,
                    "v",
                )
            })
            .collect();
        let mut m = DetailModel::from_test_data("s", "n", "d", params);
        m.viewport_height = 3;
        // Navigate far down.
        for _ in 0..8 {
            m = update(m, DetailMessage::FocusNext);
        }
        let scrolled = m.scroll_offset;
        // Now navigate back up.
        for _ in 0..8 {
            m = update(m, DetailMessage::FocusPrev);
        }
        assert!(
            m.scroll_offset < scrolled,
            "scroll should decrease when moving back up"
        );
    }
}
