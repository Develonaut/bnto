// Recipe browser screen — browse, search, and select a recipe.
//
// TEA pattern: BrowserModel (state) + BrowserMessage (events) + update() (pure transitions).
// All state logic is testable with `cargo test` — no terminal needed.

use crate::catalog::{CatalogEntry, RecipeSource};

use super::viewport;

/// Summary of a recipe for display in the browser list.
#[derive(Debug, Clone)]
pub struct RecipeSummary {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub category: String,
    /// Whether this recipe is bundled (for trust classification).
    #[allow(dead_code)]
    pub is_bundled: bool,
    /// Raw definition JSON (threaded to detail/execution screens).
    pub definition_json: String,
}

impl From<&CatalogEntry> for RecipeSummary {
    fn from(e: &CatalogEntry) -> Self {
        Self {
            slug: e.slug.clone(),
            name: e.name.clone(),
            description: e.description.clone(),
            category: e.category.clone(),
            is_bundled: e.source == RecipeSource::Bundled,
            definition_json: e.definition_json.clone(),
        }
    }
}

/// Browser screen state.
#[derive(Debug)]
pub struct BrowserModel {
    /// All available recipes.
    pub recipes: Vec<RecipeSummary>,
    /// Indices into `recipes` after search/category filtering.
    pub filtered: Vec<usize>,
    /// Cursor position within the `filtered` list.
    pub cursor: usize,
    /// Current search query text.
    pub search_query: String,
    /// Whether the search input is active (accepting text input).
    pub searching: bool,
    /// First visible item index (scroll offset).
    pub viewport_offset: usize,
    /// Number of items visible in the viewport.
    pub viewport_height: usize,
}

/// Messages the browser screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserMessage {
    /// Move cursor down one item.
    CursorDown,
    /// Move cursor up one item.
    CursorUp,
    /// User typed a character into the search input.
    SearchInput(char),
    /// User deleted a character from the search input.
    SearchBackspace,
    /// Enter search mode (activate the search input).
    EnterSearch,
    /// Exit search mode (deactivate the search input).
    ExitSearch,
    /// Clear the search query entirely.
    SearchClear,
    /// Terminal resized — update viewport height.
    Resize { height: usize },
}

/// The result of confirming a selection — the slug of the chosen recipe.
pub struct SelectionResult {
    pub slug: String,
}

impl BrowserModel {
    /// Create a new browser loaded with the given catalog recipes.
    pub fn new(entries: &[&CatalogEntry]) -> Self {
        let recipes: Vec<RecipeSummary> = entries.iter().map(|e| RecipeSummary::from(*e)).collect();
        let filtered = (0..recipes.len()).collect();
        Self {
            recipes,
            filtered,
            cursor: 0,
            search_query: String::new(),
            searching: false,
            viewport_offset: 0,
            viewport_height: 20,
        }
    }

    /// Get the currently selected recipe summary (at cursor position).
    pub fn selected_recipe(&self) -> Option<&RecipeSummary> {
        let &idx = self.filtered.get(self.cursor)?;
        self.recipes.get(idx)
    }

    /// Create a browser from a provided recipe list (for testing).
    #[cfg(test)]
    pub fn from_recipes(recipes: Vec<RecipeSummary>) -> Self {
        let filtered = (0..recipes.len()).collect();
        Self {
            recipes,
            filtered,
            cursor: 0,
            search_query: String::new(),
            searching: false,
            viewport_offset: 0,
            viewport_height: 20,
        }
    }

    /// Confirm the current selection. Returns `None` if the list is empty.
    pub fn confirm(&self) -> Option<SelectionResult> {
        let &idx = self.filtered.get(self.cursor)?;
        Some(SelectionResult {
            slug: self.recipes[idx].slug.clone(),
        })
    }
}

/// Pure state transition for the browser screen.
pub fn update(mut model: BrowserModel, msg: BrowserMessage) -> BrowserModel {
    match msg {
        BrowserMessage::CursorDown => {
            if !model.filtered.is_empty() {
                model.cursor = (model.cursor + 1) % model.filtered.len();
                model.viewport_offset = viewport::ensure_cursor_visible(
                    model.cursor,
                    model.viewport_offset,
                    model.viewport_height,
                );
            }
        }
        BrowserMessage::CursorUp => {
            if !model.filtered.is_empty() {
                model.cursor = if model.cursor == 0 {
                    model.filtered.len() - 1
                } else {
                    model.cursor - 1
                };
                model.viewport_offset = viewport::ensure_cursor_visible(
                    model.cursor,
                    model.viewport_offset,
                    model.viewport_height,
                );
            }
        }
        BrowserMessage::SearchInput(ch) => {
            model.search_query.push(ch);
            refilter(&mut model);
        }
        BrowserMessage::SearchBackspace => {
            model.search_query.pop();
            refilter(&mut model);
        }
        BrowserMessage::EnterSearch => {
            model.searching = true;
        }
        BrowserMessage::ExitSearch => {
            model.searching = false;
        }
        BrowserMessage::SearchClear => {
            model.search_query.clear();
            refilter(&mut model);
        }
        BrowserMessage::Resize { height } => {
            model.viewport_height = height;
            model.viewport_offset = viewport::ensure_cursor_visible(
                model.cursor,
                model.viewport_offset,
                model.viewport_height,
            );
        }
    }
    model
}

