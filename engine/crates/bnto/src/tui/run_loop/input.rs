// Keys: wait up to one tick for a key press and apply it to the model.

use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bnto_core::logging::{LogEntry, LogLevel, Logger};
use crossterm::event::Event;

use super::SLOW_FRAME_THRESHOLD_US;
use crate::tui::app::{AppMessage, AppModel, update};
use crate::tui::event;
use crate::tui::keys::handle_key;

/// How long the loop waits for input before drawing the next frame.
const TICK_RATE: Duration = Duration::from_millis(50);

/// Apply the next key press, if one arrives within a tick.
pub(super) fn handle_input(model: AppModel, logger: &Arc<dyn Logger>) -> io::Result<AppModel> {
    let Some(Event::Key(key)) = event::poll_event(TICK_RATE)? else {
        return Ok(model);
    };
    let Some(msg) = handle_key(&model, key) else {
        return Ok(model);
    };
    let prev_screen = model.screen.log_label();
    let update_start = Instant::now();
    let model = update(model, msg.clone());
    let update_us = update_start.elapsed().as_micros() as u64;
    log_update(logger, prev_screen, &model, &msg, update_us);
    Ok(model)
}

/// Log a screen change, or an update slow enough to notice.
fn log_update(
    logger: &Arc<dyn Logger>,
    prev_screen: &str,
    model: &AppModel,
    msg: &AppMessage,
    update_us: u64,
) {
    let new_screen = model.screen.log_label();
    let (level, message) = if prev_screen != new_screen {
        (LogLevel::Info, format!("{prev_screen} → {new_screen}"))
    } else if update_us >= SLOW_FRAME_THRESHOLD_US {
        (LogLevel::Debug, format!("update on {prev_screen}: {msg:?}"))
    } else {
        return;
    };
    logger.log(LogEntry {
        level,
        target: "tui",
        message,
        elapsed_us: Some(update_us),
    });
}
