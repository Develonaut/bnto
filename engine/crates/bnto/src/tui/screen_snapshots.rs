// Text snapshots of whole TUI screens at fixed terminal sizes: the TUI's
// counterpart to the web's page screenshots (.claude/strategy/tui-testing.md).
// Review changes with `cargo insta review`.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Terminal, backend::TestBackend};

use super::app::{AppMessage, AppModel, update};
use super::keys::handle_key;
use super::render::draw_frame;
use super::theme::ThemeVariant;
use crate::storage::paths::BntoPaths;

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

/// Snapshot `session` at every size. The version is masked so a release bump
/// doesn't rewrite every snapshot, and the working directory (a file
/// recipe's default input) so snapshots match on every machine.
fn snapshot(name: &str, session: &Session) {
    let cwd = std::env::current_dir().unwrap().display().to_string();
    let filters = [
        (r"v\d+\.\d+\.\d+".to_string(), "v[version]"),
        // Settings reads telemetry from the real ~/.config/bnto, not BNTO_HOME.
        (r"◂ (On|Off) ▸".to_string(), "◂ [telemetry] ▸"),
        // With its padding: the path's length differs between machines.
        (format!("{} *", regex_literal(&cwd)), "[cwd] "),
    ];
    let filters: Vec<(&str, &str)> = filters.iter().map(|(p, r)| (p.as_str(), *r)).collect();
    for (width, height) in SIZES {
        insta::with_settings!({ filters => filters.clone() }, {
            insta::assert_snapshot!(format!("{name}_{width}x{height}"), session.screen(width, height));
        });
    }
}

/// `text` as a regex that matches it literally (insta filters are regexes).
fn regex_literal(text: &str) -> String {
    text.chars()
        .flat_map(|c| {
            let special = "\\.+*?()|[]{}^$".contains(c);
            special.then_some('\\').into_iter().chain([c])
        })
        .collect()
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
        ("detail_compress_images", detail("compress-images")),
        (
            "library_empty",
            Session::new().send(AppMessage::OpenLibrary),
        ),
        ("settings", Session::new().send(AppMessage::OpenSettings)),
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
            let status = format!("recipes  ·  v{}", env!("CARGO_PKG_VERSION"));
            let at = bottom
                .find(&status)
                .unwrap_or_else(|| panic!("{name} {width}x{height}: status cut: {bottom:?}"));
            // What's left of the status: the hints, then at least two spaces.
            let hints = bottom[..at].trim_end().trim_end_matches(char::is_numeric);
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
