// One frame on screen: fit the detail view, draw, and size scrolling lists
// to the terminal.

use std::io;
use std::sync::Arc;
use std::time::Instant;

use bnto_core::logging::{LogEntry, LogLevel, Logger};

use super::SLOW_FRAME_THRESHOLD_US;
use crate::tui::app::{AppMessage, AppModel, Screen, update};
use crate::tui::screens::browser::BrowserMessage;
use crate::tui::screens::picker::PickerMessage;
use crate::tui::terminal::TuiTerminal;
use crate::tui::{detail_fit, render, render_layout};

/// Lines a list screen draws above its list: the search bar or folder path,
/// then a blank line.
const LIST_SCREEN_CHROME: u16 = 2;

/// Draw the current screen, then size its list to the terminal.
pub(super) fn render(
    terminal: &mut TuiTerminal,
    mut model: AppModel,
    logger: &Arc<dyn Logger>,
) -> io::Result<AppModel> {
    // Before drawing, so a key that moved focus shows it scrolled into view.
    fit_detail(terminal, &mut model)?;
    draw(terminal, &model, logger)?;
    resize_list(terminal, model)
}

fn fit_detail(terminal: &mut TuiTerminal, model: &mut AppModel) -> io::Result<()> {
    if matches!(model.screen, Screen::Detail { .. })
        && let Some(detail) = &mut model.detail
    {
        detail_fit::fit_detail(detail, &model.theme, terminal.size()?);
    }
    Ok(())
}

fn draw(terminal: &mut TuiTerminal, model: &AppModel, logger: &Arc<dyn Logger>) -> io::Result<()> {
    let draw_start = Instant::now();
    terminal.draw(|frame| render::draw_frame(frame, model))?;
    let draw_us = draw_start.elapsed().as_micros() as u64;
    if draw_us >= SLOW_FRAME_THRESHOLD_US {
        logger.log(LogEntry {
            level: LogLevel::Warn,
            target: "tui",
            message: format!("slow frame render (screen={})", model.screen.log_label()),
            elapsed_us: Some(draw_us),
        });
    }
    Ok(())
}

/// Tell the browser or picker how many rows its list has.
fn resize_list(terminal: &mut TuiTerminal, model: AppModel) -> io::Result<AppModel> {
    if !matches!(model.screen, Screen::Browser | Screen::Picker { .. }) {
        return Ok(model);
    }
    match viewport_resize(&model.screen, terminal.size()?.height) {
        Some(msg) => Ok(update(model, msg)),
        None => Ok(model),
    }
}

/// The resize message for a screen with a scrolling list, if it has room.
pub(super) fn viewport_resize(screen: &Screen, term_height: u16) -> Option<AppMessage> {
    let height = render_layout::viewport_height(term_height, LIST_SCREEN_CHROME);
    if height == 0 {
        return None;
    }
    match screen {
        Screen::Browser => Some(AppMessage::Browser(BrowserMessage::Resize { height })),
        Screen::Picker { .. } => Some(AppMessage::Picker(PickerMessage::Resize { height })),
        _ => None,
    }
}
