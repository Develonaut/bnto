// Text snapshots of whole TUI screens at fixed terminal sizes: the TUI's
// counterpart to the web's page screenshots (.claude/strategy/tui-testing.md).
// Review changes with `cargo insta review`.

use std::collections::HashMap;
use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Terminal, backend::TestBackend};

use super::app::{AppMessage, AppModel, DetailOrigin, Screen, update};
use super::keys::handle_key;
use super::render::draw_frame;
use super::screens::execution::{self, ExecutionMessage, ExecutionModel};
use super::screens::picker::PickerModel;
use super::screens::results::ResultsModel;
use super::theme::ThemeVariant;
use crate::run_message::RunMessage;
use crate::storage::paths::BntoPaths;
use crate::trust_gate::ConsentRequest;

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

// --- Run states: the execution and results screens mid-run and after ---

const LOOP_RECIPE: &str =
    include_str!("../../tests/fixtures/explicit/compress-and-rename.bnto.json");
const RUN_FILES: [&str; 3] = ["beach.jpg", "sunset.jpg", "lake.jpg"];

/// The execution screen after `events`, for a run over [`RUN_FILES`].
fn running(definition_json: &str, events: &[ExecutionMessage]) -> Session {
    let files = RUN_FILES
        .iter()
        .map(|f| PathBuf::from("/in").join(f))
        .collect();
    let mut model =
        ExecutionModel::with_inputs("run", files, HashMap::new(), definition_json.to_string());
    for event in events {
        model = execution::update(model, event.clone());
    }
    in_execution(Session::new(), model)
}

fn in_execution(mut session: Session, model: ExecutionModel) -> Session {
    session.model.screen = Screen::Execution {
        slug: "run".into(),
        from: DetailOrigin::Home,
    };
    session.model.execution = Some(model);
    session
}

/// Iteration 2 of 3 of an explicit loop: the first file done, the second
/// compressing, the third waiting, and the loop's later step still pending.
fn loop_mid_run() -> Vec<ExecutionMessage> {
    use ExecutionMessage as M;
    let iteration = |i| M::IterationStarted {
        node_id: "process-loop".into(),
        iteration: i,
        total_iterations: 3,
    };
    let child = |id: &str| M::ChildNodeStarted {
        parent_node_id: "process-loop".into(),
        node_id: id.into(),
        node_type: "image-compress".into(),
    };
    vec![
        M::PipelineStarted {
            total_nodes: 1,
            total_files: 3,
            node_info: vec![("process-loop".into(), "loop".into())],
        },
        M::NodeStarted {
            node_id: "process-loop".into(),
            node_type: "loop".into(),
        },
        iteration(0),
        child("compress"),
        M::IterationCompleted {
            node_id: "process-loop".into(),
            iteration: 0,
            total_iterations: 3,
            duration_ms: 40,
            files_produced: 1,
            output_files: vec![],
        },
        iteration(1),
        child("compress"),
        M::FileProgress {
            node_id: "process-loop".into(),
            file_index: 1,
            total_files: 3,
            percent: 50,
            message: "Compressing sunset.jpg...".into(),
        },
        M::CommandOutput {
            node_id: "compress".into(),
            line: "Compressing JPEG (quality: 80)...".into(),
        },
    ]
}

/// File 12 of 20 compressing: more files than fit, with output still showing.
fn many_files() -> Session {
    use ExecutionMessage as M;
    let files = (1..=20)
        .map(|i| PathBuf::from(format!("/in/photo{i:02}.jpg")))
        .collect();
    let mut model = ExecutionModel::with_inputs("run", files, HashMap::new(), "{}".into());
    let mut events = vec![
        M::PipelineStarted {
            total_nodes: 1,
            total_files: 20,
            node_info: vec![("c".into(), "image-compress".into())],
        },
        M::NodeStarted {
            node_id: "c".into(),
            node_type: "image-compress".into(),
        },
    ];
    events.extend((0..12).map(|i| M::FileProgress {
        node_id: "c".into(),
        file_index: i,
        total_files: 20,
        percent: 50,
        message: format!("Compressing photo{:02}.jpg...", i + 1),
    }));
    events.push(M::CommandOutput {
        node_id: "c".into(),
        line: "Compressing JPEG (quality: 80)...".into(),
    });
    for event in events {
        model = execution::update(model, event);
    }
    in_execution(Session::new(), model)
}

/// A single step that failed partway through the first file.
fn failed_run() -> Vec<ExecutionMessage> {
    use ExecutionMessage as M;
    vec![
        M::PipelineStarted {
            total_nodes: 1,
            total_files: 3,
            node_info: vec![("talk".into(), "shell-run".into())],
        },
        M::NodeStarted {
            node_id: "talk".into(),
            node_type: "shell-run".into(),
        },
        M::CommandOutput {
            node_id: "talk".into(),
            line: "fetching page 1...".into(),
        },
        M::NodeFailed {
            node_id: "talk".into(),
            error: "command exited with status 2".into(),
        },
        M::PipelineFailed {
            node_id: "talk".into(),
            error: "command exited with status 2".into(),
        },
    ]
}

/// A library shell recipe waiting for the user to allow its script.
fn consent() -> Session {
    let request = ConsentRequest {
        recipe_name: "Slow Shell".into(),
        commands: vec![
            "sh -c <script> bnto\n    script:\n      for i in 1 2 3; do echo \"line $i\"; done"
                .into(),
        ],
        hash: "h".into(),
    };
    in_execution(
        Session::new(),
        ExecutionModel::new("run").awaiting_consent(request),
    )
}

/// keep-awake's results: its session log in place of an empty file list.
fn results_with_message() -> Session {
    let message = RunMessage {
        label: Some("Session Log".into()),
        text: "Session started: staying awake for 1 hour.\nActivity simulation: off".into(),
    };
    let mut session = Session::new();
    session.model.screen = Screen::Results {
        slug: "keep-awake".into(),
    };
    session.model.results =
        Some(ResultsModel::new("keep-awake", vec![], 26, None).with_report(Some(message), vec![]));
    session
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
        (
            "execution_loop_running",
            running(LOOP_RECIPE, &loop_mid_run()),
        ),
        ("execution_failed", running("{}", &failed_run())),
        ("execution_many_files", many_files()),
        ("execution_consent", consent()),
        ("results_with_message", results_with_message()),
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
