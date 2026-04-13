// TUI — interactive terminal UI for bnto.
//
// `launch_tui()` sets up the terminal, runs the event loop, and
// restores the terminal on exit (including panics).

pub mod app;
pub mod event;
pub mod format;
mod keys;
pub mod palette;
mod render;
mod render_detail;
mod render_execution;
mod render_picker;
mod render_results;
pub mod screen;
pub mod screens;
#[allow(dead_code)]
pub mod theme;
pub mod widgets;

use std::io;
use std::time::Duration;

use crossterm::event::Event;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event as crossterm_event, execute};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};

use app::{AppMessage, AppModel, update};
use keys::handle_key;
use screens::picker::PickerMessage;
use theme::ThemeVariant;

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

        // Update picker viewport height from terminal size.
        // Chrome overhead: 2 (status+help bars) + 2 (block borders) + 2 (dir path + blank line).
        if matches!(model.screen, app::Screen::Picker { .. }) {
            let total_height = terminal.size()?.height as usize;
            let chrome = 6;
            let picker_height = total_height.saturating_sub(chrome);
            if picker_height > 0 {
                model = update(
                    model,
                    AppMessage::Picker(PickerMessage::Resize {
                        height: picker_height,
                    }),
                );
            }
        }

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

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent};

    use super::*;
    use app::{AppMessage, Screen};
    use screens::browser::BrowserMessage;
    use screens::detail::DetailMessage;
    use screens::execution::ExecutionMessage;
    use screens::picker::PickerMessage;
    use screens::results::ResultsMessage;
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
    fn detail_tab_confirms_from_any_focus() {
        let model = detail_model(); // focused = 0 (on a param)
        let key = KeyEvent::new(KeyCode::Tab, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::ConfigConfirmed {
                slug: "compress-images".into()
            })
        );
    }

    #[test]
    fn detail_enter_confirms_on_continue_action() {
        let mut model = detail_model();
        // Focus on the continue action (index = params.len())
        model.detail.as_mut().unwrap().focused = 2;
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::ConfigConfirmed {
                slug: "compress-images".into()
            })
        );
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
                size: None,
            },
            FileEntry {
                name: "cat.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/cat.jpg"),
                size: Some(290_000),
            },
        ];

        let mut picker = PickerModel::from_test_data(
            "compress-images",
            PathBuf::from("/home"),
            entries,
            vec!["jpg".into()],
        );
        picker.selected.insert(PathBuf::from("/cat.jpg")); // pre-select cat.jpg

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

    // --- Execution key handling ---

    fn execution_model() -> AppModel {
        use screens::execution::ExecutionModel;

        AppModel {
            screen: Screen::Execution {
                slug: "compress-images".into(),
            },
            execution: Some(ExecutionModel::new("compress-images")),
            ..default_model()
        }
    }

    #[test]
    fn execution_esc_cancels() {
        let model = execution_model();
        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Execution(ExecutionMessage::Cancel))
        );
    }

    #[test]
    fn execution_q_quits() {
        let model = execution_model();
        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Quit));
    }

    #[test]
    fn execution_unmapped_key_returns_none() {
        let model = execution_model();
        let key = KeyEvent::new(KeyCode::Char('x'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), None);
    }

    // --- Results key handling ---

    fn results_model() -> AppModel {
        use screens::results::{OutputFile, ResultsModel};

        let outputs = vec![
            OutputFile {
                name: "photo-1.jpg".into(),
                size_bytes: 290_000,
                original_size: Some(780_000),
            },
            OutputFile {
                name: "photo-2.jpg".into(),
                size_bytes: 340_000,
                original_size: Some(920_000),
            },
        ];

        AppModel {
            screen: Screen::Results {
                slug: "compress-images".into(),
            },
            results: Some(ResultsModel::new("compress-images", outputs, 4100, None)),
            ..default_model()
        }
    }

    #[test]
    fn results_j_moves_cursor_down() {
        let model = results_model();
        let key = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Results(ResultsMessage::CursorDown))
        );
    }

    #[test]
    fn results_k_moves_cursor_up() {
        let model = results_model();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Results(ResultsMessage::CursorUp))
        );
    }

    #[test]
    fn results_arrow_keys_navigate() {
        let model = results_model();
        let down = KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, down),
            Some(AppMessage::Results(ResultsMessage::CursorDown))
        );
        let up = KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, up),
            Some(AppMessage::Results(ResultsMessage::CursorUp))
        );
    }

    #[test]
    fn results_r_runs_another() {
        let model = results_model();
        let key = KeyEvent::new(KeyCode::Char('r'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::RunAnother));
    }

    #[test]
    fn results_q_quits() {
        let model = results_model();
        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Quit));
    }
}
