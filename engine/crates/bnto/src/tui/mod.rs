// TUI — interactive terminal UI for bnto.
//
// `launch_tui()` sets up the terminal, runs the event loop, and
// restores the terminal on exit (including panics).

pub mod app;
pub mod event;
pub mod palette;
mod render;
mod render_detail;
mod render_picker;
pub mod screen;
pub mod screens;
#[allow(dead_code)]
pub mod theme;
pub mod widgets;

use std::io;
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event as crossterm_event, execute};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};

use app::{AppMessage, AppModel, Screen, update};
use screens::browser::BrowserMessage;
use screens::detail::DetailMessage;
use screens::picker::PickerMessage;
use theme::{ALL_VARIANTS, ThemeVariant};

/// Tick rate for the event loop (how often we check for input).
const TICK_RATE: Duration = Duration::from_millis(50);

/// Launch the interactive TUI with the given theme variant.
pub fn launch_tui(variant: ThemeVariant) -> io::Result<()> {
    install_panic_hook();
    let mut terminal = setup_terminal()?;
    let result = run_loop(&mut terminal, variant);
    restore_terminal(&mut terminal)?;
    result
}

/// Install a panic hook that restores the terminal before printing the panic.
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
    execute!(stderr, crossterm_event::DisableMouseCapture)?;
    let backend = CrosstermBackend::new(io::stderr());
    Terminal::new(backend)
}

/// Leave alternate screen, disable raw mode, show cursor.
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stderr>>) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

/// Main event loop — poll input, update state, render.
fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stderr>>,
    variant: ThemeVariant,
) -> io::Result<()> {
    let mut model = AppModel::new(variant);

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let theme = &model.theme;
            let [content_area, status_area, help_area] = Layout::vertical([
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .areas(area);

            render::draw_content(frame, &model, theme, content_area);
            render::draw_status_line(frame, &model, theme, status_area);
            render::draw_help_bar(frame, &model, theme, help_area);
        })?;

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
///
/// When the browser is in search mode, screen-specific keys take priority
/// so that Esc exits search and character keys type into the query.
fn handle_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let searching = matches!(&model.screen, Screen::Browser if model.browser.searching);
    if searching {
        return handle_browser_key(model, key);
    }

    // Detail editing mode captures all keys (like browser search mode).
    let detail_editing = matches!(&model.screen, Screen::Detail { .. }
        if model.detail.as_ref().is_some_and(|d| d.editing));
    if detail_editing {
        return handle_detail_key(model, key);
    }

    if let Some(msg) = event::map_global_key(key) {
        return Some(msg);
    }

    match &model.screen {
        Screen::Browser => handle_browser_key(model, key),
        Screen::Settings => handle_settings_key(model, key),
        Screen::Detail { .. } => handle_detail_key(model, key),
        Screen::Picker { .. } => handle_picker_key(model, key),
        Screen::Execution { .. } => None,
        Screen::Results { .. } => None,
    }
}

/// Handle key events on the Browser screen.
fn handle_browser_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    if model.browser.searching {
        return match key.code {
            KeyCode::Esc => Some(AppMessage::Browser(BrowserMessage::ExitSearch)),
            KeyCode::Backspace => Some(AppMessage::Browser(BrowserMessage::SearchBackspace)),
            KeyCode::Enter => model
                .browser
                .confirm()
                .map(|r| AppMessage::RecipeSelected { slug: r.slug }),
            KeyCode::Char('u')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                Some(AppMessage::Browser(BrowserMessage::SearchClear))
            }
            KeyCode::Char(ch) => Some(AppMessage::Browser(BrowserMessage::SearchInput(ch))),
            _ => None,
        };
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Browser(BrowserMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Browser(BrowserMessage::CursorUp)),
        KeyCode::Char('/') => Some(AppMessage::Browser(BrowserMessage::EnterSearch)),
        KeyCode::Char('s') => Some(AppMessage::OpenSettings),
        KeyCode::Enter => model
            .browser
            .confirm()
            .map(|r| AppMessage::RecipeSelected { slug: r.slug }),
        _ => None,
    }
}

