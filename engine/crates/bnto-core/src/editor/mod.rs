// Editor state model — pure-data recipe editing state.
//
// Shared by all editor surfaces (TUI List, TUI Wizard, TUI Code, future
// desktop/web). Pure Rust, no TUI dependency. All mutations are tested
// as plain state transitions — no terminal or renderer needed.
//
// Sprint 15 extracts this into a standalone `bnto-editor` crate.

mod convert;
mod io;
mod mutations;
mod types;

pub use types::{EditorError, EditorModel, EditorNode, EditorSnapshot, EditorSource};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FORMAT_VERSION;
    use crate::definition::{Definition, Metadata, Position};
    use crate::metadata::{NodeCategory, NodeTypeInfo};
    use crate::pipeline::{IterationMode, PipelineSettings};
    use serde_json::json;
    use std::path::{Path, PathBuf};

    fn test_info(name: &str, label: &str) -> NodeTypeInfo {
        NodeTypeInfo {
            name: name.to_string(),
            label: label.to_string(),
            description: String::new(),
            category: NodeCategory::Image,
            is_container: false,
            platforms: vec!["browser".to_string()],
            icon: "image".to_string(),
        }
    }

    fn default_parameters() -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::new())
    }

    fn make_test_definition() -> Definition {
        Definition {
            id: "test-recipe".to_string(),
            node_type: "group".to_string(),
            version: FORMAT_VERSION.to_string(),
            parent_id: None,
            name: "Test Recipe".to_string(),
            position: Position { x: 0.0, y: 0.0 },
            metadata: Metadata {
                description: Some("A test recipe".to_string()),
                ..Default::default()
            },
            parameters: default_parameters(),
            fields: None,
            input_ports: vec![],
            output_ports: vec![],
            nodes: Some(vec![Definition {
                id: "compress-1".to_string(),
                node_type: "image-compress".to_string(),
                version: FORMAT_VERSION.to_string(),
                parent_id: None,
                name: "Compress".to_string(),
                position: Position { x: 0.0, y: 0.0 },
                metadata: Metadata::default(),
                parameters: json!({"quality": 80}),
                fields: None,
                input_ports: vec![],
                output_ports: vec![],
                nodes: None,
                edges: None,
                settings: None,
            }]),
            edges: Some(vec![]),
            settings: Some(PipelineSettings {
                iteration: IterationMode::Auto,
            }),
        }
    }

    // --- Construction ---

    #[test]
    fn new_editor_has_empty_nodes() {
        let model = EditorModel::new();
        assert!(model.nodes.is_empty());
        assert_eq!(model.selected_index, None);
        assert!(!model.dirty);
        assert!(model.undo_stack.is_empty());
        assert!(model.redo_stack.is_empty());
        assert_eq!(model.source, EditorSource::New);
    }

    // --- add_node ---

    #[test]
    fn add_node_appends_and_selects() {
        let mut model = EditorModel::new();
        let info = test_info("image-compress", "Compress Images");
        model.add_node("image-compress", &info);
        assert_eq!(model.nodes.len(), 1);
        assert_eq!(model.nodes[0].node_type, "image-compress");
        assert_eq!(model.nodes[0].label, "Compress Images");
        assert_eq!(model.selected_index, Some(0));
    }

    #[test]
    fn add_node_sets_dirty() {
        let mut model = EditorModel::new();
        model.add_node("image-compress", &test_info("image-compress", "Compress"));
        assert!(model.dirty);
    }

    #[test]
    fn add_node_with_defaults_populates_params() {
        let mut model = EditorModel::new();
        let defaults = vec![("quality".to_string(), json!(80))];
        model.add_node_with_defaults(
            "image-compress",
            &test_info("image-compress", "Compress"),
            &defaults,
        );
        assert_eq!(model.nodes[0].params.get("quality"), Some(&json!(80)));
    }

    #[test]
    fn add_multiple_nodes_selects_last() {
        let mut model = EditorModel::new();
        model.add_node("image-compress", &test_info("image-compress", "Compress"));
        model.add_node("image-resize", &test_info("image-resize", "Resize"));
        assert_eq!(model.nodes.len(), 2);
        assert_eq!(model.selected_index, Some(1));
    }

    // --- remove_node ---

    #[test]
    fn remove_node_by_index() {
        let mut model = EditorModel::new();
        let info = test_info("image-compress", "Compress");
        model.add_node("image-compress", &info);
        model.add_node("image-compress", &info);
        model.remove_node(0);
        assert_eq!(model.nodes.len(), 1);
        assert_eq!(model.nodes[0].id, "image-compress-2");
    }

    #[test]
    fn remove_node_out_of_bounds_is_noop() {
        let mut model = EditorModel::new();
        model.add_node("a", &test_info("a", "A"));
        let undo_len = model.undo_stack.len();
        model.remove_node(5);
        assert_eq!(model.nodes.len(), 1);
        assert_eq!(model.undo_stack.len(), undo_len);
    }

    #[test]
    fn remove_last_node_clears_selection() {
        let mut model = EditorModel::new();
        model.add_node("a", &test_info("a", "A"));
        model.remove_node(0);
        assert!(model.nodes.is_empty());
        assert_eq!(model.selected_index, None);
    }

    #[test]
    fn selected_index_adjusts_on_remove() {
        let mut model = EditorModel::new();
        let info = test_info("a", "A");
        model.add_node("a", &info);
        model.add_node("b", &info);
        model.add_node("c", &info);
        assert_eq!(model.selected_index, Some(2));
        model.remove_node(2);
        assert_eq!(model.selected_index, Some(1));
    }

    // --- reorder ---

    #[test]
    fn reorder_swap_adjacent() {
        let mut model = EditorModel::new();
        let info = test_info("a", "A");
        model.add_node("a", &info);
        model.add_node("b", &info);
        model.reorder(0, 1);
        assert_eq!(model.nodes[0].id, "b-2");
        assert_eq!(model.nodes[1].id, "a-1");
        assert_eq!(model.selected_index, Some(1));
    }

    #[test]
    fn reorder_bounds_check() {
        let mut model = EditorModel::new();
        model.add_node("a", &test_info("a", "A"));
        let undo_len = model.undo_stack.len();
        model.reorder(0, 5);
        assert_eq!(model.undo_stack.len(), undo_len);
    }

    #[test]
    fn reorder_same_index_is_noop() {
        let mut model = EditorModel::new();
        model.add_node("a", &test_info("a", "A"));
        let undo_len = model.undo_stack.len();
        model.reorder(0, 0);
        assert_eq!(model.undo_stack.len(), undo_len);
    }

    // --- update_param ---

    #[test]
    fn update_param_changes_value() {
        let mut model = EditorModel::new();
        model.add_node("image-compress", &test_info("image-compress", "Compress"));
        model.update_param(0, "quality", json!(50));
        assert_eq!(model.nodes[0].params.get("quality"), Some(&json!(50)));
        assert!(model.dirty);
    }

    #[test]
    fn update_param_out_of_bounds_is_noop() {
        let mut model = EditorModel::new();
        let undo_len = model.undo_stack.len();
        model.update_param(5, "quality", json!(50));
        assert_eq!(model.undo_stack.len(), undo_len);
    }

    // --- undo / redo ---

    #[test]
    fn undo_restores_previous_state() {
        let mut model = EditorModel::new();
        model.add_node("image-compress", &test_info("image-compress", "Compress"));
        assert_eq!(model.nodes.len(), 1);
        model.undo();
        assert!(model.nodes.is_empty());
    }

    #[test]
    fn redo_after_undo() {
        let mut model = EditorModel::new();
        model.add_node("image-compress", &test_info("image-compress", "Compress"));
        model.undo();
        assert!(model.nodes.is_empty());
        model.redo();
        assert_eq!(model.nodes.len(), 1);
        assert_eq!(model.nodes[0].node_type, "image-compress");
    }

    #[test]
    fn undo_stack_clears_redo_on_new_action() {
        let mut model = EditorModel::new();
        let info = test_info("image-compress", "Compress");
        model.add_node("image-compress", &info);
        model.undo();
        assert!(!model.redo_stack.is_empty());
        model.add_node("image-resize", &info);
        assert!(model.redo_stack.is_empty());
    }

    #[test]
    fn undo_on_empty_stack_is_noop() {
        let mut model = EditorModel::new();
        model.undo();
        assert!(model.nodes.is_empty());
        assert!(model.redo_stack.is_empty());
    }

    #[test]
    fn redo_on_empty_stack_is_noop() {
        let mut model = EditorModel::new();
        model.redo();
        assert!(model.nodes.is_empty());
    }

    // --- dirty flag ---

    #[test]
    fn dirty_flag_set_on_all_mutations() {
        let info = test_info("a", "A");
        let mut m = EditorModel::new();

        m.add_node("a", &info);
        assert!(m.dirty, "add_node should set dirty");

        m.dirty = false;
        m.remove_node(0);
        assert!(m.dirty, "remove_node should set dirty");

        m.dirty = false;
        m.add_node("a", &info);
        m.add_node("b", &info);
        m.dirty = false;
        m.reorder(0, 1);
        assert!(m.dirty, "reorder should set dirty");

        m.dirty = false;
        m.update_param(0, "k", json!(1));
        assert!(m.dirty, "update_param should set dirty");
    }

    #[test]
    fn mark_clean_clears_dirty() {
        let mut model = EditorModel::new();
        model.add_node("a", &test_info("a", "A"));
        assert!(model.dirty);
        model.mark_clean();
        assert!(!model.dirty);
    }

    // --- Definition round-trip ---

    #[test]
    fn from_definition_populates_nodes() {
        let def = make_test_definition();
        let model = EditorModel::from_definition(&def, EditorSource::New);
        assert_eq!(model.recipe_name, "Test Recipe");
        assert_eq!(model.nodes.len(), 1);
        assert_eq!(model.nodes[0].node_type, "image-compress");
        assert_eq!(model.nodes[0].params.get("quality"), Some(&json!(80)));
        assert!(!model.dirty);
    }

    #[test]
    fn to_definition_produces_valid_structure() {
        let mut model = EditorModel::new();
        model.recipe_name = "My Recipe".to_string();
        model.recipe_description = "A test recipe".to_string();
        model.add_node("image-compress", &test_info("image-compress", "Compress"));
        let def = model.to_definition();
        assert_eq!(def.name, "My Recipe");
        assert_eq!(def.node_type, "group");
        assert_eq!(def.metadata.description.as_deref(), Some("A test recipe"));
        assert_eq!(def.nodes.as_ref().unwrap().len(), 1);
        assert_eq!(
            def.settings,
            Some(PipelineSettings {
                iteration: IterationMode::Auto
            })
        );
    }

    #[test]
    fn new_recipe_has_auto_iteration() {
        let model = EditorModel::new();
        let def = model.to_definition();
        assert_eq!(
            def.settings,
            Some(PipelineSettings {
                iteration: IterationMode::Auto
            })
        );
    }

    #[test]
    fn definition_roundtrip_fidelity() {
        let mut model = EditorModel::new();
        model.recipe_name = "Round Trip".to_string();
        model.recipe_description = "Testing fidelity".to_string();
        let defaults = vec![("quality".to_string(), json!(75))];
        model.add_node_with_defaults(
            "image-compress",
            &test_info("image-compress", "Compress"),
            &defaults,
        );
        let def = model.to_definition();
        let model2 = EditorModel::from_definition(&def, EditorSource::New);
        assert_eq!(model.recipe_name, model2.recipe_name);
        assert_eq!(model.recipe_description, model2.recipe_description);
        assert_eq!(model.nodes.len(), model2.nodes.len());
        assert_eq!(model.nodes[0].node_type, model2.nodes[0].node_type);
        assert_eq!(
            model.nodes[0].params.get("quality"),
            model2.nodes[0].params.get("quality"),
        );
    }

    #[test]
    fn to_definition_generates_sequential_edges() {
        let mut model = EditorModel::new();
        let info = test_info("a", "A");
        model.add_node("a", &info);
        model.add_node("b", &info);
        model.add_node("c", &info);
        let def = model.to_definition();
        let edges = def.edges.as_ref().unwrap();
        assert_eq!(edges.len(), 2);
        assert_eq!(edges[0].source, "a-1");
        assert_eq!(edges[0].target, "b-2");
        assert_eq!(edges[1].source, "b-2");
        assert_eq!(edges[1].target, "c-3");
    }

    #[test]
    fn to_definition_empty_description_omitted() {
        let model = EditorModel::new();
        let def = model.to_definition();
        assert_eq!(def.metadata.description, None);
    }

    // --- File I/O ---

    #[test]
    fn load_valid_recipe() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.bnto.json");
        let json = serde_json::to_string_pretty(&make_test_definition()).unwrap();
        std::fs::write(&path, &json).unwrap();
        let model = EditorModel::load(&path).unwrap();
        assert_eq!(model.recipe_name, "Test Recipe");
        assert_eq!(model.nodes.len(), 1);
        assert_eq!(model.source, EditorSource::File(path));
        assert!(!model.dirty);
    }

    #[test]
    fn load_missing_file_returns_error() {
        let result = EditorModel::load(Path::new("/nonexistent/recipe.bnto.json"));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EditorError::NotFound(_)));
    }

    #[test]
    fn load_invalid_json_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.bnto.json");
        std::fs::write(&path, "not valid json {{{").unwrap();
        let result = EditorModel::load(&path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EditorError::InvalidJson(_)));
    }

    #[test]
    fn save_produces_valid_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("output.bnto.json");
        let mut model = EditorModel::new();
        model.recipe_name = "Saved Recipe".to_string();
        model.add_node("image-compress", &test_info("image-compress", "Compress"));
        model.save_to(&path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let def: Definition = serde_json::from_str(&content).unwrap();
        assert_eq!(def.name, "Saved Recipe");
        assert_eq!(def.nodes.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn save_roundtrip_fidelity() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("roundtrip.bnto.json");
        let mut model = EditorModel::new();
        model.recipe_name = "Roundtrip Test".to_string();
        model.recipe_description = "Fidelity check".to_string();
        let defaults = vec![("quality".to_string(), json!(90))];
        model.add_node_with_defaults(
            "image-compress",
            &test_info("image-compress", "Compress"),
            &defaults,
        );
        model.save_to(&path).unwrap();
        let loaded = EditorModel::load(&path).unwrap();
        assert_eq!(model.recipe_name, loaded.recipe_name);
        assert_eq!(model.recipe_description, loaded.recipe_description);
        assert_eq!(model.nodes.len(), loaded.nodes.len());
        assert_eq!(
            model.nodes[0].params.get("quality"),
            loaded.nodes[0].params.get("quality"),
        );
    }

    #[test]
    fn save_creates_parent_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir
            .path()
            .join("nested")
            .join("deep")
            .join("recipe.bnto.json");
        let model = EditorModel::new();
        model.save_to(&path).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn save_preserves_iteration_setting() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("iter.bnto.json");
        let model = EditorModel::new();
        model.save_to(&path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let def: Definition = serde_json::from_str(&content).unwrap();
        assert_eq!(
            def.settings,
            Some(PipelineSettings {
                iteration: IterationMode::Auto
            })
        );
    }

    // --- save_path resolution ---

    #[test]
    fn save_path_for_file_source_returns_original() {
        let original = PathBuf::from("/tmp/my-recipe.bnto.json");
        let mut model = EditorModel::new();
        model.source = EditorSource::File(original.clone());
        let recipes_dir = Path::new("/home/user/.local/share/bnto/recipes");
        assert_eq!(model.save_path(recipes_dir), original);
    }

    #[test]
    fn save_path_for_new_source_uses_recipes_dir() {
        let mut model = EditorModel::new();
        model.recipe_name = "My Recipe".to_string();
        model.source = EditorSource::New;
        let recipes_dir = Path::new("/home/user/.local/share/bnto/recipes");
        assert_eq!(
            model.save_path(recipes_dir),
            recipes_dir.join("my-recipe.bnto.json")
        );
    }

    #[test]
    fn save_path_for_predefined_source_uses_recipes_dir() {
        let mut model = EditorModel::new();
        model.recipe_name = "Compress Images".to_string();
        model.source = EditorSource::Predefined("compress-images".to_string());
        let recipes_dir = Path::new("/home/user/.local/share/bnto/recipes");
        assert_eq!(
            model.save_path(recipes_dir),
            recipes_dir.join("compress-images.bnto.json")
        );
    }

    #[test]
    fn save_path_for_empty_name_uses_untitled() {
        let model = EditorModel::new(); // name is empty, source is New
        let recipes_dir = Path::new("/tmp/recipes");
        assert_eq!(
            model.save_path(recipes_dir),
            recipes_dir.join("untitled.bnto.json")
        );
    }
}
