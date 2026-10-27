use super::bridge_events::{RunBridge, advance, apply_bridge_event, bridge_finished};
use super::frame::viewport_resize;
use crate::trust_gate::ConsentRequest;
use crate::tui::app::update;
use crate::tui::app::{AppMessage, AppModel, DetailOrigin, Screen};
use crate::tui::bridge::{BridgeEvent, RunDone};
use crate::tui::render_layout;
use crate::tui::screens::browser::BrowserMessage;
use crate::tui::screens::execution::ExecutionMessage;
use crate::tui::screens::execution::{ExecutionModel, ExecutionStatus};
use crate::tui::screens::picker::PickerMessage;
use crate::tui::theme::ThemeVariant;

fn model_on_execution(dir: &tempfile::TempDir, preview_mode: bool) -> AppModel {
    let paths = crate::storage::BntoPaths {
        home: dir.path().to_path_buf(),
    };
    let mut execution = ExecutionModel::new("s");
    execution.status = ExecutionStatus::Running;
    execution.preview_mode = preview_mode;
    AppModel {
        screen: Screen::Execution {
            slug: "s".into(),
            from: DetailOrigin::Browser,
        },
        execution: Some(execution),
        ..AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths)
    }
}

fn done(output_dir: &str) -> RunDone {
    RunDone {
        output_dir: output_dir.into(),
        in_place: false,
        file_count: 2,
        duration_ms: 1500,
        file_metadata: vec![],
        warnings: vec!["skipped one".into()],
        preview_data: vec![],
        message: None,
    }
}

// --- Bridge events ---

#[test]
fn an_error_fails_the_execution() {
    let dir = tempfile::tempdir().unwrap();
    let model = apply_bridge_event(
        model_on_execution(&dir, false),
        BridgeEvent::Error("boom".into()),
    );
    let execution = model.execution.expect("still on the execution screen");
    assert_eq!(execution.status, ExecutionStatus::Failed);
    assert_eq!(execution.error.as_deref(), Some("boom"));
}

#[test]
fn done_in_preview_mode_opens_the_preview_screen() {
    let dir = tempfile::tempdir().unwrap();
    let model = apply_bridge_event(
        model_on_execution(&dir, true),
        BridgeEvent::Done(done("/out")),
    );
    assert_eq!(
        model.screen,
        Screen::Preview {
            slug: "s".into(),
            from: DetailOrigin::Browser,
        }
    );
    assert!(model.execution.is_none());
    let preview = model.preview.expect("preview built");
    assert_eq!(preview.duration_ms, 1500);
    assert_eq!(preview.warnings, vec!["skipped one".to_string()]);
}

#[test]
fn done_in_normal_mode_reaches_results_with_the_output_dir() {
    let dir = tempfile::tempdir().unwrap();
    let model = apply_bridge_event(
        model_on_execution(&dir, false),
        BridgeEvent::Done(done("/out")),
    );
    assert_eq!(model.screen, Screen::Results { slug: "s".into() });
    assert!(model.execution.is_none());
    let results = model.results.expect("results built");
    assert_eq!(results.output_dir.as_deref(), Some("/out"));
    assert_eq!(results.total_time_ms, 1500);
}

#[test]
fn done_in_place_reaches_results_with_no_one_output_folder() {
    // Overwrite mode's output_dir is only the first file's folder.
    let dir = tempfile::tempdir().unwrap();
    let in_place = RunDone {
        in_place: true,
        ..done("/first-folder")
    };
    let model = apply_bridge_event(model_on_execution(&dir, false), BridgeEvent::Done(in_place));
    let results = model.results.expect("results built");
    assert_eq!(results.output_dir, None);
}

// --- Consent the bridge asks for (ISS-77) ---

/// A run whose bridge is checking it, and the bridge's channel.
fn checking(
    dir: &tempfile::TempDir,
) -> (AppModel, RunBridge, std::sync::mpsc::Sender<BridgeEvent>) {
    let mut model = model_on_execution(dir, false);
    model.execution.as_mut().unwrap().status = ExecutionStatus::Idle;
    let (tx, rx) = std::sync::mpsc::channel();
    (model, RunBridge::listening(rx), tx)
}

fn consent_needed(tx: &std::sync::mpsc::Sender<BridgeEvent>) -> std::sync::mpsc::Receiver<()> {
    let (reply, answer) = std::sync::mpsc::channel();
    let request = ConsentRequest {
        recipe_name: "Probe".into(),
        commands: vec!["git --version".into()],
        hash: "h".into(),
    };
    tx.send(BridgeEvent::ConsentNeeded { request, reply })
        .unwrap();
    answer
}

