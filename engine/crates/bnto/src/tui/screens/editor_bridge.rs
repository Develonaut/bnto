// Bridge between EditorNode params and bnto-form fields.
//
// Converts an EditorNode's params into ParamEntry + Field lists using
// engine registry metadata. Reuses param_to_field and visibility logic
// from detail_bridge. Also syncs committed form values back to EditorModel.

use bnto_core::editor::EditorNode;
use bnto_core::registry::NodeRegistry;
use bnto_form::{FormMessage, FormModel};

use super::detail::ParamEntry;
use super::detail_bridge;

/// Active form state for an expanded editor node.
#[derive(Debug, Clone)]
pub struct ActiveNodeForm {
    /// Index of the node in EditorModel.nodes.
    pub node_index: usize,
    /// Parameter metadata for visible_when evaluation.
    pub params: Vec<ParamEntry>,
    /// The live form model.
    pub form: FormModel,
}

/// Build an ActiveNodeForm for the given editor node using registry metadata.
///
/// Returns None if the node has no surfaceable parameters (e.g. I/O nodes).
pub fn build_form(
    node: &EditorNode,
    node_index: usize,
    registry: &NodeRegistry,
) -> Option<ActiveNodeForm> {
    let params = node_to_params(node, registry);
    if params.is_empty() {
        return None;
    }
    let fields = detail_bridge::params_to_fields(&params);
    let form = FormModel::new(fields);
    Some(ActiveNodeForm {
        node_index,
        params,
        form,
    })
}

/// Convert an EditorNode's params to ParamEntry list using registry metadata.
fn node_to_params(node: &EditorNode, registry: &NodeRegistry) -> Vec<ParamEntry> {
    let empty_map = serde_json::Map::new();
    let param_map: serde_json::Map<String, serde_json::Value> = node
        .params
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let Some(processor) = registry.resolve(&node.node_type, &empty_map) else {
        return Vec::new();
    };

    let meta = processor.metadata();
    meta.parameters
        .iter()
        .filter(|p| p.surfaceable)
        .map(|p| {
            let current_value = param_map
                .get(&p.name)
                .map(value_to_display_string)
                .unwrap_or_else(|| {
                    p.default
                        .as_ref()
                        .map(value_to_display_string)
                        .unwrap_or_default()
                });

            let default_str = p
                .default
                .as_ref()
                .map(value_to_display_string)
                .unwrap_or_default();

            ParamEntry {
                node_id: node.id.clone(),
                name: p.name.clone(),
                label: p.label.clone(),
                value: current_value,
                param_type: p.param_type.clone(),
                default: default_str,
                description: if p.description.is_empty() {
                    None
                } else {
                    Some(p.description.clone())
                },
                constraints: p.constraints.clone(),
                suffix: p.suffix.clone(),
                visible_when: p.visible_when.clone(),
            }
        })
        .collect()
}

/// Apply a form message to the active form, updating visibility afterwards.
pub fn apply_form_message(active: &mut ActiveNodeForm, msg: FormMessage) {
    active.form = bnto_form::update(active.form.clone(), msg);
    detail_bridge::update_visibility(&mut active.form, &active.params);
}

/// Sync committed form values back to EditorModel params.
///
/// Called after form edits to propagate values into the EditorNode.
/// Returns a list of (param_name, json_value) pairs that changed.
pub fn collect_param_updates(active: &ActiveNodeForm) -> Vec<(String, serde_json::Value)> {
    active
        .params
        .iter()
        .enumerate()
        .filter_map(|(i, param)| {
            let field = active.form.fields.get(i)?;
            let json_value = display_string_to_value(&field.value, &param.param_type);
            Some((param.name.clone(), json_value))
        })
        .collect()
}

/// Whether the active form is in an editing state (traps key events).
pub fn is_editing(active: &ActiveNodeForm) -> bool {
    detail_bridge::is_form_editing(&active.form)
}

/// Convert a display string back to a JSON value based on param type.
fn display_string_to_value(
    s: &str,
    param_type: &bnto_core::metadata::ParameterType,
) -> serde_json::Value {
    match param_type {
        bnto_core::metadata::ParameterType::Boolean => {
            serde_json::Value::Bool(s == "true" || s == "Yes")
        }
        bnto_core::metadata::ParameterType::Number => s
            .parse::<f64>()
            .map(|n| serde_json::json!(n))
            .unwrap_or_else(|_| serde_json::Value::String(s.to_string())),
        _ => serde_json::Value::String(s.to_string()),
    }
}

