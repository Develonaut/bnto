// App state machine — routes between TUI screens.
//
// Pure state + pure transitions. No I/O, no terminal access.
// All screen navigation logic is testable with `cargo test`.

use bnto_core::registry::NodeRegistry;
use bnto_engine::create_registry;

use super::screens::browser::{BrowserMessage, BrowserModel, update as browser_update};
use super::screens::detail::{DetailMessage, DetailModel, update as detail_update};
use super::theme::{Theme, ThemeVariant};

/// Which screen the TUI is currently showing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Browser,
    Detail { slug: String },
    Picker { slug: String },
    Execution { slug: String },
    Results { slug: String },
    Settings,
}

/// Top-level app state.
pub struct AppModel {
    pub screen: Screen,
    pub should_quit: bool,
    pub theme: Theme,
    pub theme_variant: ThemeVariant,
    pub browser: BrowserModel,
    /// Detail screen state — populated when navigating to a recipe.
    pub detail: Option<DetailModel>,
    /// Engine registry for resolving processor metadata.
    pub registry: NodeRegistry,
}

impl std::fmt::Debug for AppModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppModel")
            .field("screen", &self.screen)
            .field("should_quit", &self.should_quit)
            .field("theme_variant", &self.theme_variant)
            .field("detail", &self.detail)
            .finish_non_exhaustive()
    }
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
    /// Open the settings screen.
    OpenSettings,
    /// User selected a new theme in settings.
    ThemeChanged(ThemeVariant),
    /// Forward a message to the browser screen.
    Browser(BrowserMessage),
    /// Forward a message to the detail screen.
    Detail(DetailMessage),
    /// Quit the application.
    Quit,
}

impl AppModel {
    /// Create a new app starting on the recipe browser.
    pub fn new(variant: ThemeVariant) -> Self {
        Self {
            screen: Screen::Browser,
            should_quit: false,
            theme: Theme::from_variant(variant),
            theme_variant: variant,
            browser: BrowserModel::new(),
            detail: None,
            registry: create_registry(),
        }
    }
}

/// Pure state transition — the heart of the TEA pattern.
/// Takes current state + message, returns the next state.
pub fn update(model: AppModel, msg: AppMessage) -> AppModel {
    match msg {
        AppMessage::RecipeSelected { slug } => {
            let detail = DetailModel::from_slug(&slug, &model.registry);
            AppModel {
                screen: Screen::Detail { slug },
                detail,
                ..model
            }
        }
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
        AppMessage::Back => {
            let detail = match &model.screen {
                Screen::Detail { .. } => None,
                _ => model.detail,
            };
            AppModel {
                screen: back_screen(&model.screen),
                detail,
                ..model
            }
        }
        AppMessage::RunAnother => AppModel {
            screen: Screen::Browser,
            ..model
        },
        AppMessage::OpenSettings => AppModel {
            screen: Screen::Settings,
            ..model
        },
        AppMessage::ThemeChanged(variant) => AppModel {
            theme: Theme::from_variant(variant),
            theme_variant: variant,
            ..model
        },
        AppMessage::Browser(msg) => {
            let browser = browser_update(model.browser, msg);
            AppModel { browser, ..model }
        }
        AppMessage::Detail(msg) => {
            let detail = model.detail.map(|d| detail_update(d, msg));
            AppModel { detail, ..model }
        }
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
        Screen::Settings => Screen::Browser,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_model() -> AppModel {
        AppModel::new(ThemeVariant::LosAngeles)
    }

    /// Apply a message to a model on the given screen, return the resulting screen.
    fn transition(screen: Screen, msg: AppMessage) -> Screen {
        update(
            AppModel {
                screen,
                ..default_model()
            },
            msg,
        )
        .screen
    }

    #[test]
    fn initial_state_is_browser() {
        let app = default_model();
        assert_eq!(app.screen, Screen::Browser);
        assert!(!app.should_quit);
    }

    // --- Forward navigation (happy path) ---

    #[test]
    fn forward_navigation_follows_happy_path() {
        let s = "t".to_string();
        assert_eq!(
            transition(
                Screen::Browser,
                AppMessage::RecipeSelected { slug: s.clone() }
            ),
            Screen::Detail { slug: s.clone() }
        );
        assert_eq!(
            transition(
                Screen::Detail { slug: s.clone() },
                AppMessage::ConfigConfirmed { slug: s.clone() }
            ),
            Screen::Picker { slug: s.clone() }
        );
        assert_eq!(
            transition(
                Screen::Picker { slug: s.clone() },
                AppMessage::FilesSelected { slug: s.clone() }
            ),
            Screen::Execution { slug: s.clone() }
        );
        assert_eq!(
            transition(
                Screen::Execution { slug: s.clone() },
                AppMessage::ExecutionComplete { slug: s.clone() }
            ),
            Screen::Results { slug: s }
        );
    }

    #[test]
    fn back_navigation() {
        let s = "t".to_string();
        assert_eq!(
            transition(Screen::Browser, AppMessage::Back),
            Screen::Browser
        );
        assert_eq!(
            transition(Screen::Detail { slug: s.clone() }, AppMessage::Back),
            Screen::Browser
        );
        assert_eq!(
            transition(Screen::Picker { slug: s }, AppMessage::Back),
            Screen::Detail {
                slug: "t".to_string()
            }
        );
        assert_eq!(
            transition(Screen::Execution { slug: "r".into() }, AppMessage::Back),
            Screen::Browser
        );
        assert_eq!(
            transition(Screen::Results { slug: "r".into() }, AppMessage::Back),
            Screen::Browser
        );
        assert_eq!(
            transition(Screen::Settings, AppMessage::Back),
            Screen::Browser
        );
    }

    // --- Other actions ---

    #[test]
    fn run_another_goes_to_browser() {
        let s = transition(Screen::Results { slug: "t".into() }, AppMessage::RunAnother);
        assert_eq!(s, Screen::Browser);
    }

    #[test]
    fn quit_sets_should_quit() {
        let app = update(default_model(), AppMessage::Quit);
        assert!(app.should_quit);
    }

    #[test]
    fn settings_navigation() {
        assert_eq!(
            transition(Screen::Browser, AppMessage::OpenSettings),
            Screen::Settings
        );
        assert_eq!(
            transition(Screen::Settings, AppMessage::Back),
            Screen::Browser
        );
    }

    #[test]
    fn theme_changed_swaps_theme_without_changing_screen() {
        let app = update(
            AppModel {
                screen: Screen::Settings,
                ..default_model()
            },
            AppMessage::ThemeChanged(ThemeVariant::Tokyo),
        );
        assert_eq!(app.screen, Screen::Settings);
        assert_eq!(app.theme_variant, ThemeVariant::Tokyo);
    }
}
