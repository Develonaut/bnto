// TUI — interactive terminal UI for bnto.
//
// `launch_tui()` sets up the terminal, runs the event loop, and
// restores the terminal on exit (including panics).

pub mod app;
mod app_helpers;
mod bridge;
mod detail_fit;
pub mod event;
mod file_window;
pub mod format;
mod keys;
pub mod palette;
mod render;
mod render_detail;
mod render_editor;
mod render_execution;
mod render_home;
mod render_home_grid;
mod render_home_logo;
mod render_home_panes;
mod render_layout;
mod render_library;
mod render_picker;
mod render_preview;
mod render_results;
mod render_wizard;
mod run_loop;
pub mod screen;
#[cfg(test)]
mod screen_snapshots;
pub mod screens;
mod strip_ansi;
mod terminal;
pub mod theme;
pub mod widgets;
mod wrap_lines;

use std::io;
use std::sync::Arc;

use bnto_core::logging::{LogEntry, LogLevel, Logger};

use theme::ThemeVariant;

/// Launch the interactive TUI with the given theme variant.
///
/// If `recipe_json` is Some, starts directly on the editor screen
/// for that recipe. If `new_recipe` is true, starts with a blank editor.
#[expect(clippy::too_many_lines)]
pub fn launch_tui(
    variant: ThemeVariant,
    recipe_json: Option<String>,
    new_recipe: bool,
    logger: &Arc<dyn Logger>,
    paths: Option<crate::storage::BntoPaths>,
) -> io::Result<()> {
    logger.log(LogEntry {
        level: LogLevel::Info,
        target: "tui",
        message: format!("session init (theme={variant:?})"),
        elapsed_us: None,
    });

    terminal::install_panic_hook();
    let terminal = terminal::setup();
    logger.flush(); // Flush the init entry even if setup fails.
    let mut terminal = terminal?;

    let result = run_loop::run_loop(
        &mut terminal,
        variant,
        recipe_json,
        new_recipe,
        logger,
        paths,
    );
    logger.flush();
    terminal::restore(&mut terminal)?;
    // After the restore, so the warning lands on the normal screen.
    crate::exit_report::report_kept_on_exit();
    result
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent};

    use super::*;
    use app::{AppMessage, AppModel, Screen, update};
    use keys::handle_key;
    use screens::browser::BrowserMessage;
    use screens::execution::ExecutionMessage;
    use screens::home::HomeMessage;
    use screens::picker::PickerMessage;
    use screens::results::ResultsMessage;
    use screens::settings::SettingsModel;
    use tonkotsu::FormMessage;

    fn test_paths() -> crate::storage::BntoPaths {
        // Kept on disk, as before: the model holds only the path.
        let root = tempfile::Builder::new()
            .prefix("bnto-mod-test-")
            .tempdir()
            .unwrap()
            .keep();
        let p = crate::storage::BntoPaths { home: root };
        let _ = p.ensure_dirs();
        let _ = crate::storage::config::TomlConfig::default().save(&p);
        p
    }

    fn default_model() -> AppModel {
        AppModel::with_paths(ThemeVariant::LosAngeles, None, false, test_paths())
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
            Screen::Detail {
                slug: "t".into(),
                from: app::DetailOrigin::Browser,
            },
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
            Some(AppMessage::Home(HomeMessage::CursorDown))
        );
    }

    #[test]
    fn home_k_moves_cursor_up() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Home(HomeMessage::CursorUp))
        );
    }

    #[test]
    fn home_arrow_keys_navigate_cursor() {
        let model = default_model();
        let down = KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, down),
            Some(AppMessage::Home(HomeMessage::CursorDown))
        );
        let up = KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, up),
            Some(AppMessage::Home(HomeMessage::CursorUp))
        );
    }

    #[test]
    fn home_tab_cycles_pane() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Tab, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Home(HomeMessage::NextPane))
        );
    }

    #[test]
    fn home_shift_tab_cycles_pane_backward() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::BackTab, crossterm::event::KeyModifiers::SHIFT);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Home(HomeMessage::PrevPane))
        );
    }

    #[test]
    fn home_l_cycles_pane_forward() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Char('l'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Home(HomeMessage::NextPane))
        );
    }

    #[test]
    fn home_h_cycles_pane_backward() {
        let model = default_model();
        let key = KeyEvent::new(KeyCode::Char('h'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Home(HomeMessage::PrevPane))
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
        model.settings = Some(SettingsModel::from_toml_config(&model.toml_config));
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
        // Focus on field 1 (home_path, editable).
        model.settings.as_mut().unwrap().focused = 1;
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::OpenSettingsPicker {
                field_key: "home_path".into()
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
        // Focus on field 2 (telemetry, non-editable toggle).
        let settings = model.settings.as_mut().unwrap();
        settings.focused = 2;
        // Set explicitly so the test doesn't depend on the default.
        settings.fields[2].value = "On".to_string();
        // Value is "On", so right should toggle to Off.
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
        model.settings.as_mut().unwrap().focused = 2;
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), None);
    }

    #[test]
    fn s_key_does_nothing_outside_browser() {
        let model = AppModel {
            screen: Screen::Detail {
                slug: "t".into(),
                from: app::DetailOrigin::Browser,
            },
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
                control: None,
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
                control: None,
                visible_when: None,
            },
        ];

        AppModel {
            screen: Screen::Detail {
                slug: "compress-images".into(),
                from: app::DetailOrigin::Browser,
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
            Some(AppMessage::DetailForm(FormMessage::FocusNext))
        );
    }

    #[test]
    fn detail_k_focuses_prev_param() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::FocusPrev))
        );
    }

    #[test]
    fn detail_arrow_keys_navigate() {
        let model = detail_model();
        let down = KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, down),
            Some(AppMessage::DetailForm(FormMessage::FocusNext))
        );
        let up = KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, up),
            Some(AppMessage::DetailForm(FormMessage::FocusPrev))
        );
    }

    #[test]
    fn detail_enter_starts_edit_when_params_exist() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::StartEdit))
        );
    }

    #[test]
    fn detail_enter_confirms_when_no_params() {
        use screens::detail::DetailModel;

        let model = AppModel {
            screen: Screen::Detail {
                slug: "s".into(),
                from: app::DetailOrigin::Browser,
            },
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
    fn detail_tab_advances_focus_in_params() {
        let model = detail_model(); // focus = Params, form focused = 0
        let key = KeyEvent::new(KeyCode::Tab, crossterm::event::KeyModifiers::NONE);
        // Tab within Params section → FocusNext (form navigation).
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(tonkotsu::FormMessage::FocusNext))
        );
    }

    #[test]
    fn detail_tab_confirms_from_run_focus() {
        let mut model = detail_model();
        model.detail.as_mut().unwrap().focus = super::screens::detail::DetailFocus::Run;
        let key = KeyEvent::new(KeyCode::Tab, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::ConfigConfirmed {
                slug: "compress-images".into()
            })
        );
    }

    #[test]
    fn detail_enter_confirms_on_run_action() {
        let mut model = detail_model();
        model.detail.as_mut().unwrap().focus = super::screens::detail::DetailFocus::Run;
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::ConfigConfirmed {
                slug: "compress-images".into()
            })
        );
    }

    /// Put the detail model's first form field into TextEditing state.
    fn set_detail_editing(model: &mut AppModel) {
        let detail = model.detail.as_mut().unwrap();
        detail.form.fields[0].state = tonkotsu::FieldState::TextEditing {
            buffer: detail.form.fields[0].value.clone(),
            cursor: detail.form.fields[0].value.len(),
        };
    }

    #[test]
    fn detail_editing_captures_chars() {
        let mut model = detail_model();
        set_detail_editing(&mut model);
        let key = KeyEvent::new(KeyCode::Char('5'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::EditChar('5')))
        );
    }

    #[test]
    fn detail_editing_enter_commits() {
        let mut model = detail_model();
        set_detail_editing(&mut model);
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::CommitEdit))
        );
    }

    #[test]
    fn detail_editing_esc_cancels() {
        let mut model = detail_model();
        set_detail_editing(&mut model);
        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::CancelEdit))
        );
    }

    #[test]
    fn detail_editing_backspace() {
        let mut model = detail_model();
        set_detail_editing(&mut model);
        let key = KeyEvent::new(KeyCode::Backspace, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::EditBackspace))
        );
    }

    #[test]
    fn detail_editing_blocks_global_q() {
        let mut model = detail_model();
        set_detail_editing(&mut model);
        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        // 'q' is captured as EditChar, not Quit
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::EditChar('q')))
        );
    }

    #[test]
    fn detail_space_toggles_confirm() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Char(' '), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::ToggleConfirm))
        );
    }

    #[test]
    fn detail_left_right_cycles() {
        // h/l are vim shortcuts that map to CyclePrev/CycleNext unconditionally.
        // The form's update() handles whether it applies based on field kind.
        let model = detail_model();
        let right = KeyEvent::new(KeyCode::Char('l'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, right),
            Some(AppMessage::DetailForm(FormMessage::CycleNext))
        );
        let left = KeyEvent::new(KeyCode::Char('h'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, left),
            Some(AppMessage::DetailForm(FormMessage::CyclePrev))
        );
    }

    #[test]
    fn detail_arrow_keys_cycle_select() {
        // Arrow keys fall through to tonkotsu::map_key_event which maps them
        // to CyclePrev/CycleNext for Select/Number fields.
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
            control: None,
            visible_when: None,
        }];
        let model = AppModel {
            screen: Screen::Detail {
                slug: "s".into(),
                from: app::DetailOrigin::Browser,
            },
            detail: Some(DetailModel::from_test_data("s", "n", "d", params)),
            ..default_model()
        };
        let right = KeyEvent::new(KeyCode::Right, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, right),
            Some(AppMessage::DetailForm(FormMessage::CycleNext))
        );
        let left = KeyEvent::new(KeyCode::Left, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, left),
            Some(AppMessage::DetailForm(FormMessage::CyclePrev))
        );
    }

    #[test]
    fn detail_d_resets_default() {
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Char('d'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::ResetDefault))
        );
    }

    #[test]
    fn detail_enter_starts_edit_on_text_field() {
        // tonkotsu's idle key mapping: Enter → StartEdit (generic).
        // The form's dispatch handles per-kind behavior internally.
        let model = detail_model();
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::DetailForm(FormMessage::StartEdit))
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
                permissions: None,
                symlink_target: None,
            },
            FileEntry {
                name: "cat.jpg".into(),
                is_dir: false,
                path: PathBuf::from("/cat.jpg"),
                size: Some(290_000),
                permissions: None,
                symlink_target: None,
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
                from: app::DetailOrigin::Browser,
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
                from: app::DetailOrigin::Browser,
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
                from: app::DetailOrigin::Browser,
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
                from: app::DetailOrigin::Browser,
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
                node_info: vec![],
            },
        );
        let model = AppModel {
            screen: Screen::Execution {
                slug: "compress-images".into(),
                from: app::DetailOrigin::Browser,
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
                path: "photo-1.jpg".into(),
                size_bytes: 290_000,
                original_size: Some(780_000),
            },
            OutputFile {
                name: "photo-2.jpg".into(),
                path: "photo-2.jpg".into(),
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

    // --- Preview key handling ---

    fn preview_model() -> AppModel {
        use crate::dry_run::files::FilePreview;
        use screens::preview::PreviewModel;
        use std::collections::HashMap;

        let entries = vec![
            FilePreview {
                original: "VIDEO： Movie.mp4".into(),
                result: "Movie.mp4".into(),
            },
            FilePreview {
                original: "VIDEO： Concert.mp4".into(),
                result: "Concert.mp4".into(),
            },
        ];

        AppModel {
            screen: Screen::Preview {
                slug: "strip-video-prefix".into(),
                from: app::DetailOrigin::Browser,
            },
            preview: Some(PreviewModel::new(
                "strip-video-prefix",
                entries,
                245,
                vec![],
                vec![],
                HashMap::new(),
                String::new(),
            )),
            ..default_model()
        }
    }

    #[test]
    fn preview_j_moves_cursor_down() {
        use screens::preview::PreviewMessage;
        let model = preview_model();
        let key = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Preview(PreviewMessage::CursorDown))
        );
    }

    #[test]
    fn preview_k_moves_cursor_up() {
        use screens::preview::PreviewMessage;
        let model = preview_model();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::Preview(PreviewMessage::CursorUp))
        );
    }

    #[test]
    fn preview_enter_confirms() {
        let model = preview_model();
        let key = KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::PreviewConfirm {
                slug: "strip-video-prefix".into()
            })
        );
    }

    #[test]
    fn preview_esc_goes_back() {
        let model = preview_model();
        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Back));
    }

    #[test]
    fn preview_q_quits() {
        let model = preview_model();
        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(handle_key(&model, key), Some(AppMessage::Quit));
    }

    #[test]
    fn detail_p_key_triggers_preview() {
        let mut model = detail_model();
        model.detail.as_mut().unwrap().focus = super::screens::detail::DetailFocus::Run;
        let key = KeyEvent::new(KeyCode::Char('p'), crossterm::event::KeyModifiers::NONE);
        assert_eq!(
            handle_key(&model, key),
            Some(AppMessage::PreviewRequested {
                slug: "compress-images".into()
            })
        );
    }

    #[test]
    fn preview_back_returns_to_detail() {
        let model = preview_model();
        let model = update(model, AppMessage::Back);
        assert!(matches!(
            model.screen,
            Screen::Detail {
                slug,
                from: app::DetailOrigin::Browser,
            } if slug == "strip-video-prefix"
        ));
        assert!(model.preview.is_none());
    }
}
