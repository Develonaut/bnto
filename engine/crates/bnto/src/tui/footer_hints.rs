// The footer's key hints, composed from three layers so each is extended in
// one place (ISS-101, decisions/tui-footer-hints.md):
// - a mode's keys (a question, consent, the settings picker), or else the
//   screen's own keys (`Screen::help_hints`), come first;
// - then the base keys every screen shares (`BASE`), unless a mode takes
//   every key (a question, a search box), where `q` is an answer or a letter.
// The footer drops hints from the end when it's narrow, so the screen's keys
// outlast the base ones.

use super::app::{AppModel, Screen};
use super::keys::captures_keys;
use super::screens::execution::ExecutionStatus;

/// A key and what it does, as the footer shows them.
pub type Hint = (&'static str, &'static str);

/// Keys that work on every screen outside a mode that captures them.
const BASE: &[Hint] = &[("q", "quit")];

/// The hints the footer shows for `model`.
pub fn footer_hints(model: &AppModel) -> Vec<Hint> {
    let mut hints = mode_hints(model).unwrap_or_else(|| model.screen.help_hints());
    if !captures_keys(model) {
        hints.extend_from_slice(BASE);
    }
    hints
}

/// The keys of a mode that replaces the screen's, if one is active.
fn mode_hints(model: &AppModel) -> Option<Vec<Hint>> {
    let consent = model.execution.as_ref().map(|e| &e.status);
    if consent == Some(&ExecutionStatus::AwaitingConsent) {
        return Some(vec![("y", "allow"), ("n/Esc", "cancel")]);
    }
    if model.settings_picker_field.is_some() && matches!(model.screen, Screen::Picker { .. }) {
        return Some(vec![
            ("↑↓", "navigate"),
            ("h/l", "parent/enter"),
            (".", "hidden"),
            ("Tab", "select dir"),
            ("Esc", "back"),
        ]);
    }
    editor_question_hints(model)
}

/// While the editor asks to delete a node or to save, only its answers work.
fn editor_question_hints(model: &AppModel) -> Option<Vec<Hint>> {
    if !matches!(model.screen, Screen::Editor { .. }) {
        return None;
    }
    let editor = model.editor.as_ref()?;
    if editor.confirming_delete {
        return Some(vec![("y", "delete"), ("n", "keep")]);
    }
    editor
        .confirming_dirty_exit
        .then(|| vec![("y", "save"), ("n", "discard"), ("Esc", "cancel")])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::config::TomlConfig;
    use crate::storage::paths::BntoPaths;
    use crate::tui::app::DetailOrigin;
    use crate::tui::screens::editor::EditorScreenModel;
    use crate::tui::theme::ThemeVariant;

    fn model(screen: Screen) -> (tempfile::TempDir, AppModel) {
        let tmp = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            home: tmp.path().to_path_buf(),
        };
        paths.ensure_dirs().unwrap();
        TomlConfig::default().save(&paths).unwrap();
        let mut app = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        app.screen = screen;
        (tmp, app)
    }

    fn keys(hints: &[Hint]) -> Vec<&str> {
        hints.iter().map(|(key, _)| *key).collect()
    }

    fn editing() -> (tempfile::TempDir, AppModel) {
        let (tmp, mut app) = model(Screen::Editor {
            from: DetailOrigin::Home,
        });
        app.editor = Some(EditorScreenModel::new(bnto_core::editor::EditorModel::new()));
        (tmp, app)
    }

    #[test]
    fn the_base_keys_follow_every_screens_own() {
        for screen in [Screen::Home, Screen::Library, Screen::Settings] {
            let (_tmp, app) = model(screen.clone());
            let hints = footer_hints(&app);
            assert_eq!(hints.last(), Some(&("q", "quit")), "{screen:?}");
            let quits = hints.iter().filter(|(key, _)| *key == "q").count();
            assert_eq!(quits, 1, "{screen:?}: q listed once");
        }
    }

    #[test]
    fn a_search_box_hides_the_base_keys() {
        let (_tmp, mut app) = model(Screen::Browser);
        app.browser.searching = true;
        assert!(
            !keys(&footer_hints(&app)).contains(&"q"),
            "q types a letter"
        );
    }

    #[test]
    fn the_editors_save_question_shows_only_its_answers() {
        // RED (ISS-101): the footer kept the editor's navigation keys.
        let (_tmp, mut app) = editing();
        app.editor.as_mut().unwrap().confirming_dirty_exit = true;
        assert_eq!(
            footer_hints(&app),
            [("y", "save"), ("n", "discard"), ("Esc", "cancel")]
        );
    }

    #[test]
    fn the_editors_delete_question_shows_only_its_answers() {
        let (_tmp, mut app) = editing();
        app.editor.as_mut().unwrap().confirming_delete = true;
        assert_eq!(footer_hints(&app), [("y", "delete"), ("n", "keep")]);
    }

    #[test]
    fn the_settings_picker_shows_its_own_keys() {
        let (_tmp, mut app) = model(Screen::Picker {
            slug: "s".into(),
            from: DetailOrigin::Home,
        });
        app.settings_picker_field = Some("homePath".into());
        assert_eq!(
            keys(&footer_hints(&app)),
            ["↑↓", "h/l", ".", "Tab", "Esc", "q"]
        );
    }

    #[test]
    fn consent_shows_its_answers_and_quit() {
        // Enter does nothing while consent waits; y and n/Esc answer, q quits.
        let (_tmp, mut app) = model(Screen::Execution {
            slug: "s".into(),
            from: DetailOrigin::Home,
        });
        let mut exec = crate::tui::screens::execution::ExecutionModel::new("s");
        exec.status = ExecutionStatus::AwaitingConsent;
        app.execution = Some(exec);
        assert_eq!(keys(&footer_hints(&app)), ["y", "n/Esc", "q"]);
    }
}
