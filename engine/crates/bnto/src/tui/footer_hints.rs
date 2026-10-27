// The footer's key hints, composed from three layers so each is extended in
// one place (ISS-101, decisions/tui-footer-hints.md):
// - a mode's keys (a search box, a question, a rename, a field being edited,
//   consent, a run's state, the settings picker), or else the screen's own
//   keys (`Screen::help_hints`), come first;
// - then the base keys every screen shares (`BASE`), unless a mode takes
//   every key (a question, a search box), where `q` is an answer or a letter.
// The footer drops hints from the end when it's narrow, so the screen's keys
// outlast the base ones.

use super::app::{AppModel, Screen};
use super::keys::captures_keys;
use super::screens::execution::ExecutionStatus;
use tonkotsu::FormModel;

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

/// The settings picker chooses a folder, not files.
const SETTINGS_PICKER: &[Hint] = &[
    ("↑↓", "navigate"),
    ("h/l", "parent/enter"),
    (".", "hidden"),
    ("Tab", "select dir"),
    ("Esc", "back"),
];

/// A search box: typing filters the list; these keys act on it.
const SEARCH: &[Hint] = &[("Enter", "open"), ("Esc", "done"), ("Ctrl+U", "clear")];

/// The file picker's search box, where Enter does nothing.
const PICKER_SEARCH: &[Hint] = &[("Esc", "done"), ("Ctrl+U", "clear")];

/// The editor's add-a-node picker: typing filters it.
const NODE_PICKER: &[Hint] = &[("Enter", "add"), ("Esc", "cancel"), ("↑↓", "choose")];

/// The keys of a mode that replaces the screen's, if one is active.
fn mode_hints(model: &AppModel) -> Option<Vec<Hint>> {
    match &model.screen {
        Screen::Browser => model.browser.searching.then(|| SEARCH.to_vec()),
        Screen::Picker { .. } => picker_hints(model),
        Screen::Execution { .. } => execution_hints(model),
        Screen::Editor { .. } => editor_hints(model),
        Screen::Library => library_hints(model),
        Screen::Detail { .. } => form_hints(model.detail.as_ref().map(|d| &d.form)),
        Screen::Wizard { .. } => form_hints(model.wizard.as_ref().and_then(|w| w.form.as_ref())),
        _ => None,
    }
}

/// A field being edited takes the keys tonkotsu gives its state.
fn form_hints(form: Option<&FormModel>) -> Option<Vec<Hint>> {
    tonkotsu::key_hints(form?).map(<[Hint]>::to_vec)
}

/// The picker's search box, else the settings picker's folder keys.
fn picker_hints(model: &AppModel) -> Option<Vec<Hint>> {
    if model.picker.as_ref().is_some_and(|p| p.searching) {
        return Some(PICKER_SEARCH.to_vec());
    }
    model
        .settings_picker_field
        .is_some()
        .then(|| SETTINGS_PICKER.to_vec())
}

/// Consent takes y or n/Esc, a run in progress only Esc to cancel it, and a
/// finished run Enter for its results or Esc back to the recipe.
fn execution_hints(model: &AppModel) -> Option<Vec<Hint>> {
    match model.execution.as_ref()?.status {
        ExecutionStatus::AwaitingConsent => Some(vec![("y", "allow"), ("n/Esc", "cancel")]),
        ExecutionStatus::Idle | ExecutionStatus::Running => Some(vec![("Esc", "cancel")]),
        ExecutionStatus::Completed | ExecutionStatus::Failed => {
            Some(vec![("Enter", "continue"), ("Esc", "back")])
        }
        ExecutionStatus::Cancelled => Some(vec![("Esc", "back")]),
    }
}

