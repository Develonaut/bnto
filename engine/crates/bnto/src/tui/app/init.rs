// AppModel construction — load config, catalog and the initial screen.

use std::collections::HashMap;

use bnto_engine::create_registry;

use crate::catalog::RecipeCatalog;
use crate::storage::config::TomlConfig;
use crate::storage::paths::BntoPaths;

use super::super::app_helpers::load_editor_from_json;
use super::super::screens::browser::BrowserModel;
use super::super::screens::editor::EditorScreenModel;
use super::super::screens::home::{HomeModel, list_library_recipes};
use super::super::theme::{Theme, ThemeVariant};
use super::{AppModel, DetailOrigin, Screen};

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
        let effective_variant = effective_variant(variant, &toml_config);
        // Apply config home override — if paths.home is set, use that as the root.
        let paths = paths.with_config_override(&toml_config);
        let editor = initial_editor(new_recipe, recipe_json);
        Self::assemble(effective_variant, toml_config, paths, editor)
    }

    /// Build the model on its launch screen: every screen but Home and
    /// Browser closed, plus the catalog and library list for the Home pane.
    ///
    /// One line per field — the length is the field list, not logic.
    #[expect(clippy::too_many_lines)]
    fn assemble(
        effective_variant: ThemeVariant,
        toml_config: TomlConfig,
        paths: BntoPaths,
        editor: Option<EditorScreenModel>,
    ) -> Self {
        let registry = create_registry();
        // Build unified catalog and list library recipes for the home screen pane.
        let catalog = RecipeCatalog::load(&paths.recipes_dir());
        let library_names = list_library_recipes(&paths.recipes_dir());
        let screen = initial_screen(editor.is_some());

        Self {
            screen,
            should_quit: false,
            theme: Theme::from_variant(effective_variant),
            theme_variant: effective_variant,
            home: HomeModel::new(library_names),
            browser: BrowserModel::new(&catalog.bundled()),
            library: None,
            detail: None,
            picker: None,
            execution: None,
            results: None,
            preview: None,
            settings: None,
            editor,
            wizard: None,
            toml_config,
            paths,
            status_message: None,
            update_notice: None,
            settings_picker_field: None,
            param_overrides: HashMap::new(),
            registry,
            catalog,
        }
    }
}

/// CLI --theme flag overrides saved config.
fn effective_variant(variant: ThemeVariant, config: &TomlConfig) -> ThemeVariant {
    if variant != ThemeVariant::LosAngeles {
        variant
    } else {
        ThemeVariant::from_str_lossy(&config.tui.theme).unwrap_or(variant)
    }
}

/// Editor to open on launch: blank for `--new`, the given recipe JSON, or
/// none (start on Home — also when the JSON fails to load).
fn initial_editor(new_recipe: bool, recipe_json: Option<String>) -> Option<EditorScreenModel> {
    if new_recipe {
        let editor_model = bnto_core::editor::EditorModel::new();
        return Some(EditorScreenModel::new(editor_model));
    }
    match load_editor_from_json(&recipe_json?) {
        Ok(editor_model) => Some(EditorScreenModel::new(editor_model)),
        Err(e) => {
            eprintln!("Warning: {e} — starting on home screen.");
            None
        }
    }
}

/// The editor screen when an editor opens on launch, else Home.
fn initial_screen(has_editor: bool) -> Screen {
    if has_editor {
        Screen::Editor {
            from: DetailOrigin::Home,
        }
    } else {
        Screen::Home
    }
}
