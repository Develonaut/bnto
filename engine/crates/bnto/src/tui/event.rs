// Event loop — reads crossterm terminal events and dispatches messages.
//
// Thin translation layer: terminal keypresses → AppMessage or screen-specific
// messages. No business logic lives here.

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use super::app::AppMessage;

/// Poll for a terminal event with a timeout.
/// Returns `None` if no event is available within the timeout.
pub fn poll_event(timeout: Duration) -> std::io::Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}

/// Map a global key event to an AppMessage.
/// Returns `None` if the key doesn't map to a global action.
pub fn map_global_key(key: KeyEvent) -> Option<AppMessage> {
    match key.code {
        KeyCode::Char('q') => Some(AppMessage::Quit),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(AppMessage::Quit)
        }
        KeyCode::Esc => Some(AppMessage::Back),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn ctrl_key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::CONTROL)
    }

    #[test]
    fn q_maps_to_quit() {
        assert_eq!(
            map_global_key(key(KeyCode::Char('q'))),
            Some(AppMessage::Quit)
        );
    }

    #[test]
    fn ctrl_c_maps_to_quit() {
        assert_eq!(
            map_global_key(ctrl_key(KeyCode::Char('c'))),
            Some(AppMessage::Quit)
        );
    }

    #[test]
    fn esc_maps_to_back() {
        assert_eq!(map_global_key(key(KeyCode::Esc)), Some(AppMessage::Back));
    }

    #[test]
    fn unmapped_key_returns_none() {
        assert_eq!(map_global_key(key(KeyCode::Char('x'))), None);
    }
}
