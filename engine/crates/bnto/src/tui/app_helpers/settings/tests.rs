// The Settings home field edits the config `BntoPaths::configured` reads:
// the default home's, even when the session runs in a home it moved to.

use std::path::Path;

use super::super::super::app::{AppMessage, AppModel, DetailOrigin, Screen, update};
use super::super::super::screens::picker::PickerModel;
use super::super::super::theme::ThemeVariant;
use crate::storage::config::TomlConfig;
use crate::storage::paths::BntoPaths;

fn paths(dir: &Path) -> BntoPaths {
    BntoPaths {
        home: dir.to_path_buf(),
    }
}

/// `home`'s `config.toml` with `paths.home` set to `to`.
fn move_home(home: &Path, to: &Path) {
    let mut config = TomlConfig::default();
    config.paths.home = Some(to.to_string_lossy().into_owned());
    config.save(&paths(home)).unwrap();
}

/// A session moved from `default` to `moved`, as `bnto tui` launches it.
fn moved_session(default: &Path, moved: &Path) -> AppModel {
    move_home(default, moved);
    let configured = paths(default).configured();
    AppModel::with_paths(ThemeVariant::LosAngeles, None, false, configured)
        .with_config_home(paths(default))
}

/// Pick `dir` for the Settings home field.
fn pick_home(model: AppModel, dir: &Path) -> AppModel {
    let model = update(model, AppMessage::OpenSettings);
    let model = AppModel {
        screen: Screen::Picker {
            slug: "home_path".into(),
            from: DetailOrigin::Home,
        },
        settings_picker_field: Some("home_path".into()),
        picker: Some(PickerModel::from_dir("home_path", dir)),
        ..model
    };
    update(model, AppMessage::SettingsPathConfirmed)
}

fn home_field(model: &AppModel) -> String {
    let settings = model.settings.as_ref().expect("settings open");
    let field = settings.fields.iter().find(|f| f.key == "home_path");
    field.expect("a home field").value.clone()
}

#[test]
fn the_home_field_shows_the_default_homes_setting() {
    let (default, moved, elsewhere) = (tempdir(), tempdir(), tempdir());
    let model = moved_session(default.path(), moved.path());
    // The moved home's own `paths.home` is read by nothing.
    move_home(moved.path(), elsewhere.path());
    let model = update(model, AppMessage::OpenSettings);
    assert_eq!(home_field(&model), moved.path().to_string_lossy());
}

#[test]
fn saving_the_home_field_writes_the_config_configured_reads() {
    let (default, moved, next) = (tempdir(), tempdir(), tempdir());
    let model = moved_session(default.path(), moved.path());
    let model = pick_home(model, next.path());
    assert_eq!(paths(default.path()).configured().home, next.path());
    // The session keeps its home until the next launch, and says so.
    assert_eq!(model.paths.home, moved.path());
    let status = model.status_message.expect("a notice");
    assert!(status.contains("next launch"), "{status}");
}

#[test]
fn a_later_theme_change_keeps_a_home_saved_without_a_move() {
    // No override yet: the session's own config is the one `configured`
    // reads, so a theme saved after the home must not undo it.
    let (home, next) = (tempdir(), tempdir());
    let model = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths(home.path()));
    let model = pick_home(model, next.path());
    let _ = update(model, AppMessage::ThemeChanged(ThemeVariant::Tokyo));
    assert_eq!(paths(home.path()).configured().home, next.path());
}

fn tempdir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}
