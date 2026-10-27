// Quitting from the recipe editor (ISS-98): with unsaved changes, `q` and U
// open the editor's "save?" prompt instead of dropping the edits. Save or
// discard, then the TUI quits (and upgrades); Esc stays in the editor.

use super::*;
use crate::storage::config::TomlConfig;
use crate::storage::paths::BntoPaths;
use crate::tui::screens::editor::{EditorMessage, EditorScreenModel};

/// A model on the editor with an unsaved recipe named `name`.
fn editing(name: &str, dirty: bool) -> (tempfile::TempDir, AppModel) {
    let tmp = tempfile::tempdir().unwrap();
    let paths = BntoPaths {
        home: tmp.path().to_path_buf(),
    };
    paths.ensure_dirs().unwrap();
    TomlConfig::default().save(&paths).unwrap();
    let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
    let mut editor = bnto_core::editor::EditorModel::new();
    editor.recipe_name = name.to_string();
    editor.dirty = dirty;
    app.screen = Screen::Editor {
        from: DetailOrigin::Home,
    };
    app.editor = Some(EditorScreenModel::new(editor));
    (tmp, app)
}

fn asking(app: &AppModel) -> bool {
    app.editor.as_ref().is_some_and(|e| e.confirming_dirty_exit)
}

fn with_update(app: AppModel) -> AppModel {
    AppModel {
        update_notice: Some("bnto 99.0.0 available, U to upgrade".into()),
        ..app
    }
}

#[test]
fn quitting_with_unsaved_edits_asks_first() {
    let (_tmp, app) = editing("Draft", true);

    let app = update(app, AppMessage::Quit);

    assert!(!app.should_quit, "the edits would be lost");
    assert!(asking(&app));
}

#[test]
fn discarding_then_quits() {
    let (_tmp, app) = editing("Draft", true);
    let app = update(app, AppMessage::Quit);

    let app = update(app, AppMessage::Editor(EditorMessage::DirtyDiscard));

    assert!(app.should_quit);
    assert!(!app.upgrade_on_exit);
}

#[test]
fn saving_then_quits_with_the_recipe_saved() {
    let (_tmp, app) = editing("Kept Draft", true);
    let recipes = app.paths.recipes_dir();
    let app = update(app, AppMessage::Quit);

    let app = update(app, AppMessage::Editor(EditorMessage::DirtySave));

    assert!(app.should_quit);
    let saved = std::fs::read_dir(&recipes).unwrap().count();
    assert_eq!(saved, 1, "the recipe was saved before quitting");
}

#[test]
fn cancelling_stays_in_the_editor_and_forgets_the_quit() {
    let (_tmp, app) = editing("Draft", true);
    let app = update(app, AppMessage::Quit);
    let app = update(app, AppMessage::Editor(EditorMessage::DirtyCancel));

    assert!(!app.should_quit);
    assert!(matches!(app.screen, Screen::Editor { .. }));
    // A later Back that discards leaves the editor without quitting.
    let app = update(app, AppMessage::Editor(EditorMessage::Back));
    let app = update(app, AppMessage::Editor(EditorMessage::DirtyDiscard));
    assert!(!app.should_quit, "the cancelled quit came back");
}

#[test]
fn quitting_a_clean_editor_quits_at_once() {
    let (_tmp, app) = editing("Saved", false);

    let app = update(app, AppMessage::Quit);

    assert!(app.should_quit);
}

#[test]
fn upgrading_with_unsaved_edits_asks_then_upgrades() {
    let (_tmp, app) = editing("Draft", true);
    let app = update(with_update(app), AppMessage::Upgrade);
    assert!(!app.should_quit);
    assert!(asking(&app));

    let app = update(app, AppMessage::Editor(EditorMessage::DirtyDiscard));

    assert!(app.should_quit);
    assert!(app.upgrade_on_exit);
}
