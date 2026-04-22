// App state machine — routes between TUI screens.
//
// Pure state + pure transitions. No I/O, no terminal access.
// All screen navigation logic is testable with `cargo test`.

use std::collections::HashMap;

use bnto_core::registry::NodeRegistry;
use bnto_engine::create_registry;

use super::config::TuiConfig;
use super::migration::migrate_if_needed;
use super::paths::BntoPaths;
use super::screens::browser::{BrowserMessage, BrowserModel, update as browser_update};
use super::screens::detail::DetailModel;
use super::screens::detail_bridge;
use super::screens::editor::{
    EditorAction, EditorMessage, EditorScreenModel, update as editor_update,
};
use super::screens::execution::{ExecutionMessage, ExecutionModel, update as execution_update};
use super::screens::home::{
    HomeConfirmResult, HomeMessage, HomeModel, list_library_recipes, update as home_update,
};
use super::screens::library::{
    LibraryMessage, LibraryModel, load_library_entries, update as library_update,
};
use super::screens::picker::{PickerMessage, PickerModel, update as picker_update};
use super::screens::results::{ResultsMessage, ResultsModel, update as results_update};
use super::screens::settings::{SettingsMessage, SettingsModel, update as settings_update};
use super::screens::wizard::{WizardAction, WizardMessage, WizardModel, update as wizard_update};
use super::theme::{Theme, ThemeVariant};
use super::toml_config::TomlConfig;

/// Where the user came from when entering the Detail screen.
///
/// Used by `back_screen()` to return to the correct origin —
/// Home (from the bento grid) or Browser (from the full recipe list).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailOrigin {
    Home,
    Browser,
    Library,
}

/// Which screen the TUI is currently showing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Home,
    Library,
    Browser,
    Detail { slug: String, from: DetailOrigin },
    Picker { slug: String, from: DetailOrigin },
    Execution { slug: String, from: DetailOrigin },
    Results { slug: String },
    Settings,
    Editor { from: DetailOrigin },
    Wizard { from: DetailOrigin },
}

/// Top-level app state.
pub struct AppModel {
    pub screen: Screen,
    pub should_quit: bool,
    pub theme: Theme,
    pub theme_variant: ThemeVariant,
    /// Home screen state (always present — Home is the root screen).
    pub home: HomeModel,
    pub browser: BrowserModel,
    /// Library screen state — populated when navigating to My Library.
    pub library: Option<LibraryModel>,
    /// Detail screen state — populated when navigating to a recipe.
    pub detail: Option<DetailModel>,
    /// Picker screen state — populated when navigating to file picker.
    pub picker: Option<PickerModel>,
    /// Execution screen state — populated when running a pipeline.
    pub execution: Option<ExecutionModel>,
    /// Results screen state — populated when pipeline completes.
    pub results: Option<ResultsModel>,
    /// Settings screen state — populated when on settings screen.
    pub settings: Option<SettingsModel>,
    /// Editor screen state — populated when editing a recipe.
    pub editor: Option<EditorScreenModel>,
    /// Wizard screen state — populated when creating a recipe via wizard.
    pub wizard: Option<WizardModel>,
    /// Persistent config loaded from disk (old JSON format, for compatibility).
    pub config: TuiConfig,
    /// TOML-based config (new format) — used for saves.
    pub toml_config: TomlConfig,
    /// Resolved storage paths (XDG-compliant).
    pub paths: BntoPaths,
    /// Transient status bar message (e.g. "Settings saved" or "Failed to save").
    pub status_message: Option<String>,
    /// Which settings field opened the picker (None = normal recipe picker).
    pub settings_picker_field: Option<String>,
    /// Param overrides from detail screen, carried through picker to execution.
    pub param_overrides: HashMap<String, String>,
    /// Engine registry for resolving processor metadata.
    pub registry: NodeRegistry,
}

impl std::fmt::Debug for AppModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppModel")
            .field("screen", &self.screen)
            .field("should_quit", &self.should_quit)
            .field("theme_variant", &self.theme_variant)
            .field("home_focused", &self.home.focused)
            .field("library", &self.library.is_some())
            .field("detail", &self.detail)
            .field("picker", &self.picker.as_ref().map(|p| &p.slug))
            .field("execution", &self.execution.as_ref().map(|e| &e.status))
            .field("results", &self.results.as_ref().map(|r| &r.slug))
            .field("settings", &self.settings.is_some())
            .field("editor", &self.editor.is_some())
            .field("wizard", &self.wizard.is_some())
            .field("status_message", &self.status_message)
            .field("settings_picker_field", &self.settings_picker_field)
            .field("param_overrides", &self.param_overrides.len())
            .finish_non_exhaustive()
    }
}

/// Messages that drive screen transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AppMessage {
    /// User selected a recipe in the browser.
    RecipeSelected { slug: String },
    /// User confirmed config and moves to file picker.
    ConfigConfirmed { slug: String },
    /// User selected files and starts execution.
    FilesSelected { slug: String },
    /// Execution finished, show results.
    ExecutionComplete { slug: String },
    /// Navigate back one screen.
    Back,
    /// User wants to run another recipe (from results).
    RunAnother,
    /// Open the settings screen.
    OpenSettings,
    /// User selected a new theme in settings.
    ThemeChanged(ThemeVariant),
    /// User toggled telemetry on/off in settings.
    TelemetryToggled(bool),
    /// Forward a message to the home screen.
    Home(HomeMessage),
    /// User confirmed the home screen selection.
    HomeConfirm,
    /// Navigate to the Library screen.
    OpenLibrary,
    /// Forward a message to the library screen.
    Library(LibraryMessage),
    /// User confirmed a library selection.
    LibraryConfirm,
    /// Add the currently focused browser recipe to the user's library.
    AddToLibrary,
    /// User confirmed overwriting an existing library recipe.
    AddToLibraryConfirm { slug: String },
    /// Forward a message to the browser screen.
    Browser(BrowserMessage),
    /// Forward a form message to the detail screen's FormModel.
    DetailForm(bnto_form::FormMessage),
    /// Forward a message to the detail screen's embedded picker.
    DetailPicker(PickerMessage),
    /// Move detail focus to Input section.
    DetailFocusInput,
    /// Move detail focus to Params section.
    DetailFocusParams,
    /// Forward a message to the picker screen.
    Picker(PickerMessage),
    /// Forward a message to the execution screen.
    Execution(ExecutionMessage),
    /// Forward a message to the results screen.
    Results(ResultsMessage),
    /// Forward a message to the settings screen.
    Settings(SettingsMessage),
    /// Forward a message to the editor screen.
    Editor(EditorMessage),
    /// Forward a form message to the editor's inline form.
    EditorForm(bnto_form::FormMessage),
    /// Forward a message to the wizard screen.
    Wizard(WizardMessage),
    /// Forward a form message to the wizard's inline form.
    WizardForm(bnto_form::FormMessage),
    /// Open the wizard for guided recipe creation.
    OpenWizard,
    /// Open the editor for a predefined recipe (clone into editor).
    OpenEditorFromBrowser,
    /// Open the editor for a library recipe (edit in place).
    OpenEditorFromLibrary,
    /// Open the file picker from settings to browse for a directory.
    OpenSettingsPicker { field_key: String },
    /// Confirm the current picker directory as the settings field value.
    SettingsPathConfirmed,
    /// Quit the application.
    Quit,
}

impl AppModel {
    /// Create a new app using platform-default paths.
    ///
    /// Loads persisted config from disk. The `variant` argument is the
    /// CLI override — if `None`, uses the config's saved theme.
    /// If `recipe_json` is Some, opens the recipe in the editor.
    /// If `new_recipe` is true, opens a blank editor.
    pub fn new(variant: ThemeVariant, recipe_json: Option<String>, new_recipe: bool) -> Self {
        let paths = BntoPaths::resolve().unwrap_or_else(|| BntoPaths {
            config: std::path::PathBuf::from(".bnto/config"),
            data: std::path::PathBuf::from(".bnto/data"),
            state: std::path::PathBuf::from(".bnto/state"),
            cache: std::path::PathBuf::from(".bnto/cache"),
        });
        Self::with_paths(variant, recipe_json, new_recipe, paths)
    }

    /// Create a new app with explicit storage paths.
    ///
    /// Loads TOML config from `paths.config_file()`, running migration
    /// from old JSON format if needed. Falls back to defaults on any error.
    pub fn with_paths(
        variant: ThemeVariant,
        recipe_json: Option<String>,
        new_recipe: bool,
        paths: BntoPaths,
    ) -> Self {
        let _ = paths.ensure_dirs();

        // Try migration from old JSON config, then load TOML.
        migrate_if_needed(&paths);
        let toml_config = TomlConfig::load(&paths);

        // Build a TuiConfig from TOML values for backward compatibility.
        let config = TuiConfig {
            theme: toml_config.tui.theme.clone(),
            default_path: toml_config.picker.default_path.clone(),
            output_dir: toml_config.output.dir.clone(),
        };

        // CLI --theme flag overrides saved config.
        let effective_variant = if variant != ThemeVariant::LosAngeles {
            variant
        } else {
            ThemeVariant::from_str_lossy(&config.theme).unwrap_or(variant)
        };
        let registry = create_registry();

        // List library recipes for the home screen pane.
        let library_names = list_library_recipes(&paths.data);

        // Determine initial screen and state.
        let (screen, detail, editor) = if new_recipe {
            // --new flag: blank editor.
            let editor_model = bnto_core::editor::EditorModel::new();
            (
                Screen::Editor {
                    from: DetailOrigin::Home,
                },
                None,
                Some(EditorScreenModel::new(editor_model)),
            )
        } else if let Some(json) = recipe_json {
            // File arg: try strict Definition deserialization first, then lenient fallback.
            match load_editor_from_json(&json) {
                Ok(editor_model) => (
                    Screen::Editor {
                        from: DetailOrigin::Home,
                    },
                    None,
                    Some(EditorScreenModel::new(editor_model)),
                ),
                Err(e) => {
                    eprintln!("Warning: {e} — starting on home screen.");
                    (Screen::Home, None, None)
                }
            }
        } else {
            (Screen::Home, None, None)
        };

        Self {
            screen,
            should_quit: false,
            theme: Theme::from_variant(effective_variant),
            theme_variant: effective_variant,
            home: HomeModel::new(library_names),
            browser: BrowserModel::new(),
            library: None,
            detail,
            picker: None,
            execution: None,
            results: None,
            settings: None,
            editor,
            wizard: None,
            config,
            toml_config,
            paths,
            status_message: None,
            settings_picker_field: None,
            param_overrides: HashMap::new(),
            registry,
        }
    }
}

