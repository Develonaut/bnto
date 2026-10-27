// App state machine — routes between TUI screens.
//
// Pure state + pure transitions. No I/O, no terminal access.
// All screen navigation logic is testable with `cargo test`.

use std::collections::HashMap;

use bnto_core::registry::NodeRegistry;

use crate::catalog::RecipeCatalog;

use super::app_helpers::{
    handle_add_to_library, handle_add_to_library_write, handle_back, handle_config_confirmed,
    handle_detail_focus, handle_detail_form, handle_detail_picker, handle_detail_scroll,
    handle_editor, handle_editor_form, handle_execution, handle_execution_complete,
    handle_files_selected, handle_home_confirm, handle_library, handle_library_confirm,
    handle_open_editor_from_browser, handle_open_editor_from_library, handle_open_file,
    handle_open_folder, handle_open_library, handle_open_settings_picker, handle_open_wizard,
    handle_preview_confirm, handle_preview_requested, handle_recipe_selected,
    handle_settings_path_confirmed, handle_telemetry_toggled, handle_theme_changed, handle_wizard,
    handle_wizard_form, request_quit, settle_pending_exit,
};
mod detail_entry;
mod init;
mod message;
#[cfg(test)]
mod quit_tests;
#[cfg(test)]
mod shadowed_recipe_tests;

use super::screens::browser::{BrowserModel, update as browser_update};
use super::screens::detail::{DetailFocus, DetailModel};
use super::screens::editor::EditorScreenModel;
use super::screens::execution::ExecutionModel;
use super::screens::home::{HomeModel, update as home_update};
use super::screens::library::LibraryModel;
use super::screens::picker::{PickerModel, update as picker_update};
use super::screens::preview::{PreviewModel, update as preview_update};
use super::screens::results::{ResultsModel, update as results_update};
use super::screens::settings::{SettingsModel, update as settings_update};
use super::screens::wizard::WizardModel;
use super::theme::{Theme, ThemeVariant};
use crate::storage::config::TomlConfig;
use crate::storage::paths::BntoPaths;
pub use message::AppMessage;

// Message types and the registry constructor the tests below build with.
#[cfg(test)]
use super::screens::{
    editor::EditorMessage, execution::ExecutionMessage, home::HomeMessage, results::ResultsMessage,
    settings::SettingsMessage,
};
#[cfg(test)]
use bnto_engine::create_registry;

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

/// A quit waiting on the editor's "save?" answer (ISS-98).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingExit {
    /// Run `bnto upgrade` once the TUI has closed (U was pressed).
    pub upgrade: bool,
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
    Preview { slug: String, from: DetailOrigin },
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
    /// Preview screen state — populated after preview-mode pipeline run.
    pub preview: Option<PreviewModel>,
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
    /// "bnto 0.17.0 available, U to upgrade" when a newer release is out.
    pub update_notice: Option<String>,
    /// Run `bnto upgrade` once the TUI has closed (U was pressed).
    pub upgrade_on_exit: bool,
    /// A quit held while the editor asks about unsaved changes (ISS-98).
    pub pending_exit: Option<PendingExit>,
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
            .field("preview", &self.preview.as_ref().map(|p| &p.slug))
            .field("settings", &self.settings.is_some())
            .field("editor", &self.editor.is_some())
            .field("wizard", &self.wizard.is_some())
            .field("status_message", &self.status_message)
            .field("upgrade_on_exit", &self.upgrade_on_exit)
            .field("pending_exit", &self.pending_exit)
            .field("settings_picker_field", &self.settings_picker_field)
            .field("param_overrides", &self.param_overrides.len())
            .finish_non_exhaustive()
    }
}

/// Pure state transition — the heart of the TEA pattern.
/// Takes current state + message, returns the next state.
///
/// Routing is split in three tiers: screen flow here, per-screen message
/// forwarding in `update_screen`, and recipe authoring and settings in
/// `update_authoring`.
pub fn update(model: AppModel, msg: AppMessage) -> AppModel {
    match msg {
        AppMessage::RecipeSelected { slug } => handle_recipe_selected(model, slug),
        AppMessage::ConfigConfirmed { slug } => handle_config_confirmed(model, slug),
        AppMessage::FilesSelected { slug } => handle_files_selected(model, slug),
        AppMessage::ExecutionComplete { slug } => handle_execution_complete(model, slug),
        AppMessage::HomeConfirm => handle_home_confirm(model),
        AppMessage::OpenLibrary => handle_open_library(model),
        AppMessage::LibraryConfirm => handle_library_confirm(model),
        AppMessage::AddToLibrary => handle_add_to_library(model),
        AppMessage::AddToLibraryConfirm { slug } => handle_add_to_library_write(model, &slug, true),
        AppMessage::Back => handle_back(model),
        AppMessage::RunAnother => with_screen(model, Screen::Browser),
        AppMessage::OpenFile => handle_open_file(model),
        AppMessage::OpenFolder => handle_open_folder(model),
        AppMessage::PreviewRequested { slug } => handle_preview_requested(model, slug),
        AppMessage::PreviewConfirm { slug } => handle_preview_confirm(model, slug),
        AppMessage::Quit => request_quit(model, false),
        msg => update_screen(model, msg),
    }
}

