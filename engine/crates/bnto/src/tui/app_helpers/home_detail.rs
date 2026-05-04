// Home, detail, and execution helpers — recipe selection, config confirm, execution flow.

use std::collections::HashMap;

use super::super::app::{AppModel, DetailOrigin, Screen};
use super::super::screens::detail_bridge;
use super::super::screens::detail_loader::load_detail_from_entry;
use super::super::screens::editor::EditorScreenModel;
use super::super::screens::execution::{
    ExecutionMessage, ExecutionModel, update as execution_update,
};
use super::super::screens::home::HomeConfirmResult;
use super::super::screens::library::{LibraryModel, load_library_entries};
use super::super::screens::results::ResultsModel;
use super::super::screens::settings::SettingsModel;
use super::navigation::{is_at_last_visible_field, resolve_start_dir};

/// Handle the HomeConfirm message — dispatches based on home screen selection.
pub(crate) fn handle_home_confirm(model: AppModel) -> AppModel {
    match model.home.confirm() {
        HomeConfirmResult::Navigate(screen) => match screen {
            Screen::Settings => {
                let settings = SettingsModel::from_toml_config(&model.toml_config);
                AppModel {
                    screen: Screen::Settings,
                    settings: Some(settings),
                    ..model
                }
            }
            other => AppModel {
                screen: other,
                ..model
            },
        },
        HomeConfirmResult::StatusMessage(ref msg) if msg.contains("No recipes") => {
            let library = Some(LibraryModel::new(vec![]));
            AppModel {
                screen: Screen::Library,
                library,
                ..model
            }
        }
        HomeConfirmResult::StatusMessage(msg) => AppModel {
            status_message: Some(msg),
            ..model
        },
        HomeConfirmResult::RecipeAtIndex(idx) => {
            if model.browser.recipes.is_empty() {
                return AppModel {
                    status_message: Some("No recipes available".into()),
                    ..model
                };
            }
            let clamped = idx.min(model.browser.recipes.len() - 1);
            let slug = model.browser.recipes[clamped].slug.clone();
            let start_dir = resolve_start_dir();
            let entry = model.catalog.resolve(&slug);
            let detail =
                entry.and_then(|e| load_detail_from_entry(e, &model.registry, Some(&start_dir)));
            AppModel {
                screen: Screen::Detail {
                    slug,
                    from: DetailOrigin::Home,
                },
                detail,
                ..model
            }
        }
        HomeConfirmResult::LibraryRecipe(_slug) => {
            let entries = load_library_entries(&model.paths.recipes_dir());
            let library = Some(LibraryModel::new(entries));
            AppModel {
                screen: Screen::Library,
                library,
                ..model
            }
        }
        HomeConfirmResult::NewRecipe => {
            let editor_model = bnto_core::editor::EditorModel::new();
            AppModel {
                screen: Screen::Editor {
                    from: DetailOrigin::Home,
                },
                editor: Some(EditorScreenModel::new(editor_model)),
                ..model
            }
        }
    }
}

/// Route a form message to the detail screen's focus sections.
pub(crate) fn handle_detail_form(model: AppModel, msg: tonkotsu::FormMessage) -> AppModel {
    let detail = model.detail.map(|mut d| {
        use super::super::screens::detail::DetailFocus;

        match d.focus {
            DetailFocus::Input => {
                if matches!(msg, tonkotsu::FormMessage::FocusNext) {
                    d.focus = DetailFocus::Params;
                    let line = d.estimate_section_line(DetailFocus::Params);
                    d.viewport.ensure_visible(line);
                }
            }
            DetailFocus::Params => {
                let at_last = matches!(msg, tonkotsu::FormMessage::FocusNext)
                    && is_at_last_visible_field(&d.form);
                let at_first =
                    matches!(msg, tonkotsu::FormMessage::FocusPrev) && d.form.focused == 0;

                if at_last {
                    d.focus = DetailFocus::Run;
                    let line = d.estimate_section_line(DetailFocus::Run);
                    d.viewport.ensure_visible(line);
                } else if at_first && d.input_picker.is_some() {
                    d.focus = DetailFocus::Input;
                    d.viewport.go_to_top();
                } else {
                    d.form = tonkotsu::update(d.form, msg);
                    detail_bridge::update_visibility(&mut d.form, &d.params);
                    // Keep focused field visible after navigation
                    let line = d.estimate_focused_field_line();
                    d.viewport.ensure_visible(line);
                }
            }
            DetailFocus::Run => {
                if matches!(msg, tonkotsu::FormMessage::FocusPrev) {
                    d.focus = DetailFocus::Params;
                    let line = d.estimate_focused_field_line();
                    d.viewport.ensure_visible(line);
                }
            }
        }
        d
    });
    AppModel { detail, ..model }
}

/// Navigate to a recipe's detail screen from the browser.
pub(crate) fn handle_recipe_selected(model: AppModel, slug: String) -> AppModel {
    let start_dir = resolve_start_dir();
    let entry = model.catalog.resolve(&slug);
    let detail = entry.and_then(|e| load_detail_from_entry(e, &model.registry, Some(&start_dir)));
    AppModel {
        screen: Screen::Detail {
            slug,
            from: DetailOrigin::Browser,
        },
        detail,
        ..model
    }
}

