// Navigation helpers — back-screen resolution, state cleanup on screen exit.

use super::super::app::{AppModel, DetailOrigin, Screen};
use super::super::screens::home::{HomeModel, list_library_recipes};

/// Check if the form's focus is on the last visible field.
pub(crate) fn is_at_last_visible_field(form: &tonkotsu::FormModel) -> bool {
    let last_visible = form
        .fields
        .iter()
        .enumerate()
        .rev()
        .find(|(_, f)| f.visible)
        .map(|(i, _)| i);
    last_visible == Some(form.focused)
}

/// Navigate back one screen, clearing the state of the screen we're leaving.
pub(crate) fn handle_back(model: AppModel) -> AppModel {
    // Settings picker: return to Settings, not Detail.
    if matches!(&model.screen, Screen::Picker { .. }) && model.settings_picker_field.is_some() {
        return AppModel {
            screen: Screen::Settings,
            picker: None,
            settings_picker_field: None,
            ..model
        };
    }

    // Refresh home library count when returning to Home from Library.
    let home = if matches!(back_screen(&model.screen), Screen::Home) {
        let library_names = list_library_recipes(&model.paths.recipes_dir());
        HomeModel::new(library_names)
    } else {
        model.home
    };
    let library = match &model.screen {
        Screen::Library => None,
        _ => model.library,
    };
    let detail = match &model.screen {
        Screen::Detail { .. } => None,
        _ => model.detail,
    };
    let picker = match &model.screen {
        Screen::Picker { .. } => None,
        _ => model.picker,
    };
    let execution = match &model.screen {
        Screen::Execution { .. } => None,
        _ => model.execution,
    };
    let results = match &model.screen {
        Screen::Results { .. } => None,
        _ => model.results,
    };
    let preview = match &model.screen {
        Screen::Preview { .. } => None,
        _ => model.preview,
    };
    let settings = match &model.screen {
        Screen::Settings => None,
        _ => model.settings,
    };
    let editor = match &model.screen {
        Screen::Editor { .. } => None,
        _ => model.editor,
    };
    let wizard = match &model.screen {
        Screen::Wizard { .. } => None,
        _ => model.wizard,
    };
    AppModel {
        screen: back_screen(&model.screen),
        home,
        library,
        detail,
        picker,
        execution,
        results,
        preview,
        settings,
        editor,
        wizard,
        ..model
    }
}

/// Determine which screen to go back to from the current screen.
pub(crate) fn back_screen(current: &Screen) -> Screen {
    match current {
        Screen::Home => Screen::Home,
        Screen::Library => Screen::Home,
        Screen::Browser => Screen::Home,
        Screen::Detail {
            from: DetailOrigin::Home,
            ..
        } => Screen::Home,
        Screen::Detail {
            from: DetailOrigin::Browser,
            ..
        } => Screen::Browser,
        Screen::Detail {
            from: DetailOrigin::Library,
            ..
        } => Screen::Library,
        Screen::Picker { slug, from } => Screen::Detail {
            slug: slug.clone(),
            from: *from,
        },
        Screen::Execution { .. } => Screen::Home,
        Screen::Results { .. } => Screen::Home,
        Screen::Preview { slug, from } => Screen::Detail {
            slug: slug.clone(),
            from: *from,
        },
        Screen::Settings => Screen::Home,
        Screen::Editor { from } => back_screen_for_editor(*from),
        Screen::Wizard { from } => back_screen_for_editor(*from),
    }
}

/// Determine the back target for the Editor screen.
pub(crate) fn back_screen_for_editor(from: DetailOrigin) -> Screen {
    match from {
        DetailOrigin::Home => Screen::Home,
        DetailOrigin::Browser => Screen::Browser,
        DetailOrigin::Library => Screen::Library,
    }
}

/// Resolve the starting directory for file pickers.
///
/// Returns the current working directory (or "." as fallback).
pub(crate) fn resolve_start_dir() -> std::path::PathBuf {
    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
}