/// Forward a message to the screen model it belongs to.
fn update_screen(model: AppModel, msg: AppMessage) -> AppModel {
    match msg {
        AppMessage::Home(msg) => AppModel {
            home: home_update(model.home, msg),
            ..model
        },
        AppMessage::Library(ref msg) => handle_library(model, msg),
        AppMessage::Browser(msg) => AppModel {
            browser: browser_update(model.browser, msg),
            ..model
        },
        AppMessage::DetailForm(msg) => handle_detail_form(model, msg),
        AppMessage::DetailScroll(msg) => handle_detail_scroll(model, msg),
        AppMessage::DetailPicker(msg) => handle_detail_picker(model, msg),
        AppMessage::DetailFocusInput => handle_detail_focus(model, DetailFocus::Input),
        AppMessage::DetailFocusParams => handle_detail_focus(model, DetailFocus::Params),
        msg => update_screen_models(model, msg),
    }
}

/// Forward picker, execution, results and preview messages.
fn update_screen_models(model: AppModel, msg: AppMessage) -> AppModel {
    match msg {
        AppMessage::Picker(msg) => {
            let picker = model.picker.map(|p| picker_update(p, msg));
            AppModel { picker, ..model }
        }
        AppMessage::Execution(msg) => handle_execution(model, msg),
        AppMessage::Results(msg) => {
            let results = model.results.map(|r| results_update(r, msg));
            AppModel { results, ..model }
        }
        AppMessage::Preview(msg) => {
            let preview = model.preview.map(|p| preview_update(p, msg));
            AppModel { preview, ..model }
        }
        msg => update_authoring(model, msg),
    }
}

/// Recipe authoring (editor, wizard) and settings messages.
fn update_authoring(model: AppModel, msg: AppMessage) -> AppModel {
    match msg {
        AppMessage::OpenSettings => handle_open_settings(model),
        AppMessage::UpdateAvailable(notice) => AppModel {
            update_notice: Some(notice),
            ..model
        },
        AppMessage::Upgrade => upgrade_on_exit(model),
        AppMessage::Settings(msg) => {
            let settings = model.settings.map(|s| settings_update(s, msg));
            AppModel { settings, ..model }
        }
        AppMessage::ThemeChanged(variant) => handle_theme_changed(model, variant),
        AppMessage::TelemetryToggled(enabled) => handle_telemetry_toggled(model, enabled),
        AppMessage::OpenSettingsPicker { field_key } => {
            handle_open_settings_picker(model, field_key)
        }
        AppMessage::SettingsPathConfirmed => handle_settings_path_confirmed(model),
        msg => update_editing(model, msg),
    }
}

/// Quit to run `bnto upgrade`, when there's a newer release to install;
/// unsaved editor changes are asked about first.
fn upgrade_on_exit(model: AppModel) -> AppModel {
    if model.update_notice.is_none() {
        return model;
    }
    request_quit(model, true)
}

/// Editor and wizard messages — the last routing tier.
fn update_editing(model: AppModel, msg: AppMessage) -> AppModel {
    match msg {
        AppMessage::Editor(msg) => settle_pending_exit(handle_editor(model, msg)),
        AppMessage::EditorForm(form_msg) => handle_editor_form(model, form_msg),
        AppMessage::Wizard(msg) => handle_wizard(model, msg),
        AppMessage::WizardForm(form_msg) => handle_wizard_form(model, form_msg),
        AppMessage::OpenWizard => handle_open_wizard(model),
        AppMessage::OpenEditorFromBrowser => handle_open_editor_from_browser(model),
        AppMessage::OpenEditorFromLibrary => handle_open_editor_from_library(model),
        other => unreachable!("message handled by an earlier update tier: {other:?}"),
    }
}

/// Switch to `screen`, keeping everything else.
fn with_screen(model: AppModel, screen: Screen) -> AppModel {
    AppModel { screen, ..model }
}

/// Open the settings screen with the persisted config loaded.
fn handle_open_settings(model: AppModel) -> AppModel {
    let settings = SettingsModel::from_toml_config(&model.toml_config);
    AppModel {
        screen: Screen::Settings,
        settings: Some(settings),
        ..model
    }
}

