// The terminal the TUI draws on: raw mode and the alternate screen on the way
// in, both undone on the way out, and on a panic before its message prints.

use std::io;

use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event as crossterm_event, execute};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

/// The terminal the TUI renders to (stderr, so stdout stays free).
pub(super) type TuiTerminal = Terminal<CrosstermBackend<io::Stderr>>;

/// Install a panic hook that restores the terminal before printing the panic.
pub(super) fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stderr(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}

/// Enter raw mode, switch to alternate screen, create terminal.
pub(super) fn setup() -> io::Result<TuiTerminal> {
    terminal::enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    execute!(stderr, crossterm_event::DisableMouseCapture)?;
    let backend = CrosstermBackend::new(io::stderr());
    Terminal::new(backend)
}

/// Leave alternate screen, disable raw mode, show cursor.
pub(super) fn restore(terminal: &mut TuiTerminal) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}