/// Convert a JSON value to a display string.
fn value_to_display_string(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::editor::EditorModel;
    use bnto_core::metadata::NodeTypeInfo;
    use bnto_engine::create_registry;
    use std::collections::HashMap;

    fn registry() -> NodeRegistry {
        create_registry()
    }

    fn image_compress_node() -> EditorNode {
        EditorNode {
            id: "compress-1".into(),
            node_type: "image-compress".into(),
            label: "Compress Image".into(),
            params: {
                let mut m = HashMap::new();
                m.insert("quality".into(), serde_json::json!(80));
                m
            },
            expanded: true,
        }
    }

    fn io_node() -> EditorNode {
        EditorNode {
            id: "input-1".into(),
            node_type: "input".into(),
            label: "Input".into(),
            params: HashMap::new(),
            expanded: true,
        }
    }

    // --- build_form ---

    #[test]
    fn build_form_creates_fields_for_processor_node() {
        let node = image_compress_node();
        let active = build_form(&node, 0, &registry()).expect("should create form");
        assert!(!active.params.is_empty(), "should have params");
        assert_eq!(active.form.fields.len(), active.params.len());
        assert_eq!(active.node_index, 0);
    }

    #[test]
    fn build_form_returns_none_for_io_node() {
        let node = io_node();
        assert!(
            build_form(&node, 0, &registry()).is_none(),
            "I/O nodes have no surfaceable params"
        );
    }

    #[test]
    fn build_form_returns_none_for_unknown_type() {
        let node = EditorNode {
            id: "x-1".into(),
            node_type: "nonexistent-type".into(),
            label: "Unknown".into(),
            params: HashMap::new(),
            expanded: true,
        };
        assert!(build_form(&node, 0, &registry()).is_none());
    }

    #[test]
    fn build_form_populates_current_values() {
        let node = image_compress_node();
        let active = build_form(&node, 0, &registry()).unwrap();
        let quality_field = active.form.fields.iter().find(|f| f.id == "quality");
        assert!(quality_field.is_some(), "should have quality field");
        assert_eq!(quality_field.unwrap().value, "80");
    }

    // --- collect_param_updates ---

    #[test]
    fn collect_param_updates_returns_current_values() {
        let node = image_compress_node();
        let active = build_form(&node, 0, &registry()).unwrap();
        let updates = collect_param_updates(&active);
        let quality = updates.iter().find(|(k, _)| k == "quality");
        assert!(quality.is_some());
    }

    // --- apply_form_message ---

    #[test]
    fn apply_form_message_updates_form_state() {
        let node = image_compress_node();
        let mut active = build_form(&node, 0, &registry()).unwrap();
        // Focus to quality field and start editing.
        let quality_idx = active.form.fields.iter().position(|f| f.id == "quality");
        assert!(quality_idx.is_some());
        // Navigate to quality field (it might already be focused).
        apply_form_message(&mut active, FormMessage::StartEdit);
        assert!(
            is_editing(&active),
            "form should be in editing state after StartEdit"
        );
    }

    // --- display_string_to_value ---

    #[test]
    fn bool_conversion() {
        let t = display_string_to_value("true", &bnto_core::metadata::ParameterType::Boolean);
        assert_eq!(t, serde_json::Value::Bool(true));
        let f = display_string_to_value("false", &bnto_core::metadata::ParameterType::Boolean);
        assert_eq!(f, serde_json::Value::Bool(false));
    }

    #[test]
    fn number_conversion() {
        let n = display_string_to_value("42", &bnto_core::metadata::ParameterType::Number);
        assert_eq!(n, serde_json::json!(42.0));
    }

    #[test]
    fn string_conversion() {
        let s = display_string_to_value("hello", &bnto_core::metadata::ParameterType::String);
        assert_eq!(s, serde_json::Value::String("hello".into()));
    }

    // --- Integration: form edit → sync → EditorModel ---

    #[test]
    fn form_edit_produces_undo_snapshot_via_update_param() {
        let reg = registry();
        let mut editor = EditorModel::new();
        let info = NodeTypeInfo {
            name: "image-compress".into(),
            label: "Compress Image".into(),
            description: String::new(),
            category: bnto_core::metadata::NodeCategory::Image,
            is_container: false,
            platforms: vec!["browser".into()],
            icon: "image".into(),
        };
        editor.add_node_with_defaults(
            "image-compress",
            &info,
            &[("quality".into(), serde_json::json!(80))],
        );
        editor.undo_stack.clear();
        editor.dirty = false;

        let node = &editor.nodes[0];
        let active = build_form(node, 0, &reg).unwrap();

        // Simulate sync: update_param triggers undo snapshot.
        let updates = collect_param_updates(&active);
        for (key, value) in updates {
            editor.update_param(0, &key, value);
        }
        assert!(!editor.undo_stack.is_empty(), "should have undo snapshot");
        assert!(editor.dirty, "should be dirty after param update");
    }
}
