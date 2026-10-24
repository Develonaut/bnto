use super::bridge_events::{apply_bridge_event, bridge_finished};
use super::frame::viewport_resize;
use crate::tui::app::{AppMessage, AppModel, DetailOrigin, Screen};
use crate::tui::bridge::{BridgeEvent, RunDone};
use crate::tui::render_layout;
use crate::tui::screens::browser::BrowserMessage;
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
