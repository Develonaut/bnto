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

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::metadata::ParameterType;

    fn registry() -> NodeRegistry {
        bnto_engine::create_registry()
    }

    // --- compress-images: Number control ---

    #[test]
    fn compress_images_has_quality_param() {
        let detail = load_detail("compress-images", &registry()).unwrap();
        let quality = detail.params.iter().find(|p| p.name == "quality").unwrap();
        assert!(matches!(quality.param_type, ParameterType::Number));
        assert_eq!(quality.value, "80");
        assert_eq!(quality.suffix.as_deref(), Some("%"));
    }

    #[test]
    fn compress_images_quality_has_bounds() {
        let detail = load_detail("compress-images", &registry()).unwrap();
        let quality = detail.params.iter().find(|p| p.name == "quality").unwrap();
        let constraints = quality
            .constraints
            .as_ref()
            .expect("quality should have constraints");
        assert_eq!(constraints.min, Some(1.0));
        assert_eq!(constraints.max, Some(100.0));
    }

    // --- convert-image-format: Enum + Number controls ---

    #[test]
    fn convert_format_has_enum_param() {
        let detail = load_detail("convert-image-format", &registry()).unwrap();
        let format = detail.params.iter().find(|p| p.name == "format").unwrap();
        let ParameterType::Enum { options } = &format.param_type else {
            panic!("format should be Enum, got {:?}", format.param_type);
        };
        let values: Vec<&str> = options.iter().map(|o| o.value.as_str()).collect();
        assert!(values.contains(&"jpeg"));
        assert!(values.contains(&"png"));
        assert!(values.contains(&"webp"));
    }

    #[test]
    fn convert_format_enum_has_labels() {
        let detail = load_detail("convert-image-format", &registry()).unwrap();
        let format = detail.params.iter().find(|p| p.name == "format").unwrap();
        let ParameterType::Enum { options } = &format.param_type else {
            panic!("expected Enum");
        };
        for opt in options {
            assert!(
                !opt.label.is_empty(),
                "option '{}' should have a label",
                opt.value
            );
        }
    }

    // --- resize-images: Boolean + Number controls ---

    #[test]
    fn resize_images_has_boolean_param() {
        let detail = load_detail("resize-images", &registry()).unwrap();
        let aspect = detail
            .params
            .iter()
            .find(|p| p.name == "maintainAspect")
            .unwrap();
        assert!(matches!(aspect.param_type, ParameterType::Boolean));
        assert_eq!(aspect.value, "true");
    }

    #[test]
    fn resize_images_has_dimension_params() {
        let detail = load_detail("resize-images", &registry()).unwrap();
        let width = detail.params.iter().find(|p| p.name == "width").unwrap();
        assert!(matches!(width.param_type, ParameterType::Number));
        assert_eq!(width.suffix.as_deref(), Some("px"));
    }

    // --- clean-csv: Multiple Boolean controls ---

    #[test]
    fn clean_csv_has_boolean_controls() {
        let detail = load_detail("clean-csv", &registry()).unwrap();
        let bools: Vec<&ParamEntry> = detail
            .params
            .iter()
            .filter(|p| matches!(p.param_type, ParameterType::Boolean))
            .collect();
        assert!(
            bools.len() >= 3,
            "clean-csv should have at least 3 boolean params, got {}",
            bools.len()
        );
    }

    // --- rename-files: String + Enum controls ---

    #[test]
    fn rename_files_has_string_params() {
        let detail = load_detail("rename-files", &registry()).unwrap();
        let find = detail.params.iter().find(|p| p.name == "find").unwrap();
        assert!(matches!(find.param_type, ParameterType::String));
    }

    #[test]
    fn rename_files_has_case_enum() {
        let detail = load_detail("rename-files", &registry()).unwrap();
        let case = detail.params.iter().find(|p| p.name == "case").unwrap();
        let ParameterType::Enum { options } = &case.param_type else {
            panic!("case should be Enum, got {:?}", case.param_type);
        };
        let values: Vec<&str> = options.iter().map(|o| o.value.as_str()).collect();
        assert!(values.contains(&"lower"));
        assert!(values.contains(&"upper"));
    }

    // --- Description metadata ---

    #[test]
    fn params_carry_descriptions() {
        let detail = load_detail("compress-images", &registry()).unwrap();
        let quality = detail.params.iter().find(|p| p.name == "quality").unwrap();
        assert!(
            quality.description.is_some(),
            "quality should have a description"
        );
    }

    // --- All built-in recipes load without panic ---

    #[test]
    fn all_builtin_recipes_load() {
        let registry = registry();
        let recipes = bnto_engine::recipes::builtin_recipes();
        for recipe in &recipes {
            let result = load_detail(&recipe.slug, &registry);
            assert!(
                result.is_some(),
                "Failed to load detail for recipe '{}'",
                recipe.slug
            );
        }
    }

    #[test]
    fn all_loaded_params_have_labels() {
        let registry = registry();
        let recipes = bnto_engine::recipes::builtin_recipes();
        for recipe in &recipes {
            let detail = load_detail(&recipe.slug, &registry).unwrap();
            for param in &detail.params {
                assert!(
                    !param.label.is_empty(),
                    "param '{}' in recipe '{}' has empty label",
                    param.name,
                    recipe.slug
                );
            }
        }
    }
}
