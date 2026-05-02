// Library helpers — add, delete, rename, confirm, and message dispatch.

use super::super::app::{AppModel, DetailOrigin, Screen};
use super::super::screens::home::{HomeModel, list_library_recipes};
use super::super::screens::library::{
    LibraryMessage, LibraryModel, load_library_entries, update as library_update,
};
use super::navigation::resolve_start_dir;

/// Delete a library recipe file from disk.
pub(crate) fn handle_library_delete(model: &AppModel) -> Option<String> {
    let lib = model.library.as_ref()?;
    let idx = lib.confirming_delete?;
    let entry = lib.entries.get(idx)?;
    let path = model
        .paths
        .recipes_dir()
        .join(format!("{}.bnto.json", entry.slug));
    match std::fs::remove_file(&path) {
        Ok(()) => Some(format!("Deleted '{}'", entry.name)),
        Err(e) => Some(format!("Failed to delete: {e}")),
    }
}

/// Rename a library recipe by updating its JSON name field on disk.
pub(crate) fn handle_library_rename(model: &AppModel) -> Option<String> {
    let lib = model.library.as_ref()?;
    let (idx, new_name) = lib.renaming.as_ref()?;
    let entry = lib.entries.get(*idx)?;
    if new_name.is_empty() {
        return None;
    }

    let path = model
        .paths
        .recipes_dir()
        .join(format!("{}.bnto.json", entry.slug));
    let json_str = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => return Some(format!("Failed to read recipe: {e}")),
    };
    let mut doc: serde_json::Value = match serde_json::from_str(&json_str) {
        Ok(v) => v,
        Err(e) => return Some(format!("Failed to parse recipe: {e}")),
    };
    doc["name"] = serde_json::Value::String(new_name.clone());
    let updated = serde_json::to_string_pretty(&doc).unwrap_or_default();
    match crate::storage::atomic::atomic_write(&path, updated.as_bytes()) {
        Ok(()) => Some(format!("Renamed to '{new_name}'")),
        Err(e) => Some(format!("Failed to rename: {e}")),
    }
}

/// Handle "Add to Library" — copies the focused browser recipe to the user's library.
pub(crate) fn handle_add_to_library(model: AppModel) -> AppModel {
    let slug = match model.browser.confirm() {
        Some(r) => r.slug,
        None => {
            return AppModel {
                status_message: Some("No recipe selected".into()),
                ..model
            };
        }
    };

    let dest = model.paths.recipes_dir().join(format!("{slug}.bnto.json"));
    if dest.exists() {
        return AppModel {
            status_message: Some(format!(
                "'{slug}' already in library. Press 'A' to replace."
            )),
            ..model
        };
    }

    handle_add_to_library_write(model, &slug, false)
}

/// Write a built-in recipe to the user's library directory.
pub(crate) fn handle_add_to_library_write(
    model: AppModel,
    slug: &str,
    _overwrite: bool,
) -> AppModel {
    let recipe = match bnto_engine::recipes::builtin_recipe_by_slug(slug) {
        Some(r) => r,
        None => {
            return AppModel {
                status_message: Some(format!("Unknown recipe: {slug}")),
                ..model
            };
        }
    };

    let dest = model.paths.recipes_dir().join(format!("{slug}.bnto.json"));
    let status_message =
        match crate::storage::atomic::atomic_write(&dest, recipe.definition_json.as_bytes()) {
            Ok(()) => {
                let name = recipe.name;
                Some(format!("Added '{name}' to library"))
            }
            Err(e) => Some(format!("Failed to save: {e}")),
        };

    // Refresh home screen library names.
    let library_names = list_library_recipes(&model.paths.recipes_dir());
    let home = HomeModel::new(library_names);

    AppModel {
        home,
        status_message,
        ..model
    }
}

/// Open the library screen.
pub(crate) fn handle_open_library(model: AppModel) -> AppModel {
    let entries = load_library_entries(&model.paths.recipes_dir());
    let library = Some(LibraryModel::new(entries));
    AppModel {
        screen: Screen::Library,
        library,
        ..model
    }
}

/// Forward a library message, intercepting delete/rename for file I/O.
pub(crate) fn handle_library(model: AppModel, msg: &LibraryMessage) -> AppModel {
    let status_message = match msg {
        LibraryMessage::DeleteConfirm => handle_library_delete(&model),
        LibraryMessage::RenameConfirm => handle_library_rename(&model),
        _ => None,
    };
    let library = model.library.map(|l| library_update(l, msg.clone()));
    let home = if matches!(msg, LibraryMessage::DeleteConfirm) {
        let library_names = list_library_recipes(&model.paths.recipes_dir());
        HomeModel::new(library_names)
    } else {
        model.home
    };
    AppModel {
        library,
        home,
        status_message: status_message.or(model.status_message),
        ..model
    }
}

/// Confirm a library selection and navigate to detail.
///
/// Tries loading from the library file on disk first (handles recipes
/// that aren't builtins), then falls back to builtin lookup.
pub(crate) fn handle_library_confirm(model: AppModel) -> AppModel {
    let slug = model
        .library
        .as_ref()
        .and_then(|l| l.confirm())
        .map(|s| s.slug);
    match slug {
        Some(slug) => {
            let start_dir = resolve_start_dir();
            let detail = super::super::screens::detail_loader::load_detail_from_library(
                &slug,
                &model.paths.recipes_dir(),
                &model.registry,
                Some(&start_dir),
            )
            .or_else(|| {
                super::super::screens::detail_loader::load_detail_with_dir(
                    &slug,
                    &model.registry,
                    Some(&start_dir),
                )
            });
            AppModel {
                screen: Screen::Detail {
                    slug,
                    from: DetailOrigin::Library,
                },
                detail,
                ..model
            }
        }
        None => AppModel {
            status_message: Some("No recipe selected".into()),
            ..model
        },
    }
}
