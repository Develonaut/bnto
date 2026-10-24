// A run's pipeline bridge: start it when the execution screen is ready,
// apply what it reports each frame, and let it go when the run is over.

use std::sync::mpsc;
use std::time::Instant;

use super::finish::finish_run;
use crate::tui::app::{AppMessage, AppModel, Screen, update};
use crate::tui::bridge::{self, BridgeEvent};
use crate::tui::screens::execution::{ExecutionMessage, ExecutionStatus};

/// The current run's bridge, while one is going.
#[derive(Default)]
pub(super) struct RunBridge {
    rx: Option<mpsc::Receiver<BridgeEvent>>,
    /// When the run started, for its elapsed timer.
    started: Option<Instant>,
}

/// One frame of run upkeep: start the bridge, apply its events, tick the
/// timer, and let the bridge go once the run is over.
pub(super) fn advance(model: AppModel, run: &mut RunBridge) -> AppModel {
    if run.rx.is_none() {
        start_if_idle(&model, run);
    }
    let model = drain(model, run);
    let model = tick(model, run.started);
    if run.rx.is_some() && bridge_finished(&model) {
        release(run);
    }
    model
}

/// Spawn the pipeline bridge when the execution screen is idle.
fn start_if_idle(model: &AppModel, run: &mut RunBridge) {
    if let Some(exec) = &model.execution
        && exec.status == ExecutionStatus::Idle
        && let Screen::Execution { slug, .. } = &model.screen
    {
        run.rx = Some(bridge::spawn_pipeline(
            slug.clone(),
            exec.definition_json.clone(),
            exec.selected_files.clone(),
            exec.param_overrides.clone(),
            exec.preview_mode,
        ));
        run.started = Some(Instant::now());
    }
}

/// Apply every event the bridge sent since the last frame (non-blocking).
fn drain(mut model: AppModel, run: &mut RunBridge) -> AppModel {
    let Some(rx) = &run.rx else { return model };
    while let Ok(event) = rx.try_recv() {
        model = apply_bridge_event(model, event);
    }
    model
}

/// The model after one bridge event. Progressive files are already on disk:
/// the bridge thread writes them before it sends the event.
pub(super) fn apply_bridge_event(model: AppModel, event: BridgeEvent) -> AppModel {
    match event {
        BridgeEvent::Progress(pe) => update(model, bridge::map_pipeline_event(pe)),
        BridgeEvent::Done(done) => finish_run(model, done),
        BridgeEvent::Error(error) => update(
            model,
            AppMessage::Execution(ExecutionMessage::PipelineFailed {
                node_id: String::new(),
                error,
            }),
        ),
    }
}

/// Keep the elapsed timer live while the run is going.
fn tick(model: AppModel, started: Option<Instant>) -> AppModel {
    let running = model
        .execution
        .as_ref()
        .is_some_and(|e| e.status == ExecutionStatus::Running);
    match started {
        Some(start) if running => {
            let elapsed_ms = start.elapsed().as_millis() as u64;
            update(
                model,
                AppMessage::Execution(ExecutionMessage::Tick { elapsed_ms }),
            )
        }
        _ => model,
    }
}

/// Whether to let the run's bridge go: after a cancel, or once the screen
/// has moved on (auto-transition fired). Not on Completed/Failed: the
/// bridge thread may still be writing files and hasn't sent `Done` yet.
pub(super) fn bridge_finished(model: &AppModel) -> bool {
    let cancelled = model
        .execution
        .as_ref()
        .is_some_and(|e| e.status == ExecutionStatus::Cancelled);
    cancelled || !matches!(model.screen, Screen::Execution { .. })
}

fn release(run: &mut RunBridge) {
    *run = RunBridge::default();
    // Abandoning a run must kill its spawned command tree — dropping the
    // receiver alone leaves children running.
    crate::process_registry::global().terminate_all();
}
