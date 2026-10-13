// Wizard configuration — the param form for the chosen operation and the
// EditorModel the finished wizard hands to the editor.

use bnto_core::editor::{EditorModel, EditorSource};
use bnto_core::metadata::{ParameterDef, ParameterType};
use bnto_core::registry::NodeRegistry;
use tonkotsu::FormModel;

use super::super::detail::ParamEntry;
use super::super::detail_bridge;
use super::WizardModel;

/// Build the config form for a specific operation using registry metadata.
pub(super) fn build_config_form(
    node_type: &str,
    registry: &NodeRegistry,
) -> (Vec<ParamEntry>, FormModel) {
    let empty_map = serde_json::Map::new();
    let Some(processor) = registry.resolve(node_type, &empty_map) else {
        return (Vec::new(), FormModel::new(Vec::new()));
    };

    let meta = processor.metadata();
    let params: Vec<ParamEntry> = meta
        .parameters
        .iter()
        .filter(|p| p.surfaceable)
        .map(param_entry)
        .collect();

    let fields = detail_bridge::params_to_fields(&params);
    let form = FormModel::new(fields);
    (params, form)
}

/// A form row for one parameter, pre-filled with its default.
fn param_entry(p: &ParameterDef) -> ParamEntry {
    let default_str = p.default.as_ref().map(value_to_display).unwrap_or_default();
    ParamEntry {
        node_id: String::new(),
        name: p.name.clone(),
        label: p.label.clone(),
        value: default_str.clone(),
        param_type: p.param_type.clone(),
        default: default_str,
        description: if p.description.is_empty() {
            None
        } else {
            Some(p.description.clone())
        },
        constraints: p.constraints.clone(),
        suffix: p.suffix.clone(),
        control: p.control.clone(),
        visible_when: p.visible_when.clone(),
    }
}

/// Generate a recipe name from the selected operation.
pub(super) fn generate_name(model: &WizardModel) -> String {
    model
        .operations
        .iter()
        .find(|op| Some(&op.name) == model.selected_operation.as_ref())
        .map(|op| op.label.clone())
        .unwrap_or_else(|| "New Recipe".to_string())
}

/// Build an EditorModel from wizard selections.
pub(super) fn build_editor_model(model: &WizardModel, _registry: &NodeRegistry) -> EditorModel {
    let mut editor = EditorModel::new();
    editor.recipe_name = model.recipe_name.clone();
    editor.source = EditorSource::New;

    if let Some(ref op_name) = model.selected_operation {
        let types = bnto_core::metadata::all_node_types();
        if let Some(info) = types.iter().find(|t| &t.name == op_name) {
            editor.add_node_with_defaults(op_name, info, &form_values(model));
            // The add_node_with_defaults call sets dirty + pushes undo.
            // For a freshly created recipe, clear both.
            editor.dirty = false;
            editor.undo_stack.clear();
        }
    }

    editor
}

/// Param values from the form (empty if the config step was skipped).
fn form_values(model: &WizardModel) -> Vec<(String, serde_json::Value)> {
    let Some(ref form) = model.form else {
        return Vec::new();
    };
    model
        .params
        .iter()
        .enumerate()
        .filter_map(|(i, param)| {
            let field = form.fields.get(i)?;
            let val = display_to_value(&field.value, &param.param_type);
            Some((param.name.clone(), val))
        })
        .collect()
}

/// Convert a JSON value to display string (for form default values).
fn value_to_display(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Convert a display string back to JSON based on param type.
fn display_to_value(s: &str, param_type: &ParameterType) -> serde_json::Value {
    match param_type {
        ParameterType::Boolean => serde_json::Value::Bool(s == "true" || s == "Yes"),
        ParameterType::Number => s
            .parse::<f64>()
            .map(|n| serde_json::json!(n))
            .unwrap_or_else(|_| serde_json::Value::String(s.to_string())),
        _ => serde_json::Value::String(s.to_string()),
    }
}
