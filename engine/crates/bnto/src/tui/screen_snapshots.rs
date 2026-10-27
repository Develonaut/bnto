// Text snapshots of whole TUI screens at fixed terminal sizes: the TUI's
// counterpart to the web's page screenshots (.claude/strategy/tui-testing.md).
// Review changes with `cargo insta review`.

use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Terminal, backend::TestBackend};

use super::app::{AppMessage, AppModel, DetailOrigin, Screen, update};
use super::keys::handle_key;
use super::render::draw_frame;
use super::screens::editor::EditorScreenModel;
use super::screens::picker::PickerModel;
use super::theme::ThemeVariant;
use crate::storage::paths::BntoPaths;

mod run_states;

use run_states::{
    LOOP_RECIPE, consent, failed_run, loop_mid_run, many_files, results_with_message,
    results_written_in_place, running,
};

/// The terminal sizes every screen is snapshotted at. 80x24 is the classic
/// default and catches overflow; 120x36 is a typical modern window.
const SIZES: [(u16, u16); 2] = [(120, 36), (80, 24)];

/// A TUI session over a throwaway bnto home, driven in-process.
struct Session {
    model: AppModel,
    _home: tempfile::TempDir,
}

impl Session {
    fn new() -> Self {
        let home = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            home: home.path().to_path_buf(),
        };
        let model = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        Self { model, _home: home }
    }

    fn send(mut self, msg: AppMessage) -> Self {
        self.model = update(self.model, msg);
        self
    }

    /// Press `keys` in order, as the running TUI would map them.
    fn press(mut self, keys: &[KeyCode]) -> Self {
        for &code in keys {
            let key = KeyEvent::new(code, KeyModifiers::NONE);
            if let Some(msg) = handle_key(&self.model, key) {
                self.model = update(self.model, msg);
            }
        }
        self
    }

    /// Root the open recipe's file picker in a folder holding `names`, shown
    /// as `~/photos`, so a snapshot is the same in every checkout: no
    /// working directory, no listing of whatever sits next to it.
    fn with_inputs(mut self, names: &[&str]) -> Self {
        let dir = self._home.path().join("photos");
        std::fs::create_dir_all(&dir).unwrap();
        for name in names {
            std::fs::write(dir.join(name), b"").unwrap();
        }
        let detail = self.model.detail.as_mut().expect("a recipe is open");
        let mut picker = PickerModel::from_slug(
            &detail.slug,
            &dir,
            &self.model.registry,
            &detail.definition_json,
        );
        picker.current_dir = PathBuf::from("~/photos");
        detail.input_picker = Some(picker);
        self
    }

    /// The screen's text, one line per row, trailing spaces trimmed.
    fn screen(&self, width: u16, height: u16) -> String {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|f| draw_frame(f, &self.model)).unwrap();
        let buffer = terminal.backend().buffer();
        (0..height)
            .map(|y| {
                let row: String = (0..width).map(|x| buffer[(x, y)].symbol()).collect();
                row.trim_end().to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Snapshot `session` at every size, with the version masked so a release
/// bump doesn't rewrite every snapshot.
fn snapshot(name: &str, session: &Session) {
    for (width, height) in SIZES {
        insta::with_settings!({ filters => vec![(r"v\d+\.\d+\.\d+", "v[version]")] }, {
            insta::assert_snapshot!(format!("{name}_{width}x{height}"), session.screen(width, height));
        });
    }
}

/// Home while a newer bnto is out: the status line names it.
fn update_available() -> Session {
    let mut session = Session::new();
    session.model.update_notice = Some("bnto 99.0.0 available, U to upgrade".into());
    session
}

/// The editor on a modified recipe after `q`: it asks to save (ISS-98).
fn editor_asking_to_save() -> Session {
    let mut session = Session::new();
    let mut editor = bnto_core::editor::EditorModel::new();
    editor.recipe_name = "Draft".into();
    editor.dirty = true;
    session.model.screen = Screen::Editor {
        from: DetailOrigin::Home,
    };
    session.model.editor = Some(EditorScreenModel::new(editor));
    session.press(&[KeyCode::Char('q')])
}

/// Each screen the snapshots cover, by name.
fn screens() -> Vec<(&'static str, Session)> {
    let detail = |slug: &str| {
        Session::new().send(AppMessage::RecipeSelected {
            slug: slug.to_string(),
        })
    };
    vec![
        ("home", Session::new()),
        (
            "home_recipes_focused",
            Session::new().press(&[KeyCode::Tab]),
        ),
        ("detail_keep_awake", detail("keep-awake")),
        ("detail_download_video", detail("download-video")),
        (
            "detail_compress_images",
            detail("compress-images").with_inputs(&["beach.jpg", "notes.txt", "sunset.png"]),
        ),
        (
            "library_empty",
            Session::new().send(AppMessage::OpenLibrary),
        ),
        ("settings", Session::new().send(AppMessage::OpenSettings)),
        ("home_update_available", update_available()),
        (
            "execution_loop_running",
            running(LOOP_RECIPE, &loop_mid_run()),
        ),
        ("execution_failed", running("{}", &failed_run())),
        ("execution_many_files", many_files()),
        ("execution_consent", consent()),
        ("results_with_message", results_with_message()),
        ("results_written_in_place", results_written_in_place()),
        ("editor_asking_to_save", editor_asking_to_save()),
    ]
}

#[test]
fn screens_match_their_snapshots() {
    for (name, session) in screens() {
        snapshot(name, &session);
    }
}

#[test]
fn the_bottom_bar_status_is_never_overwritten() {
    // Key hints and the status share the bottom row; long hints used to run
    // into the status ("Space se22 recipes").
    for (name, session) in screens() {
        for (width, height) in SIZES {
            let screen = session.screen(width, height);
            let bottom = screen.lines().rev().find(|l| !l.trim().is_empty()).unwrap();
            // The status is the counts line ("22 recipes  ·  v…"), or an
            // upgrade notice in its place. Left of it: hints, then 2+ spaces.
            let counts = format!("recipes  ·  v{}", env!("CARGO_PKG_VERSION"));
            let hints = if let Some(at) = bottom.find(&counts) {
                bottom[..at].trim_end().trim_end_matches(char::is_numeric)
            } else {
                let at = bottom
                    .find(" available, U to upgrade")
                    .and_then(|end| bottom[..end].rfind("bnto "))
                    .unwrap_or_else(|| panic!("{name} {width}x{height}: status cut: {bottom:?}"));
                &bottom[..at]
            };
            assert!(
                hints.trim().is_empty() || hints.ends_with("  "),
                "{name} {width}x{height}: hints run into the status: {bottom:?}"
            );
        }
    }
}

#[test]
fn long_descriptions_wrap_instead_of_being_cut() {
    // At 80 columns these used to end at the panel edge mid-word.
    let session = Session::new().send(AppMessage::RecipeSelected {
        slug: "keep-awake".to_string(),
    });
    let words = |text: &str| -> String {
        text.split(|c: char| c.is_whitespace() || c == '│')
            .filter(|w| !w.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    };
    let screen = words(&session.screen(80, 24));
    for full in [
        "Start a new session, stop the running one, or check what's currently active.",
        "By default it also nudges the cursor once you go idle",
    ] {
        assert!(screen.contains(full), "cut: {full:?}\n{screen}");
    }
}

#[test]
fn run_output_stays_visible_on_a_small_terminal() {
    // A tall FILES list used to take every row, leaving OUTPUT none.
    let words = |session: &Session| session.screen(80, 24);
    assert!(
        words(&running(LOOP_RECIPE, &loop_mid_run())).contains("Compressing JPEG (quality: 80)...")
    );
    assert!(words(&many_files()).contains("Compressing JPEG (quality: 80)..."));
    assert!(
        words(&many_files()).contains("Compressing photo12.jpg..."),
        "the active file stays in view"
    );
    assert!(words(&running("{}", &failed_run())).contains("fetching page 1..."));
}