/// Rebuild the filtered index list based on the current search query.
fn refilter(model: &mut BrowserModel) {
    let query = model.search_query.to_lowercase();
    model.filtered = model
        .recipes
        .iter()
        .enumerate()
        .filter(|(_, r)| {
            if query.is_empty() {
                return true;
            }
            r.name.to_lowercase().contains(&query)
                || r.description.to_lowercase().contains(&query)
                || r.category.to_lowercase().contains(&query)
        })
        .map(|(i, _)| i)
        .collect();

    // Clamp cursor to new bounds.
    if model.filtered.is_empty() {
        model.cursor = 0;
    } else if model.cursor >= model.filtered.len() {
        model.cursor = model.filtered.len() - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_recipes() -> Vec<RecipeSummary> {
        vec![
            RecipeSummary {
                slug: "compress-images".into(),
                name: "Compress Images".into(),
                description: "Reduce image file size".into(),
                category: "image".into(),
                is_bundled: true,
                definition_json: "{}".into(),
            },
            RecipeSummary {
                slug: "resize-images".into(),
                name: "Resize Images".into(),
                description: "Change image dimensions".into(),
                category: "image".into(),
                is_bundled: true,
                definition_json: "{}".into(),
            },
            RecipeSummary {
                slug: "clean-csv".into(),
                name: "Clean CSV".into(),
                description: "Remove empty rows and columns".into(),
                category: "spreadsheet".into(),
                is_bundled: true,
                definition_json: "{}".into(),
            },
            RecipeSummary {
                slug: "rename-files".into(),
                name: "Rename Files".into(),
                description: "Batch rename files with patterns".into(),
                category: "file".into(),
                is_bundled: true,
                definition_json: "{}".into(),
            },
            RecipeSummary {
                slug: "download-video".into(),
                name: "Download Video".into(),
                description: "Download video from URL".into(),
                category: "video".into(),
                is_bundled: true,
                definition_json: "{}".into(),
            },
        ]
    }

    fn browser() -> BrowserModel {
        BrowserModel::from_recipes(sample_recipes())
    }

    // --- Initial state ---

    #[test]
    fn initial_state_shows_all_recipes() {
        let m = browser();
        assert_eq!(m.filtered.len(), 5);
        assert_eq!(m.cursor, 0);
        assert_eq!(m.search_query, "");
        assert!(!m.searching);
    }

    // --- Cursor navigation ---

    #[test]
    fn cursor_down_advances_by_one() {
        let m = browser();
        let m = update(m, BrowserMessage::CursorDown);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_down_wraps_at_end() {
        let mut m = browser();
        m.cursor = 4; // last item
        let m = update(m, BrowserMessage::CursorDown);
        assert_eq!(m.cursor, 0);
    }

    #[test]
    fn cursor_up_moves_back_by_one() {
        let mut m = browser();
        m.cursor = 2;
        let m = update(m, BrowserMessage::CursorUp);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_up_wraps_at_start() {
        let m = browser(); // cursor = 0
        let m = update(m, BrowserMessage::CursorUp);
        assert_eq!(m.cursor, 4);
    }

    #[test]
    fn cursor_movement_on_empty_list_is_safe() {
        let m = BrowserModel::from_recipes(vec![]);
        let m = update(m, BrowserMessage::CursorDown);
        assert_eq!(m.cursor, 0);
        let m = update(m, BrowserMessage::CursorUp);
        assert_eq!(m.cursor, 0);
    }

    // --- Search filtering ---

    #[test]
    fn search_filters_by_name() {
        let m = browser();
        let m = update(m, BrowserMessage::SearchInput('c'));
        let m = update(m, BrowserMessage::SearchInput('o'));
        let m = update(m, BrowserMessage::SearchInput('m'));
        let m = update(m, BrowserMessage::SearchInput('p'));
        // "comp" matches "Compress Images"
        assert_eq!(m.filtered.len(), 1);
        assert_eq!(m.recipes[m.filtered[0]].slug, "compress-images");
    }

    #[test]
    fn search_filters_by_description() {
        let m = browser();
        let m = update(m, BrowserMessage::SearchInput('d'));
        let m = update(m, BrowserMessage::SearchInput('i'));
        let m = update(m, BrowserMessage::SearchInput('m'));
        // "dim" matches "Change image dimensions"
        assert_eq!(m.filtered.len(), 1);
        assert_eq!(m.recipes[m.filtered[0]].slug, "resize-images");
    }

    #[test]
    fn search_filters_by_category() {
        let mut m = browser();
        for ch in "spread".chars() {
            m = update(m, BrowserMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 1);
        assert_eq!(m.recipes[m.filtered[0]].slug, "clean-csv");
    }

    #[test]
    fn search_is_case_insensitive() {
        let mut m = browser();
        for ch in "COMPRESS".chars() {
            m = update(m, BrowserMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 1);
        assert_eq!(m.recipes[m.filtered[0]].slug, "compress-images");
    }

    #[test]
    fn empty_search_shows_all_recipes() {
        let mut m = browser();
        m = update(m, BrowserMessage::SearchInput('x'));
        assert!(m.filtered.len() < 5);
        m = update(m, BrowserMessage::SearchBackspace);
        assert_eq!(m.filtered.len(), 5);
    }

    #[test]
    fn search_clamps_cursor_when_results_shrink() {
        let mut m = browser();
        m.cursor = 4; // last item
        // Search that narrows to 1 result — cursor must clamp.
        for ch in "video".chars() {
            m = update(m, BrowserMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 1);
        assert_eq!(m.cursor, 0);
    }

    #[test]
    fn search_clear_resets_filter() {
        let mut m = browser();
        for ch in "video".chars() {
            m = update(m, BrowserMessage::SearchInput(ch));
        }
        assert_eq!(m.filtered.len(), 1);
        m = update(m, BrowserMessage::SearchClear);
        assert_eq!(m.filtered.len(), 5);
        assert_eq!(m.search_query, "");
    }

    // --- Search mode toggle ---

    #[test]
    fn enter_search_activates_search_mode() {
        let m = browser();
        let m = update(m, BrowserMessage::EnterSearch);
        assert!(m.searching);
    }

    #[test]
    fn exit_search_deactivates_search_mode() {
        let mut m = browser();
        m.searching = true;
        let m = update(m, BrowserMessage::ExitSearch);
        assert!(!m.searching);
    }

    // --- Selection ---

    #[test]
    fn confirm_returns_selected_recipe_slug() {
        let mut m = browser();
        m.cursor = 2;
        let result = m.confirm().expect("should have a selection");
        assert_eq!(result.slug, "clean-csv");
    }

    #[test]
    fn confirm_on_empty_list_returns_none() {
        let m = BrowserModel::from_recipes(vec![]);
        assert!(m.confirm().is_none());
    }

    // --- Integration: loads from catalog ---

    #[test]
    fn new_loads_bundled_catalog_recipes() {
        let catalog = crate::catalog::RecipeCatalog::load(std::path::Path::new("/nonexistent"));
        let m = BrowserModel::new(&catalog.bundled());
        assert_eq!(m.recipes.len(), 20);
        assert_eq!(m.filtered.len(), 20);
    }

    #[test]
    fn selected_recipe_returns_current() {
        let m = BrowserModel::from_recipes(sample_recipes());
        let r = m.selected_recipe().unwrap();
        assert_eq!(r.slug, "compress-images");
    }

    #[test]
    fn selected_recipe_returns_none_on_empty() {
        let m = BrowserModel::from_recipes(vec![]);
        assert!(m.selected_recipe().is_none());
    }

    // --- Viewport scrolling ---

    fn big_browser() -> BrowserModel {
        let recipes: Vec<RecipeSummary> = (0..30)
            .map(|i| RecipeSummary {
                slug: format!("recipe-{i:02}"),
                name: format!("Recipe {i}"),
                description: format!("Desc {i}"),
                category: "test".into(),
                is_bundled: true,
                definition_json: "{}".into(),
            })
            .collect();
        let mut m = BrowserModel::from_recipes(recipes);
        m.viewport_height = 10;
        m
    }

    #[test]
    fn cursor_down_scrolls_viewport() {
        let mut m = big_browser();
        for _ in 0..10 {
            m = update(m, BrowserMessage::CursorDown);
        }
        assert_eq!(m.cursor, 10);
        assert!(m.viewport_offset > 0, "viewport should have scrolled");
    }

    #[test]
    fn cursor_up_scrolls_viewport_back() {
        let mut m = big_browser();
        m.cursor = 15;
        m.viewport_offset = 10;
        m = update(m, BrowserMessage::CursorUp);
        assert_eq!(m.cursor, 14);
        assert_eq!(m.viewport_offset, 10);
    }

    #[test]
    fn cursor_up_wrap_scrolls_to_bottom() {
        let mut m = big_browser();
        // cursor at 0, up wraps to last item
        m = update(m, BrowserMessage::CursorUp);
        assert_eq!(m.cursor, 29);
        assert_eq!(m.viewport_offset, 20);
    }

    #[test]
    fn resize_updates_viewport_height() {
        let m = big_browser();
        assert_eq!(m.viewport_height, 10);
        let m = update(m, BrowserMessage::Resize { height: 15 });
        assert_eq!(m.viewport_height, 15);
    }

    #[test]
    fn resize_keeps_cursor_visible() {
        let mut m = big_browser();
        m.cursor = 25;
        m.viewport_offset = 20;
        let m = update(m, BrowserMessage::Resize { height: 5 });
        assert_eq!(m.viewport_height, 5);
        assert!(m.cursor >= m.viewport_offset);
        assert!(m.cursor < m.viewport_offset + m.viewport_height);
    }
}