#[test]
fn the_bridges_request_holds_the_run_until_the_user_allows_it() {
    let dir = tempfile::tempdir().unwrap();
    let (model, mut run, tx) = checking(&dir);
    let answer = consent_needed(&tx);
    let model = advance(model, &mut run);
    let exec = model.execution.as_ref().unwrap();
    assert_eq!(exec.status, ExecutionStatus::AwaitingConsent);
    assert_eq!(
        exec.consent.as_ref().unwrap().commands,
        vec!["git --version"]
    );
    assert!(answer.try_recv().is_err(), "answered before the user did");

    let allowed_at = std::time::Instant::now();
    let model = update(
        model,
        AppMessage::Execution(ExecutionMessage::AllowCommands),
    );
    let _model = advance(model, &mut run);
    assert!(
        answer.try_recv().is_ok(),
        "the bridge was not told to go on"
    );
    // The timer starts at Allow: the time on the prompt isn't the run's.
    assert!(run.started().is_some_and(|t| t >= allowed_at));
}

#[test]
fn declining_the_bridges_request_lets_the_bridge_go() {
    // Letting it go drops the reply, which stops the bridge (tui/bridge.rs).
    let dir = tempfile::tempdir().unwrap();
    let (model, mut run, tx) = checking(&dir);
    let _answer = consent_needed(&tx);
    let model = advance(model, &mut run);
    let model = update(model, AppMessage::Execution(ExecutionMessage::Cancel));
    assert!(bridge_finished(&model));
}

// --- When the bridge is let go ---

#[test]
fn the_bridge_is_kept_while_the_run_is_on_screen() {
    let dir = tempfile::tempdir().unwrap();
    let model = model_on_execution(&dir, false);
    assert!(!bridge_finished(&model));
    let mut completed = model_on_execution(&dir, false);
    completed.execution.as_mut().unwrap().status = ExecutionStatus::Completed;
    assert!(!bridge_finished(&completed), "Done may still be on its way");
}

#[test]
fn the_bridge_is_let_go_after_a_cancel_or_leaving_the_screen() {
    let dir = tempfile::tempdir().unwrap();
    let mut cancelled = model_on_execution(&dir, false);
    cancelled.execution.as_mut().unwrap().status = ExecutionStatus::Cancelled;
    assert!(bridge_finished(&cancelled));
    let left = AppModel {
        screen: Screen::Home,
        ..model_on_execution(&dir, false)
    };
    assert!(bridge_finished(&left));
}

// --- Viewport resizes ---

#[test]
fn the_browser_and_picker_get_their_viewport_height() {
    let height = render_layout::viewport_height(30, 2);
    assert_eq!(
        viewport_resize(&Screen::Browser, 30),
        Some(AppMessage::Browser(BrowserMessage::Resize { height }))
    );
    let picker = Screen::Picker {
        slug: "s".into(),
        from: DetailOrigin::Browser,
    };
    assert_eq!(
        viewport_resize(&picker, 30),
        Some(AppMessage::Picker(PickerMessage::Resize { height }))
    );
}

#[test]
fn other_screens_and_a_terminal_too_short_get_no_resize() {
    assert_eq!(viewport_resize(&Screen::Home, 30), None);
    assert_eq!(viewport_resize(&Screen::Browser, 0), None);
}

// --- Quitting mid-write ---

#[test]
fn quitting_waits_for_a_write_in_progress_and_starts_no_new_one() {
    // ISS-48 review: Ctrl+C / q mid-write ended the session while the bridge
    // thread was copying, and `main` returning killed it mid-swap.
    use std::sync::mpsc;
    use std::time::Duration;

    let gate = crate::ordered_write::WriteGate::new();
    let commands = crate::process_registry::ProcessRegistry::new();
    let (ended_tx, ended_rx) = mpsc::channel();
    std::thread::scope(|s| {
        let bridge_writing = gate.enter().expect("no quit yet");
        s.spawn(|| {
            super::end_session(&gate, &commands);
            ended_tx.send(()).unwrap();
        });
        let early = ended_rx.recv_timeout(Duration::from_millis(100));
        assert!(early.is_err(), "the session ended under a running write");
        drop(bridge_writing);
        ended_rx.recv_timeout(Duration::from_secs(5)).unwrap();
    });
    assert!(gate.enter().is_none(), "a write started after quitting");
}
