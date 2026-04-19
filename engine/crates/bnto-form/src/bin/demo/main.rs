//! bnto-form kitchen-sink demo binary.
//!
//! Launch: `cargo run -p bnto-form --bin bnto-form-demo`
//!
//! Interactive demo showcasing every field type, validator, and feature
//! in bnto-form. Mirrors the bnto TUI visual shell (logo, bordered panel,
//! help bar) but stands alone with zero bnto dependency.

mod layout;
mod logo;
mod model;

use std::io;
use std::time::Duration;

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::text::Text;
use ratatui::widgets::Paragraph;

use bnto_form::demo::help_bar::render_help_bar;
use bnto_form::{DefaultTheme, FormMessage, FormTheme, render_form};

use model::{DemoModel, handle_demo_key, update_demo};

const TICK_RATE: Duration = Duration::from_millis(50);

fn main() -> io::Result<()> {
    install_panic_hook();
    let mut terminal = setup_terminal()?;
    let result = run_loop(&mut terminal);
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stderr>>> {
    terminal::enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(io::stderr()))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stderr>>) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

fn install_panic_hook() {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stderr(), LeaveAlternateScreen);
        original(info);
    }));
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stderr>>) -> io::Result<()> {
    let term_h = terminal.size()?.height;
    let vh = layout::viewport_height(term_h);
    let mut model = DemoModel::new(vh);
    let theme = DefaultTheme;

    loop {
        // Render
        terminal.draw(|frame| {
            let area = frame.area();
            let (logo_area, content_area, bottom_area) = layout::app_frame(area);

            // Logo
            let logo_lines = logo::render_logo(theme.border());
            let logo_text = Text::from(logo_lines);
            frame.render_widget(Paragraph::new(logo_text), logo_area);

            // Bordered content panel
            let inner = layout::content_panel(
                frame,
                theme.border(),
                theme.heading(),
                content_area,
                "kitchen sink",
            );

            // Form
            let form_lines = render_form(&model.form, &theme);
            let form_text = Text::from(form_lines);
            frame.render_widget(Paragraph::new(form_text), inner);

            // Help bar
            let help = render_help_bar(model.form.fields.len(), &theme);
            frame.render_widget(Paragraph::new(help), bottom_area);
        })?;

        // Handle resize
        let new_h = terminal.size()?.height;
        let new_vh = layout::viewport_height(new_h);
        if new_vh != model.form.viewport_height {
            model.form = bnto_form::update(model.form, FormMessage::Resize { height: new_vh });
        }

        if model.should_quit {
            break;
        }

        // Poll input
        if crossterm::event::poll(TICK_RATE)?
            && let crossterm::event::Event::Key(key) = crossterm::event::read()?
            && let Some(msg) = handle_demo_key(key, &model)
        {
            model = update_demo(model, msg);
        }
    }

    Ok(())
}
