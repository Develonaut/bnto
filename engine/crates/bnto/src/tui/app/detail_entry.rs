// Which copy of a recipe a detail screen shows: the Recipes pane opens the
// shipped recipe, Library opens the user's, even when the two share a slug.

use super::DetailOrigin;
use crate::catalog::{CatalogEntry, RecipeCatalog};

impl DetailOrigin {
    /// The catalog entry a detail opened from here should show.
    pub(crate) fn entry<'a>(
        self,
        catalog: &'a RecipeCatalog,
        slug: &str,
    ) -> Option<&'a CatalogEntry> {
        match self {
            DetailOrigin::Home | DetailOrigin::Browser => catalog.resolve_bundled(slug),
            DetailOrigin::Library => catalog.resolve(slug),
        }
    }
}
