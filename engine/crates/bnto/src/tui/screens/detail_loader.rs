// Recipe detail loading — resolve engine metadata into editable param entries.
//
// Extracted from detail.rs to keep files under 250 lines.
// Walks recipe definition JSON, resolves processors, collects surfaceable params.

use bnto_core::metadata::ParameterType;
use bnto_core::pipeline::PipelineNode;
use bnto_core::registry::NodeRegistry;
use bnto_core::{InputMode, PipelineDefinition};
use bnto_engine::recipes::builtin_recipe_by_slug;

use super::detail::{DetailFocus, DetailModel, ParamEntry};
use super::detail_fields::fields_to_params;
use super::picker::PickerModel;

/// Build a detail model from a recipe slug using engine metadata.
///
/// Looks up the recipe, parses its definition JSON, walks the node list
/// to find processor nodes, then resolves each via the registry to get
/// parameter metadata. Skips input/output nodes (not user-configurable).
///
/// For file-mode recipes, creates an embedded `PickerModel` starting at
/// the given `start_dir`. For URL/text-mode, no picker is created.
pub fn load_detail(slug: &str, registry: &NodeRegistry) -> Option<DetailModel> {
    load_detail_with_dir(slug, registry, None)
}

/// Build a detail model with an explicit start directory for the embedded picker.
pub fn load_detail_with_dir(
    slug: &str,
    registry: &NodeRegistry,
    start_dir: Option<&std::path::Path>,
) -> Option<DetailModel> {
    let recipe = builtin_recipe_by_slug(slug)?;
    build_detail_from_json(
        slug,
        &recipe.name,
        &recipe.description,
        recipe.definition_json,
        registry,
        start_dir,
    )
}

/// Build a detail model from a library recipe file on disk.
///
/// Used for user-saved recipes that may not exist as builtins (e.g. removed
/// recipes, custom creations). Falls back to builtin lookup if the file
/// can't be read.
pub fn load_detail_from_library(
    slug: &str,
    recipes_dir: &std::path::Path,
    registry: &NodeRegistry,
    start_dir: Option<&std::path::Path>,
) -> Option<DetailModel> {
    let path = recipes_dir.join(format!("{slug}.bnto.json"));
    let json_str = std::fs::read_to_string(&path).ok()?;
    let top: serde_json::Value = serde_json::from_str(&json_str).ok()?;
    let name = top["name"].as_str().unwrap_or(slug).to_string();
    let description = top["description"].as_str().unwrap_or("").to_string();
    build_detail_from_json(slug, &name, &description, &json_str, registry, start_dir)
}

/// Shared logic: parse definition JSON and build a DetailModel.
fn build_detail_from_json(
    slug: &str,
    name: &str,
    description: &str,
    definition_json: &str,
    registry: &NodeRegistry,
    start_dir: Option<&std::path::Path>,
) -> Option<DetailModel> {
    let def_json: serde_json::Value = serde_json::from_str(definition_json).ok()?;
    let nodes = def_json["nodes"].as_array()?;

    // Parse typed definition for field declarations and input mode.
    let pipeline_def = serde_json::from_str::<PipelineDefinition>(definition_json).ok();

    // If any node declares fields, use those instead of extracting from processors.
    let field_params = pipeline_def
        .as_ref()
        .map(|def| collect_field_params(&def.nodes))
        .unwrap_or_default();
    let mut params = if !field_params.is_empty() {
        field_params
    } else {
        extract_surfaceable_params(nodes, registry)
    };

    // Surface URL/text input node (prepend if not already present from fields).
    if let Some(input_param) = find_input_param_in_nodes(nodes)
        && !params
            .iter()
            .any(|p| p.name == input_param.name && p.node_id == input_param.node_id)
    {
        params.insert(0, input_param);
    }

    let fields = super::detail_bridge::params_to_fields(&params);
    let form = tonkotsu::FormModel::new(fields);

    let input_mode = pipeline_def
        .as_ref()
        .map(bnto_core::resolve_input_mode)
        .unwrap_or(InputMode::FileUpload);

    let (focus, input_picker) = match input_mode {
        InputMode::FileUpload => {
            let dir = start_dir.map(std::path::PathBuf::from).unwrap_or_else(|| {
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
            });
            let picker = PickerModel::from_slug(slug, &dir, registry);
            (DetailFocus::Input, Some(picker))
        }
        InputMode::Url | InputMode::Text => (DetailFocus::Params, None),
    };

    Some(DetailModel {
        slug: slug.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        definition_json: definition_json.to_string(),
        params,
        form,
        focus,
        input_mode,
        input_picker,
    })
}

