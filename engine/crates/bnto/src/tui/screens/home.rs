// Home screen — bento grid with pane-based navigation.
//
// TEA pattern: HomeModel (state) + HomeMessage (events) + update() (pure transitions).
// Four panes: Library (top-left), Recipes (right), NewRecipe (bottom-left top),
// Settings (bottom-left bottom). Tab cycles between panes, j/k navigates within.

use super::super::app::Screen;

/// Which bento compartment is focused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomePane {
    /// Top-left — user's saved recipes.
    Library,
    /// Right column — built-in recipe catalog.
    Recipes,
    /// Bottom-left top — create new recipe action.
    NewRecipe,
    /// Bottom-left bottom — preferences action.
    Settings,
}

/// Pane cycling order for Tab navigation.
const PANE_ORDER: [HomePane; 4] = [
    HomePane::Library,
    HomePane::Recipes,
    HomePane::NewRecipe,
    HomePane::Settings,
];

/// Home screen state.
#[derive(Debug)]
pub struct HomeModel {
    /// Which pane is currently focused.
    pub focused: HomePane,
    /// Cursor within the Library pane.
    pub library_cursor: usize,
    /// File stems of `.bnto.json` files in the library directory.
    pub library_names: Vec<String>,
    /// Cursor within the Recipes pane.
    pub recipes_cursor: usize,
}

/// Messages the home screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeMessage {
    /// Tab / l / Right — move focus to the next pane.
    NextPane,
    /// Shift-Tab / h / Left — move focus to the previous pane.
    PrevPane,
    /// j / Down — move cursor within the focused pane.
    CursorDown,
    /// k / Up — move cursor within the focused pane.
    CursorUp,
}

/// Result of confirming a selection — which screen to navigate to.
pub enum HomeConfirmResult {
    /// Navigate to the given screen.
    Navigate(Screen),
    /// Show a status message (e.g. "Coming soon" for New Recipe).
    StatusMessage(String),
    /// Select a recipe from the Recipes pane by index into browser.recipes.
    RecipeAtIndex(usize),
    /// Select a library recipe by its slug.
    LibraryRecipe(String),
}

impl HomeModel {
    /// Create a new home screen with library recipe names.
    pub fn new(library_names: Vec<String>) -> Self {
        Self {
            focused: HomePane::Library,
            library_cursor: 0,
            library_names,
            recipes_cursor: 0,
        }
    }

    /// Confirm the current selection based on focused pane.
    pub fn confirm(&self) -> HomeConfirmResult {
        match self.focused {
            HomePane::Library => {
                if self.library_names.is_empty() {
                    HomeConfirmResult::StatusMessage("No recipes in library".into())
                } else {
                    let slug = self.library_names
                        [self.library_cursor.min(self.library_names.len() - 1)]
                    .clone();
                    HomeConfirmResult::LibraryRecipe(slug)
                }
            }
            HomePane::Recipes => HomeConfirmResult::RecipeAtIndex(self.recipes_cursor),
            HomePane::NewRecipe => {
                HomeConfirmResult::StatusMessage("New Recipe — coming soon".into())
            }
            HomePane::Settings => HomeConfirmResult::Navigate(Screen::Settings),
        }
    }
}

/// Pure state transition for the home screen.
pub fn update(model: HomeModel, msg: HomeMessage) -> HomeModel {
    match msg {
        HomeMessage::NextPane => {
            let idx = pane_index(model.focused);
            let next = PANE_ORDER[(idx + 1) % PANE_ORDER.len()];
            HomeModel {
                focused: next,
                ..model
            }
        }
        HomeMessage::PrevPane => {
            let idx = pane_index(model.focused);
            let prev = if idx == 0 {
                PANE_ORDER[PANE_ORDER.len() - 1]
            } else {
                PANE_ORDER[idx - 1]
            };
            HomeModel {
                focused: prev,
                ..model
            }
        }
        HomeMessage::CursorDown => match model.focused {
            HomePane::Library if !model.library_names.is_empty() => {
                let next = (model.library_cursor + 1) % model.library_names.len();
                HomeModel {
                    library_cursor: next,
                    ..model
                }
            }
            HomePane::Recipes => {
                // Cursor bound is checked at render time against browser.recipes.len().
                // Here we just increment — app.rs clamps if needed.
                HomeModel {
                    recipes_cursor: model.recipes_cursor + 1,
                    ..model
                }
            }
            _ => model, // No-op on action panes or empty library
        },
        HomeMessage::CursorUp => match model.focused {
            HomePane::Library if !model.library_names.is_empty() => {
                let prev = if model.library_cursor == 0 {
                    model.library_names.len() - 1
                } else {
                    model.library_cursor - 1
                };
                HomeModel {
                    library_cursor: prev,
                    ..model
                }
            }
            HomePane::Recipes => HomeModel {
                recipes_cursor: model.recipes_cursor.saturating_sub(1),
                ..model
            },
            _ => model,
        },
    }
}