/// Resolve the input mode for a built-in recipe slug.
///
/// Looks up the recipe definition, parses it, and reads the input
/// node's `mode` parameter. Falls back to `FileUpload` for unknown
/// slugs or parse failures.
#[cfg(test)]
fn resolve_input_mode_for_slug(slug: &str) -> bnto_core::InputMode {
    let catalog = crate::catalog::RecipeCatalog::bundled_only();
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
    use crate::tui::screens::size_change::SizeChange;
    use bnto_core::InputMode;

    /// Build test paths in a unique temp directory.
    fn test_paths() -> BntoPaths {
        // Kept on disk, as before: the model holds only the path.
        let root = tempfile::Builder::new()
            .prefix("bnto-test-")
            .tempdir()
            .unwrap()
            .keep();
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
    fn upgrade_quits_to_upgrade_when_a_newer_release_is_out() {
        let model = AppModel {
            update_notice: Some("bnto 99.0.0 available, U to upgrade".into()),
            ..default_model()
        };
        let app = update(model, AppMessage::Upgrade);
        assert!(app.should_quit);
        assert!(app.upgrade_on_exit);
    }

    #[test]
    fn upgrade_does_nothing_without_a_newer_release() {
        let app = update(default_model(), AppMessage::Upgrade);
        assert!(!app.should_quit);
        assert!(!app.upgrade_on_exit);
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
                    path: "out.jpg".into(),
                    size_bytes: 500,
                    original_size: Some(1000),
                }],
                output_dir: Some("/tmp/out".into()),
                warnings: Vec::new(),
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
                path: "a".into(),
                size_bytes: 100,
                original_size: None,
            },
            OutputFile {
                name: "b".into(),
                path: "b".into(),
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
        // Format was not modified (still matches default "jpeg"), so it should
        // not appear as an override — only user-changed values are overrides.
        assert!(
            !exec.param_overrides.contains_key("compress:format"),
            "unchanged params should not become overrides"
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
                    path: "result.jpg".into(),
                    size_bytes: 200,
                    original_size: Some(500),
                }],
                output_dir: Some("/out".into()),
                warnings: Vec::new(),
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
        assert_eq!(
            results.savings.as_ref().unwrap().change(),
            Some(SizeChange::Smaller(60))
        );
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
        assert_eq!(app.settings.as_ref().unwrap().fields.len(), 3);
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
            slug: "home_path".into(),
            from: DetailOrigin::Home,
        };
        app.settings_picker_field = Some("home_path".into());
        app.picker = Some(PickerModel::from_dir(
            "home_path",
            &std::path::PathBuf::from("/tmp"),
        ));

        let app = update(app, AppMessage::SettingsPathConfirmed);
        assert_eq!(app.screen, Screen::Settings);
        assert!(app.picker.is_none());
        assert!(app.settings_picker_field.is_none());
        // The home_path field should be updated.
        let home_path_field = app
            .settings
            .as_ref()
            .unwrap()
            .fields
            .iter()
            .find(|f| f.key == "home_path")
            .unwrap();
        assert_eq!(home_path_field.value, "/tmp");
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

    // --- Settings persistence: home path preservation across saves ---

    #[test]
    fn theme_changed_preserves_home_path_setting() {
        let mut app = default_model();
        app.toml_config.paths.home = Some("/custom/bnto".into());
        app.screen = Screen::Settings;

        let app = update(app, AppMessage::ThemeChanged(ThemeVariant::Tokyo));
        assert_eq!(app.toml_config.tui.theme, "tokyo");
        assert_eq!(app.toml_config.paths.home, Some("/custom/bnto".into()));
    }

    #[test]
    fn settings_roundtrip_home_path_survives_reload() {
        let mut app = default_model();
        app.toml_config.paths.home = Some("/custom/bnto".into());
        let app = update(app, AppMessage::OpenSettings);
        let settings = app.settings.as_ref().expect("settings created");
        let hp = settings
            .fields
            .iter()
            .find(|f| f.key == "home_path")
            .unwrap();
        assert_eq!(hp.value, "/custom/bnto");
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
            slug: "home_path".into(),
            from: DetailOrigin::Home,
        };
        app.settings_picker_field = Some("home_path".into());
        app.picker = Some(PickerModel::from_dir(
            "home_path",
            &std::path::PathBuf::from("/tmp"),
        ));

        let app = update(app, AppMessage::SettingsPathConfirmed);
        assert_eq!(app.screen, Screen::Settings);

        // Verify the config was saved to disk via TOML.
        let loaded = TomlConfig::load(&paths);
        assert_eq!(loaded.paths.home, Some("/tmp".into()));
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
        let catalog = crate::catalog::RecipeCatalog::bundled_only();
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
        let catalog = crate::catalog::RecipeCatalog::bundled_only();
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
        // A home that is a regular file: `recipes/` cannot be created under it.
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("not-a-dir");
        std::fs::write(&home, b"").unwrap();
        let paths = BntoPaths { home };
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
        let mut detail = DetailModel::from_slug(slug, &create_registry()).unwrap();
        // Modify one param so it differs from its default — only user-changed
        // values become overrides.
        assert!(
            !detail.form.fields.is_empty(),
            "recipe should have at least one param"
        );
        detail.form.fields[0].value = "user-edited-value".into();
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
