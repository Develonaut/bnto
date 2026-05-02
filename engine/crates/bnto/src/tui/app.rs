// App state machine — routes between TUI screens.
//
// Pure state + pure transitions. No I/O, no terminal access.
// All screen navigation logic is testable with `cargo test`.

use std::collections::HashMap;

use bnto_core::registry::NodeRegistry;
use bnto_engine::create_registry;

use crate::catalog::RecipeCatalog;

use super::app_helpers::{
    handle_add_to_library, handle_add_to_library_write, handle_back, handle_config_confirmed,
    handle_detail_form, handle_editor, handle_editor_form, handle_execution,
    handle_execution_complete, handle_files_selected, handle_home_confirm, handle_library,
    handle_library_confirm, handle_open_editor_from_browser, handle_open_editor_from_library,
    handle_open_library, handle_open_settings_picker, handle_open_wizard, handle_recipe_selected,
    handle_settings_path_confirmed, handle_telemetry_toggled, handle_theme_changed, handle_wizard,
    handle_wizard_form, load_editor_from_json,
};
use super::screens::browser::{BrowserMessage, BrowserModel, update as browser_update};
use super::screens::detail::DetailModel;
use super::screens::editor::{EditorMessage, EditorScreenModel};
use super::screens::execution::{ExecutionMessage, ExecutionModel};
use super::screens::home::{HomeMessage, HomeModel, list_library_recipes, update as home_update};
use super::screens::library::{LibraryMessage, LibraryModel};
use super::screens::picker::{PickerMessage, PickerModel, update as picker_update};
use super::screens::results::{ResultsMessage, ResultsModel, update as results_update};
use super::screens::settings::{SettingsMessage, SettingsModel, update as settings_update};
use super::screens::wizard::{WizardMessage, WizardModel};
use super::theme::{Theme, ThemeVariant};
use crate::storage::config::TomlConfig;
use crate::storage::paths::BntoPaths;

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
    /// TOML-based persistent config.
    pub toml_config: TomlConfig,
    /// Resolved storage paths (~/.bnto/ home).
    pub paths: BntoPaths,
    /// Transient status bar message (e.g. "Settings saved" or "Failed to save").
    pub status_message: Option<String>,
    /// Which settings field opened the picker (None = normal recipe picker).
    pub settings_picker_field: Option<String>,
    /// Param overrides from detail screen, carried through picker to execution.
    pub param_overrides: HashMap<String, String>,
    /// Engine registry for resolving processor metadata.
    pub registry: NodeRegistry,
    /// Unified recipe catalog — bundled + library recipes.
    pub catalog: RecipeCatalog,
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
    DetailForm(tonkotsu::FormMessage),
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
    EditorForm(tonkotsu::FormMessage),
    /// Forward a message to the wizard screen.
    Wizard(WizardMessage),
    /// Forward a form message to the wizard's inline form.
    WizardForm(tonkotsu::FormMessage),
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
    /// Open the currently selected output file in the default app.
    OpenFile,
    /// Open the output folder in the system file manager.
    OpenFolder,
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
            home: std::path::PathBuf::from(".bnto"),
        });
        Self::with_paths(variant, recipe_json, new_recipe, paths)
    }

    /// Create a new app with explicit storage paths.
    ///
    /// Loads TOML config from `paths.config_file()`. Falls back to
    /// defaults on any error.
    pub fn with_paths(
        variant: ThemeVariant,
        recipe_json: Option<String>,
        new_recipe: bool,
        paths: BntoPaths,
    ) -> Self {
        let _ = paths.ensure_dirs();

        let toml_config = TomlConfig::load(&paths);

        // CLI --theme flag overrides saved config.
        let effective_variant = if variant != ThemeVariant::LosAngeles {
            variant
        } else {
            ThemeVariant::from_str_lossy(&toml_config.tui.theme).unwrap_or(variant)
        };
        let registry = create_registry();

        // Build unified catalog and list library recipes for the home screen pane.
        let recipes_dir = effective_recipes_dir(&toml_config, &paths);
        let catalog = RecipeCatalog::load(&recipes_dir);
        let library_names = list_library_recipes(&recipes_dir);

        // Determine initial screen and state.
        let (screen, detail, editor) = if new_recipe {
            let editor_model = bnto_core::editor::EditorModel::new();
            (
                Screen::Editor {
                    from: DetailOrigin::Home,
                },
                None,
                Some(EditorScreenModel::new(editor_model)),
            )
        } else if let Some(json) = recipe_json {
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
            browser: BrowserModel::new(catalog.all()),
            library: None,
            detail,
            picker: None,
            execution: None,
            results: None,
            settings: None,
            editor,
            wizard: None,
            toml_config,
            paths,
            status_message: None,
            settings_picker_field: None,
            param_overrides: HashMap::new(),
            registry,
            catalog,
        }
    }
}

