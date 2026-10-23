// The TUI's event loop: draw, feed the model what happened since the last
// frame (resizes, update notices, run progress, keys), repeat until quit.

mod bridge_events;
mod finish;
mod frame;
mod input;

use std::io;
use std::sync::{Arc, mpsc};

use bnto_core::logging::{LogEntry, LogLevel, Logger};

use super::app::{AppMessage, AppModel, update};
use super::terminal::TuiTerminal;
use super::theme::ThemeVariant;
use crate::storage::BntoPaths;
use bridge_events::RunBridge;

/// Render or update time in microseconds above which a frame is logged.
const SLOW_FRAME_THRESHOLD_US: u64 = 16_000;

/// Main event loop — poll input, update state, render.
pub(super) fn run_loop(
    terminal: &mut TuiTerminal,
    variant: ThemeVariant,
    recipe_json: Option<String>,
    new_recipe: bool,
    logger: &Arc<dyn Logger>,
    paths: Option<BntoPaths>,
) -> io::Result<()> {
    let mut model = new_model(variant, recipe_json, new_recipe, paths);
    let (update_notice, mut update_rx) = crate::update_check::for_tui(&model.paths);
    model.update_notice = update_notice;
    let mut run = RunBridge::default();
    let label = model.screen.log_label();
    log_session(logger, format!("session start (screen={label})"));
    loop {
        model = frame::render(terminal, model, logger)?;
        model = poll_update_notice(model, &mut update_rx);
        model = bridge_events::advance(model, &mut run);
        if model.should_quit {
            break;
        }
        model = input::handle_input(model, logger)?;
    }
    // Quitting mid-execution must not orphan spawned commands (no-op when
    // nothing is running).
    crate::process_registry::global().terminate_all();
    log_session(logger, "session end".into());
    Ok(())
}

/// The model the TUI starts on.
fn new_model(
    variant: ThemeVariant,
    recipe_json: Option<String>,
    new_recipe: bool,
    paths: Option<BntoPaths>,
) -> AppModel {
    match paths {
        Some(p) => AppModel::with_paths(variant, recipe_json, new_recipe, p),
        None => AppModel::new(variant, recipe_json, new_recipe),
    }
}

/// Apply a newer release the background update watch found mid-session.
fn poll_update_notice(model: AppModel, rx: &mut Option<mpsc::Receiver<String>>) -> AppModel {
    match rx.as_ref().map(mpsc::Receiver::try_recv) {
        Some(Ok(notice)) => update(model, AppMessage::UpdateAvailable(notice)),
        Some(Err(mpsc::TryRecvError::Disconnected)) => {
            *rx = None;
            model
        }
        _ => model,
    }
}

/// Log a session milestone.
fn log_session(logger: &Arc<dyn Logger>, message: String) {
    logger.log(LogEntry {
        level: LogLevel::Info,
        target: "tui",
        message,
        elapsed_us: None,
    });
}

#[cfg(test)]
mod tests;