/// Collect field params from all nodes that declare fields.
///
/// Walks nodes (including container children) and converts each node's
/// fields into ParamEntries tagged with the owning node_id.
fn collect_field_params(nodes: &[PipelineNode]) -> Vec<ParamEntry> {
    let mut params = Vec::new();
    collect_field_params_recursive(nodes, &mut params);
    params
}

fn collect_field_params_recursive(nodes: &[PipelineNode], params: &mut Vec<ParamEntry>) {
    for node in nodes {
        if !node.fields.is_empty() {
            params.extend(fields_to_params(&node.fields, &node.id));
        }
        if let Some(children) = &node.children {
            collect_field_params_recursive(children, params);
        }
    }
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

        // Surface URL/text input nodes as the first param, then skip.
        if node_type == "input" {
            if let Some(param) = extract_input_param(node, node_id) {
                params.insert(0, param);
            }
            continue;
        }
        if node_type == "output" {
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

/// Find the input param entry across all nodes in the definition.
fn find_input_param_in_nodes(nodes: &[serde_json::Value]) -> Option<ParamEntry> {
    for node in nodes {
        if node["type"].as_str() == Some("input") {
            let node_id = node["id"].as_str().unwrap_or("input");
            return extract_input_param(node, node_id);
        }
    }
    None
}

/// Extract a ParamEntry from a URL/text-mode input node.
///
/// Returns `Some(ParamEntry)` for url and text modes so the detail screen
/// shows a text field for the user to enter a URL or text before running.
/// Returns `None` for file-upload mode (handled by the file picker).
fn extract_input_param(node: &serde_json::Value, node_id: &str) -> Option<ParamEntry> {
    let params = node["parameters"].as_object()?;
    let mode = params.get("mode")?.as_str()?;

    let (name, default_label) = match mode {
        "url" => ("url", "URL"),
        "text" => ("text", "Text"),
        _ => return None,
    };

    let label = params
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or(default_label)
        .to_string();

    let placeholder = params
        .get("placeholder")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    Some(ParamEntry {
        node_id: node_id.to_string(),
        name: name.to_string(),
        label,
        value: String::new(),
        param_type: ParameterType::String,
        default: String::new(),
        description: if placeholder.is_empty() {
            None
        } else {
            Some(placeholder)
        },
        constraints: None,
        suffix: None,
        control: None,
        visible_when: None,
    })
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
            control: param_def.control.clone(),
            visible_when: param_def.visible_when.clone(),
        });
    }
}