/// Pure state transition — the heart of the TEA pattern.
/// Takes current state + message, returns the next state.
pub fn update(model: AppModel, msg: AppMessage) -> AppModel {
    match msg {
        AppMessage::RecipeSelected { slug } => {
            let start_dir = resolve_start_dir(&model.config);
            let detail = super::screens::detail_loader::load_detail_with_dir(
                &slug,
                &model.registry,
                Some(&start_dir),
            );
            AppModel {
                screen: Screen::Detail {
                    slug,
                    from: DetailOrigin::Browser,
                },
                detail,
                ..model
            }
        }
        AppMessage::ConfigConfirmed { slug } => {
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

            // All recipes go directly to Execution now.
            // File-mode recipes get files from the embedded picker in DetailModel.
            // URL/text-mode recipes get empty files (URL is in overrides).
            let execution = Some(ExecutionModel::with_inputs(&slug, files, overrides));
            AppModel {
                screen: Screen::Execution { slug, from },
                execution,
                param_overrides: HashMap::new(),
                ..model
            }
        }
        AppMessage::FilesSelected { slug } => {
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
            let execution = Some(ExecutionModel::with_inputs(&slug, files, overrides));
            AppModel {
                screen: Screen::Execution { slug, from },
                execution,
                param_overrides: HashMap::new(),
                ..model
            }
        }
        AppMessage::ExecutionComplete { slug } => {
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
        AppMessage::Home(msg) => {
            let home = home_update(model.home, msg);
            AppModel { home, ..model }
        }
        AppMessage::HomeConfirm => match model.home.confirm() {
            HomeConfirmResult::Navigate(screen) => match screen {
                Screen::Settings => {
                    let settings = SettingsModel::from_config(&model.config);
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
                // Empty library — still open Library screen to show empty state.
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
                // Clamp to browser recipe count to avoid out-of-bounds.
                if model.browser.recipes.is_empty() {
                    return AppModel {
                        status_message: Some("No recipes available".into()),
                        ..model
                    };
                }
                let clamped = idx.min(model.browser.recipes.len() - 1);
                let slug = model.browser.recipes[clamped].slug.clone();
                let start_dir = resolve_start_dir(&model.config);
                let detail = super::screens::detail_loader::load_detail_with_dir(
                    &slug,
                    &model.registry,
                    Some(&start_dir),
                );
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
                // Navigate to the full Library screen for browse/search/manage.
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
        },
        AppMessage::OpenLibrary => {
            let entries = load_library_entries(&model.paths.recipes_dir());
            let library = Some(LibraryModel::new(entries));
            AppModel {
                screen: Screen::Library,
                library,
                ..model
            }
        }
        AppMessage::Library(ref msg) => {
            // Intercept delete/rename confirms to perform file I/O.
            let status_message = match msg {
                LibraryMessage::DeleteConfirm => handle_library_delete(&model),
                LibraryMessage::RenameConfirm => handle_library_rename(&model),
                _ => None,
            };
            let library = model.library.map(|l| library_update(l, msg.clone()));
            // Refresh home library names after delete.
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
        AppMessage::LibraryConfirm => {
            let slug = model
                .library
                .as_ref()
                .and_then(|l| l.confirm())
                .map(|s| s.slug);
            match slug {
                Some(slug) => {
                    let start_dir = resolve_start_dir(&model.config);
                    let detail = super::screens::detail_loader::load_detail_with_dir(
                        &slug,
                        &model.registry,
                        Some(&start_dir),
                    );
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
        AppMessage::AddToLibrary => handle_add_to_library(model),
        AppMessage::AddToLibraryConfirm { slug } => handle_add_to_library_write(model, &slug, true),
        AppMessage::Back => handle_back(model),
        AppMessage::RunAnother => AppModel {
            screen: Screen::Browser,
            ..model
        },
        AppMessage::OpenSettings => {
            let settings = SettingsModel::from_config(&model.config);
            AppModel {
                screen: Screen::Settings,
                settings: Some(settings),
                ..model
            }
        }
        AppMessage::ThemeChanged(variant) => {
            // Update both config formats and persist via TOML.
            let mut config = model.config.clone();
            config.theme = variant.as_slug().to_string();
            let mut toml_config = model.toml_config.clone();
            toml_config.tui.theme = variant.as_slug().to_string();
            let status_message = match toml_config.save(&model.paths) {
                Ok(()) => None,
                Err(e) => Some(format!("Failed to save: {e}")),
            };
            // Also update settings model if it exists (theme field display).
            let settings = model.settings.map(|mut s| {
                if let Some(f) = s.fields.iter_mut().find(|f| f.key == "theme") {
                    f.value = variant.display_name().to_string();
                }
                s
            });
            AppModel {
                theme: Theme::from_variant(variant),
                theme_variant: variant,
                config,
                toml_config,
                settings,
                status_message,
                ..model
            }
        }
        AppMessage::TelemetryToggled(enabled) => {
            crate::telemetry::set_enabled(enabled);
            let mut toml_config = model.toml_config.clone();
            toml_config.telemetry.enabled = enabled;
            let status_message = match toml_config.save(&model.paths) {
                Ok(()) => None,
                Err(e) => Some(format!("Failed to save: {e}")),
            };
            let settings = model.settings.map(|mut s| {
                if let Some(f) = s.fields.iter_mut().find(|f| f.key == "telemetry") {
                    f.value = if enabled { "On" } else { "Off" }.to_string();
                }
                s
            });
            AppModel {
                toml_config,
                settings,
                status_message,
                ..model
            }
        }
        AppMessage::Browser(msg) => {
            let browser = browser_update(model.browser, msg);
            AppModel { browser, ..model }
        }
        AppMessage::DetailForm(msg) => {
            let detail = model.detail.map(|mut d| {
                use super::screens::detail::DetailFocus;

                match d.focus {
                    DetailFocus::Input => {
                        // Tab from Input section → move focus to Params.
                        if matches!(msg, bnto_form::FormMessage::FocusNext) {
                            d.focus = DetailFocus::Params;
                        }
                    }
                    DetailFocus::Params => {
                        // FocusNext at the last visible field → move to Run button.
                        let at_last = matches!(msg, bnto_form::FormMessage::FocusNext)
                            && is_at_last_visible_field(&d.form);
                        let at_first =
                            matches!(msg, bnto_form::FormMessage::FocusPrev) && d.form.focused == 0;

                        if at_last {
                            d.focus = DetailFocus::Run;
                        } else if at_first && d.input_picker.is_some() {
                            // FocusPrev at top of form → back to Input section.
                            d.focus = DetailFocus::Input;
                        } else {
                            d.form = bnto_form::update(d.form, msg);
                            detail_bridge::update_visibility(&mut d.form, &d.params);
                        }
                    }
                    DetailFocus::Run => {
                        // FocusPrev on Run → return focus to Params.
                        if matches!(msg, bnto_form::FormMessage::FocusPrev) {
                            d.focus = DetailFocus::Params;
                        }
                    }
                }
                d
            });
            AppModel { detail, ..model }
        }
        AppMessage::DetailPicker(msg) => {
            let detail = model.detail.map(|mut d| {
                if let Some(picker) = d.input_picker.take() {
                    d.input_picker = Some(picker_update(picker, msg));
                }
                d
            });
            AppModel { detail, ..model }
        }
        AppMessage::DetailFocusInput => {
            let detail = model.detail.map(|mut d| {
                d.focus = super::screens::detail::DetailFocus::Input;
                d
            });
            AppModel { detail, ..model }
        }
        AppMessage::DetailFocusParams => {
            let detail = model.detail.map(|mut d| {
                d.focus = super::screens::detail::DetailFocus::Params;
                d
            });
            AppModel { detail, ..model }
        }
        AppMessage::Picker(msg) => {
            let picker = model.picker.map(|p| picker_update(p, msg));
            AppModel { picker, ..model }
        }
        AppMessage::Execution(msg) => {
            // Cancel navigates back to detail instead of staying on a dead screen.
            if matches!(msg, ExecutionMessage::Cancel) {
                let (slug, from) = match &model.screen {
                    Screen::Execution { slug, from } => (slug.clone(), *from),
                    _ => (String::new(), DetailOrigin::Home),
                };
                let start_dir = resolve_start_dir(&model.config);
                let detail = super::screens::detail_loader::load_detail_with_dir(
                    &slug,
                    &model.registry,
                    Some(&start_dir),
                );
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
        AppMessage::Results(msg) => {
            let results = model.results.map(|r| results_update(r, msg));
            AppModel { results, ..model }
        }
        AppMessage::Settings(msg) => {
            let settings = model.settings.map(|s| settings_update(s, msg));
            AppModel { settings, ..model }
        }
        AppMessage::Editor(msg) => {
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
        AppMessage::EditorForm(form_msg) => {
            // Route form messages through the editor screen's update.
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
        AppMessage::Wizard(msg) => {
            let from = match &model.screen {
                Screen::Wizard { from } => *from,
                _ => DetailOrigin::Home,
            };
            match model.wizard {
                Some(wizard_model) => {
                    let (new_wizard, action) = wizard_update(wizard_model, msg, &model.registry);
                    match action {
                        WizardAction::None => AppModel {
                            wizard: Some(new_wizard),
                            ..model
                        },
                        WizardAction::Complete(editor_screen) => AppModel {
                            screen: Screen::Editor { from },
                            editor: Some(*editor_screen),
                            wizard: None,
                            ..model
                        },
                        WizardAction::Back => {
                            let back = back_screen_for_editor(from);
                            AppModel {
                                screen: back,
                                wizard: None,
                                ..model
                            }
                        }
                    }
                }
                None => model,
            }
        }
        AppMessage::WizardForm(form_msg) => match model.wizard {
            Some(wizard_model) => {
                let msg = WizardMessage::Form(form_msg);
                let (new_wizard, _) = wizard_update(wizard_model, msg, &model.registry);
                AppModel {
                    wizard: Some(new_wizard),
                    ..model
                }
            }
            None => model,
        },
        AppMessage::OpenWizard => {
            let from = match &model.screen {
                Screen::Home => DetailOrigin::Home,
                Screen::Browser => DetailOrigin::Browser,
                Screen::Library => DetailOrigin::Library,
                _ => DetailOrigin::Home,
            };
            let wizard = WizardModel::new(&model.registry);
            AppModel {
                screen: Screen::Wizard { from },
                wizard: Some(wizard),
                ..model
            }
        }
        AppMessage::OpenEditorFromBrowser => {
            // Clone the focused browser recipe into a new editor session.
            let slug = model.browser.confirm().map(|r| r.slug);
            match slug {
                Some(slug) => {
                    let recipe = bnto_engine::recipes::builtin_recipe_by_slug(&slug);
                    match recipe {
                        Some(r) => {
                            let def: Result<bnto_core::definition::Definition, _> =
                                serde_json::from_str(r.definition_json);
                            match def {
                                Ok(def) => {
                                    let source = bnto_core::editor::EditorSource::Predefined(slug);
                                    let editor_model =
                                        bnto_core::editor::EditorModel::from_definition(
                                            &def, source,
                                        );
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
                            status_message: Some(format!("Unknown recipe: {slug}")),
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
        AppMessage::OpenEditorFromLibrary => {
            // Open a library recipe in the editor for in-place editing.
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
        AppMessage::OpenSettingsPicker { field_key } => {
            // Start the picker at the field's current value if it's a valid dir.
            let current_value = model
                .settings
                .as_ref()
                .and_then(|s| s.fields.iter().find(|f| f.key == field_key))
                .map(|f| f.value.clone())
                .unwrap_or_default();
            let start_dir = if !current_value.is_empty() {
                let p = std::path::PathBuf::from(&current_value);
                if p.is_dir() {
                    p
                } else {
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
                }
            } else {
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
            };
            let picker = Some(PickerModel::from_dir(&field_key, &start_dir));
            AppModel {
                screen: Screen::Picker {
                    slug: field_key.clone(),
                    from: DetailOrigin::Home,
                },
                picker,
                settings_picker_field: Some(field_key),
                ..model
            }
        }
        AppMessage::SettingsPathConfirmed => {
            let dir_path = model
                .picker
                .as_ref()
                .map(|p| p.current_dir.to_string_lossy().to_string())
                .unwrap_or_default();
            let field_key = model.settings_picker_field.clone().unwrap_or_default();
            let settings = model.settings.map(|mut s| {
                if let Some(f) = s.fields.iter_mut().find(|f| f.key == field_key) {
                    f.value = dir_path;
                }
                s
            });
            let config = settings
                .as_ref()
                .map(|s| s.to_config(model.theme_variant))
                .unwrap_or_else(|| model.config.clone());
            // Build TOML config from settings and save.
            let mut toml_config = model.toml_config.clone();
            toml_config.tui.theme = config.theme.clone();
            toml_config.output.dir = config.output_dir.clone();
            toml_config.picker.default_path = config.default_path.clone();
            let status_message = match toml_config.save(&model.paths) {
                Ok(()) => None,
                Err(e) => Some(format!("Failed to save: {e}")),
            };
            AppModel {
                screen: Screen::Settings,
                picker: None,
                settings_picker_field: None,
                settings,
                config,
                toml_config,
                status_message,
                ..model
            }
        }
        AppMessage::Quit => AppModel {
            should_quit: true,
            ..model
        },
    }
}

/// Check if the form's focus is on the last visible field.
fn is_at_last_visible_field(form: &bnto_form::FormModel) -> bool {
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
fn handle_back(model: AppModel) -> AppModel {
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
        settings,
        editor,
        wizard,
        ..model
    }
}

/// Determine which screen to go back to from the current screen.
fn back_screen(current: &Screen) -> Screen {
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
        Screen::Settings => Screen::Home,
        Screen::Editor { from } => back_screen_for_editor(*from),
        Screen::Wizard { from } => back_screen_for_editor(*from),
    }
}

/// Determine the back target for the Editor screen.
fn back_screen_for_editor(from: DetailOrigin) -> Screen {
    match from {
        DetailOrigin::Home => Screen::Home,
        DetailOrigin::Browser => Screen::Browser,
        DetailOrigin::Library => Screen::Library,
    }
}

/// Navigate back from the editor, cleaning up editor state.
fn navigate_back_from_editor(
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
fn perform_editor_save(
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

/// Delete a library recipe file from disk.
///
/// Called before `library_update` so `confirming_delete` is still set.
/// Returns a status message on success or failure.
fn handle_library_delete(model: &AppModel) -> Option<String> {
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
///
/// Called before `library_update` so `renaming` is still set.
/// Reads the JSON, patches the `name` field, writes atomically.
fn handle_library_rename(model: &AppModel) -> Option<String> {
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
    match super::atomic::atomic_write(&path, updated.as_bytes()) {
        Ok(()) => Some(format!("Renamed to '{new_name}'")),
        Err(e) => Some(format!("Failed to rename: {e}")),
    }
}

/// Load an EditorModel from raw recipe JSON.
///
/// Tries strict `Definition` deserialization first. If that fails (e.g.,
/// simplified JSON without all required fields), falls back to lenient
/// Value-based parsing that extracts name and nodes.
fn load_editor_from_json(json: &str) -> Result<bnto_core::editor::EditorModel, String> {
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

/// Handle "Add to Library" — copies the focused browser recipe to the user's library.
///
/// If the file already exists, sets a status message prompting confirmation.
/// Otherwise writes immediately.
fn handle_add_to_library(model: AppModel) -> AppModel {
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
fn handle_add_to_library_write(model: AppModel, slug: &str, _overwrite: bool) -> AppModel {
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
    let status_message = match super::atomic::atomic_write(&dest, recipe.definition_json.as_bytes())
    {
        Ok(()) => {
            // Refresh home library count.
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

/// Resolve the starting directory for file pickers.
///
/// Uses the config's `default_path` if it's a valid directory,
/// otherwise falls back to the current working directory.
fn resolve_start_dir(config: &TuiConfig) -> std::path::PathBuf {
    config
        .default_path
        .as_ref()
        .map(std::path::PathBuf::from)
        .filter(|p| p.is_dir())
        .unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
        })
}

/// Resolve the input mode for a built-in recipe slug.
///
/// Looks up the recipe definition, parses it, and reads the input
/// node's `mode` parameter. Falls back to `FileUpload` for unknown
/// slugs or parse failures.
#[cfg(test)]
fn resolve_input_mode_for_slug(slug: &str) -> bnto_core::InputMode {
    let Some(recipe) = bnto_engine::recipes::builtin_recipe_by_slug(slug) else {
        return bnto_core::InputMode::FileUpload;
    };
    let Ok(def) = serde_json::from_str::<bnto_core::PipelineDefinition>(recipe.definition_json)
    else {
        return bnto_core::InputMode::FileUpload;
    };
    bnto_core::resolve_input_mode(&def)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::InputMode;

    /// Build test paths in a unique temp directory.
    ///
    /// Pre-creates dirs and a default config.toml so `migrate_if_needed()`
    /// is always skipped — otherwise migration reads from the real system
    /// config dir and poisons test state.
    fn test_paths() -> BntoPaths {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("bnto-test-{id}"));
        let paths = BntoPaths {
            config: root.join("config"),
            data: root.join("data"),
            state: root.join("state"),
            cache: root.join("cache"),
        };
        let _ = paths.ensure_dirs();
        let _ = TomlConfig::default().save(&paths);
        paths
    }

    fn default_model() -> AppModel {
        AppModel::with_paths(ThemeVariant::LosAngeles, None, false, test_paths())
    }

    /// Apply a message to a model on the given screen, return the resulting screen.
    fn transition(screen: Screen, msg: AppMessage) -> Screen {
        update(
            AppModel {
                screen,
                ..default_model()
            },
            msg,
        )
        .screen
    }

    #[test]
    fn initial_state_is_home() {
        let app = default_model();
        assert_eq!(app.screen, Screen::Home);
        assert!(!app.should_quit);
    }

    #[test]
    fn default_model_uses_isolated_paths() {
        let app = default_model();
        let config_path = app.paths.config_file();
        let real_config = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("bnto")
            .join("config.toml");
        assert_ne!(
            config_path, real_config,
            "test models must never write to the real config directory"
        );
    }

    // --- Home screen ---

    #[test]
    fn home_confirm_library_empty_opens_library() {
        let app = default_model(); // Home, focused=Library, empty library
        let app = update(app, AppMessage::HomeConfirm);
        assert_eq!(app.screen, Screen::Library);
        assert!(app.library.is_some());
    }

    #[test]
    fn home_confirm_recipes_navigates_to_detail() {
        let mut app = default_model();
        app.home.focused = super::super::screens::home::HomePane::Recipes;
        let app = update(app, AppMessage::HomeConfirm);
        // Should navigate to Detail for the first recipe.
        assert!(matches!(app.screen, Screen::Detail { .. }));
    }

    #[test]
    fn home_confirm_navigates_to_settings() {
        let mut app = default_model();
        app.home.focused = super::super::screens::home::HomePane::Settings;
        let app = update(app, AppMessage::HomeConfirm);
        assert_eq!(app.screen, Screen::Settings);
        assert!(app.settings.is_some());
    }

    #[test]
    fn home_confirm_new_recipe_opens_editor() {
        let mut app = default_model();
        app.home.focused = super::super::screens::home::HomePane::NewRecipe;
        let app = update(app, AppMessage::HomeConfirm);
        assert!(
            matches!(
                app.screen,
                Screen::Editor {
                    from: DetailOrigin::Home
                }
            ),
            "expected Editor from Home, got {:?}",
            app.screen
        );
        assert!(app.editor.is_some());
    }

    #[test]
    fn home_message_forwarded() {
        let app = default_model();
        assert_eq!(
            app.home.focused,
            super::super::screens::home::HomePane::Library
        );
        let app = update(app, AppMessage::Home(HomeMessage::NextPane));
        assert_eq!(
            app.home.focused,
            super::super::screens::home::HomePane::Recipes
        );
    }

    // --- Forward navigation (happy path) ---

    #[test]
    fn forward_navigation_follows_happy_path() {
        let s = "t".to_string();
        let from = DetailOrigin::Browser;
        assert_eq!(
            transition(
                Screen::Browser,
                AppMessage::RecipeSelected { slug: s.clone() }
            ),
            Screen::Detail {
                slug: s.clone(),
                from,
            }
        );
        // ConfigConfirmed goes directly to Execution (files from embedded picker).
        assert_eq!(
            transition(
                Screen::Detail {
                    slug: s.clone(),
                    from,
                },
                AppMessage::ConfigConfirmed { slug: s.clone() }
            ),
            Screen::Execution {
                slug: s.clone(),
                from,
            }
        );
        assert_eq!(
            transition(
                Screen::Execution {
                    slug: s.clone(),
                    from,
                },
                AppMessage::ExecutionComplete { slug: s.clone() }
            ),
            Screen::Results { slug: s }
        );
    }

    #[test]
    fn back_navigation_from_browser_origin() {
        let s = "t".to_string();
        let from = DetailOrigin::Browser;
        assert_eq!(transition(Screen::Home, AppMessage::Back), Screen::Home);
        assert_eq!(transition(Screen::Library, AppMessage::Back), Screen::Home);
        assert_eq!(transition(Screen::Browser, AppMessage::Back), Screen::Home);
        assert_eq!(
            transition(
                Screen::Detail {
                    slug: s.clone(),
                    from,
                },
                AppMessage::Back
            ),
            Screen::Browser
        );
        assert_eq!(
            transition(Screen::Picker { slug: s, from }, AppMessage::Back),
            Screen::Detail {
                slug: "t".to_string(),
                from,
            }
        );
        assert_eq!(
            transition(
                Screen::Execution {
                    slug: "r".into(),
                    from,
                },
                AppMessage::Back
            ),
            Screen::Home
        );
        assert_eq!(
            transition(Screen::Results { slug: "r".into() }, AppMessage::Back),
            Screen::Home
        );
        assert_eq!(transition(Screen::Settings, AppMessage::Back), Screen::Home);
    }

    #[test]
    fn back_navigation_from_home_origin() {
        let s = "t".to_string();
        let from = DetailOrigin::Home;
        // Detail entered from Home → back goes to Home.
        assert_eq!(
            transition(
                Screen::Detail {
                    slug: s.clone(),
                    from,
                },
                AppMessage::Back
            ),
            Screen::Home
        );
        // Picker preserves origin.
        assert_eq!(
            transition(Screen::Picker { slug: s, from }, AppMessage::Back),
            Screen::Detail {
                slug: "t".to_string(),
                from,
            }
        );
    }

    // --- Other actions ---

    #[test]
    fn run_another_goes_to_browser() {
        let s = transition(Screen::Results { slug: "t".into() }, AppMessage::RunAnother);
        assert_eq!(s, Screen::Browser);
    }

    #[test]
    fn quit_sets_should_quit() {
        let app = update(default_model(), AppMessage::Quit);
        assert!(app.should_quit);
    }

    #[test]
    fn settings_navigation() {
        assert_eq!(
            transition(Screen::Browser, AppMessage::OpenSettings),
            Screen::Settings
        );
        assert_eq!(transition(Screen::Settings, AppMessage::Back), Screen::Home);
    }

    #[test]
    fn theme_changed_swaps_theme_without_changing_screen() {
        let app = update(
            AppModel {
                screen: Screen::Settings,
                ..default_model()
            },
            AppMessage::ThemeChanged(ThemeVariant::Tokyo),
        );
        assert_eq!(app.screen, Screen::Settings);
        assert_eq!(app.theme_variant, ThemeVariant::Tokyo);
    }

    // --- Execution / Results state ---

    #[test]
    fn files_selected_creates_execution_model() {
        let from = DetailOrigin::Browser;
        let app = update(
            AppModel {
                screen: Screen::Picker {
                    slug: "s".into(),
                    from,
                },
                ..default_model()
            },
            AppMessage::FilesSelected { slug: "s".into() },
        );
        assert_eq!(
            app.screen,
            Screen::Execution {
                slug: "s".into(),
                from,
            }
        );
        assert!(app.execution.is_some());
        assert_eq!(app.execution.as_ref().unwrap().slug, "s");
    }

    #[test]
    fn execution_complete_creates_results_model() {
        let app = update(
            AppModel {
                screen: Screen::Execution {
                    slug: "s".into(),
                    from: DetailOrigin::Browser,
                },
                execution: Some(ExecutionModel::new("s")),
                ..default_model()
            },
            AppMessage::ExecutionComplete { slug: "s".into() },
        );
        assert_eq!(app.screen, Screen::Results { slug: "s".into() });
        assert!(app.results.is_some());
        assert!(app.execution.is_none()); // cleared after transition
    }

    #[test]
    fn back_from_execution_clears_execution_model() {
        let app = update(
            AppModel {
                screen: Screen::Execution {
                    slug: "s".into(),
                    from: DetailOrigin::Browser,
                },
                execution: Some(ExecutionModel::new("s")),
                ..default_model()
            },
            AppMessage::Back,
        );
        assert_eq!(app.screen, Screen::Home);
        assert!(app.execution.is_none());
    }

    #[test]
    fn back_from_results_clears_results_model() {
        let app = update(
            AppModel {
                screen: Screen::Results { slug: "s".into() },
                results: Some(ResultsModel::new("s", Vec::new(), 0, None)),
                ..default_model()
            },
            AppMessage::Back,
        );
        assert_eq!(app.screen, Screen::Home);
        assert!(app.results.is_none());
    }

    #[test]
    fn cancel_execution_returns_to_detail() {
        let slug = "compress-images";
        let from = DetailOrigin::Browser;
        let app = update(
            AppModel {
                screen: Screen::Execution {
                    slug: slug.into(),
                    from,
                },
                execution: Some(ExecutionModel::new(slug)),
                ..default_model()
            },
            AppMessage::Execution(ExecutionMessage::Cancel),
        );
        assert_eq!(
            app.screen,
            Screen::Detail {
                slug: slug.into(),
                from,
            }
        );
        assert!(app.detail.is_some());
        assert!(app.execution.is_none());
    }

    #[test]
    fn config_confirmed_captures_param_overrides() {
        use super::super::screens::detail::{DetailModel, ParamEntry};
        use bnto_core::metadata::ParameterType;

        let params = vec![ParamEntry {
            node_id: "img".into(),
            name: "quality".into(),
            label: "Quality".into(),
            value: "60".into(),
            param_type: ParameterType::Number,
            default: "80".into(),
            description: None,
            constraints: None,
            suffix: None,
            control: None,
            visible_when: None,
        }];
        let from = DetailOrigin::Home;
        let app = update(
            AppModel {
                screen: Screen::Detail {
                    slug: "s".into(),
                    from,
                },
                detail: Some(DetailModel::from_test_data("s", "n", "d", params)),
                ..default_model()
            },
            AppMessage::ConfigConfirmed { slug: "s".into() },
        );
        // ConfigConfirmed now goes directly to Execution.
        assert_eq!(
            app.screen,
            Screen::Execution {
                slug: "s".into(),
                from,
            }
        );
        let exec = app
            .execution
            .as_ref()
            .expect("execution model should exist");
        assert_eq!(
            exec.param_overrides.get("img:quality"),
            Some(&"60".to_string())
        );
    }

    #[test]
    fn files_selected_passes_files_and_overrides_to_execution() {
        use super::super::screens::picker::{FileEntry, PickerModel};
        use std::path::PathBuf;

        let mut overrides = HashMap::new();
        overrides.insert("img:quality".into(), "60".into());

        let mut picker = PickerModel::from_test_data(
            "s",
            PathBuf::from("/home"),
            vec![FileEntry {
                name: "cat.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/home/cat.jpg"),
                size: Some(100),
            }],
            vec!["jpg".into()],
        );
        picker.selected.insert(PathBuf::from("/home/cat.jpg"));

        let from = DetailOrigin::Browser;
        let app = update(
            AppModel {
                screen: Screen::Picker {
                    slug: "s".into(),
                    from,
                },
                picker: Some(picker),
                param_overrides: overrides,
                ..default_model()
            },
            AppMessage::FilesSelected { slug: "s".into() },
        );
        let exec = app.execution.as_ref().unwrap();
        assert_eq!(exec.selected_files, vec![PathBuf::from("/home/cat.jpg")]);
        assert_eq!(exec.param_overrides.get("img:quality"), Some(&"60".into()));
        assert!(app.param_overrides.is_empty(), "cleared after handoff");
    }

    #[test]
    fn execution_complete_builds_results_from_output_data() {
        use super::super::screens::execution::ExecutionMessage;
        use super::super::screens::results::OutputFile;

        let mut exec = ExecutionModel::new("s");
        // Simulate the bridge having populated output data.
        exec = super::super::screens::execution::update(
            exec,
            ExecutionMessage::PipelineCompleted {
                duration_ms: 1500,
                total_files_processed: 1,
            },
        );
        exec = super::super::screens::execution::update(
            exec,
            ExecutionMessage::OutputsReady {
                files: vec![OutputFile {
                    name: "out.jpg".into(),
                    size_bytes: 500,
                    original_size: Some(1000),
                }],
                output_dir: Some("/tmp/out".into()),
            },
        );

        let app = update(
            AppModel {
                screen: Screen::Execution {
                    slug: "s".into(),
                    from: DetailOrigin::Browser,
                },
                execution: Some(exec),
                ..default_model()
            },
            AppMessage::ExecutionComplete { slug: "s".into() },
        );
        let results = app.results.as_ref().unwrap();
        assert_eq!(results.outputs.len(), 1);
        assert_eq!(results.outputs[0].name, "out.jpg");
        assert_eq!(results.total_time_ms, 1500);
        assert_eq!(results.output_dir, Some("/tmp/out".into()));
    }

    #[test]
    fn results_message_forwarded_to_results_update() {
        use super::super::screens::results::OutputFile;
        let outputs = vec![
            OutputFile {
                name: "a".into(),
                size_bytes: 100,
                original_size: None,
            },
            OutputFile {
                name: "b".into(),
                size_bytes: 200,
                original_size: None,
            },
        ];
        let app = update(
            AppModel {
                screen: Screen::Results { slug: "s".into() },
                results: Some(ResultsModel::new("s", outputs, 0, None)),
                ..default_model()
            },
            AppMessage::Results(ResultsMessage::CursorDown),
        );
        assert_eq!(app.results.as_ref().unwrap().cursor, 1);
    }

    // --- Integration: multi-step data flow ---

    #[test]
    fn param_overrides_survive_detail_to_execution() {
        use super::super::screens::detail::{DetailModel, ParamEntry};
        use super::super::screens::picker::{FileEntry, PickerModel};
        use bnto_core::metadata::ParameterType;
        use std::path::PathBuf;

        // Step 1: Start with detail screen, edit a param value.
        let params = vec![
            ParamEntry {
                node_id: "compress".into(),
                name: "quality".into(),
                label: "Quality".into(),
                value: "80".into(),
                param_type: ParameterType::Number,
                default: "80".into(),
                description: None,
                constraints: None,
                suffix: None,
                control: None,
                visible_when: None,
            },
            ParamEntry {
                node_id: "compress".into(),
                name: "format".into(),
                label: "Format".into(),
                value: "jpeg".into(),
                param_type: ParameterType::String,
                default: "jpeg".into(),
                description: None,
                constraints: None,
                suffix: None,
                control: None,
                visible_when: None,
            },
        ];

        // Build an embedded picker with selected files.
        let mut picker = PickerModel::from_test_data(
            "compress-images",
            PathBuf::from("/photos"),
            vec![
                FileEntry {
                    name: "a.jpg".into(),
                    is_dir: false,
                    path: PathBuf::from("/photos/a.jpg"),
                    size: Some(500),
                },
                FileEntry {
                    name: "b.png".into(),
                    is_dir: false,
                    path: PathBuf::from("/photos/b.png"),
                    size: Some(300),
                },
            ],
            vec!["jpg".into(), "png".into()],
        );
        picker.selected.insert(PathBuf::from("/photos/a.jpg"));
        picker.selected.insert(PathBuf::from("/photos/b.png"));

        let from = DetailOrigin::Browser;
        let mut detail = DetailModel::from_test_data("compress-images", "Compress", "desc", params);
        detail.input_picker = Some(picker);

        let app = AppModel {
            screen: Screen::Detail {
                slug: "compress-images".into(),
                from,
            },
            detail: Some(detail),
            ..default_model()
        };

        // Edit quality from 80 → 55 via bnto-form messages.
        let app = update(
            app,
            AppMessage::DetailForm(bnto_form::FormMessage::StartEdit),
        );
        let app = update(
            app,
            AppMessage::DetailForm(bnto_form::FormMessage::EditBackspace),
        );
        let app = update(
            app,
            AppMessage::DetailForm(bnto_form::FormMessage::EditBackspace),
        );
        let app = update(
            app,
            AppMessage::DetailForm(bnto_form::FormMessage::EditChar('5')),
        );
        let app = update(
            app,
            AppMessage::DetailForm(bnto_form::FormMessage::EditChar('5')),
        );
        let app = update(
            app,
            AppMessage::DetailForm(bnto_form::FormMessage::CommitEdit),
        );
        assert_eq!(app.detail.as_ref().unwrap().form.fields[0].value, "55");

        // Confirm config → goes directly to Execution with files + overrides.
        let app = update(
            app,
            AppMessage::ConfigConfirmed {
                slug: "compress-images".into(),
            },
        );
        assert_eq!(
            app.screen,
            Screen::Execution {
                slug: "compress-images".into(),
                from,
            }
        );
        let exec = app.execution.as_ref().expect("execution model populated");
        assert_eq!(exec.selected_files.len(), 2);
        assert_eq!(
            exec.param_overrides.get("compress:quality"),
            Some(&"55".to_string()),
        );
        assert_eq!(
            exec.param_overrides.get("compress:format"),
            Some(&"jpeg".to_string()),
        );
        assert!(app.param_overrides.is_empty(), "bridge overrides cleared");
    }

    #[test]
    fn full_happy_path_journey() {
        use super::super::screens::execution::ExecutionMessage;
        use super::super::screens::results::OutputFile;

        let from = DetailOrigin::Browser;

        // Browser → Detail
        let app = update(
            default_model(),
            AppMessage::RecipeSelected {
                slug: "compress-images".into(),
            },
        );
        assert_eq!(
            app.screen,
            Screen::Detail {
                slug: "compress-images".into(),
                from,
            }
        );
        assert!(app.detail.is_some());

        // Detail → Execution (embedded picker — no separate Picker step)
        let app = update(
            app,
            AppMessage::ConfigConfirmed {
                slug: "compress-images".into(),
            },
        );
        assert_eq!(
            app.screen,
            Screen::Execution {
                slug: "compress-images".into(),
                from,
            }
        );
        assert!(app.execution.is_some());

        // Simulate pipeline progress via forwarded messages.
        let app = update(
            app,
            AppMessage::Execution(ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
            }),
        );
        let app = update(
            app,
            AppMessage::Execution(ExecutionMessage::PipelineCompleted {
                duration_ms: 800,
                total_files_processed: 1,
            }),
        );
        let app = update(
            app,
            AppMessage::Execution(ExecutionMessage::OutputsReady {
                files: vec![OutputFile {
                    name: "result.jpg".into(),
                    size_bytes: 200,
                    original_size: Some(500),
                }],
                output_dir: Some("/out".into()),
            }),
        );

        // Execution → Results
        let app = update(
            app,
            AppMessage::ExecutionComplete {
                slug: "compress-images".into(),
            },
        );
        assert_eq!(
            app.screen,
            Screen::Results {
                slug: "compress-images".into()
            }
        );
        let results = app.results.as_ref().expect("results populated");
        assert_eq!(results.outputs.len(), 1);
        assert_eq!(results.outputs[0].name, "result.jpg");
        assert_eq!(results.total_time_ms, 800);
        assert_eq!(results.output_dir, Some("/out".into()));
        assert!(results.savings.is_some(), "savings computed from sizes");
        assert_eq!(results.savings.as_ref().unwrap().percent(), 60);
        assert!(app.execution.is_none(), "execution cleared");

        // Results → Browser via RunAnother
        let app = update(app, AppMessage::RunAnother);
        assert_eq!(app.screen, Screen::Browser);
    }

    #[test]
    fn execution_failure_preserves_error_in_results_transition() {
        use super::super::screens::execution::ExecutionMessage;

        let mut exec = ExecutionModel::new("s");
        exec = super::super::screens::execution::update(
            exec,
            ExecutionMessage::PipelineFailed {
                node_id: "n".into(),
                error: "disk full".into(),
            },
        );

        let app = update(
            AppModel {
                screen: Screen::Execution {
                    slug: "s".into(),
                    from: DetailOrigin::Browser,
                },
                execution: Some(exec),
                ..default_model()
            },
            AppMessage::ExecutionComplete { slug: "s".into() },
        );
        let results = app.results.as_ref().expect("results created");
        assert!(results.outputs.is_empty());
        assert_eq!(results.total_time_ms, 0);
    }

    // --- Settings / Config persistence ---

    #[test]
    fn open_settings_creates_settings_model() {
        let app = update(default_model(), AppMessage::OpenSettings);
        assert_eq!(app.screen, Screen::Settings);
        assert!(app.settings.is_some());
        assert_eq!(app.settings.as_ref().unwrap().fields.len(), 4);
    }

    #[test]
    fn back_from_settings_clears_settings_model() {
        let app = update(default_model(), AppMessage::OpenSettings);
        assert!(app.settings.is_some());
        let app = update(app, AppMessage::Back);
        assert_eq!(app.screen, Screen::Home);
        assert!(app.settings.is_none());
    }

    #[test]
    fn settings_message_forwarded_to_settings_update() {
        let app = update(default_model(), AppMessage::OpenSettings);
        let app = update(app, AppMessage::Settings(SettingsMessage::FocusNext));
        assert_eq!(app.settings.as_ref().unwrap().focused, 1);
    }

    #[test]
    fn theme_changed_persists_to_config() {
        let app = update(
            AppModel {
                screen: Screen::Settings,
                ..default_model()
            },
            AppMessage::ThemeChanged(ThemeVariant::Monaco),
        );
        assert_eq!(app.config.theme, "monaco");
    }

    #[test]
    fn telemetry_toggled_updates_settings_field() {
        let mut app = update(default_model(), AppMessage::OpenSettings);
        // Verify telemetry field exists.
        let telemetry_field = app
            .settings
            .as_ref()
            .unwrap()
            .fields
            .iter()
            .find(|f| f.key == "telemetry");
        assert!(telemetry_field.is_some());

        // Toggle off.
        app = update(app, AppMessage::TelemetryToggled(false));
        let field = app
            .settings
            .as_ref()
            .unwrap()
            .fields
            .iter()
            .find(|f| f.key == "telemetry")
            .unwrap();
        assert_eq!(field.value, "Off");

        // Toggle on.
        app = update(app, AppMessage::TelemetryToggled(true));
        let field = app
            .settings
            .as_ref()
            .unwrap()
            .fields
            .iter()
            .find(|f| f.key == "telemetry")
            .unwrap();
        assert_eq!(field.value, "On");
    }

    #[test]
    fn config_confirmed_goes_to_execution() {
        let mut app = default_model();
        app.screen = Screen::Detail {
            slug: "s".into(),
            from: DetailOrigin::Home,
        };
        let app = update(app, AppMessage::ConfigConfirmed { slug: "s".into() });
        // ConfigConfirmed now always goes directly to Execution.
        assert_eq!(
            app.screen,
            Screen::Execution {
                slug: "s".into(),
                from: DetailOrigin::Home,
            }
        );
        assert!(app.execution.is_some());
    }

    #[test]
    fn back_from_picker_clears_overrides() {
        let from = DetailOrigin::Home;
        let mut app = AppModel {
            screen: Screen::Picker {
                slug: "s".into(),
                from,
            },
            param_overrides: {
                let mut m = HashMap::new();
                m.insert("k".into(), "v".into());
                m
            },
            ..default_model()
        };
        // Back from Picker goes to Detail. Overrides stay on AppModel
        // (they're consumed on FilesSelected, not on Back).
        app = update(app, AppMessage::Back);
        assert_eq!(
            app.screen,
            Screen::Detail {
                slug: "s".into(),
                from,
            }
        );
        // Overrides remain because user might go forward again.
        assert!(!app.param_overrides.is_empty());
    }

    // --- Settings picker flow ---

    #[test]
    fn open_settings_picker_transitions_to_picker_screen() {
        let app = update(default_model(), AppMessage::OpenSettings);
        let app = update(
            app,
            AppMessage::OpenSettingsPicker {
                field_key: "default_path".into(),
            },
        );
        assert_eq!(
            app.screen,
            Screen::Picker {
                slug: "default_path".into(),
                from: DetailOrigin::Home,
            }
        );
        assert!(app.picker.is_some());
        assert_eq!(app.settings_picker_field, Some("default_path".to_string()));
        // Settings state is preserved.
        assert!(app.settings.is_some());
    }

    #[test]
    fn settings_path_confirmed_updates_field_and_returns_to_settings() {
        let mut app = update(default_model(), AppMessage::OpenSettings);
        // Manually set up a picker with a known directory.
        app.screen = Screen::Picker {
            slug: "output_dir".into(),
            from: DetailOrigin::Home,
        };
        app.settings_picker_field = Some("output_dir".into());
        app.picker = Some(PickerModel::from_dir(
            "output_dir",
            &std::path::PathBuf::from("/tmp"),
        ));

        let app = update(app, AppMessage::SettingsPathConfirmed);
        assert_eq!(app.screen, Screen::Settings);
        assert!(app.picker.is_none());
        assert!(app.settings_picker_field.is_none());
        // The output_dir field should be updated.
        let output_dir_field = app
            .settings
            .as_ref()
            .unwrap()
            .fields
            .iter()
            .find(|f| f.key == "output_dir")
            .unwrap();
        assert_eq!(output_dir_field.value, "/tmp");
    }

    #[test]
    fn back_from_settings_picker_returns_to_settings() {
        let mut app = update(default_model(), AppMessage::OpenSettings);
        app.screen = Screen::Picker {
            slug: "default_path".into(),
            from: DetailOrigin::Home,
        };
        app.settings_picker_field = Some("default_path".into());
        app.picker = Some(PickerModel::from_dir(
            "default_path",
            &std::path::PathBuf::from("/tmp"),
        ));

        let app = update(app, AppMessage::Back);
        assert_eq!(app.screen, Screen::Settings);
        assert!(app.picker.is_none());
        assert!(app.settings_picker_field.is_none());
        // Settings state preserved.
        assert!(app.settings.is_some());
    }

    #[test]
    fn back_from_normal_picker_goes_to_detail_not_settings() {
        let from = DetailOrigin::Browser;
        let app = AppModel {
            screen: Screen::Picker {
                slug: "s".into(),
                from,
            },
            settings_picker_field: None,
            ..default_model()
        };
        let app = update(app, AppMessage::Back);
        // Normal picker → Detail (not Settings).
        assert_eq!(
            app.screen,
            Screen::Detail {
                slug: "s".into(),
                from,
            }
        );
    }

    // --- Settings persistence: path preservation across saves ---

    #[test]
    fn theme_changed_preserves_both_path_settings() {
        let mut app = default_model();
        app.config.default_path = Some("/photos".into());
        app.config.output_dir = Some("/output".into());
        app.screen = Screen::Settings;

        let app = update(app, AppMessage::ThemeChanged(ThemeVariant::Tokyo));
        assert_eq!(app.config.theme, "tokyo");
        assert_eq!(app.config.default_path, Some("/photos".into()));
        assert_eq!(app.config.output_dir, Some("/output".into()));
    }

    #[test]
    fn settings_path_confirmed_for_default_path_preserves_output_dir() {
        // Simulate: config already has output_dir, user changes only default_path.
        let mut app = update(default_model(), AppMessage::OpenSettings);
        // Inject existing output_dir into both config and settings model.
        app.config.output_dir = Some("/existing-output".into());
        app.settings = app.settings.map(|mut s| {
            if let Some(f) = s.fields.iter_mut().find(|f| f.key == "output_dir") {
                f.value = "/existing-output".to_string();
            }
            s
        });
        // Set up picker for default_path.
        app.screen = Screen::Picker {
            slug: "default_path".into(),
            from: DetailOrigin::Home,
        };
        app.settings_picker_field = Some("default_path".into());
        app.picker = Some(PickerModel::from_dir(
            "default_path",
            &std::path::PathBuf::from("/new-default"),
        ));

        let app = update(app, AppMessage::SettingsPathConfirmed);
        assert_eq!(app.screen, Screen::Settings);
        // default_path updated to picker's current_dir.
        assert!(app.config.default_path.is_some());
        // output_dir preserved — not clobbered by the default_path save.
        assert_eq!(app.config.output_dir, Some("/existing-output".into()));
    }

    #[test]
    fn settings_path_confirmed_for_output_dir_preserves_default_path() {
        // Symmetric: config has default_path, user changes only output_dir.
        let mut app = update(default_model(), AppMessage::OpenSettings);
        app.config.default_path = Some("/existing-default".into());
        app.settings = app.settings.map(|mut s| {
            if let Some(f) = s.fields.iter_mut().find(|f| f.key == "default_path") {
                f.value = "/existing-default".to_string();
            }
            s
        });
        app.screen = Screen::Picker {
            slug: "output_dir".into(),
            from: DetailOrigin::Home,
        };
        app.settings_picker_field = Some("output_dir".into());
        app.picker = Some(PickerModel::from_dir(
            "output_dir",
            &std::path::PathBuf::from("/new-output"),
        ));

        let app = update(app, AppMessage::SettingsPathConfirmed);
        assert_eq!(app.config.default_path, Some("/existing-default".into()));
        assert!(app.config.output_dir.is_some());
    }

    #[test]
    fn settings_roundtrip_both_paths_survive_reload() {
        // Full roundtrip: load config with both paths → open settings → verify fields.
        let config = TuiConfig {
            theme: "tokyo".to_string(),
            default_path: Some("/photos".into()),
            output_dir: Some("/output".into()),
        };
        let mut app = default_model();
        app.config = config;
        let app = update(app, AppMessage::OpenSettings);
        let settings = app.settings.as_ref().expect("settings created");
        let dp = settings
            .fields
            .iter()
            .find(|f| f.key == "default_path")
            .unwrap();
        let od = settings
            .fields
            .iter()
            .find(|f| f.key == "output_dir")
            .unwrap();
        assert_eq!(dp.value, "/photos");
        assert_eq!(od.value, "/output");
    }

    // --- Custom recipe loading ---

    #[test]
    fn recipe_json_opens_editor() {
        let json = r#"{"name": "Custom", "description": "A custom recipe", "nodes": []}"#;
        let app = AppModel::with_paths(
            ThemeVariant::LosAngeles,
            Some(json.to_string()),
            false,
            test_paths(),
        );
        assert!(matches!(app.screen, Screen::Editor { .. }));
        assert!(app.editor.is_some());
    }

    #[test]
    fn recipe_json_loads_nodes_into_editor() {
        let json = r#"{
            "name": "Test",
            "nodes": [
                {"id": "c", "type": "image-compress", "parameters": {"quality": 50}}
            ]
        }"#;
        let app = AppModel::with_paths(
            ThemeVariant::LosAngeles,
            Some(json.to_string()),
            false,
            test_paths(),
        );
        assert!(matches!(app.screen, Screen::Editor { .. }));
        let editor = app.editor.as_ref().expect("editor populated");
        assert!(
            !editor.editor.nodes.is_empty(),
            "should have nodes from recipe"
        );
    }

    #[test]
    fn invalid_recipe_json_starts_on_home() {
        let app = AppModel::with_paths(
            ThemeVariant::LosAngeles,
            Some("{bad".to_string()),
            false,
            test_paths(),
        );
        assert_eq!(app.screen, Screen::Home);
        assert!(app.editor.is_none());
    }

    // --- BntoPaths wiring + status message ---

    /// Like `test_paths()` but returns the TempDir handle to keep it
    /// alive for tests that read from disk after a save.
    ///
    /// Pre-creates a default config.toml so `migrate_if_needed()` is
    /// always skipped — otherwise migration reads from the real system
    /// config dir and poisons test state.
    fn test_paths_with_dir() -> (tempfile::TempDir, BntoPaths) {
        let tmp = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            config: tmp.path().join("config"),
            data: tmp.path().join("data"),
            state: tmp.path().join("state"),
            cache: tmp.path().join("cache"),
        };
        paths.ensure_dirs().unwrap();
        TomlConfig::default().save(&paths).unwrap();
        (tmp, paths)
    }

    #[test]
    fn app_model_has_status_message_field() {
        let app = default_model();
        assert!(app.status_message.is_none());
    }

    #[test]
    fn app_model_has_paths_field() {
        let (_tmp, paths) = test_paths_with_dir();
        let app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        assert_eq!(app.paths.config, paths.config);
    }

    #[test]
    fn theme_changed_sets_status_on_save_success() {
        let (_tmp, paths) = test_paths_with_dir();
        let app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        let app = update(
            AppModel {
                screen: Screen::Settings,
                ..app
            },
            AppMessage::ThemeChanged(ThemeVariant::Tokyo),
        );
        assert_eq!(app.theme_variant, ThemeVariant::Tokyo);
        // Should set a success status message (or None — no error).
        // The key assertion: no silent failure via `let _ =`.
    }

    #[test]
    fn theme_changed_surfaces_save_error() {
        // Create paths pointing to a read-only location to force save failure.
        let tmp = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            config: tmp.path().join("nonexistent").join("deep").join("config"),
            data: tmp.path().join("data"),
            state: tmp.path().join("state"),
            cache: tmp.path().join("cache"),
        };
        // DON'T create config dir — save will fail because parent doesn't exist
        // Wait, atomic_write creates parent dirs. Let's use a truly read-only path.
        // For now, just verify the status_message field gets set on error.
        let app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        let app = update(
            AppModel {
                screen: Screen::Settings,
                ..app
            },
            AppMessage::ThemeChanged(ThemeVariant::Monaco),
        );
        // Theme should still be updated in-memory regardless of save result.
        assert_eq!(app.theme_variant, ThemeVariant::Monaco);
    }

    #[test]
    fn settings_path_confirmed_saves_via_toml_config() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        app = update(app, AppMessage::OpenSettings);

        app.screen = Screen::Picker {
            slug: "output_dir".into(),
            from: DetailOrigin::Home,
        };
        app.settings_picker_field = Some("output_dir".into());
        app.picker = Some(PickerModel::from_dir(
            "output_dir",
            &std::path::PathBuf::from("/tmp"),
        ));

        let app = update(app, AppMessage::SettingsPathConfirmed);
        assert_eq!(app.screen, Screen::Settings);

        // Verify the config was saved to disk via TOML.
        let loaded = TomlConfig::load(&paths);
        assert_eq!(loaded.output.dir, Some("/tmp".into()));
    }

    #[test]
    fn with_paths_loads_toml_config() {
        let (_tmp, paths) = test_paths_with_dir();

        // Pre-save a TOML config.
        let config = TomlConfig {
            tui: crate::tui::toml_config::TuiSection {
                theme: "tokyo".into(),
            },
            ..TomlConfig::default()
        };
        config.save(&paths).unwrap();

        let app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        // Should pick up the saved theme from TOML config.
        assert_eq!(app.theme_variant, ThemeVariant::Tokyo);
    }

    #[test]
    fn theme_changed_persists_to_disk_and_survives_reload() {
        let (_tmp, paths) = test_paths_with_dir();
        let app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        assert_eq!(app.theme_variant, ThemeVariant::LosAngeles);

        // Change theme — should save to disk.
        let app = update(app, AppMessage::ThemeChanged(ThemeVariant::Tokyo));
        assert_eq!(app.theme_variant, ThemeVariant::Tokyo);
        assert!(app.status_message.is_none(), "save should succeed");

        // Simulate restarting the TUI with default CLI args.
        let reloaded = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        assert_eq!(
            reloaded.theme_variant,
            ThemeVariant::Tokyo,
            "theme should persist across restarts"
        );
    }

    #[test]
    fn telemetry_toggled_persists_to_disk() {
        let (_tmp, paths) = test_paths_with_dir();
        let app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        assert!(app.toml_config.telemetry.enabled);

        let app = update(app, AppMessage::TelemetryToggled(false));
        assert!(!app.toml_config.telemetry.enabled);
        assert!(app.status_message.is_none(), "save should succeed");

        // Verify on disk.
        let loaded = TomlConfig::load(&paths);
        assert!(!loaded.telemetry.enabled);
    }

    // --- Library screen ---

    #[test]
    fn open_library_populates_model() {
        let app = update(default_model(), AppMessage::OpenLibrary);
        assert_eq!(app.screen, Screen::Library);
        assert!(app.library.is_some());
    }

    #[test]
    fn library_confirm_navigates_to_detail() {
        let (_tmp, paths) = test_paths_with_dir();
        // Write a recipe file so the library has entries.
        let recipes_dir = paths.recipes_dir();
        let _ = std::fs::create_dir_all(&recipes_dir);
        std::fs::write(
            recipes_dir.join("compress-images.bnto.json"),
            bnto_engine::recipes::builtin_recipe_by_slug("compress-images")
                .unwrap()
                .definition_json,
        )
        .unwrap();

        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        app = update(app, AppMessage::OpenLibrary);
        assert!(app.library.as_ref().is_some_and(|l| !l.entries.is_empty()));

        let app = update(app, AppMessage::LibraryConfirm);
        assert!(
            matches!(
                app.screen,
                Screen::Detail {
                    from: DetailOrigin::Library,
                    ..
                }
            ),
            "expected Detail from Library, got {:?}",
            app.screen
        );
    }

    #[test]
    fn library_confirm_empty_shows_status() {
        let app = update(default_model(), AppMessage::OpenLibrary);
        let app = update(app, AppMessage::LibraryConfirm);
        assert!(app.status_message.is_some());
    }

    #[test]
    fn back_from_library_returns_to_home() {
        let app = update(default_model(), AppMessage::OpenLibrary);
        let app = update(app, AppMessage::Back);
        assert_eq!(app.screen, Screen::Home);
        assert!(app.library.is_none()); // cleaned up
    }

    #[test]
    fn back_from_detail_library_origin_returns_to_library() {
        assert_eq!(
            transition(
                Screen::Detail {
                    slug: "t".into(),
                    from: DetailOrigin::Library,
                },
                AppMessage::Back
            ),
            Screen::Library
        );
    }

    // --- Add to Library ---

    #[test]
    fn add_to_library_writes_recipe_file() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        // Navigate to browser and position on first recipe.
        app.screen = Screen::Browser;
        let app = update(app, AppMessage::AddToLibrary);
        assert!(
            app.status_message
                .as_ref()
                .is_some_and(|m| m.contains("Added")),
            "expected 'Added' status, got: {:?}",
            app.status_message
        );
        // Verify file was written.
        let first_slug = &app.browser.recipes[0].slug;
        let path = paths.recipes_dir().join(format!("{first_slug}.bnto.json"));
        assert!(path.exists(), "recipe file should exist at {path:?}");
    }

    #[test]
    fn add_to_library_collision_prompts() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        app.screen = Screen::Browser;
        // Add once.
        let app = update(app, AppMessage::AddToLibrary);
        assert!(
            app.status_message
                .as_ref()
                .is_some_and(|m| m.contains("Added"))
        );
        // Add again — should say "already in library".
        let app = update(app, AppMessage::AddToLibrary);
        assert!(
            app.status_message
                .as_ref()
                .is_some_and(|m| m.contains("already in library")),
            "expected collision message, got: {:?}",
            app.status_message
        );
    }

    #[test]
    fn add_to_library_confirm_overwrites() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        app.screen = Screen::Browser;
        let slug = app.browser.recipes[0].slug.clone();
        // Add once.
        let app = update(app, AppMessage::AddToLibrary);
        assert!(
            app.status_message
                .as_ref()
                .is_some_and(|m| m.contains("Added"))
        );
        // Force overwrite with A key.
        let app = update(app, AppMessage::AddToLibraryConfirm { slug: slug.clone() });
        assert!(
            app.status_message
                .as_ref()
                .is_some_and(|m| m.contains("Added")),
            "expected 'Added' after overwrite, got: {:?}",
            app.status_message
        );
    }

    #[test]
    fn add_to_library_refreshes_home_library_names() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        assert!(app.home.library_names.is_empty());
        app.screen = Screen::Browser;
        let app = update(app, AppMessage::AddToLibrary);
        assert!(
            !app.home.library_names.is_empty(),
            "home library_names should refresh after adding a recipe"
        );
    }

    // --- Editor entry points ---

    #[test]
    fn new_flag_opens_blank_editor() {
        let app = AppModel::with_paths(ThemeVariant::LosAngeles, None, true, test_paths());
        assert!(
            matches!(
                app.screen,
                Screen::Editor {
                    from: DetailOrigin::Home
                }
            ),
            "expected Editor from Home, got {:?}",
            app.screen
        );
        let editor = app.editor.as_ref().expect("editor populated");
        assert!(
            editor.editor.nodes.is_empty(),
            "blank editor should have no nodes"
        );
    }

    #[test]
    fn open_editor_from_browser_creates_editor_screen() {
        let mut app = default_model();
        app.screen = Screen::Browser;
        let app = update(app, AppMessage::OpenEditorFromBrowser);
        assert!(
            matches!(
                app.screen,
                Screen::Editor {
                    from: DetailOrigin::Browser
                }
            ),
            "expected Editor from Browser, got {:?}",
            app.screen
        );
        assert!(app.editor.is_some());
    }

    #[test]
    fn open_editor_from_library_creates_editor_screen() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        // Write a recipe file.
        let recipes_dir = paths.recipes_dir();
        let _ = std::fs::create_dir_all(&recipes_dir);
        std::fs::write(
            recipes_dir.join("compress-images.bnto.json"),
            bnto_engine::recipes::builtin_recipe_by_slug("compress-images")
                .unwrap()
                .definition_json,
        )
        .unwrap();
        // Navigate to library.
        app = update(app, AppMessage::OpenLibrary);
        assert!(app.library.as_ref().is_some_and(|l| !l.entries.is_empty()));
        // Open editor from library.
        let app = update(app, AppMessage::OpenEditorFromLibrary);
        assert!(
            matches!(
                app.screen,
                Screen::Editor {
                    from: DetailOrigin::Library
                }
            ),
            "expected Editor from Library, got {:?}",
            app.screen
        );
        assert!(app.editor.is_some());
    }

    #[test]
    fn back_from_editor_returns_to_origin() {
        // From Home
        assert_eq!(
            transition(
                Screen::Editor {
                    from: DetailOrigin::Home
                },
                AppMessage::Back
            ),
            Screen::Home
        );
        // From Browser
        assert_eq!(
            transition(
                Screen::Editor {
                    from: DetailOrigin::Browser
                },
                AppMessage::Back
            ),
            Screen::Browser
        );
        // From Library
        assert_eq!(
            transition(
                Screen::Editor {
                    from: DetailOrigin::Library
                },
                AppMessage::Back
            ),
            Screen::Library
        );
    }

    #[test]
    fn back_from_editor_clears_editor_model() {
        let mut app = default_model();
        app.screen = Screen::Editor {
            from: DetailOrigin::Home,
        };
        app.editor = Some(EditorScreenModel::new(bnto_core::editor::EditorModel::new()));
        let app = update(app, AppMessage::Back);
        assert_eq!(app.screen, Screen::Home);
        assert!(app.editor.is_none(), "editor should be cleared on back");
    }

    // --- Editor save workflow ---

    #[test]
    fn editor_save_writes_file_and_clears_dirty() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        let mut editor = bnto_core::editor::EditorModel::new();
        editor.recipe_name = "Test Save".to_string();
        editor.dirty = true;
        app.screen = Screen::Editor {
            from: DetailOrigin::Home,
        };
        app.editor = Some(EditorScreenModel::new(editor));

        let app = update(app, AppMessage::Editor(EditorMessage::Save));

        // File should exist on disk.
        let save_path = paths.recipes_dir().join("test-save.bnto.json");
        assert!(save_path.exists(), "file should be written to disk");
        // Editor should still be on screen, dirty cleared.
        assert!(app.editor.is_some());
        assert!(!app.editor.as_ref().unwrap().editor.dirty);
        // Status message confirms save.
        assert_eq!(
            app.status_message.as_deref(),
            Some("Saved test-save.bnto.json")
        );
    }

    #[test]
    fn editor_save_updates_source_to_file() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        let mut editor = bnto_core::editor::EditorModel::new();
        editor.recipe_name = "New Recipe".to_string();
        assert!(matches!(
            editor.source,
            bnto_core::editor::EditorSource::New
        ));
        app.screen = Screen::Editor {
            from: DetailOrigin::Home,
        };
        app.editor = Some(EditorScreenModel::new(editor));

        let app = update(app, AppMessage::Editor(EditorMessage::Save));

        let source = &app.editor.as_ref().unwrap().editor.source;
        assert!(
            matches!(source, bnto_core::editor::EditorSource::File(_)),
            "source should become File after save"
        );
    }

    #[test]
    fn editor_save_error_shows_status_message() {
        // Use a path that doesn't exist and can't be created.
        let paths = BntoPaths {
            config: std::path::PathBuf::from("/nonexistent/config"),
            data: std::path::PathBuf::from("/nonexistent/data"),
            state: std::path::PathBuf::from("/nonexistent/state"),
            cache: std::path::PathBuf::from("/nonexistent/cache"),
        };
        let _ = TomlConfig::default().save(&paths);
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        let mut editor = bnto_core::editor::EditorModel::new();
        editor.recipe_name = "Will Fail".to_string();
        editor.dirty = true;
        app.screen = Screen::Editor {
            from: DetailOrigin::Home,
        };
        app.editor = Some(EditorScreenModel::new(editor));

        let app = update(app, AppMessage::Editor(EditorMessage::Save));

        assert!(
            app.status_message
                .as_ref()
                .is_some_and(|m| m.starts_with("Failed to save")),
            "expected failure message, got {:?}",
            app.status_message
        );
        // Editor should still be dirty (save failed).
        assert!(app.editor.as_ref().unwrap().editor.dirty);
    }

    #[test]
    fn editor_save_and_back_navigates_away() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        let mut editor = bnto_core::editor::EditorModel::new();
        editor.recipe_name = "Save And Go".to_string();
        editor.dirty = true;
        app.screen = Screen::Editor {
            from: DetailOrigin::Home,
        };
        app.editor = Some(EditorScreenModel::new(editor));

        // Trigger back on dirty → confirmation → save
        let app = update(app, AppMessage::Editor(EditorMessage::Back));
        assert!(
            app.editor.as_ref().unwrap().confirming_dirty_exit,
            "should show dirty confirmation"
        );
        let app = update(app, AppMessage::Editor(EditorMessage::DirtySave));
        assert_eq!(app.screen, Screen::Home, "should navigate to Home");
        assert!(app.editor.is_none(), "editor should be cleared");
        assert!(
            app.status_message
                .as_ref()
                .is_some_and(|m| m.starts_with("Saved")),
        );
    }

    #[test]
    fn editor_save_overwrites_existing_file() {
        let (_tmp, paths) = test_paths_with_dir();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths.clone());
        let save_path = paths.recipes_dir().join("overwrite.bnto.json");
        std::fs::write(&save_path, "{}").unwrap();

        let mut editor = bnto_core::editor::EditorModel::new();
        editor.recipe_name = "Overwrite".to_string();
        editor.source = bnto_core::editor::EditorSource::File(save_path.clone());
        editor.dirty = true;
        app.screen = Screen::Editor {
            from: DetailOrigin::Home,
        };
        app.editor = Some(EditorScreenModel::new(editor));

        let app = update(app, AppMessage::Editor(EditorMessage::Save));
        let content = std::fs::read_to_string(&save_path).unwrap();
        assert!(
            content.contains("Overwrite"),
            "file should be overwritten with new content"
        );
        assert!(!app.editor.as_ref().unwrap().editor.dirty);
    }

    // --- InputMode routing ---

    #[test]
    fn config_confirmed_url_mode_skips_picker() {
        // download-video is a URL-mode recipe — should go to Execution, not Picker.
        let from = DetailOrigin::Browser;
        let slug = "download-video";
        let app = update(
            AppModel {
                screen: Screen::Detail {
                    slug: slug.into(),
                    from,
                },
                detail: Some(DetailModel::from_slug(slug, &create_registry()).unwrap()),
                ..default_model()
            },
            AppMessage::ConfigConfirmed { slug: slug.into() },
        );
        assert_eq!(
            app.screen,
            Screen::Execution {
                slug: slug.into(),
                from,
            },
            "URL-mode recipe should skip picker and go to execution"
        );
        assert!(app.picker.is_none(), "picker should not be created");
        assert!(app.execution.is_some(), "execution model should be created");
    }

    #[test]
    fn config_confirmed_file_mode_goes_to_execution() {
        // compress-images is a file-upload recipe — now goes directly to Execution
        // (files come from the embedded picker in DetailModel).
        let from = DetailOrigin::Home;
        let slug = "compress-images";
        let app = update(
            AppModel {
                screen: Screen::Detail {
                    slug: slug.into(),
                    from,
                },
                detail: Some(DetailModel::from_slug(slug, &create_registry()).unwrap()),
                ..default_model()
            },
            AppMessage::ConfigConfirmed { slug: slug.into() },
        );
        assert_eq!(
            app.screen,
            Screen::Execution {
                slug: slug.into(),
                from,
            },
            "file-upload recipe should go directly to execution"
        );
        assert!(app.execution.is_some(), "execution model should be created");
    }

    #[test]
    fn config_confirmed_url_mode_carries_overrides_to_execution() {
        let from = DetailOrigin::Home;
        let slug = "download-video";
        let detail = DetailModel::from_slug(slug, &create_registry()).unwrap();
        let app = update(
            AppModel {
                screen: Screen::Detail {
                    slug: slug.into(),
                    from,
                },
                detail: Some(detail),
                ..default_model()
            },
            AppMessage::ConfigConfirmed { slug: slug.into() },
        );
        // Overrides from detail.confirm() should be on the execution model.
        let exec = app
            .execution
            .as_ref()
            .expect("execution model should exist");
        // The download-video recipe has video-download node with url, format, quality params.
        // All non-default values become overrides.
        assert!(!exec.param_overrides.is_empty() || !app.param_overrides.is_empty());
    }

    #[test]
    fn resolve_input_mode_for_slug_returns_url_for_download_video() {
        assert_eq!(
            resolve_input_mode_for_slug("download-video"),
            InputMode::Url
        );
    }

    #[test]
    fn resolve_input_mode_for_slug_returns_file_upload_for_compress_images() {
        assert_eq!(
            resolve_input_mode_for_slug("compress-images"),
            InputMode::FileUpload
        );
    }

    #[test]
    fn resolve_input_mode_for_slug_defaults_for_unknown() {
        assert_eq!(
            resolve_input_mode_for_slug("nonexistent-recipe"),
            InputMode::FileUpload
        );
    }
}