/// The editor's node picker, its delete or save question, or a field being
/// edited: each takes only its own keys.
fn editor_hints(model: &AppModel) -> Option<Vec<Hint>> {
    let editor = model.editor.as_ref()?;
    if editor.picker.is_some() {
        return Some(NODE_PICKER.to_vec());
    }
    if editor.confirming_delete {
        return Some(vec![("y", "delete"), ("n/Esc", "keep")]);
    }
    if editor.confirming_dirty_exit {
        return Some(vec![("y", "save"), ("n", "discard"), ("Esc", "stay")]);
    }
    form_hints(editor.active_form.as_ref().map(|active| &active.form))
}

/// The library's search box, its delete question, or a rename.
fn library_hints(model: &AppModel) -> Option<Vec<Hint>> {
    let library = model.library.as_ref()?;
    if library.searching {
        return Some(SEARCH.to_vec());
    }
    if library.confirming_delete.is_some() {
        return Some(vec![("y", "delete"), ("n/Esc", "keep")]);
    }
    library
        .renaming
        .is_some()
        .then(|| vec![("Enter", "save name"), ("Esc", "cancel")])
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
    fn the_base_keys_follow_a_screens_own() {
        for screen in [Screen::Home, Screen::Library, Screen::Settings] {
            let (_tmp, app) = model(screen.clone());
            let hints = footer_hints(&app);
            assert_eq!(hints.last(), Some(&("q", "quit")), "{screen:?}");
            let quits = hints.iter().filter(|(key, _)| *key == "q").count();
            assert_eq!(quits, 1, "{screen:?}: q listed once");
        }
    }

    #[test]
    fn a_search_box_shows_its_keys_and_no_quit() {
        // RED (ISS-102): the browser kept its usual hints while searching.
        let (_tmp, mut app) = model(Screen::Browser);
        app.browser.searching = true;
        assert_eq!(footer_hints(&app), SEARCH, "q types a letter here");
    }

    #[test]
    fn the_settings_pickers_search_shows_the_search_keys() {
        let (_tmp, mut app) = model(Screen::Picker {
            slug: "s".into(),
            from: DetailOrigin::Home,
        });
        app.settings_picker_field = Some("homePath".into());
        let mut picker = crate::tui::screens::picker::PickerModel::from_test_data(
            "s",
            std::env::temp_dir(),
            vec![],
            vec![],
        );
        picker.searching = true;
        app.picker = Some(picker);
        assert_eq!(footer_hints(&app), PICKER_SEARCH);
    }

    #[test]
    fn the_editors_node_picker_shows_its_keys() {
        let (_tmp, mut app) = editing();
        let editor = app.editor.take().unwrap();
        let (editor, _) = crate::tui::screens::editor::update(
            editor,
            crate::tui::screens::editor::EditorMessage::OpenPicker,
            &app.registry,
        );
        app.editor = Some(editor);
        assert_eq!(footer_hints(&app), NODE_PICKER);
    }

    #[test]
    fn a_field_being_edited_shows_tonkotsus_keys() {
        let (_tmp, app) = model(Screen::Home);
        let mut app = crate::tui::app::update(
            app,
            crate::tui::app::AppMessage::RecipeSelected {
                slug: "compress-images".into(),
            },
        );
        let detail = app.detail.as_mut().expect("the recipe opened");
        let form = tonkotsu::FormModel::new(vec![tonkotsu::text("Name").build()]);
        detail.form = tonkotsu::update(form, tonkotsu::FormMessage::StartEdit);
        assert_eq!(footer_hints(&app), [("Enter", "save"), ("Esc", "cancel")]);
    }

    #[test]
    fn a_finished_run_offers_continue_and_back() {
        let (_tmp, mut app) = model(Screen::Execution {
            slug: "s".into(),
            from: DetailOrigin::Home,
        });
        let mut exec = crate::tui::screens::execution::ExecutionModel::new("s");
        exec.status = ExecutionStatus::Failed;
        app.execution = Some(exec);
        let hints = footer_hints(&app);
        assert_eq!(
            hints,
            [("Enter", "continue"), ("Esc", "back"), ("q", "quit")]
        );
    }

    #[test]
    fn a_cancelled_run_offers_back() {
        let (_tmp, mut app) = model(Screen::Execution {
            slug: "s".into(),
            from: DetailOrigin::Home,
        });
        let mut exec = crate::tui::screens::execution::ExecutionModel::new("s");
        exec.status = ExecutionStatus::Cancelled;
        app.execution = Some(exec);
        assert_eq!(footer_hints(&app), [("Esc", "back"), ("q", "quit")]);
    }

    /// A one-field form, its field being edited.
    fn editing_form() -> tonkotsu::FormModel {
        let form = tonkotsu::FormModel::new(vec![tonkotsu::text("Name").build()]);
        tonkotsu::update(form, tonkotsu::FormMessage::StartEdit)
    }

    #[test]
    fn an_editor_field_being_edited_shows_tonkotsus_keys() {
        let (_tmp, mut app) = editing();
        app.editor.as_mut().unwrap().active_form =
            Some(crate::tui::screens::editor_bridge::ActiveNodeForm {
                node_index: 0,
                params: vec![],
                form: editing_form(),
            });
        assert_eq!(footer_hints(&app), [("Enter", "save"), ("Esc", "cancel")]);
    }

    #[test]
    fn a_wizard_field_being_edited_shows_tonkotsus_keys() {
        let (_tmp, mut app) = model(Screen::Wizard {
            from: DetailOrigin::Home,
        });
        let mut wizard = crate::tui::screens::wizard::WizardModel::new(&app.registry);
        wizard.form = Some(editing_form());
        app.wizard = Some(wizard);
        assert_eq!(footer_hints(&app), [("Enter", "save"), ("Esc", "cancel")]);
    }

    #[test]
    fn a_library_search_shows_the_search_keys() {
        let (_tmp, mut app) = library();
        app.library.as_mut().unwrap().searching = true;
        assert_eq!(footer_hints(&app), SEARCH);
    }

    #[test]
    fn the_editors_save_question_shows_only_its_answers() {
        // RED (ISS-101): the footer kept the editor's navigation keys.
        let (_tmp, mut app) = editing();
        app.editor.as_mut().unwrap().confirming_dirty_exit = true;
        assert_eq!(
            footer_hints(&app),
            [("y", "save"), ("n", "discard"), ("Esc", "stay")]
        );
    }

    #[test]
    fn the_editors_delete_question_shows_only_its_answers() {
        let (_tmp, mut app) = editing();
        app.editor.as_mut().unwrap().confirming_delete = true;
        assert_eq!(footer_hints(&app), [("y", "delete"), ("n/Esc", "keep")]);
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

    fn library() -> (tempfile::TempDir, AppModel) {
        let (tmp, app) = model(Screen::Home);
        let app = crate::tui::app::update(app, crate::tui::app::AppMessage::OpenLibrary);
        (tmp, app)
    }

    #[test]
    fn the_librarys_delete_question_shows_only_its_answers() {
        let (_tmp, mut app) = library();
        app.library.as_mut().unwrap().confirming_delete = Some(0);
        assert_eq!(footer_hints(&app), [("y", "delete"), ("n/Esc", "keep")]);
    }

    #[test]
    fn a_library_rename_shows_its_keys() {
        let (_tmp, mut app) = library();
        app.library.as_mut().unwrap().renaming = Some((0, "draft".into()));
        assert_eq!(
            footer_hints(&app),
            [("Enter", "save name"), ("Esc", "cancel")]
        );
    }

    #[test]
    fn a_run_in_progress_offers_cancel_and_quit() {
        // Enter does nothing until the run ends.
        let (_tmp, mut app) = model(Screen::Execution {
            slug: "s".into(),
            from: DetailOrigin::Home,
        });
        let mut exec = crate::tui::screens::execution::ExecutionModel::new("s");
        exec.status = ExecutionStatus::Running;
        app.execution = Some(exec);
        assert_eq!(keys(&footer_hints(&app)), ["Esc", "q"]);
    }
}
