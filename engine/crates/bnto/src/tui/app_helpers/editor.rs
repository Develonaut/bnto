// Editor & wizard helpers — open, save, form routing, back navigation.

use std::collections::HashMap;

use super::super::app::{AppModel, DetailOrigin, Screen};
use super::super::screens::editor::{
    EditorAction, EditorMessage, EditorScreenModel, update as editor_update,
};
use super::super::screens::home::{HomeModel, list_library_recipes};
use super::super::screens::library::{LibraryModel, load_library_entries};
use super::navigation::back_screen_for_editor;

/// Navigate back from the editor, cleaning up editor state.
pub(crate) fn navigate_back_from_editor(
    model: AppModel,
    _editor: EditorScreenModel,
    from: DetailOrigin,
) -> AppModel {
    let back = back_screen_for_editor(from);
    let home = if matches!(back, Screen::Home) {
        let library_names = list_library_recipes(&model.paths.recipes_dir());
        HomeModel::new(library_names)
    } else {
        model.home
    };
    // Reload library when returning to it (recipe may have been saved).
    let library = if matches!(back, Screen::Library) {
        Some(LibraryModel::new(load_library_entries(
            &model.paths.recipes_dir(),
        )))
    } else {
        model.library
    };
    AppModel {
        screen: back,
        editor: None,
        home,
        library,
        ..model
    }
}

/// Save the editor's recipe to disk. Optionally navigate back after saving.
pub(crate) fn perform_editor_save(
    model: AppModel,
    mut editor: EditorScreenModel,
    navigate_back: bool,
) -> AppModel {
    let save_path = editor.editor.save_path(&model.paths.recipes_dir());
    match editor.editor.save_to(&save_path) {
        Ok(()) => {
            editor.editor.mark_clean();
            // Update source so subsequent saves go to the same file.
            editor.editor.source = bnto_core::editor::EditorSource::File(save_path.clone());
            let display_path = save_path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| save_path.display().to_string());
            if navigate_back {
                let from = match &model.screen {
                    Screen::Editor { from } => *from,
                    _ => DetailOrigin::Home,
                };
                let mut result = navigate_back_from_editor(model, editor, from);
                result.status_message = Some(format!("Saved {display_path}"));
                result
            } else {
                AppModel {
                    editor: Some(editor),
                    status_message: Some(format!("Saved {display_path}")),
                    ..model
                }
            }
        }
        Err(e) => AppModel {
            editor: Some(editor),
            status_message: Some(format!("Failed to save: {e}")),
            ..model
        },
    }
}

/// Load an EditorModel from raw recipe JSON.
pub(crate) fn load_editor_from_json(json: &str) -> Result<bnto_core::editor::EditorModel, String> {
    // Try strict deserialization.
    if let Ok(def) = serde_json::from_str::<bnto_core::definition::Definition>(json) {
        return Ok(bnto_core::editor::EditorModel::from_definition(
            &def,
            bnto_core::editor::EditorSource::Predefined("custom".into()),
        ));
    }
    // Lenient fallback: extract name and nodes from a Value.
    let val: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("invalid JSON: {e}"))?;
    let name = val["name"].as_str().unwrap_or("Untitled").to_string();
    let nodes = val["nodes"]
        .as_array()
        .ok_or_else(|| "missing 'nodes' array".to_string())?;
    let editor_nodes: Vec<bnto_core::editor::EditorNode> = nodes
        .iter()
        .map(|n| bnto_core::editor::EditorNode {
            id: n["id"].as_str().unwrap_or("").to_string(),
            node_type: n["type"].as_str().unwrap_or("").to_string(),
            label: n["name"].as_str().unwrap_or("").to_string(),
            params: json_to_editor_params(n["parameters"].as_object()),
            expanded: false,
        })
        .collect();
    let selected_index = if editor_nodes.is_empty() {
        None
    } else {
        Some(0)
    };
    Ok(bnto_core::editor::EditorModel {
        recipe_name: name,
        recipe_description: val["description"].as_str().unwrap_or("").to_string(),
        nodes: editor_nodes,
        selected_index,
        dirty: false,
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
        source: bnto_core::editor::EditorSource::Predefined("custom".into()),
    })
}

/// Convert JSON parameters to the editor's param format.
fn json_to_editor_params(
    params: Option<&serde_json::Map<String, serde_json::Value>>,
) -> HashMap<String, serde_json::Value> {
    params
        .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default()
}

/// Forward an editor message, handling actions (back, save, save+back).
pub(crate) fn handle_editor(model: AppModel, msg: EditorMessage) -> AppModel {
    let from = match &model.screen {
        Screen::Editor { from } => *from,
        _ => DetailOrigin::Home,
    };
    let editor_opt = model.editor;
    let model = AppModel {
        editor: None,
        ..model
    };
    match editor_opt {
        Some(editor_model) => {
            let (new_editor, action) = editor_update(editor_model, msg, &model.registry);
            match action {
                EditorAction::Back => navigate_back_from_editor(model, new_editor, from),
                EditorAction::Save => perform_editor_save(model, new_editor, false),
                EditorAction::SaveAndBack => perform_editor_save(model, new_editor, true),
                EditorAction::None => AppModel {
                    editor: Some(new_editor),
                    ..model
                },
            }
        }
        None => model,
    }
}

/// Route form messages through the editor screen's update.
pub(crate) fn handle_editor_form(model: AppModel, form_msg: tonkotsu::FormMessage) -> AppModel {
    let msg = EditorMessage::Form(form_msg);
    match model.editor {
        Some(editor_model) => {
            let (new_editor, _) = editor_update(editor_model, msg, &model.registry);
            AppModel {
                editor: Some(new_editor),
                ..model
            }
        }
        None => model,
    }
}

/// Clone a browser recipe into the editor.
pub(crate) fn handle_open_editor_from_browser(model: AppModel) -> AppModel {
    let recipe = model.browser.selected_recipe();
    match recipe {
        Some(r) => {
            let def: Result<bnto_core::definition::Definition, _> =
                serde_json::from_str(&r.definition_json);
            let slug = r.slug.clone();
            match def {
                Ok(def) => {
                    let source = bnto_core::editor::EditorSource::Predefined(slug);
                    let editor_model =
                        bnto_core::editor::EditorModel::from_definition(&def, source);
                    AppModel {
                        screen: Screen::Editor {
                            from: DetailOrigin::Browser,
                        },
                        editor: Some(EditorScreenModel::new(editor_model)),
                        ..model
                    }
                }
                Err(e) => AppModel {
                    status_message: Some(format!("Failed to parse recipe: {e}")),
                    ..model
                },
            }
        }
        None => AppModel {
            status_message: Some("No recipe selected".into()),
            ..model
        },
    }
}

/// Open a library recipe in the editor for in-place editing.
pub(crate) fn handle_open_editor_from_library(model: AppModel) -> AppModel {
    let slug = model
        .library
        .as_ref()
        .and_then(|l| l.confirm())
        .map(|s| s.slug);
    match slug {
        Some(slug) => {
            let path = model.paths.recipes_dir().join(format!("{slug}.bnto.json"));
            match bnto_core::editor::EditorModel::load(&path) {
                Ok(editor_model) => AppModel {
                    screen: Screen::Editor {
                        from: DetailOrigin::Library,
                    },
                    editor: Some(EditorScreenModel::new(editor_model)),
                    ..model
                },
                Err(e) => AppModel {
                    status_message: Some(format!("Failed to load recipe: {e}")),
                    ..model
                },
            }
        }
        None => AppModel {
            status_message: Some("No recipe selected".into()),
            ..model
        },
    }
}
