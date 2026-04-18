//! Test helpers for bnto-form journey tests.
//!
//! Provides key constructors, simulation helpers, and terminal buffer
//! assertion helpers. Uses only the public API surface of bnto-form.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::text::Text;
use ratatui::widgets::Paragraph;

use bnto_form::{FormMessage, FormModel, map_key_event, render_form, update};

// --- Key constructors ---

pub fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

pub fn char_key(ch: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE)
}

pub fn ctrl(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::CONTROL)
}

pub fn shift_tab() -> KeyEvent {
    KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT)
}

// --- Simulation ---

/// Feed one key event through the full pipeline: map_key_event → update.
pub fn simulate_key(model: FormModel, key: KeyEvent) -> FormModel {
    match map_key_event(key, &model) {
        Some(msg) => update(model, msg),
        None => model,
    }
}

/// Feed a sequence of key events through the pipeline.
pub fn simulate_keys(mut model: FormModel, keys: &[KeyEvent]) -> FormModel {
    for k in keys {
        model = simulate_key(model, *k);
    }
    model
}

// --- Rendering ---

/// Render the form into a ratatui `Buffer` via `TestBackend`.
pub fn render_to_buffer(model: &FormModel, width: u16, height: u16) -> Buffer {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).expect("terminal creation failed");
    terminal
        .draw(|frame| {
            let lines = render_form(model);
            let text = Text::from(lines);
            let paragraph = Paragraph::new(text);
            frame.render_widget(paragraph, frame.area());
        })
        .expect("draw failed");
    terminal.backend().buffer().clone()
}

/// Extract a single row from the buffer as a trimmed-end string.
pub fn buffer_line(buffer: &Buffer, row: u16) -> String {
    let width = buffer.area.width;
    let mut line = String::new();
    for x in 0..width {
        let cell = &buffer[(x, row)];
        line.push_str(cell.symbol());
    }
    line.trim_end().to_string()
}

/// Extract the full buffer as text (all rows joined by newlines).
pub fn buffer_text(buffer: &Buffer) -> String {
    let height = buffer.area.height;
    (0..height)
        .map(|y| buffer_line(buffer, y))
        .collect::<Vec<_>>()
        .join("\n")
}

// --- Assertions ---

pub fn assert_line_contains(buffer: &Buffer, row: u16, expected: &str) {
    let line = buffer_line(buffer, row);
    assert!(
        line.contains(expected),
        "row {row} does not contain {expected:?}. actual: {line:?}"
    );
}

pub fn assert_buffer_contains(buffer: &Buffer, expected: &str) {
    let text = buffer_text(buffer);
    assert!(
        text.contains(expected),
        "buffer does not contain {expected:?}. buffer:\n{text}"
    );
}

pub fn assert_buffer_not_contains(buffer: &Buffer, expected: &str) {
    let text = buffer_text(buffer);
    assert!(
        !text.contains(expected),
        "buffer unexpectedly contains {expected:?}. buffer:\n{text}"
    );
}

/// Send a FormMessage directly (bypassing key mapping).
#[allow(dead_code)]
pub fn send_msg(model: FormModel, msg: FormMessage) -> FormModel {
    update(model, msg)
}
