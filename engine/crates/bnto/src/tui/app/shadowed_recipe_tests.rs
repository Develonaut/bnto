// A library recipe with a built-in's slug: the Recipes pane still lists and
// opens the shipped recipe, and Library opens the user's copy.

use super::*;
use crate::storage::config::TomlConfig;
use crate::storage::paths::BntoPaths;
use crate::tui::screens::home::HomePane;

const SLUG: &str = "compress-images";

/// A model whose library holds its own `compress-images`, renamed.
fn model_with_override() -> (tempfile::TempDir, AppModel) {
    let tmp = tempfile::tempdir().unwrap();
    let paths = BntoPaths {
        home: tmp.path().to_path_buf(),
    };
    paths.ensure_dirs().unwrap();
    TomlConfig::default().save(&paths).unwrap();
    let shipped = bnto_engine::recipes::builtin_recipes()
        .into_iter()
        .find(|r| r.slug == SLUG)
        .unwrap();
    let copy = shipped
        .definition_json
        .replacen("\"Compress Images\"", "\"My Compress\"", 1);
    assert_ne!(copy, shipped.definition_json, "the copy must differ");
    std::fs::write(paths.recipes_dir().join(format!("{SLUG}.bnto.json")), copy).unwrap();
    let app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
    (tmp, app)
}

#[test]
fn the_recipes_pane_still_lists_every_built_in() {
    let (_tmp, app) = model_with_override();
    assert_eq!(app.browser.recipes.len(), 22);
    assert!(app.browser.recipes.iter().any(|r| r.slug == SLUG));
}

#[test]
fn opening_it_from_the_recipes_pane_opens_the_shipped_recipe() {
    let (_tmp, mut app) = model_with_override();
    app.home.focused = HomePane::Recipes;
    let index = app
        .browser
        .recipes
        .iter()
        .position(|r| r.slug == SLUG)
        .unwrap();
    app.home.recipes_cursor = index;
    let app = update(app, AppMessage::HomeConfirm);
    let detail = app.detail.expect("detail opened");
    assert_eq!(detail.name, "Compress Images");
    assert!(detail.is_bundled);
}

#[test]
fn selecting_it_in_the_browser_opens_the_shipped_recipe() {
    let (_tmp, app) = model_with_override();
    let app = update(app, AppMessage::RecipeSelected { slug: SLUG.into() });
    assert_eq!(app.detail.expect("detail opened").name, "Compress Images");
}

#[test]
fn opening_it_from_the_library_opens_the_users_copy() {
    let (_tmp, app) = model_with_override();
    let app = update(app, AppMessage::OpenLibrary);
    let app = update(app, AppMessage::LibraryConfirm);
    let detail = app.detail.expect("detail opened");
    assert_eq!(detail.name, "My Compress");
    assert!(!detail.is_bundled);
}
