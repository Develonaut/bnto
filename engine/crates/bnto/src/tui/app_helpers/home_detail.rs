// Home, detail, and execution helpers — recipe selection, config confirm, execution flow.

use std::collections::HashMap;
use std::path::PathBuf;

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
use super::consent;
use super::navigation::{is_at_last_visible_field, resolve_start_dir};

/// Handle the HomeConfirm message — dispatches based on home screen selection.
#[expect(clippy::too_many_lines)]
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
            // The cursor is a position in the list the pane shows (`filtered`,
            // narrowed by a browser search), not in every recipe (ISS-66).
            let shown = &model.browser.filtered;
            let Some(&recipe) = shown.get(idx.min(shown.len().saturating_sub(1))) else {
                return AppModel {
                    status_message: Some("No recipes available".into()),
                    ..model
                };
            };
            let slug = model.browser.recipes[recipe].slug.clone();
            let start_dir = resolve_start_dir();
            let entry = DetailOrigin::Home.entry(&model.catalog, &slug);
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
///
/// Only moves focus; the main loop's `fit_detail` scrolls the new focus
/// into view, since that needs the terminal width.
#[expect(clippy::too_many_lines)]
pub(crate) fn handle_detail_form(model: AppModel, msg: tonkotsu::FormMessage) -> AppModel {
    let detail = model.detail.map(|mut d| {
        use super::super::screens::detail::DetailFocus;

        match d.focus {
            DetailFocus::Input => {
                if matches!(msg, tonkotsu::FormMessage::FocusNext) {
                    d.focus = DetailFocus::Params;
                }
            }
            DetailFocus::Params => {
                let at_last = matches!(msg, tonkotsu::FormMessage::FocusNext)
                    && is_at_last_visible_field(&d.form);
                let at_first =
                    matches!(msg, tonkotsu::FormMessage::FocusPrev) && d.form.focused == 0;

                if at_last {
                    d.focus = DetailFocus::Run;
                } else if at_first && d.input_picker.is_some() {
                    d.focus = DetailFocus::Input;
                } else {
                    d.form = tonkotsu::update(d.form, msg);
                    detail_bridge::update_visibility(&mut d.form, &d.params);
                }
            }
            DetailFocus::Run => {
                if matches!(msg, tonkotsu::FormMessage::FocusPrev) {
                    d.focus = DetailFocus::Params;
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
    let entry = DetailOrigin::Browser.entry(&model.catalog, &slug);
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

/// Extract detail config and start execution with the given builder.
///
/// Shared by `handle_config_confirmed` (normal run) and
/// `handle_preview_requested` (preview mode) — only the
/// `ExecutionModel` constructor differs.
#[expect(clippy::too_many_lines)]
fn start_from_detail(
    model: AppModel,
    slug: String,
    builder: fn(&str, Vec<PathBuf>, HashMap<String, String>, String) -> ExecutionModel,
) -> AppModel {
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
    let execution = Some(builder(&slug, files, overrides, definition_json));
    AppModel {
        screen: Screen::Execution { slug, from },
        execution,
        param_overrides: HashMap::new(),
        ..model
    }
}

/// Confirm config and start execution with the detail screen's settings.
pub(crate) fn handle_config_confirmed(model: AppModel, slug: String) -> AppModel {
    start_from_detail(model, slug, ExecutionModel::with_inputs)
}

/// Confirm file selection and start execution.
#[expect(clippy::too_many_lines)]
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
    let exec = ExecutionModel::with_inputs(&slug, files, overrides, definition_json);
    AppModel {
        screen: Screen::Execution { slug, from },
        execution: Some(exec),
        param_overrides: HashMap::new(),
        ..model
    }
}

/// Execution finished — transition to the results screen.
pub(crate) fn handle_execution_complete(model: AppModel, slug: String) -> AppModel {
    let results = model.execution.as_ref().map(|e| {
        let log = e.output_lines.iter().cloned().collect();
        ResultsModel::new(
            &slug,
            e.output_files.clone(),
            e.elapsed_ms,
            e.output_dir.clone(),
        )
        .with_report(e.message.clone(), log)
    });
    let results = results.or_else(|| Some(ResultsModel::new(&slug, Vec::new(), 0, None)));
    AppModel {
        screen: Screen::Results { slug },
        execution: None,
        results,
        ..model
    }
}

/// Start preview mode — same as config_confirmed but with preview_mode flag.
pub(crate) fn handle_preview_requested(model: AppModel, slug: String) -> AppModel {
    start_from_detail(model, slug, ExecutionModel::with_preview)
}

/// Confirm preview — re-run with writes using the stored inputs.
#[expect(clippy::too_many_lines)]
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
    let exec = ExecutionModel::with_inputs(&slug, files, overrides, definition_json);
    AppModel {
        screen: Screen::Execution { slug, from },
        execution: Some(exec),
        preview: None,
        param_overrides: HashMap::new(),
        ..model
    }
}

/// Handle execution messages — cancel navigates back to detail.
#[expect(clippy::too_many_lines)]
pub(crate) fn handle_execution(model: AppModel, msg: ExecutionMessage) -> AppModel {
    if matches!(msg, ExecutionMessage::AllowCommands) {
        return consent::handle_allow(model);
    }
    if matches!(msg, ExecutionMessage::Cancel) {
        let (slug, from) = match &model.screen {
            Screen::Execution { slug, from } => (slug.clone(), *from),
            _ => (String::new(), DetailOrigin::Home),
        };
        let start_dir = resolve_start_dir();
        let entry = from.entry(&model.catalog, &slug);
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
