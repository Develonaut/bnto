// Recipe detail loading — resolve engine metadata into editable param entries.
//
// Extracted from detail.rs to keep files under 250 lines.
// Walks recipe definition JSON, resolves processors, collects surfaceable params.

use bnto_core::registry::NodeRegistry;
use bnto_engine::recipes::builtin_recipe_by_slug;

use super::detail::{DetailModel, ParamEntry};

/// Build a detail model from a recipe slug using engine metadata.
///
/// Looks up the recipe, parses its definition JSON, walks the node list
/// to find processor nodes, then resolves each via the registry to get
/// parameter metadata. Skips input/output nodes (not user-configurable).
pub fn load_detail(slug: &str, registry: &NodeRegistry) -> Option<DetailModel> {
    let recipe = builtin_recipe_by_slug(slug)?;
    let def: serde_json::Value = serde_json::from_str(recipe.definition_json).ok()?;
    let nodes = def["nodes"].as_array()?;
    let params = extract_surfaceable_params(nodes, registry);

    Some(DetailModel {
        slug: recipe.slug.clone(),
        name: recipe.name.clone(),
        description: recipe.description.clone(),
        params,
        focused: 0,
        editing: false,
        edit_buffer: String::new(),
        error: None,
    })
}

/// Walk definition nodes, resolve processors, and collect surfaceable params.
fn extract_surfaceable_params(
    nodes: &[serde_json::Value],
    registry: &NodeRegistry,
) -> Vec<ParamEntry> {
    let mut params = Vec::new();
    for node in nodes {
        let node_type = node["type"].as_str().unwrap_or_default();
        let node_id = node["id"].as_str().unwrap_or_default();

        // Skip I/O nodes — only processor nodes have user-configurable params.
        if node_type == "input" || node_type == "output" {
            continue;
        }

        let node_params = node["parameters"].as_object();
        let empty_map = serde_json::Map::new();
        let resolve_params = node_params.unwrap_or(&empty_map);
        let Some(processor) = registry.resolve(node_type, resolve_params) else {
            continue;
        };

        collect_params_from_processor(node_id, &node_params, processor, &mut params);
    }
    params
}

/// Extract surfaceable params from a single processor's metadata.
fn collect_params_from_processor(
    node_id: &str,
    node_params: &Option<&serde_json::Map<String, serde_json::Value>>,
    processor: &dyn bnto_core::NodeProcessor,
    params: &mut Vec<ParamEntry>,
) {
    for param_def in &processor.metadata().parameters {
        if !param_def.surfaceable {
            continue;
        }

        let current_value = node_params
            .and_then(|m| m.get(&param_def.name))
            .map(value_to_display_string)
            .unwrap_or_else(|| {
                param_def
                    .default
                    .as_ref()
                    .map(value_to_display_string)
                    .unwrap_or_default()
            });

        let default_str = param_def
            .default
            .as_ref()
            .map(value_to_display_string)
            .unwrap_or_default();

        params.push(ParamEntry {
            node_id: node_id.to_string(),
            name: param_def.name.clone(),
            label: param_def.label.clone(),
            value: current_value,
            param_type: param_def.param_type.clone(),
            default: default_str,
            description: if param_def.description.is_empty() {
                None
            } else {
                Some(param_def.description.clone())
            },
            constraints: param_def.constraints.clone(),
            suffix: param_def.suffix.clone(),
        });
    }
}

/// Convert a JSON value to a display string for the TUI.
fn value_to_display_string(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    }
}