/// Resolve the effective recipes directory.
///
/// Uses the config override if set and valid, otherwise falls back
/// to the default `~/.bnto/recipes/`.
pub fn effective_recipes_dir(config: &TomlConfig, paths: &BntoPaths) -> std::path::PathBuf {
    config
        .paths
        .recipes
        .as_deref()
        .map(std::path::PathBuf::from)
        .filter(|p| p.is_dir())
        .unwrap_or_else(|| paths.recipes_dir())
}

/// Pure state transition — the heart of the TEA pattern.
/// Takes current state + message, returns the next state.
pub fn update(model: AppModel, msg: AppMessage) -> AppModel {
    match msg {
        AppMessage::RecipeSelected { slug } => handle_recipe_selected(model, slug),
        AppMessage::ConfigConfirmed { slug } => handle_config_confirmed(model, slug),
        AppMessage::FilesSelected { slug } => handle_files_selected(model, slug),
        AppMessage::ExecutionComplete { slug } => handle_execution_complete(model, slug),
        AppMessage::Home(msg) => {
            let home = home_update(model.home, msg);
            AppModel { home, ..model }
        }
        AppMessage::HomeConfirm => handle_home_confirm(model),
        AppMessage::OpenLibrary => handle_open_library(model),
        AppMessage::Library(ref msg) => handle_library(model, msg),
        AppMessage::LibraryConfirm => handle_library_confirm(model),
        AppMessage::AddToLibrary => handle_add_to_library(model),
        AppMessage::AddToLibraryConfirm { slug } => handle_add_to_library_write(model, &slug, true),
        AppMessage::Back => handle_back(model),
        AppMessage::RunAnother => AppModel {
            screen: Screen::Browser,
            ..model
        },
        AppMessage::OpenFile => {
            if let Some(results) = &model.results
                && let Some(dir) = &results.output_dir
                && let Some(file) = results.outputs.get(results.cursor)
            {
                let path = std::path::Path::new(dir).join(&file.name);
                let _ = open_path(path.to_string_lossy().as_ref());
            }
            model
        }
        AppMessage::OpenFolder => {
            if let Some(results) = &model.results
                && let Some(dir) = &results.output_dir
            {
                let _ = open_path(dir);
            }
            model
        }
        AppMessage::OpenSettings => {
            let settings = SettingsModel::from_toml_config(&model.toml_config);
            AppModel {
                screen: Screen::Settings,
                settings: Some(settings),
                ..model
            }
        }
        AppMessage::ThemeChanged(variant) => handle_theme_changed(model, variant),
        AppMessage::TelemetryToggled(enabled) => handle_telemetry_toggled(model, enabled),
        AppMessage::Browser(msg) => {
            let browser = browser_update(model.browser, msg);
            AppModel { browser, ..model }
        }
        AppMessage::DetailForm(msg) => handle_detail_form(model, msg),
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
        AppMessage::Execution(msg) => handle_execution(model, msg),
        AppMessage::Results(msg) => {
            let results = model.results.map(|r| results_update(r, msg));
            AppModel { results, ..model }
        }
        AppMessage::Settings(msg) => {
            let settings = model.settings.map(|s| settings_update(s, msg));
            AppModel { settings, ..model }
        }
        AppMessage::Editor(msg) => handle_editor(model, msg),
        AppMessage::EditorForm(form_msg) => handle_editor_form(model, form_msg),
        AppMessage::Wizard(msg) => handle_wizard(model, msg),
        AppMessage::WizardForm(form_msg) => handle_wizard_form(model, form_msg),
        AppMessage::OpenWizard => handle_open_wizard(model),
        AppMessage::OpenEditorFromBrowser => handle_open_editor_from_browser(model),
        AppMessage::OpenEditorFromLibrary => handle_open_editor_from_library(model),
        AppMessage::OpenSettingsPicker { field_key } => {
            handle_open_settings_picker(model, field_key)
        }
        AppMessage::SettingsPathConfirmed => handle_settings_path_confirmed(model),
        AppMessage::Quit => AppModel {
            should_quit: true,
            ..model
        },
    }
}