/// Handle key events on the Detail screen.
///
/// When editing a parameter, char keys feed the edit buffer and Enter/Esc
/// commit or cancel. When not editing, j/k navigate params and Enter starts
/// editing or confirms when no params exist.
fn handle_detail_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let editing = model.detail.as_ref().is_some_and(|d| d.editing);

    if editing {
        return match key.code {
            KeyCode::Enter => Some(AppMessage::Detail(DetailMessage::CommitEdit)),
            KeyCode::Esc => Some(AppMessage::Detail(DetailMessage::CancelEdit)),
            KeyCode::Backspace => Some(AppMessage::Detail(DetailMessage::EditBackspace)),
            KeyCode::Char(ch) => Some(AppMessage::Detail(DetailMessage::EditChar(ch))),
            _ => None,
        };
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Detail(DetailMessage::FocusNext)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Detail(DetailMessage::FocusPrev)),
        KeyCode::Enter => {
            let has_params = model.detail.as_ref().is_some_and(|d| !d.params.is_empty());
            if has_params {
                Some(AppMessage::Detail(DetailMessage::StartEdit))
            } else {
                Some(AppMessage::ConfigConfirmed {
                    slug: model
                        .detail
                        .as_ref()
                        .map(|d| d.slug.clone())
                        .unwrap_or_default(),
                })
            }
        }
        KeyCode::Esc => Some(AppMessage::Back),
        _ => None,
    }
}

/// Handle key events on the Picker screen.
fn handle_picker_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let picker = model.picker.as_ref()?;

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(AppMessage::Picker(PickerMessage::CursorDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(AppMessage::Picker(PickerMessage::CursorUp)),
        KeyCode::Char(' ') => Some(AppMessage::Picker(PickerMessage::ToggleSelect)),
        KeyCode::Backspace => Some(AppMessage::Picker(PickerMessage::ParentDir)),
        KeyCode::Enter => {
            if picker.cursor < picker.entries.len() && picker.entries[picker.cursor].is_dir {
                Some(AppMessage::Picker(PickerMessage::EnterDir))
            } else if !picker.selected.is_empty() {
                let slug = picker.slug.clone();
                Some(AppMessage::FilesSelected { slug })
            } else {
                None
            }
        }
        KeyCode::Esc => Some(AppMessage::Back),
        _ => None,
    }
}