/// Find the index of a pane in the cycling order.
fn pane_index(pane: HomePane) -> usize {
    PANE_ORDER.iter().position(|p| *p == pane).unwrap_or(0)
}

/// List `.bnto.json` file stems in a directory (non-recursive).
///
/// Returns stems like `["compress-images", "resize-images"]`.
pub fn list_library_recipes(dir: &std::path::Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.ends_with(".bnto.json"))
                })
                .filter_map(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.trim_end_matches(".bnto.json").to_string())
                })
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_focuses_library() {
        let m = HomeModel::new(vec![]);
        assert_eq!(m.focused, HomePane::Library);
        assert_eq!(m.library_cursor, 0);
        assert_eq!(m.recipes_cursor, 0);
    }

    #[test]
    fn next_pane_cycles_library_to_recipes() {
        let m = HomeModel::new(vec![]);
        let m = update(m, HomeMessage::NextPane);
        assert_eq!(m.focused, HomePane::Recipes);
    }

    #[test]
    fn next_pane_cycles_recipes_to_new_recipe() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::Recipes;
        let m = update(m, HomeMessage::NextPane);
        assert_eq!(m.focused, HomePane::NewRecipe);
    }

    #[test]
    fn next_pane_cycles_new_recipe_to_settings() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::NewRecipe;
        let m = update(m, HomeMessage::NextPane);
        assert_eq!(m.focused, HomePane::Settings);
    }

    #[test]
    fn next_pane_cycles_settings_to_library() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::Settings;
        let m = update(m, HomeMessage::NextPane);
        assert_eq!(m.focused, HomePane::Library);
    }

    #[test]
    fn prev_pane_cycles_library_to_settings() {
        let m = HomeModel::new(vec![]);
        let m = update(m, HomeMessage::PrevPane);
        assert_eq!(m.focused, HomePane::Settings);
    }

    #[test]
    fn prev_pane_cycles_recipes_to_library() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::Recipes;
        let m = update(m, HomeMessage::PrevPane);
        assert_eq!(m.focused, HomePane::Library);
    }

    #[test]
    fn cursor_down_in_library_moves() {
        let m = HomeModel::new(vec!["a".into(), "b".into(), "c".into()]);
        let m = update(m, HomeMessage::CursorDown);
        assert_eq!(m.library_cursor, 1);
    }

    #[test]
    fn cursor_up_in_library_moves() {
        let mut m = HomeModel::new(vec!["a".into(), "b".into()]);
        m.library_cursor = 1;
        let m = update(m, HomeMessage::CursorUp);
        assert_eq!(m.library_cursor, 0);
    }

    #[test]
    fn cursor_wraps_at_library_end() {
        let mut m = HomeModel::new(vec!["a".into(), "b".into()]);
        m.library_cursor = 1;
        let m = update(m, HomeMessage::CursorDown);
        assert_eq!(m.library_cursor, 0);
    }

    #[test]
    fn cursor_wraps_at_library_start() {
        let m = HomeModel::new(vec!["a".into(), "b".into()]);
        let m = update(m, HomeMessage::CursorUp);
        assert_eq!(m.library_cursor, 1);
    }

    #[test]
    fn cursor_down_in_recipes_moves() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::Recipes;
        let m = update(m, HomeMessage::CursorDown);
        assert_eq!(m.recipes_cursor, 1);
    }

    #[test]
    fn cursor_up_in_recipes_moves() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::Recipes;
        m.recipes_cursor = 2;
        let m = update(m, HomeMessage::CursorUp);
        assert_eq!(m.recipes_cursor, 1);
    }

    #[test]
    fn cursor_up_in_recipes_saturates_at_zero() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::Recipes;
        m.recipes_cursor = 0;
        let m = update(m, HomeMessage::CursorUp);
        assert_eq!(m.recipes_cursor, 0);
    }

    #[test]
    fn cursor_noop_on_action_panes() {
        for pane in [HomePane::NewRecipe, HomePane::Settings] {
            let mut m = HomeModel::new(vec![]);
            m.focused = pane;
            let m = update(m, HomeMessage::CursorDown);
            assert_eq!(m.library_cursor, 0);
            assert_eq!(m.recipes_cursor, 0);
        }
    }

    #[test]
    fn cursor_noop_on_empty_library() {
        let m = HomeModel::new(vec![]);
        let m = update(m, HomeMessage::CursorDown);
        assert_eq!(m.library_cursor, 0);
    }

    #[test]
    fn confirm_library_with_recipes_returns_slug() {
        let m = HomeModel::new(vec!["compress-images".into(), "resize".into()]);
        match m.confirm() {
            HomeConfirmResult::LibraryRecipe(slug) => {
                assert_eq!(slug, "compress-images");
            }
            other => panic!("expected LibraryRecipe, got {:?}", confirm_debug(&other)),
        }
    }

    #[test]
    fn confirm_library_empty_shows_status() {
        let m = HomeModel::new(vec![]);
        match m.confirm() {
            HomeConfirmResult::StatusMessage(msg) => {
                assert!(msg.contains("No recipes"), "got: {msg}");
            }
            other => panic!("expected StatusMessage, got {:?}", confirm_debug(&other)),
        }
    }

    #[test]
    fn confirm_recipes_returns_index() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::Recipes;
        m.recipes_cursor = 3;
        match m.confirm() {
            HomeConfirmResult::RecipeAtIndex(idx) => assert_eq!(idx, 3),
            other => panic!("expected RecipeAtIndex, got {:?}", confirm_debug(&other)),
        }
    }

    #[test]
    fn confirm_new_recipe_shows_coming_soon() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::NewRecipe;
        match m.confirm() {
            HomeConfirmResult::StatusMessage(msg) => {
                assert!(msg.contains("coming soon"), "got: {msg}");
            }
            other => panic!("expected StatusMessage, got {:?}", confirm_debug(&other)),
        }
    }

    #[test]
    fn confirm_settings_navigates() {
        let mut m = HomeModel::new(vec![]);
        m.focused = HomePane::Settings;
        match m.confirm() {
            HomeConfirmResult::Navigate(Screen::Settings) => {}
            other => panic!(
                "expected Navigate(Settings), got {:?}",
                confirm_debug(&other)
            ),
        }
    }

    #[test]
    fn library_names_len_returns_count() {
        let m = HomeModel::new(vec!["a".into(), "b".into()]);
        assert_eq!(m.library_names.len(), 2);
    }

    #[test]
    fn list_library_recipes_returns_stems() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("compress.bnto.json"), "{}").unwrap();
        std::fs::write(tmp.path().join("resize.bnto.json"), "{}").unwrap();
        std::fs::write(tmp.path().join("notes.txt"), "not a recipe").unwrap();
        let names = list_library_recipes(tmp.path());
        assert_eq!(names, vec!["compress", "resize"]);
    }

    #[test]
    fn list_library_recipes_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(list_library_recipes(tmp.path()).is_empty());
    }

    #[test]
    fn list_library_recipes_missing_dir() {
        assert!(list_library_recipes(std::path::Path::new("/nonexistent/path")).is_empty());
    }

    #[test]
    fn list_library_recipes_count_matches_len() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("a.bnto.json"), "{}").unwrap();
        std::fs::write(tmp.path().join("b.bnto.json"), "{}").unwrap();
        assert_eq!(list_library_recipes(tmp.path()).len(), 2);
    }

    /// Debug helper for HomeConfirmResult (not derived).
    fn confirm_debug(result: &HomeConfirmResult) -> String {
        match result {
            HomeConfirmResult::Navigate(s) => format!("Navigate({s:?})"),
            HomeConfirmResult::StatusMessage(m) => format!("StatusMessage({m})"),
            HomeConfirmResult::RecipeAtIndex(i) => format!("RecipeAtIndex({i})"),
            HomeConfirmResult::LibraryRecipe(s) => format!("LibraryRecipe({s})"),
        }
    }
}