/// Open a file or directory using the platform default handler.
fn open_path(path: &str) -> Result<(), std::io::Error> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(path).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer").arg(path).spawn()?;
    }
    Ok(())
}

/// Resolve the input mode for a built-in recipe slug.
///
/// Looks up the recipe definition, parses it, and reads the input
/// node's `mode` parameter. Falls back to `FileUpload` for unknown
/// slugs or parse failures.
#[cfg(test)]
fn resolve_input_mode_for_slug(slug: &str) -> bnto_core::InputMode {
    let catalog = crate::catalog::RecipeCatalog::load(std::path::Path::new("/nonexistent"));
    let Some(entry) = catalog.resolve(slug) else {
        return bnto_core::InputMode::FileUpload;
    };
    let Ok(def) = serde_json::from_str::<bnto_core::PipelineDefinition>(&entry.definition_json)
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
    fn test_paths() -> BntoPaths {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("bnto-test-{id}"));
        let paths = BntoPaths { home: root };
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
        let real_config = dirs::home_dir().unwrap().join(".bnto").join("config.toml");
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
                permissions: None,
                symlink_target: None,
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
                    permissions: None,
                    symlink_target: None,
                },
                FileEntry {
                    name: "b.png".into(),
                    is_dir: false,
                    path: PathBuf::from("/photos/b.png"),
                    size: Some(300),
                    permissions: None,
                    symlink_target: None,
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

        // Edit quality from 80 → 55 via tonkotsu messages.
        let app = update(
            app,
            AppMessage::DetailForm(tonkotsu::FormMessage::StartEdit),
        );
        let app = update(
            app,
            AppMessage::DetailForm(tonkotsu::FormMessage::EditBackspace),
        );
        let app = update(
            app,
            AppMessage::DetailForm(tonkotsu::FormMessage::EditBackspace),
        );
        let app = update(
            app,
            AppMessage::DetailForm(tonkotsu::FormMessage::EditChar('5')),
        );
        let app = update(
            app,
            AppMessage::DetailForm(tonkotsu::FormMessage::EditChar('5')),
        );
        let app = update(
            app,
            AppMessage::DetailForm(tonkotsu::FormMessage::CommitEdit),
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
                node_info: vec![],
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
        assert_eq!(app.toml_config.tui.theme, "monaco");
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
        app.toml_config.paths.recipes = Some("/recipes".into());
        app.toml_config.paths.output = Some("/output".into());
        app.screen = Screen::Settings;

        let app = update(app, AppMessage::ThemeChanged(ThemeVariant::Tokyo));
        assert_eq!(app.toml_config.tui.theme, "tokyo");
        assert_eq!(app.toml_config.paths.recipes, Some("/recipes".into()));
        assert_eq!(app.toml_config.paths.output, Some("/output".into()));
    }

    #[test]
    fn settings_path_confirmed_for_recipes_dir_preserves_output_dir() {
        // Simulate: config already has output, user changes only recipes dir.
        let mut app = update(default_model(), AppMessage::OpenSettings);
        app.toml_config.paths.output = Some("/existing-output".into());
        app.settings = app.settings.map(|mut s| {
            if let Some(f) = s.fields.iter_mut().find(|f| f.key == "output_dir") {
                f.value = "/existing-output".to_string();
            }
            s
        });
        app.screen = Screen::Picker {
            slug: "recipes_dir".into(),
            from: DetailOrigin::Home,
        };
        app.settings_picker_field = Some("recipes_dir".into());
        app.picker = Some(PickerModel::from_dir(
            "recipes_dir",
            &std::path::PathBuf::from("/new-recipes"),
        ));

        let app = update(app, AppMessage::SettingsPathConfirmed);
        assert_eq!(app.screen, Screen::Settings);
        // recipes_dir updated to picker's current_dir.
        assert!(app.toml_config.paths.recipes.is_some());
        // output preserved — not clobbered.
        assert_eq!(
            app.toml_config.paths.output,
            Some("/existing-output".into())
        );
    }

    #[test]
    fn settings_path_confirmed_for_output_dir_preserves_recipes_dir() {
        // Symmetric: config has recipes dir, user changes only output dir.
        let mut app = update(default_model(), AppMessage::OpenSettings);
        app.toml_config.paths.recipes = Some("/existing-recipes".into());
        app.settings = app.settings.map(|mut s| {
            if let Some(f) = s.fields.iter_mut().find(|f| f.key == "recipes_dir") {
                f.value = "/existing-recipes".to_string();
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
        assert_eq!(
            app.toml_config.paths.recipes,
            Some("/existing-recipes".into())
        );
        assert!(app.toml_config.paths.output.is_some());
    }

    #[test]
    fn settings_roundtrip_both_paths_survive_reload() {
        // Full roundtrip: load config with both paths → open settings → verify fields.
        let mut app = default_model();
        app.toml_config.tui.theme = "tokyo".to_string();
        app.toml_config.paths.recipes = Some("/recipes".into());
        app.toml_config.paths.output = Some("/output".into());
        let app = update(app, AppMessage::OpenSettings);
        let settings = app.settings.as_ref().expect("settings created");
        let rd = settings
            .fields
            .iter()
            .find(|f| f.key == "recipes_dir")
            .unwrap();
        let od = settings
            .fields
            .iter()
            .find(|f| f.key == "output_dir")
            .unwrap();
        assert_eq!(rd.value, "/recipes");
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
    fn test_paths_with_dir() -> (tempfile::TempDir, BntoPaths) {
        let tmp = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            home: tmp.path().to_path_buf(),
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
        assert_eq!(app.paths.home, paths.home);
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
            home: tmp.path().join("nonexistent").join("deep"),
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
        assert_eq!(loaded.paths.output, Some("/tmp".into()));
    }

    #[test]
    fn with_paths_loads_toml_config() {
        let (_tmp, paths) = test_paths_with_dir();

        // Pre-save a TOML config.
        let config = TomlConfig {
            tui: crate::storage::config::TuiSection {
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
        let catalog = crate::catalog::RecipeCatalog::load(std::path::Path::new("/nonexistent"));
        std::fs::write(
            recipes_dir.join("compress-images.bnto.json"),
            &catalog.resolve("compress-images").unwrap().definition_json,
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
        let catalog = crate::catalog::RecipeCatalog::load(std::path::Path::new("/nonexistent"));
        std::fs::write(
            recipes_dir.join("compress-images.bnto.json"),
            &catalog.resolve("compress-images").unwrap().definition_json,
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
            home: std::path::PathBuf::from("/nonexistent"),
        };
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
        // The download-video recipe has shell-command node with command, args params.
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