/// Confirm config and start execution with the detail screen's settings.
pub(crate) fn handle_config_confirmed(model: AppModel, slug: String) -> AppModel {
    let from = match &model.screen {
        Screen::Detail { from, .. } => *from,
        _ => DetailOrigin::Home,
    };
    let config_result = model.detail.as_ref().map(|d| d.confirm());
    let overrides = config_result
        .as_ref()
        .map(|r| r.overrides.clone())
        .unwrap_or_default();
    let files = config_result
        .as_ref()
        .map(|r| r.files.clone())
        .unwrap_or_default();
    let definition_json = model
        .detail
        .as_ref()
        .map(|d| d.definition_json.clone())
        .unwrap_or_default();
    let execution = Some(ExecutionModel::with_inputs(
        &slug,
        files,
        overrides,
        definition_json,
    ));
    AppModel {
        screen: Screen::Execution { slug, from },
        execution,
        param_overrides: HashMap::new(),
        ..model
    }
}

/// Confirm file selection and start execution.
pub(crate) fn handle_files_selected(model: AppModel, slug: String) -> AppModel {
    let from = match &model.screen {
        Screen::Picker { from, .. } => *from,
        _ => DetailOrigin::Home,
    };
    let files = model
        .picker
        .as_ref()
        .and_then(|p| p.confirm())
        .map(|r| r.files)
        .unwrap_or_default();
    let overrides = model.param_overrides.clone();
    let definition_json = model
        .detail
        .as_ref()
        .map(|d| d.definition_json.clone())
        .unwrap_or_default();
    let execution = Some(ExecutionModel::with_inputs(
        &slug,
        files,
        overrides,
        definition_json,
    ));
    AppModel {
        screen: Screen::Execution { slug, from },
        execution,
        param_overrides: HashMap::new(),
        ..model
    }
}

/// Execution finished — transition to the results screen.
pub(crate) fn handle_execution_complete(model: AppModel, slug: String) -> AppModel {
    let (outputs, elapsed, output_dir) = model
        .execution
        .as_ref()
        .map(|e| (e.output_files.clone(), e.elapsed_ms, e.output_dir.clone()))
        .unwrap_or_default();
    let results = Some(ResultsModel::new(&slug, outputs, elapsed, output_dir));
    AppModel {
        screen: Screen::Results { slug },
        execution: None,
        results,
        ..model
    }
}

/// Start preview mode — same as config_confirmed but with preview_mode flag.
pub(crate) fn handle_preview_requested(model: AppModel, slug: String) -> AppModel {
    let from = match &model.screen {
        Screen::Detail { from, .. } => *from,
        _ => DetailOrigin::Home,
    };
    let config_result = model.detail.as_ref().map(|d| d.confirm());
    let overrides = config_result
        .as_ref()
        .map(|r| r.overrides.clone())
        .unwrap_or_default();
    let files = config_result
        .as_ref()
        .map(|r| r.files.clone())
        .unwrap_or_default();
    let definition_json = model
        .detail
        .as_ref()
        .map(|d| d.definition_json.clone())
        .unwrap_or_default();
    let execution = Some(ExecutionModel::with_preview(
        &slug,
        files,
        overrides,
        definition_json,
    ));
    AppModel {
        screen: Screen::Execution { slug, from },
        execution,
        param_overrides: HashMap::new(),
        ..model
    }
}

/// Confirm preview — re-run with writes using the stored inputs.
pub(crate) fn handle_preview_confirm(model: AppModel, slug: String) -> AppModel {
    let from = match &model.screen {
        Screen::Preview { from, .. } => *from,
        _ => DetailOrigin::Home,
    };
    let (files, overrides, definition_json) = model
        .preview
        .as_ref()
        .map(|p| {
            (
                p.selected_files.clone(),
                p.param_overrides.clone(),
                p.definition_json.clone(),
            )
        })
        .unwrap_or_default();
    let execution = Some(ExecutionModel::with_inputs(
        &slug,
        files,
        overrides,
        definition_json,
    ));
    AppModel {
        screen: Screen::Execution { slug, from },
        execution,
        preview: None,
        param_overrides: HashMap::new(),
        ..model
    }
}

/// Handle execution messages — cancel navigates back to detail.
pub(crate) fn handle_execution(model: AppModel, msg: ExecutionMessage) -> AppModel {
    if matches!(msg, ExecutionMessage::Cancel) {
        let (slug, from) = match &model.screen {
            Screen::Execution { slug, from } => (slug.clone(), *from),
            _ => (String::new(), DetailOrigin::Home),
        };
        let start_dir = resolve_start_dir();
        let entry = model.catalog.resolve(&slug);
        let detail =
            entry.and_then(|e| load_detail_from_entry(e, &model.registry, Some(&start_dir)));
        return AppModel {
            screen: Screen::Detail { slug, from },
            detail,
            execution: None,
            ..model
        };
    }
    let execution = model.execution.map(|e| execution_update(e, msg));
    AppModel { execution, ..model }
}
