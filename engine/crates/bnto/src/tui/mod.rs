// TUI — interactive terminal UI for bnto.
//
// `launch_tui()` sets up the terminal, runs the event loop, and
// restores the terminal on exit (including panics).

pub mod app;
pub mod event;
pub mod screens;
#[allow(dead_code)]
pub mod theme;
pub mod widgets;

use std::io;
use std::time::Duration;

use crossterm::event::{Event, KeyEvent};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, event as crossterm_event};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Terminal;

use app::{AppMessage, AppModel, Screen, update};
use theme::ROUNDED_BORDERS;

/// Tick rate for the event loop (how often we check for input).
const TICK_RATE: Duration = Duration::from_millis(50);

/// Launch the interactive TUI.
pub fn launch_tui() -> io::Result<()> {
    install_panic_hook();
    let mut terminal = setup_terminal()?;
    let result = run_loop(&mut terminal);
    restore_terminal(&mut terminal)?;
    result
}

/// Install a panic hook that restores the terminal before printing the panic.
/// Without this, a panic leaves the terminal in raw mode (unusable).
fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stderr(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}

/// Enter raw mode, switch to alternate screen, create terminal.
fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stderr>>> {
    terminal::enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;

    // Disable mouse capture and bracketed paste — keyboard only.
    execute!(stderr, crossterm_event::DisableMouseCapture)?;

    let backend = CrosstermBackend::new(io::stderr());
    Terminal::new(backend)
}

/// Leave alternate screen, disable raw mode, show cursor.
fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

/// Main event loop — poll input, update state, render.
fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
) -> io::Result<()> {
    let mut model = AppModel::new();

    loop {
        terminal.draw(|frame| draw(frame, &model))?;

        if model.should_quit {
            break;
        }

        if let Some(Event::Key(key)) = event::poll_event(TICK_RATE)?
            && let Some(msg) = handle_key(&model, key)
        {
            model = update(model, msg);
        }
    }

    Ok(())
}

/// Map a key event to an AppMessage based on the current screen.
fn handle_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    // Global keys take precedence.
    if let Some(msg) = event::map_global_key(key) {
        return Some(msg);
    }

    // Screen-specific keys (browser, detail, etc.) will be added
    // in subsequent waves as each screen is implemented.
    match &model.screen {
        Screen::Browser => None,
        Screen::Detail { .. } => None,
        Screen::Picker { .. } => None,
        Screen::Execution { .. } => None,
        Screen::Results { .. } => None,
    }
}

/// Render the current screen to the terminal frame.
fn draw(frame: &mut ratatui::Frame, model: &AppModel) {
    let area = frame.area();

    // Layout: main content + help bar at the bottom.
    let [content_area, help_area] = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(area);

    draw_content(frame, model, content_area);
    draw_help_bar(frame, model, help_area);
}

/// Render the main content area based on the current screen.
fn draw_content(frame: &mut ratatui::Frame, model: &AppModel, area: Rect) {
    // Each screen will render its own content in subsequent waves.
    // For now, show a placeholder with the screen name.
    let title = match &model.screen {
        Screen::Browser => " bnto ",
        Screen::Detail { .. } => " Recipe Detail ",
        Screen::Picker { .. } => " File Picker ",
        Screen::Execution { .. } => " Running ",
        Screen::Results { .. } => " Results ",
    };

    let block = Block::bordered()
        .title(title)
        .title_style(theme::heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme::muted());

    let label = match &model.screen {
        Screen::Browser => "Select a recipe to get started.".to_string(),
        Screen::Detail { slug } => format!("Configure {slug}"),
        Screen::Picker { slug } => format!("Pick files for {slug}"),
        Screen::Execution { slug } => format!("Running {slug}..."),
        Screen::Results { slug } => format!("Results for {slug}"),
    };

    let content = Paragraph::new(label)
        .style(theme::text())
        .block(block);

    frame.render_widget(content, area);
}

/// Render the bottom help bar with contextual key hints.
fn draw_help_bar(frame: &mut ratatui::Frame, model: &AppModel, area: Rect) {
    let hints = help_hints(&model.screen);
    let spans: Vec<Span> = hints
        .iter()
        .enumerate()
        .flat_map(|(i, (key, desc))| {
            let mut parts = vec![
                Span::styled(*key, theme::key_hint()),
                Span::styled(format!(" {desc}"), theme::key_desc()),
            ];
            if i < hints.len() - 1 {
                parts.push(Span::raw("  "));
            }
            parts
        })
        .collect();

    let bar = Paragraph::new(Line::from(spans));
    frame.render_widget(bar, area);
}

/// Contextual key hints for each screen.
fn help_hints(screen: &Screen) -> Vec<(&'static str, &'static str)> {
    match screen {
        Screen::Browser => vec![
            ("↑↓", "navigate"),
            ("/", "search"),
            ("Enter", "select"),
            ("q", "quit"),
        ],
        Screen::Detail { .. } => vec![
            ("↑↓", "navigate"),
            ("Enter", "edit/confirm"),
            ("Esc", "back"),
            ("q", "quit"),
        ],
        Screen::Picker { .. } => vec![
            ("↑↓", "navigate"),
            ("Space", "select"),
            ("Enter", "confirm"),
            ("Esc", "back"),
        ],
        Screen::Execution { .. } => vec![("Esc", "cancel")],
        Screen::Results { .. } => vec![
            ("o", "open file"),
            ("O", "open folder"),
            ("r", "run another"),
            ("q", "quit"),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_hints_non_empty_for_all_screens() {
        let screens = vec![
            Screen::Browser,
            Screen::Detail { slug: "test".into() },
            Screen::Picker { slug: "test".into() },
            Screen::Execution { slug: "test".into() },
            Screen::Results { slug: "test".into() },
        ];
        for screen in screens {
            assert!(
                !help_hints(&screen).is_empty(),
                "help_hints empty for {screen:?}"
            );
        }
    }

    #[test]
    fn handle_key_q_quits_from_any_screen() {
        let screens = vec![
            Screen::Browser,
            Screen::Detail { slug: "test".into() },
            Screen::Results { slug: "test".into() },
        ];
        let key = KeyEvent::new(
            crossterm::event::KeyCode::Char('q'),
            crossterm::event::KeyModifiers::NONE,
        );
        for screen in screens {
            let model = AppModel {
                screen,
                should_quit: false,
            };
            let msg = handle_key(&model, key);
            assert_eq!(msg, Some(AppMessage::Quit));
        }
    }
}
