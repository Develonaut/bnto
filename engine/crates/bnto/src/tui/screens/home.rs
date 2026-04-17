// Home screen — main menu with navigation to Library, Recipes, New Recipe, Settings.
//
// TEA pattern: HomeModel (state) + HomeMessage (events) + update() (pure transitions).

use super::super::app::Screen;

/// Menu items on the home screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeItem {
    Library,
    Recipes,
    NewRecipe,
    Settings,
}

impl HomeItem {
    /// Display label for the menu item.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Library => "My Library",
            Self::Recipes => "Recipes",
            Self::NewRecipe => "New Recipe",
            Self::Settings => "Settings",
        }
    }

    /// Description shown beside the label.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Library => "Your recipes",
            Self::Recipes => "Browse & discover",
            Self::NewRecipe => "Create from scratch",
            Self::Settings => "Preferences",
        }
    }
}

/// All menu items in display order.
pub const HOME_ITEMS: [HomeItem; 4] = [
    HomeItem::Library,
    HomeItem::Recipes,
    HomeItem::NewRecipe,
    HomeItem::Settings,
];

/// Home screen state.
#[derive(Debug)]
pub struct HomeModel {
    /// Cursor position (0..3).
    pub selected: usize,
    /// Number of recipes in the user's library directory.
    pub library_count: usize,
}

/// Messages the home screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeMessage {
    SelectNext,
    SelectPrev,
}

/// Result of confirming a selection — which screen to navigate to.
pub enum HomeConfirmResult {
    /// Navigate to the given screen.
    Navigate(Screen),
    /// Show a status message (e.g. "Coming soon" for New Recipe).
    StatusMessage(String),
}

impl HomeModel {
    /// Create a new home screen with library count.
    pub fn new(library_count: usize) -> Self {
        Self {
            selected: 0,
            library_count,
        }
    }

    /// Which item is currently selected.
    pub fn selected_item(&self) -> HomeItem {
        HOME_ITEMS[self.selected]
    }

    /// Confirm the current selection — returns a screen or status message.
    pub fn confirm(&self) -> HomeConfirmResult {
        match self.selected_item() {
            HomeItem::Library => HomeConfirmResult::Navigate(Screen::Library),
            HomeItem::Recipes => HomeConfirmResult::Navigate(Screen::Browser),
            HomeItem::NewRecipe => {
                HomeConfirmResult::StatusMessage("New Recipe — coming soon".into())
            }
            HomeItem::Settings => HomeConfirmResult::Navigate(Screen::Settings),
        }
    }
}

/// Pure state transition for the home screen.
pub fn update(model: HomeModel, msg: HomeMessage) -> HomeModel {
    let count = HOME_ITEMS.len();
    let selected = match msg {
        HomeMessage::SelectNext => (model.selected + 1) % count,
        HomeMessage::SelectPrev => {
            if model.selected == 0 {
                count - 1
            } else {
                model.selected - 1
            }
        }
    };
    HomeModel { selected, ..model }
}

/// Count `.bnto.json` files in a directory (non-recursive).
pub fn count_library_recipes(dir: &std::path::Path) -> usize {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().extension().is_some_and(|ext| ext == "json")
                        && e.path()
                            .file_name()
                            .and_then(|n| n.to_str())
                            .is_some_and(|n| n.ends_with(".bnto.json"))
                })
                .count()
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_selects_first_item() {
        let m = HomeModel::new(0);
        assert_eq!(m.selected, 0);
        assert_eq!(m.library_count, 0);
        assert_eq!(m.selected_item(), HomeItem::Library);
    }

    #[test]
    fn select_next_moves_down() {
        let m = HomeModel::new(0);
        let m = update(m, HomeMessage::SelectNext);
        assert_eq!(m.selected, 1);
        assert_eq!(m.selected_item(), HomeItem::Recipes);
    }

    #[test]
    fn select_prev_moves_up() {
        let mut m = HomeModel::new(0);
        m.selected = 2;
        let m = update(m, HomeMessage::SelectPrev);
        assert_eq!(m.selected, 1);
    }

    #[test]
    fn select_next_wraps_at_end() {
        let mut m = HomeModel::new(0);
        m.selected = 3; // last item
        let m = update(m, HomeMessage::SelectNext);
        assert_eq!(m.selected, 0);
    }

    #[test]
    fn select_prev_wraps_at_start() {
        let m = HomeModel::new(0);
        let m = update(m, HomeMessage::SelectPrev);
        assert_eq!(m.selected, 3);
    }

    #[test]
    fn confirm_library_navigates_to_library() {
        let m = HomeModel::new(0); // Library is item 0
        match m.confirm() {
            HomeConfirmResult::Navigate(Screen::Library) => {}
            other => panic!(
                "expected Navigate(Library), got {:?}",
                confirm_debug(&other)
            ),
        }
    }

    #[test]
    fn confirm_recipes_navigates_to_browser() {
        let mut m = HomeModel::new(0);
        m.selected = 1;
        match m.confirm() {
            HomeConfirmResult::Navigate(Screen::Browser) => {}
            other => panic!(
                "expected Navigate(Browser), got {:?}",
                confirm_debug(&other)
            ),
        }
    }

    #[test]
    fn confirm_new_recipe_shows_status() {
        let mut m = HomeModel::new(0);
        m.selected = 2;
        match m.confirm() {
            HomeConfirmResult::StatusMessage(msg) => {
                assert!(
                    msg.contains("coming soon"),
                    "expected coming soon, got: {msg}"
                );
            }
            other => panic!("expected StatusMessage, got {:?}", confirm_debug(&other)),
        }
    }

    #[test]
    fn confirm_settings_navigates_to_settings() {
        let mut m = HomeModel::new(0);
        m.selected = 3;
        match m.confirm() {
            HomeConfirmResult::Navigate(Screen::Settings) => {}
            other => panic!(
                "expected Navigate(Settings), got {:?}",
                confirm_debug(&other)
            ),
        }
    }

    #[test]
    fn library_count_reflected_in_model() {
        let m = HomeModel::new(5);
        assert_eq!(m.library_count, 5);
    }

    #[test]
    fn count_library_recipes_with_files() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("compress.bnto.json"), "{}").unwrap();
        std::fs::write(tmp.path().join("resize.bnto.json"), "{}").unwrap();
        std::fs::write(tmp.path().join("notes.txt"), "not a recipe").unwrap();
        assert_eq!(count_library_recipes(tmp.path()), 2);
    }

    #[test]
    fn count_library_recipes_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(count_library_recipes(tmp.path()), 0);
    }

    #[test]
    fn count_library_recipes_missing_dir() {
        assert_eq!(
            count_library_recipes(std::path::Path::new("/nonexistent/path")),
            0
        );
    }

    /// Debug helper for HomeConfirmResult (not derived).
    fn confirm_debug(result: &HomeConfirmResult) -> String {
        match result {
            HomeConfirmResult::Navigate(s) => format!("Navigate({s:?})"),
            HomeConfirmResult::StatusMessage(m) => format!("StatusMessage({m})"),
        }
    }
}
