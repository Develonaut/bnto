// The home Recipes pane's cursor (ISS-66): it moves within the recipes the
// pane shows, and Enter opens the one it marks.

use super::*;
use crate::storage::config::TomlConfig;
use crate::storage::paths::BntoPaths;
use crate::tui::screens::browser::BrowserMessage;
use crate::tui::screens::home::{HomeMessage, HomePane};

fn model() -> (tempfile::TempDir, AppModel) {
    let tmp = tempfile::tempdir().unwrap();
    let paths = BntoPaths {
        home: tmp.path().to_path_buf(),
    };
    paths.ensure_dirs().unwrap();
    TomlConfig::default().save(&paths).unwrap();
    let app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
    (tmp, app)
}

fn on_recipes(app: AppModel) -> AppModel {
    AppModel {
        home: crate::tui::screens::home::HomeModel {
            focused: HomePane::Recipes,
            ..app.home
        },
        ..app
    }
}

/// Search the browser for `query`, then come back home.
fn searched_then_home(app: AppModel, query: &str) -> AppModel {
    let mut app = AppModel {
        screen: Screen::Browser,
        ..app
    };
    app = update(app, AppMessage::Browser(BrowserMessage::EnterSearch));
    for ch in query.chars() {
        app = update(app, AppMessage::Browser(BrowserMessage::SearchInput(ch)));
    }
    app = update(app, AppMessage::Browser(BrowserMessage::ExitSearch));
    update(app, AppMessage::Back)
}

#[test]
fn down_past_the_last_shown_recipe_wraps_to_the_first() {
    // A search narrows the list the pane shows; the wrap follows it.
    let (_tmp, app) = model();
    let app = on_recipes(searched_then_home(app, "csv"));
    let shown = app.browser.filtered.len();
    assert!(
        shown > 1 && shown < app.browser.recipes.len(),
        "{shown} shown"
    );
    let mut app = app;
    for _ in 1..shown {
        app = update(app, AppMessage::Home(HomeMessage::CursorDown));
    }
    assert_eq!(
        app.home.recipes_cursor,
        shown - 1,
        "on the last shown recipe"
    );

    let app = update(app, AppMessage::Home(HomeMessage::CursorDown));

    assert_eq!(app.home.recipes_cursor, 0);
}

#[test]
fn enter_opens_the_recipe_the_cursor_marks_after_a_browser_search() {
    let (_tmp, app) = model();
    let app = searched_then_home(app, "csv");
    assert_eq!(app.screen, Screen::Home);
    let shown: Vec<String> = app
        .browser
        .filtered
        .iter()
        .map(|&i| app.browser.recipes[i].slug.clone())
        .collect();
    let app = on_recipes(app);

    let app = update(app, AppMessage::HomeConfirm);

    let Screen::Detail { slug, .. } = &app.screen else {
        panic!("Enter opened {:?}", app.screen);
    };
    assert_eq!(slug, &shown[0], "the pane marks {} first", shown[0]);
}
