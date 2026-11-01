// The terminal the TUI draws on: raw mode and the alternate screen on the way
// in, both undone on the way out, and on a panic before its message prints.

use std::io;

use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
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

/// Why the TUI can't start without a Windows console, and how to get one.
const NO_CONSOLE: &str = "this terminal gives bnto no Windows console to read keys from \
(Git Bash's MINGW64 window pipes them). Run `winpty bnto tui`, or start it from \
PowerShell or Windows Terminal.";

/// Enter raw mode, switch to alternate screen, create terminal.
pub(super) fn setup() -> io::Result<TuiTerminal> {
    require_console(stdin_is_console())?;
    terminal::enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    // On Windows crossterm runs this through the console API, which can only
    // restore a mode an `EnableMouseCapture` saved. The TUI never enables
    // capture, so there it always fails with "Initial console modes not set".
    #[cfg(not(windows))]
    execute!(stderr, crossterm::event::DisableMouseCapture)?;
    let backend = CrosstermBackend::new(io::stderr());
    Terminal::new(backend)
}

/// Leave alternate screen, disable raw mode, show cursor.
pub(super) fn restore(terminal: &mut TuiTerminal) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

/// Refuse to start before touching the terminal when keys can't reach it.
/// Without this, crossterm fails mid-setup with a bare "Initial console modes
/// not set" and leaves the alternate screen on.
fn require_console(stdin_is_console: bool) -> io::Result<()> {
    if stdin_is_console {
        return Ok(());
    }
    Err(io::Error::other(NO_CONSOLE))
}

/// Whether stdin is a Windows console. A pipe (mintty), file or NUL isn't,
/// even when the window around it looks like a terminal.
#[cfg(windows)]
fn stdin_is_console() -> bool {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::System::Console::GetConsoleMode;

    let mut mode = 0;
    // SAFETY: stdin's handle lives as long as the process, and
    // `GetConsoleMode` only writes `mode`; it returns 0 for a non-console.
    unsafe { GetConsoleMode(io::stdin().as_raw_handle(), &mut mode) != 0 }
}

/// Off Windows crossterm reads keys from `/dev/tty` when stdin isn't one.
#[cfg(not(windows))]
fn stdin_is_console() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn console_stdin_starts() {
        assert!(require_console(true).is_ok());
    }

    #[test]
    fn no_console_says_how_to_start() {
        let message = require_console(false).unwrap_err().to_string();
        assert!(message.contains("winpty bnto tui"), "{message}");
        assert!(message.contains("PowerShell"), "{message}");
    }
}
