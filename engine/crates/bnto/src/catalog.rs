// Unified recipe catalog — loads all recipes from all sources into a single list.
//
// Consumers resolve recipes through the catalog. No special treatment for
// bundled vs library recipes — the source field is only used for trust
// classification.

use std::path::{Path, PathBuf};

/// Where a recipe was loaded from — used ONLY for trust classification,
/// not for execution or display priority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeSource {
    /// Shipped with the binary. Trusted without consent prompt.
    Bundled,
    /// Loaded from the user's library directory. May need consent for shell commands.
    Library(PathBuf),
}

/// A recipe entry in the catalog. Source-agnostic — consumers don't care
/// where it came from except for trust classification.
#[derive(Debug, Clone)]
pub struct CatalogEntry {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub definition_json: String,
    pub source: RecipeSource,
}

/// Single flat list of all recipes from all sources.
pub struct RecipeCatalog {
    entries: Vec<CatalogEntry>,
}

impl RecipeCatalog {
    /// Load bundled + library recipes into a single list.
    ///
    /// If the same slug exists in both, the library version wins
    /// (user's copy overrides the bundled recipe).
    pub fn load(library_dir: &Path) -> Self {
        let mut entries = load_bundled();
        let library_entries = load_library(library_dir);

        // Library overrides bundled: remove bundled entries whose slugs
        // collide with library entries, then append library entries.
        if !library_entries.is_empty() {
            let library_slugs: std::collections::HashSet<&str> =
                library_entries.iter().map(|e| e.slug.as_str()).collect();
            entries.retain(|e| !library_slugs.contains(e.slug.as_str()));
            entries.extend(library_entries);
        }

        entries.sort_by(|a, b| a.slug.cmp(&b.slug));
        Self { entries }
    }

    /// All catalog entries.
    pub fn all(&self) -> &[CatalogEntry] {
        &self.entries
    }

    /// Find a recipe by slug.
    pub fn resolve(&self, slug: &str) -> Option<&CatalogEntry> {
        self.entries.iter().find(|e| e.slug == slug)
    }

    /// Whether a slug is a bundled recipe (for trust classification only).
    pub fn is_bundled(&self, slug: &str) -> bool {
        self.entries
            .iter()
            .any(|e| e.slug == slug && e.source == RecipeSource::Bundled)
    }
}

/// Load all engine-embedded recipes as catalog entries.
fn load_bundled() -> Vec<CatalogEntry> {
    bnto_engine::recipes::builtin_recipes()
        .into_iter()
        .map(|r| CatalogEntry {
            slug: r.slug,
            name: r.name,
            description: r.description,
            category: r.category,
            tags: r.tags,
            definition_json: r.definition_json.to_string(),
            source: RecipeSource::Bundled,
        })
        .collect()
}

/// Load `.bnto.json` files from the user's library directory.
fn load_library(dir: &Path) -> Vec<CatalogEntry> {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return Vec::new();
    };

    rd.filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".bnto.json"))
        })
        .filter_map(|e| {
            let path = e.path();
            let slug = path
                .file_name()?
                .to_str()?
                .trim_end_matches(".bnto.json")
                .to_string();
            let content = std::fs::read_to_string(&path).ok()?;
            let val: serde_json::Value = serde_json::from_str(&content).ok()?;

            let name = val["name"].as_str().unwrap_or(&slug).to_string();
            let description = val["metadata"]["description"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            let category = val["metadata"]["category"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            let tags = val["metadata"]["tags"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            Some(CatalogEntry {
                slug,
                name,
                description,
                category,
                tags,
                definition_json: content,
                source: RecipeSource::Library(path),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_recipe(dir: &Path, slug: &str, name: &str, category: &str) {
        let json = format!(
            r#"{{"id":"{slug}","name":"{name}","metadata":{{"description":"Test recipe","category":"{category}","tags":["test"]}},"nodes":[]}}"#,
        );
        std::fs::write(dir.join(format!("{slug}.bnto.json")), json).unwrap();
    }

    #[test]
    fn catalog_loads_bundled_recipes() {
        let tmp = tempfile::tempdir().unwrap();
        let catalog = RecipeCatalog::load(tmp.path());
        assert_eq!(catalog.all().len(), 20);
    }

    #[test]
    fn catalog_loads_library_from_disk() {
        let tmp = tempfile::tempdir().unwrap();
        write_recipe(tmp.path(), "my-custom", "My Custom", "custom");
        let catalog = RecipeCatalog::load(tmp.path());

        // 20 bundled + 1 library
        assert_eq!(catalog.all().len(), 21);
        let custom = catalog.resolve("my-custom").expect("should find custom");
        assert_eq!(custom.name, "My Custom");
        assert!(matches!(custom.source, RecipeSource::Library(_)));
    }

    #[test]
    fn catalog_library_overrides_bundled() {
        let tmp = tempfile::tempdir().unwrap();
        // Create a library recipe with the same slug as a bundled one.
        write_recipe(tmp.path(), "compress-images", "My Compress", "image");
        let catalog = RecipeCatalog::load(tmp.path());

        // Total should still be 20 (replaced, not added).
        assert_eq!(catalog.all().len(), 20);
        let entry = catalog.resolve("compress-images").unwrap();
        assert_eq!(entry.name, "My Compress");
        assert!(matches!(entry.source, RecipeSource::Library(_)));
    }

    #[test]
    fn catalog_resolve_by_slug() {
        let tmp = tempfile::tempdir().unwrap();
        let catalog = RecipeCatalog::load(tmp.path());
        let entry = catalog.resolve("compress-images");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().name, "Compress Images");
    }

    #[test]
    fn catalog_resolve_unknown_returns_none() {
        let tmp = tempfile::tempdir().unwrap();
        let catalog = RecipeCatalog::load(tmp.path());
        assert!(catalog.resolve("nonexistent-recipe").is_none());
    }

    #[test]
    fn catalog_is_bundled_correct() {
        let tmp = tempfile::tempdir().unwrap();
        write_recipe(tmp.path(), "my-custom", "Custom", "custom");
        let catalog = RecipeCatalog::load(tmp.path());

        assert!(catalog.is_bundled("compress-images"));
        assert!(!catalog.is_bundled("my-custom"));
        assert!(!catalog.is_bundled("nonexistent"));
    }

    #[test]
    fn catalog_empty_library_dir_still_has_bundled() {
        let tmp = tempfile::tempdir().unwrap();
        let catalog = RecipeCatalog::load(tmp.path());
        assert_eq!(catalog.all().len(), 20);
    }

    #[test]
    fn catalog_missing_library_dir_still_has_bundled() {
        let catalog = RecipeCatalog::load(Path::new("/nonexistent/path"));
        assert_eq!(catalog.all().len(), 20);
    }

    #[test]
    fn catalog_entries_sorted_by_slug() {
        let tmp = tempfile::tempdir().unwrap();
        let catalog = RecipeCatalog::load(tmp.path());
        let slugs: Vec<&str> = catalog.all().iter().map(|e| e.slug.as_str()).collect();
        let mut sorted = slugs.clone();
        sorted.sort();
        assert_eq!(slugs, sorted);
    }

    #[test]
    fn catalog_library_skips_invalid_json() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("bad.bnto.json"), "not json").unwrap();
        write_recipe(tmp.path(), "good", "Good Recipe", "test");
        let catalog = RecipeCatalog::load(tmp.path());

        // 20 bundled + 1 valid library
        assert_eq!(catalog.all().len(), 21);
        assert!(catalog.resolve("good").is_some());
        assert!(catalog.resolve("bad").is_none());
    }
}
