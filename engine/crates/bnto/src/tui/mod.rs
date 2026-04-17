// TUI — interactive terminal UI for bnto.
//
// `launch_tui()` sets up the terminal, runs the event loop, and
// restores the terminal on exit (including panics).

pub mod app;
pub mod atomic;
mod bridge;
pub mod config;
pub mod event;
pub mod format;
mod keys;
pub mod migration;
pub mod palette;
pub mod paths;
mod render;
mod render_detail;
mod render_execution;
mod render_home;
mod render_picker;
mod render_results;
pub mod screen;
pub mod screens;
pub mod theme;
pub mod toml_config;
pub mod widgets;

use std::io;
use std::sync::mpsc;
use std::time::Duration;

use crossterm::event::Event;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event as crossterm_event, execute};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};

use app::{AppMessage, AppModel, update};
use bridge::BridgeEvent;
use keys::handle_key;
use screens::execution::{ExecutionMessage, ExecutionStatus};
use screens::picker::PickerMessage;
use theme::ThemeVariant;

/// Tick rate for the event loop (how often we check for input).
const TICK_RATE: Duration = Duration::from_millis(50);

/// Launch the interactive TUI with the given theme variant.
///
/// If `recipe_json` is Some, starts directly on the detail screen
/// for that recipe instead of the browser.
pub fn launch_tui(variant: ThemeVariant, recipe_json: Option<String>) -> io::Result<()> {
    install_panic_hook();
    let mut terminal = setup_terminal()?;
    let result = run_loop(&mut terminal, variant, recipe_json);
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
    recipe_json: Option<String>,
) -> io::Result<()> {
    let mut model = AppModel::new(variant, recipe_json);
    let mut bridge_rx: Option<mpsc::Receiver<BridgeEvent>> = None;

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

        // Update detail viewport height from terminal size.
        if matches!(model.screen, app::Screen::Detail { .. }) {
            let total_height = terminal.size()?.height as usize;
            let chrome = 6; // 2 (status+help) + 2 (block borders) + 2 (padding)
            let detail_height = total_height.saturating_sub(chrome);
            if detail_height > 0 {
                model = update(
                    model,
                    AppMessage::Detail(screens::detail::DetailMessage::Resize {
                        height: detail_height,
                    }),
                );
            }
        }

        // Spawn the pipeline bridge when execution screen enters Idle state.
        if bridge_rx.is_none()
            && let Some(exec) = &model.execution
            && exec.status == ExecutionStatus::Idle
            && let app::Screen::Execution { slug } = &model.screen
        {
            bridge_rx = Some(bridge::spawn_pipeline(
                slug.clone(),
                exec.selected_files.clone(),
                exec.param_overrides.clone(),
                model.config.output_dir.clone(),
            ));
        }

        // Poll bridge events (non-blocking) and map to AppMessages.
        if let Some(rx) = &bridge_rx {
            while let Ok(event) = rx.try_recv() {
                model = match event {
                    BridgeEvent::Progress(pe) => {
                        let msg = bridge::map_pipeline_event(pe);
                        update(model, msg)
                    }
                    BridgeEvent::Done {
                        output_dir,
                        file_count,
                        duration_ms,
                        file_metadata,
                    } => {
                        let outputs = bridge::build_output_files(&output_dir, &file_metadata);
                        let slug = match &model.screen {
                            app::Screen::Execution { slug } => slug.clone(),
                            _ => String::new(),
                        };
                        let model = update(
                            model,
                            AppMessage::Execution(ExecutionMessage::OutputsReady {
                                files: outputs,
                                output_dir: Some(output_dir),
                            }),
                        );
                        let model = update(
                            model,
                            AppMessage::Execution(ExecutionMessage::PipelineCompleted {
                                duration_ms,
                                total_files_processed: file_count,
                            }),
                        );
                        // Auto-transition to results screen.
                        update(model, AppMessage::ExecutionComplete { slug })
                    }
                    BridgeEvent::Error(err) => update(
                        model,
                        AppMessage::Execution(ExecutionMessage::PipelineFailed {
                            node_id: String::new(),
                            error: err,
                        }),
                    ),
                };
            }
        }

        // Clear the bridge receiver when we've left the execution screen
        // (auto-transition fired) or user cancelled. Don't clear on Completed/Failed
        // status — the bridge thread may still be writing files and hasn't sent
        // BridgeEvent::Done yet.
        if bridge_rx.is_some() {
            let cancelled = model
                .execution
                .as_ref()
                .is_some_and(|e| e.status == ExecutionStatus::Cancelled);
            if cancelled || !matches!(model.screen, app::Screen::Execution { .. }) {
                bridge_rx = None;
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
    use config::TuiConfig;
    use screens::browser::BrowserMessage;
    use screens::detail::DetailMessage;
    use screens::execution::ExecutionMessage;
    use screens::home::HomeMessage;
    use screens::picker::PickerMessage;
    use screens::results::ResultsMessage;
    use screens::settings::SettingsModel;

    fn default_model() -> AppModel {
        AppModel::new(ThemeVariant::LosAngeles, None)
    }

    fn browser_model() -> AppModel {
        AppModel {
            screen: Screen::Browser,
            ..default_model()
        }
    }

    #[test]
    fn handle_key_q_quits_from_any_screen() {
        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        for screen in [
            Screen::Home,
            Screen::Library,
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

    // --- Home screen key handling ---

    #[test]
    fn home_j_moves_cursor_down() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Home(HomeMessage::SelectNext))
        );
    }

    #[test]
    fn home_k_moves_cursor_up() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Home(HomeMessage::SelectPrev))
        );
    }

    #[test]
    fn home_arrow_keys_navigate() {
        let model = default_model();
        let down = KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, down),
            Some(AppMessage::Home(HomeMessage::SelectNext))
        );
        let up = KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, up),
            Some(AppMessage::Home(HomeMessage::SelectPrev))
        );
    }

    #[test]
    fn home_enter_confirms_selection() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::HomeConfirm));
    }

    #[test]
    fn s_key_opens_settings_from_browser() {
        let model = browser_model();
        let key = KeyEvent::new(KeyCode::Char('s'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::OpenSettings));
    }

    /// Helper: create a model on the Settings screen with a SettingsModel.
    /// Uses explicit LosAngeles theme to avoid reading saved config from disk.
    fn settings_model() -> AppModel {
        let mut model = default_model();
        model.screen = Screen::Settings;
        model.theme_variant = ThemeVariant::LosAngeles;
        model.settings = Some(SettingsModel::from_config(&TuiConfig::default()));
        model
    }

    #[test]
    fn settings_left_right_cycle_themes_on_theme_field() {
        let model = settings_model(); // focused=0 (theme field)
        let right = KeyEvent::new(KeyCode::Right, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, right),
            Some(AppMessage::ThemeChanged(ThemeVariant::Tokyo))
        );
        let left = KeyEvent::new(KeyCode::Left, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, left),
            Some(AppMessage::ThemeChanged(ThemeVariant::Monaco))
        );
    }

    #[test]
    fn settings_jk_navigate_fields() {
        use screens::settings::SettingsMessage;
        let model = settings_model();
        let j = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, j),
            Some(AppMessage::Settings(SettingsMessage::FocusNext))
        );
        let k = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, k),
            Some(AppMessage::Settings(SettingsMessage::FocusPrev))
        );
    }

    #[test]
    fn settings_enter_opens_picker_on_editable_field() {
        let mut model = settings_model();
        // Focus on field 1 (default_path, editable).
        model.settings.as_mut().unwrap().focused = 1;
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::OpenSettingsPicker {
                field_key: "default_path".into()
            })
        );
    }

    #[test]
    fn settings_enter_on_theme_field_returns_none() {
        let model = settings_model(); // focused=0 (theme, non-editable)
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), None);
    }

    #[test]
    fn settings_esc_goes_back() {
        let model = settings_model();
        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Back));
    }

    #[test]
    fn settings_unmapped_key_returns_none() {
        let model = settings_model();
        let key = KeyEvent::new(KeyCode::Char('x'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), None);
    }

    #[test]
    fn settings_left_right_toggles_telemetry_on_telemetry_field() {
        let mut model = settings_model();
        // Focus on field 3 (telemetry, non-editable toggle).
        model.settings.as_mut().unwrap().focused = 3;
        // Default value is "On", so right should toggle to Off.
        let right = KeyEvent::new(KeyCode::Right, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, right),
            Some(AppMessage::TelemetryToggled(false))
        );
        // Left also toggles.
        let left = KeyEvent::new(KeyCode::Left, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, left),
            Some(AppMessage::TelemetryToggled(false))
        );
    }

    #[test]
    fn settings_enter_on_telemetry_field_returns_none() {
        let mut model = settings_model();
        model.settings.as_mut().unwrap().focused = 3;
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
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
        let model = browser_model();
        let key = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::CursorDown))
        );
    }

    #[test]
    fn browser_k_moves_cursor_up() {
        let model = browser_model();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::CursorUp))
        );
    }

    #[test]
    fn browser_arrow_keys_navigate() {
        let model = browser_model();
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
        let model = browser_model();
        let key = KeyEvent::new(KeyCode::Char('/'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::EnterSearch))
        );
    }

    #[test]
    fn browser_enter_selects_recipe() {
        let model = browser_model();
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        let msg = handle_key(&model, key);
        assert!(matches!(msg, Some(AppMessage::RecipeSelected { .. })));
    }

    #[test]
    fn browser_search_mode_captures_chars() {
        let mut model = browser_model();
        model.browser.searching = true;
        let key = KeyEvent::new(KeyCode::Char('a'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::SearchInput('a')))
        );
    }

    #[test]
    fn browser_search_mode_esc_exits() {
        let mut model = browser_model();
        model.browser.searching = true;
        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Browser(BrowserMessage::ExitSearch))
        );
    }

    #[test]
    fn browser_search_mode_backspace() {
        let mut model = browser_model();
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
                description: None,
                constraints: None,
                suffix: None,
                visible_when: None,
            },
            ParamEntry {
                node_id: "n".into(),
                name: "format".into(),
                label: "Format".into(),
                value: "jpeg".into(),
                param_type: ParameterType::String,
                default: "jpeg".into(),
                description: None,
                constraints: None,
                suffix: None,
                visible_when: None,
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

    #[test]
    fn detail_space_toggles_bool() {
        use bnto_core::metadata::ParameterType;
        use screens::detail::{DetailModel, ParamEntry};

        let params = vec![ParamEntry {
            node_id: "n".into(),
            name: "strip".into(),
            label: "Strip".into(),
            value: "true".into(),
            param_type: ParameterType::Boolean,
            default: "true".into(),
            description: None,
            constraints: None,
            suffix: None,
            visible_when: None,
        }];
        let model = AppModel {
            screen: Screen::Detail { slug: "s".into() },
            detail: Some(DetailModel::from_test_data("s", "n", "d", params)),
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Char(' '), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::ToggleBool))
        );
    }

    #[test]
    fn detail_left_right_cycles_enum() {
        use bnto_core::metadata::{OptionEntry, ParameterType};
        use screens::detail::{DetailModel, ParamEntry};

        let params = vec![ParamEntry {
            node_id: "n".into(),
            name: "fmt".into(),
            label: "Format".into(),
            value: "jpeg".into(),
            param_type: ParameterType::Enum {
                options: vec![
                    OptionEntry {
                        value: "jpeg".into(),
                        label: "JPEG".into(),
                    },
                    OptionEntry {
                        value: "png".into(),
                        label: "PNG".into(),
                    },
                ],
            },
            default: "jpeg".into(),
            description: None,
            constraints: None,
            suffix: None,
            visible_when: None,
        }];
        let model = AppModel {
            screen: Screen::Detail { slug: "s".into() },
            detail: Some(DetailModel::from_test_data("s", "n", "d", params)),
            ..default_model()
        };
        let right = KeyEvent::new(KeyCode::Right, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, right),
            Some(AppMessage::Detail(DetailMessage::EnumNext))
        );
        let left = KeyEvent::new(KeyCode::Left, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, left),
            Some(AppMessage::Detail(DetailMessage::EnumPrev))
        );
    }

    #[test]
    fn detail_left_right_steps_bounded_number() {
        use bnto_core::metadata::{Constraints, ParameterType};
        use screens::detail::{DetailModel, ParamEntry};

        let params = vec![ParamEntry {
            node_id: "n".into(),
            name: "quality".into(),
            label: "Quality".into(),
            value: "80".into(),
            param_type: ParameterType::Number,
            default: "80".into(),
            description: None,
            constraints: Some(Constraints {
                min: Some(1.0),
                max: Some(100.0),
                required: false,
            }),
            suffix: None,
            visible_when: None,
        }];
        let model = AppModel {
            screen: Screen::Detail { slug: "s".into() },
            detail: Some(DetailModel::from_test_data("s", "n", "d", params)),
            ..default_model()
        };
        let right = KeyEvent::new(KeyCode::Right, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, right),
            Some(AppMessage::Detail(DetailMessage::NumberIncrement))
        );
        let left = KeyEvent::new(KeyCode::Left, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, left),
            Some(AppMessage::Detail(DetailMessage::NumberDecrement))
        );
    }

    #[test]
    fn detail_d_resets_default() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Char('d'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::ResetDefault))
        );
    }

    #[test]
    fn detail_enter_toggles_bool_instead_of_edit() {
        use bnto_core::metadata::ParameterType;
        use screens::detail::{DetailModel, ParamEntry};

        let params = vec![ParamEntry {
            node_id: "n".into(),
            name: "strip".into(),
            label: "Strip".into(),
            value: "true".into(),
            param_type: ParameterType::Boolean,
            default: "true".into(),
            description: None,
            constraints: None,
            suffix: None,
            visible_when: None,
        }];
        let model = AppModel {
            screen: Screen::Detail { slug: "s".into() },
            detail: Some(DetailModel::from_test_data("s", "n", "d", params)),
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::ToggleBool)),
            "Enter on bool should toggle, not start edit"
        );
    }

    #[test]
    fn detail_enter_cycles_enum_instead_of_edit() {
        use bnto_core::metadata::{OptionEntry, ParameterType};
        use screens::detail::{DetailModel, ParamEntry};

        let params = vec![ParamEntry {
            node_id: "n".into(),
            name: "fmt".into(),
            label: "Format".into(),
            value: "jpeg".into(),
            param_type: ParameterType::Enum {
                options: vec![OptionEntry {
                    value: "jpeg".into(),
                    label: "JPEG".into(),
                }],
            },
            default: "jpeg".into(),
            description: None,
            constraints: None,
            suffix: None,
            visible_when: None,
        }];
        let model = AppModel {
            screen: Screen::Detail { slug: "s".into() },
            detail: Some(DetailModel::from_test_data("s", "n", "d", params)),
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Detail(DetailMessage::EnumNext)),
            "Enter on enum should cycle, not start edit"
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

    #[test]
    fn picker_h_goes_to_parent() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('h'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::ParentDir))
        );
    }

    #[test]
    fn picker_left_arrow_goes_to_parent() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Left, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::ParentDir))
        );
    }

    #[test]
    fn picker_l_on_dir_enters_dir() {
        let model = picker_model(); // cursor=0 is a dir
        let key = KeyEvent::new(KeyCode::Char('l'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::EnterDir))
        );
    }

    #[test]
    fn picker_l_on_file_is_noop() {
        let mut model = picker_model();
        model.picker.as_mut().unwrap().cursor = 1; // on a file
        let key = KeyEvent::new(KeyCode::Char('l'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), None);
    }

    #[test]
    fn picker_right_on_dir_enters_dir() {
        let model = picker_model(); // cursor=0 is a dir
        let key = KeyEvent::new(KeyCode::Right, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::EnterDir))
        );
    }

    #[test]
    fn picker_g_goes_to_top() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('g'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::GoToTop))
        );
    }

    #[test]
    fn picker_shift_g_goes_to_bottom() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('G'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::GoToBottom))
        );
    }

    #[test]
    fn picker_shift_j_pages_down() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('J'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::PageDown))
        );
    }

    #[test]
    fn picker_pgdn_pages_down() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::PageDown, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::PageDown))
        );
    }

    #[test]
    fn picker_shift_k_pages_up() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('K'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::PageUp))
        );
    }

    #[test]
    fn picker_pgup_pages_up() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::PageUp, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::PageUp))
        );
    }

    #[test]
    fn picker_dot_toggles_hidden() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('.'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::ToggleHidden))
        );
    }

    #[test]
    fn picker_a_selects_all() {
        let model = picker_model();
        let key = KeyEvent::new(KeyCode::Char('a'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Picker(PickerMessage::SelectAll))
        );
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

    #[test]
    fn execution_enter_continues_when_completed() {
        use screens::execution::ExecutionModel;

        let mut exec = ExecutionModel::new("compress-images");
        exec = screens::execution::update(
            exec,
            ExecutionMessage::PipelineCompleted {
                duration_ms: 100,
                total_files_processed: 1,
            },
        );
        let model = AppModel {
            screen: Screen::Execution {
                slug: "compress-images".into(),
            },
            execution: Some(exec),
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::ExecutionComplete {
                slug: "compress-images".into()
            })
        );
    }

    #[test]
    fn execution_enter_continues_when_failed() {
        use screens::execution::ExecutionModel;

        let mut exec = ExecutionModel::new("compress-images");
        exec = screens::execution::update(
            exec,
            ExecutionMessage::PipelineFailed {
                node_id: "n".into(),
                error: "err".into(),
            },
        );
        let model = AppModel {
            screen: Screen::Execution {
                slug: "compress-images".into(),
            },
            execution: Some(exec),
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::ExecutionComplete {
                slug: "compress-images".into()
            })
        );
    }

    #[test]
    fn execution_enter_ignored_while_running() {
        use screens::execution::ExecutionModel;

        let mut exec = ExecutionModel::new("compress-images");
        exec = screens::execution::update(
            exec,
            ExecutionMessage::PipelineStarted {
                total_nodes: 1,
                total_files: 1,
            },
        );
        let model = AppModel {
            screen: Screen::Execution {
                slug: "compress-images".into(),
            },
            execution: Some(exec),
            ..default_model()
        };
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
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