/// Handle key events on the Settings screen.
fn handle_settings_key(model: &AppModel, key: KeyEvent) -> Option<AppMessage> {
    let current_idx = ALL_VARIANTS
        .iter()
        .position(|v| *v == model.theme_variant)
        .unwrap_or(0);

    match key.code {
        KeyCode::Up => {
            let prev = if current_idx == 0 {
                ALL_VARIANTS.len() - 1
            } else {
                current_idx - 1
            };
            Some(AppMessage::ThemeChanged(ALL_VARIANTS[prev]))
        }
        KeyCode::Down => {
            let next = (current_idx + 1) % ALL_VARIANTS.len();
            Some(AppMessage::ThemeChanged(ALL_VARIANTS[next]))
        }
        KeyCode::Enter => Some(AppMessage::Back),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use theme::Theme;

    fn default_model() -> AppModel {
        AppModel::new(ThemeVariant::LosAngeles)
    }

    #[test]
    fn handle_key_q_quits_from_any_screen() {
        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        for screen in [
            Screen::Browser,
            Screen::Detail { slug: "t".into() },
            Screen::Results { slug: "t".into() },
        ] {
            let model = AppModel {
                screen,
                ..default_model()
            };
            assert_eq!(handle_key(&model, key), Some(AppMessage::Quit));
        }
    }

    #[test]
    fn s_key_opens_settings_from_browser() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Char('s'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::OpenSettings));
    }

    #[test]
    fn settings_arrow_keys_cycle_themes() {
        let model = AppModel {
            screen: Screen::Settings,
            ..default_model()
        };
        let down = KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, down),
            Some(AppMessage::ThemeChanged(ThemeVariant::Tokyo))
        );
        let up = KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, up),
            Some(AppMessage::ThemeChanged(ThemeVariant::Monaco))
        );
    }

    #[test]
    fn settings_enter_goes_back() {
        let model = AppModel {
            screen: Screen::Settings,
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Back));
    }

    #[test]
    fn settings_down_wraps_from_last_to_first() {
        let model = AppModel {
            screen: Screen::Settings,
            theme: Theme::from_variant(ThemeVariant::Monaco),
            theme_variant: ThemeVariant::Monaco,
            ..default_model()
        };
        let down = KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, down),
            Some(AppMessage::ThemeChanged(ThemeVariant::LosAngeles))
        );
    }

    #[test]
    fn settings_up_wraps_from_first_to_last() {
        let model = AppModel {
            screen: Screen::Settings,
            ..default_model() // LosAngeles is first
        };
        let up = KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, up),
            Some(AppMessage::ThemeChanged(ThemeVariant::Monaco))
        );
    }

    #[test]
    fn settings_unmapped_key_returns_none() {
        let model = AppModel {
            screen: Screen::Settings,
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Char('x'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), None);
    }

    #[test]
    fn s_key_does_nothing_outside_browser() {
        let model = AppModel {
            screen: Screen::Detail { slug: "t".into() },
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Char('s'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), None);
    }

    // --- Browser key handling ---

    #[test]
    fn browser_j_moves_cursor_down() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::CursorDown))
        );
    }

    #[test]
    fn browser_k_moves_cursor_up() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::CursorUp))
        );
    }

    #[test]
    fn browser_arrow_keys_navigate() {
        let model = default_model();
        let down = KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, down),
            Some(AppMessage::Browser(BrowserMessage::CursorDown))
        );
        let up = KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, up),
            Some(AppMessage::Browser(BrowserMessage::CursorUp))
        );
    }

    #[test]
    fn browser_slash_enters_search() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Char('/'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::EnterSearch))
        );
    }

    #[test]
    fn browser_enter_selects_recipe() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        let msg = handle_key(&model, key);
        assert!(matches!(msg, Some(AppMessage::RecipeSelected { .. })));
    }

    #[test]
    fn browser_search_mode_captures_chars() {
        let mut model = default_model();
        model.browser.searching = true;
        let key = KeyEvent::new(KeyCode::Char('a'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::SearchInput('a')))
        );
    }

    #[test]
    fn browser_search_mode_esc_exits() {
        let mut model = default_model();
        model.browser.searching = true;
        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::ExitSearch))
        );
    }

    #[test]
    fn browser_search_mode_backspace() {
        let mut model = default_model();
        model.browser.searching = true;
        let key = KeyEvent::new(KeyCode::Backspace, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::SearchBackspace))
        );
    }

    // --- Detail key handling ---

    fn detail_model() -> AppModel {
        use bnto_core::metadata::ParameterType;
        use screens::detail::{DetailModel, ParamEntry};

        let params = vec![
            ParamEntry {
                node_id: "n".into(),
                name: "quality".into(),
                label: "Quality".into(),
                value: "80".into(),
                param_type: ParameterType::Number,
                default: "80".into(),
            },
            ParamEntry {
                node_id: "n".into(),
                name: "format".into(),
                label: "Format".into(),
                value: "jpeg".into(),
                param_type: ParameterType::String,
                default: "jpeg".into(),
            },
        ];

        AppModel {
            screen: Screen::Detail {
                slug: "compress-images".into(),
            },
            detail: Some(DetailModel::from_test_data(
                "compress-images",
                "Compress Images",
                "desc",
                params,
            )),
            ..default_model()
        }
    }

    #[test]
    fn detail_j_focuses_next_param() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::FocusNext))
        );
    }

    #[test]
    fn detail_k_focuses_prev_param() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::FocusPrev))
        );
    }

    #[test]
    fn detail_arrow_keys_navigate() {
        let model = detail_model();
        let down = KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, down),
            Some(AppMessage::Detail(DetailMessage::FocusNext))
        );
        let up = KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, up),
            Some(AppMessage::Detail(DetailMessage::FocusPrev))
        );
    }

    #[test]
    fn detail_enter_starts_edit_when_params_exist() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::StartEdit))
        );
    }

    #[test]
    fn detail_enter_confirms_when_no_params() {
        use screens::detail::DetailModel;

        let model = AppModel {
            screen: Screen::Detail { slug: "s".into() },
            detail: Some(DetailModel::from_test_data("s", "n", "d", vec![])),
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::ConfigConfirmed { slug: "s".into() })
        );
    }

    #[test]
    fn detail_esc_goes_back() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Back));
    }

    #[test]
    fn detail_q_quits() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Quit));
    }

    #[test]
    fn detail_editing_captures_chars() {
        let mut model = detail_model();
        model.detail.as_mut().unwrap().editing = true;
        let key = KeyEvent::new(KeyCode::Char('5'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::EditChar('5')))
        );
    }

    #[test]
    fn detail_editing_enter_commits() {
        let mut model = detail_model();
        model.detail.as_mut().unwrap().editing = true;
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::CommitEdit))
        );
    }

    #[test]
    fn detail_editing_esc_cancels() {
        let mut model = detail_model();
        model.detail.as_mut().unwrap().editing = true;
        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::CancelEdit))
        );
    }

    #[test]
    fn detail_editing_backspace() {
        let mut model = detail_model();
        model.detail.as_mut().unwrap().editing = true;
        let key = KeyEvent::new(KeyCode::Backspace, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::EditBackspace))
        );
    }

    #[test]
    fn detail_editing_blocks_global_q() {
        let mut model = detail_model();
        model.detail.as_mut().unwrap().editing = true;
        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        // 'q' is captured as EditChar, not Quit
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::EditChar('q')))
        );
    }

    // --- Picker key handling ---

    fn picker_model() -> AppModel {
        use screens::picker::{FileEntry, PickerModel};
        use std::path::PathBuf;

        let entries = vec![
            FileEntry {
                name: "photos".into(),
                is_dir: true,
                path: PathBuf::from("/photos"),
            },
            FileEntry {
                name: "cat.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/cat.jpg"),
            },
        ];

        let mut picker = PickerModel::from_test_data(
            "compress-images",
            PathBuf::from("/home"),
            entries,
            vec!["jpg".into()],
        );
        picker.selected.insert(1); // pre-select cat.jpg

        AppModel {
            screen: Screen::Picker {
                slug: "compress-images".into(),
            },
            picker: Some(picker),
            ..default_model()
        }
    }

    #[test]
    fn picker_j_moves_cursor_down() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::CursorDown))
        );
    }

    #[test]
    fn picker_k_moves_cursor_up() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::CursorUp))
        );
    }

    #[test]
    fn picker_space_toggles_select() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char(' '), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::ToggleSelect))
        );
    }

    #[test]
    fn picker_enter_on_dir_enters_dir() {
        let model = picker_model(); // cursor=0 is a dir
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::EnterDir))
        );
    }

    #[test]
    fn picker_enter_with_selected_files_confirms() {
        let mut model = picker_model();
        model.picker.as_mut().unwrap().cursor = 1; // on a file, not a dir
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::FilesSelected {
                slug: "compress-images".into()
            })
        );
    }

    #[test]
    fn picker_backspace_goes_to_parent() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Backspace, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::ParentDir))
        );
    }

    #[test]
    fn picker_esc_goes_back() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Back));
    }

    #[test]
    fn picker_q_quits() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Quit));
    }
}