/// Build a detail model from raw recipe JSON (for custom .bnto.json files).
///
/// Parses the JSON, extracts name/description from top-level fields,
/// walks the node list to resolve processors and collect params.
/// Returns Err if JSON is invalid or missing a `nodes` array.
#[cfg(test)]
pub fn load_detail_from_json(json: &str, registry: &NodeRegistry) -> Result<DetailModel, String> {
    let def_json: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {e}"))?;
    let nodes = def_json["nodes"]
        .as_array()
        .ok_or_else(|| "Missing or invalid 'nodes' array".to_string())?;
    let name = def_json["name"]
        .as_str()
        .unwrap_or("Custom Recipe")
        .to_string();
    let description = def_json["description"].as_str().unwrap_or("").to_string();

    let pipeline_def = serde_json::from_str::<PipelineDefinition>(json).ok();
    let field_params = pipeline_def
        .as_ref()
        .map(|def| collect_field_params(&def.nodes))
        .unwrap_or_default();
    let params = if !field_params.is_empty() {
        field_params
    } else {
        extract_surfaceable_params(nodes, registry)
    };

    let fields = super::detail_bridge::params_to_fields(&params);
    let form = tonkotsu::FormModel::new(fields);

    Ok(DetailModel {
        slug: "custom".to_string(),
        name,
        description,
        definition_json: json.to_string(),
        params,
        form,
        focus: DetailFocus::Params,
        input_mode: InputMode::FileUpload,
        input_picker: None,
    })
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

    #[test]
    fn compress_images_quality_has_slider_control() {
        let detail = load_detail("compress-images", &registry()).unwrap();
        let quality = detail.params.iter().find(|p| p.name == "quality").unwrap();
        assert_eq!(
            quality.control.as_deref(),
            Some("slider"),
            "quality should have slider control hint"
        );
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

    // --- Custom JSON loading ---

    #[test]
    fn load_from_json_valid_recipe() {
        let json = r#"{
            "name": "Compress Images",
            "description": "Test recipe",
            "nodes": [
                {"id": "input", "type": "input", "parameters": {}},
                {"id": "compress-image", "type": "image-compress", "parameters": {"quality": 80}},
                {"id": "output", "type": "output", "parameters": {}}
            ]
        }"#;
        let result = load_detail_from_json(json, &registry());
        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.name, "Compress Images");
        assert_eq!(model.description, "Test recipe");
        assert_eq!(model.slug, "custom");
        assert!(
            !model.params.is_empty(),
            "should have params from processor"
        );
    }

    #[test]
    fn load_from_json_minimal() {
        let json = r#"{"nodes": [], "name": "Empty"}"#;
        let result = load_detail_from_json(json, &registry());
        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.name, "Empty");
        assert!(model.params.is_empty());
    }

    #[test]
    fn load_from_json_invalid() {
        let result = load_detail_from_json("{bad json", &registry());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid JSON"));
    }

    #[test]
    fn load_from_json_missing_nodes() {
        let result = load_detail_from_json(r#"{"name": "No nodes"}"#, &registry());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("nodes"));
    }

    // --- extract_input_param: URL/text mode surfacing ---

    fn input_node(mode: &str, label: Option<&str>, placeholder: Option<&str>) -> serde_json::Value {
        let mut params = serde_json::Map::new();
        params.insert("mode".into(), serde_json::Value::String(mode.into()));
        if let Some(l) = label {
            params.insert("label".into(), serde_json::Value::String(l.into()));
        }
        if let Some(p) = placeholder {
            params.insert("placeholder".into(), serde_json::Value::String(p.into()));
        }
        serde_json::json!({
            "id": "input",
            "type": "input",
            "parameters": params
        })
    }

    #[test]
    fn extract_input_param_returns_some_for_url_mode() {
        let node = input_node("url", Some("Video URL"), Some("https://youtube.com/..."));
        let param = extract_input_param(&node, "input").unwrap();
        assert_eq!(param.name, "url");
        assert_eq!(param.label, "Video URL");
        assert_eq!(param.node_id, "input");
        assert!(matches!(param.param_type, ParameterType::String));
    }

    #[test]
    fn extract_input_param_returns_some_for_text_mode() {
        let node = input_node("text", Some("Prompt"), None);
        let param = extract_input_param(&node, "input").unwrap();
        assert_eq!(param.name, "text");
        assert_eq!(param.label, "Prompt");
    }

    #[test]
    fn extract_input_param_returns_none_for_file_upload() {
        let node = input_node("file-upload", None, None);
        assert!(extract_input_param(&node, "input").is_none());
    }

    #[test]
    fn extract_input_param_returns_none_for_missing_mode() {
        let node = serde_json::json!({"id": "input", "type": "input", "parameters": {}});
        assert!(extract_input_param(&node, "input").is_none());
    }

    #[test]
    fn extract_input_param_uses_placeholder_as_description() {
        let node = input_node("url", None, Some("https://youtube.com/watch?v=..."));
        let param = extract_input_param(&node, "input").unwrap();
        assert_eq!(
            param.description.as_deref(),
            Some("https://youtube.com/watch?v=...")
        );
    }

    #[test]
    fn extract_input_param_uses_default_label_when_missing() {
        let node = input_node("url", None, None);
        let param = extract_input_param(&node, "input").unwrap();
        assert_eq!(param.label, "URL");
    }

    #[test]
    fn download_video_has_url_as_first_param() {
        let detail = load_detail("download-video", &registry()).unwrap();
        assert!(
            !detail.params.is_empty(),
            "download-video should have at least one param"
        );
        let first = &detail.params[0];
        assert_eq!(first.name, "url");
        assert_eq!(first.node_id, "input");
        assert_eq!(first.label, "Video URL");
    }

    #[test]
    fn compress_images_has_no_url_param() {
        let detail = load_detail("compress-images", &registry()).unwrap();
        assert!(
            detail.params.iter().all(|p| p.name != "url"),
            "file-upload recipes should not have a url param"
        );
    }

    // --- fields_to_params: Integration tests (unit tests in detail_fields.rs) ---

    #[test]
    fn recipe_with_node_fields_shows_field_params() {
        let json = r#"{
            "name": "Test Recipe",
            "nodes": [
                {"id": "input", "type": "input", "parameters": {}},
                {
                    "id": "proc",
                    "type": "shell-command",
                    "parameters": {"command": "echo"},
                    "fields": {
                        "format": {"type":"enum","label":"Format","options":[{"value":"mp4","label":"MP4"}],"default":"mp4"}
                    }
                },
                {"id": "output", "type": "output", "parameters": {}}
            ]
        }"#;
        let model = load_detail_from_json(json, &registry()).unwrap();
        assert_eq!(model.params.len(), 1);
        assert_eq!(model.params[0].name, "format");
        assert_eq!(model.params[0].label, "Format");
    }

    #[test]
    fn recipe_without_fields_shows_processor_params() {
        let json = r#"{
            "name": "Compress Images",
            "nodes": [
                {"id": "input", "type": "input", "parameters": {}},
                {"id": "compress", "type": "image-compress", "parameters": {"quality": 80}},
                {"id": "output", "type": "output", "parameters": {}}
            ]
        }"#;
        let model = load_detail_from_json(json, &registry()).unwrap();
        assert!(
            model.params.iter().any(|p| p.name == "quality"),
            "should fall back to processor params when no fields declared"
        );
    }

    #[test]
    fn download_video_shows_url_and_field_params() {
        let detail = load_detail("download-video", &registry()).unwrap();
        // Should have URL param first, then field params.
        assert!(
            detail.params.len() >= 2,
            "download-video should have URL + field params, got {}",
            detail.params.len()
        );
        assert_eq!(detail.params[0].name, "url");
        assert_eq!(detail.params[0].node_id, "input");
    }

    #[test]
    fn download_video_recipe_json_parses() {
        let recipe = builtin_recipe_by_slug("download-video").unwrap();
        let def: PipelineDefinition =
            serde_json::from_str(recipe.definition_json).expect("should parse");
        assert!(def.nodes.len() >= 2);
    }

    // --- Library recipe loading (from disk) ---

    fn write_library_recipe(dir: &std::path::Path, slug: &str, json: &str) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join(format!("{slug}.bnto.json")), json).unwrap();
    }

    #[test]
    fn load_library_recipe_from_disk() {
        let tmp = tempfile::tempdir().unwrap();
        let json = r#"{
            "name": "My Custom Recipe",
            "description": "A custom library recipe",
            "nodes": [
                {"id": "input", "type": "input", "parameters": {"mode": "file-upload"}},
                {"id": "compress", "type": "image-compress", "parameters": {"quality": 90}},
                {"id": "output", "type": "output", "parameters": {}}
            ]
        }"#;
        write_library_recipe(tmp.path(), "my-custom", json);

        let detail = load_detail_from_library("my-custom", tmp.path(), &registry(), None);
        assert!(detail.is_some(), "should load library recipe from disk");
        let detail = detail.unwrap();
        assert_eq!(detail.slug, "my-custom");
        assert_eq!(detail.name, "My Custom Recipe");
        assert_eq!(detail.description, "A custom library recipe");
        assert!(
            !detail.params.is_empty(),
            "should have params from processor"
        );
    }

    #[test]
    fn load_library_recipe_missing_file_returns_none() {
        let tmp = tempfile::tempdir().unwrap();
        let detail = load_detail_from_library("nonexistent", tmp.path(), &registry(), None);
        assert!(detail.is_none());
    }

    #[test]
    fn load_library_recipe_invalid_json_returns_none() {
        let tmp = tempfile::tempdir().unwrap();
        write_library_recipe(tmp.path(), "broken", "{bad json");
        let detail = load_detail_from_library("broken", tmp.path(), &registry(), None);
        assert!(detail.is_none());
    }

    #[test]
    fn load_library_recipe_with_fields() {
        let tmp = tempfile::tempdir().unwrap();
        let json = r#"{
            "name": "Field Recipe",
            "nodes": [
                {"id": "input", "type": "input", "parameters": {}},
                {
                    "id": "proc",
                    "type": "shell-command",
                    "parameters": {"command": "echo"},
                    "fields": {
                        "format": {"type":"enum","label":"Format","options":[{"value":"mp4","label":"MP4"}],"default":"mp4"}
                    }
                },
                {"id": "output", "type": "output", "parameters": {}}
            ]
        }"#;
        write_library_recipe(tmp.path(), "field-recipe", json);

        let detail = load_detail_from_library("field-recipe", tmp.path(), &registry(), None);
        assert!(detail.is_some());
        let detail = detail.unwrap();
        assert_eq!(detail.params.len(), 1);
        assert_eq!(detail.params[0].name, "format");
    }

    #[test]
    fn load_library_recipe_url_mode_has_no_picker() {
        let tmp = tempfile::tempdir().unwrap();
        let json = r#"{
            "name": "URL Recipe",
            "nodes": [
                {"id": "input", "type": "input", "parameters": {"mode": "url", "label": "URL"}},
                {"id": "proc", "type": "shell-command", "parameters": {"command": "curl"}},
                {"id": "output", "type": "output", "parameters": {}}
            ]
        }"#;
        write_library_recipe(tmp.path(), "url-recipe", json);

        let detail = load_detail_from_library("url-recipe", tmp.path(), &registry(), None);
        assert!(detail.is_some());
        let detail = detail.unwrap();
        assert!(
            detail.input_picker.is_none(),
            "URL mode should not have a picker"
        );
        assert_eq!(detail.params[0].name, "url");
    }

    #[test]
    fn builtin_recipe_saved_to_library_still_loads() {
        let tmp = tempfile::tempdir().unwrap();
        // Simulate adding a builtin recipe to library, then removing it from builtins.
        let recipe = builtin_recipe_by_slug("compress-images").unwrap();
        write_library_recipe(tmp.path(), "compress-images", recipe.definition_json);

        let detail = load_detail_from_library("compress-images", tmp.path(), &registry(), None);
        assert!(detail.is_some());
        let detail = detail.unwrap();
        assert_eq!(detail.slug, "compress-images");
        assert!(
            detail.params.iter().any(|p| p.name == "quality"),
            "should have quality param from processor"
        );
    }
}
