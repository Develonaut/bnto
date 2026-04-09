// App state machine — routes between TUI screens.
//
// Pure state + pure transitions. No I/O, no terminal access.
// All screen navigation logic is testable with `cargo test`.

/// Which screen the TUI is currently showing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Browser,
    Detail { slug: String },
    Picker { slug: String },
    Execution { slug: String },
    Results { slug: String },
}

/// Top-level app state.
#[derive(Debug)]
pub struct AppModel {
    pub screen: Screen,
    pub should_quit: bool,
}

/// Messages that drive screen transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AppMessage {
    /// User selected a recipe in the browser.
    RecipeSelected { slug: String },
    /// User confirmed config and moves to file picker.
    ConfigConfirmed { slug: String },
    /// User selected files and starts execution.
    FilesSelected { slug: String },
    /// Execution finished, show results.
    ExecutionComplete { slug: String },
    /// Navigate back one screen.
    Back,
    /// User wants to run another recipe (from results).
    RunAnother,
    /// Quit the application.
    Quit,
}

impl AppModel {
    /// Create a new app starting on the recipe browser.
    pub fn new() -> Self {
        Self {
            screen: Screen::Browser,
            should_quit: false,
        }
    }
}

/// Pure state transition — the heart of the TEA pattern.
/// Takes current state + message, returns the next state.
pub fn update(model: AppModel, msg: AppMessage) -> AppModel {
    match msg {
        AppMessage::RecipeSelected { slug } => AppModel {
            screen: Screen::Detail { slug },
            ..model
        },
        AppMessage::ConfigConfirmed { slug } => AppModel {
            screen: Screen::Picker { slug },
            ..model
        },
        AppMessage::FilesSelected { slug } => AppModel {
            screen: Screen::Execution { slug },
            ..model
        },
        AppMessage::ExecutionComplete { slug } => AppModel {
            screen: Screen::Results { slug },
            ..model
        },
        AppMessage::Back => AppModel {
            screen: back_screen(&model.screen),
            ..model
        },
        AppMessage::RunAnother => AppModel {
            screen: Screen::Browser,
            ..model
        },
        AppMessage::Quit => AppModel {
            should_quit: true,
            ..model
        },
    }
}

/// Determine which screen to go back to from the current screen.
fn back_screen(current: &Screen) -> Screen {
    match current {
        Screen::Browser => Screen::Browser,
        Screen::Detail { .. } => Screen::Browser,
        Screen::Picker { slug } => Screen::Detail { slug: slug.clone() },
        Screen::Execution { .. } => Screen::Browser,
        Screen::Results { .. } => Screen::Browser,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_browser() {
        let app = AppModel::new();
        assert_eq!(app.screen, Screen::Browser);
        assert!(!app.should_quit);
    }

    #[test]
    fn recipe_selected_navigates_to_detail() {
        let app = AppModel::new();
        let app = update(
            app,
            AppMessage::RecipeSelected {
                slug: "compress-images".into(),
            },
        );
        assert_eq!(
            app.screen,
            Screen::Detail {
                slug: "compress-images".into()
            }
        );
    }

    #[test]
    fn config_confirmed_navigates_to_picker() {
        let app = AppModel {
            screen: Screen::Detail {
                slug: "resize-images".into(),
            },
            should_quit: false,
        };
        let app = update(
            app,
            AppMessage::ConfigConfirmed {
                slug: "resize-images".into(),
            },
        );
        assert_eq!(
            app.screen,
            Screen::Picker {
                slug: "resize-images".into()
            }
        );
    }

    #[test]
    fn files_selected_navigates_to_execution() {
        let app = AppModel {
            screen: Screen::Picker {
                slug: "clean-csv".into(),
            },
            should_quit: false,
        };
        let app = update(
            app,
            AppMessage::FilesSelected {
                slug: "clean-csv".into(),
            },
        );
        assert_eq!(
            app.screen,
            Screen::Execution {
                slug: "clean-csv".into()
            }
        );
    }

    #[test]
    fn execution_complete_navigates_to_results() {
        let app = AppModel {
            screen: Screen::Execution {
                slug: "clean-csv".into(),
            },
            should_quit: false,
        };
        let app = update(
            app,
            AppMessage::ExecutionComplete {
                slug: "clean-csv".into(),
            },
        );
        assert_eq!(
            app.screen,
            Screen::Results {
                slug: "clean-csv".into()
            }
        );
    }

    #[test]
    fn back_from_detail_goes_to_browser() {
        let app = AppModel {
            screen: Screen::Detail {
                slug: "compress-images".into(),
            },
            should_quit: false,
        };
        let app = update(app, AppMessage::Back);
        assert_eq!(app.screen, Screen::Browser);
    }

    #[test]
    fn back_from_picker_goes_to_detail() {
        let app = AppModel {
            screen: Screen::Picker {
                slug: "compress-images".into(),
            },
            should_quit: false,
        };
        let app = update(app, AppMessage::Back);
        assert_eq!(
            app.screen,
            Screen::Detail {
                slug: "compress-images".into()
            }
        );
    }

    #[test]
    fn back_from_browser_stays_on_browser() {
        let app = AppModel::new();
        let app = update(app, AppMessage::Back);
        assert_eq!(app.screen, Screen::Browser);
    }

    #[test]
    fn run_another_goes_to_browser() {
        let app = AppModel {
            screen: Screen::Results {
                slug: "clean-csv".into(),
            },
            should_quit: false,
        };
        let app = update(app, AppMessage::RunAnother);
        assert_eq!(app.screen, Screen::Browser);
    }

    #[test]
    fn quit_sets_should_quit() {
        let app = AppModel::new();
        let app = update(app, AppMessage::Quit);
        assert!(app.should_quit);
    }
}
