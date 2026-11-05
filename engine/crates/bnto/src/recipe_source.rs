// How a recipe argument on the command line becomes recipe JSON, the one
// rule `bnto run`, `bnto dry-run` and `bnto install` share, and where that
// JSON came from, which decides its consent.

use crate::catalog::{RecipeCatalog, SourceKind};

/// A slug in the catalog, else a `.bnto.json` path read from disk. A slug
/// wins over a file of the same name. The kind is how it resolved: the
/// catalog entry's source, or `Path` for a file, each with the file read.
pub fn read_recipe(arg: &str, catalog: &RecipeCatalog) -> std::io::Result<(String, SourceKind)> {
    match catalog.resolve(arg) {
        Some(entry) => Ok((entry.definition_json.clone(), entry.source.kind())),
        None => Ok((std::fs::read_to_string(arg)?, SourceKind::Path(arg.into()))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{CatalogEntry, RecipeSource};
    use std::path::PathBuf;

    fn catalog() -> (tempfile::TempDir, RecipeCatalog) {
        let library = tempfile::tempdir().unwrap();
        let catalog = RecipeCatalog::load(library.path());
        (library, catalog)
    }

    fn entry(slug: &str, source: RecipeSource) -> CatalogEntry {
        CatalogEntry {
            slug: slug.into(),
            name: slug.into(),
            description: String::new(),
            category: String::new(),
            tags: Vec::new(),
            definition_json: format!(r#"{{"name":"{slug}"}}"#),
            source,
        }
    }

    #[test]
    fn a_slug_reads_the_catalog_definition() {
        let (_library, catalog) = catalog();
        let (json, kind) = read_recipe("compress-images", &catalog).unwrap();
        let expected = &catalog.resolve("compress-images").unwrap().definition_json;
        assert_eq!(&json, expected);
        assert_eq!(kind, SourceKind::Bundled);
    }

    #[test]
    fn a_library_slug_is_a_library_recipe() {
        let (library, _) = catalog();
        std::fs::write(library.path().join("mine.bnto.json"), r#"{"name":"Mine"}"#).unwrap();
        let catalog = RecipeCatalog::load(library.path());
        let (_, kind) = read_recipe("mine", &catalog).unwrap();
        let file = library.path().join("mine.bnto.json");
        assert_eq!(kind, SourceKind::Library(Some(file)));
    }

    #[test]
    fn a_project_slug_is_a_project_recipe_of_its_root_and_file() {
        let root = PathBuf::from("project");
        let file = root.join(".bnto/recipes/tidy.bnto.json");
        let source = RecipeSource::Project {
            root: root.clone(),
            file: file.clone(),
        };
        let catalog = RecipeCatalog::bundled_only().with_entry(entry("tidy", source));
        let (_, kind) = read_recipe("tidy", &catalog).unwrap();
        assert_eq!(
            kind,
            SourceKind::Project {
                root,
                file: Some(file)
            }
        );
    }

    #[test]
    fn a_path_reads_the_file() {
        let (_library, catalog) = catalog();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mine.bnto.json");
        std::fs::write(&path, r#"{"name":"Mine"}"#).unwrap();
        let (json, kind) = read_recipe(path.to_str().unwrap(), &catalog).unwrap();
        assert_eq!(json, r#"{"name":"Mine"}"#);
        assert_eq!(kind, SourceKind::Path(path));
    }

    #[test]
    fn a_path_spelled_like_a_slug_gets_the_kind_it_resolved_to() {
        // A path argument whose text is a built-in's slug runs the built-in
        // (a slug wins), so it gets the built-in's kind. A path holding a
        // built-in's slug, but naming a file, is a by-path recipe.
        let (_library, catalog) = catalog();
        let (_, kind) = read_recipe("compress-images", &catalog).unwrap();
        assert_eq!(kind, SourceKind::Bundled);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("compress-images");
        std::fs::write(&path, r#"{"name":"Mine"}"#).unwrap();
        let (json, kind) = read_recipe(path.to_str().unwrap(), &catalog).unwrap();
        assert_eq!(json, r#"{"name":"Mine"}"#);
        assert_eq!(kind, SourceKind::Path(path));
    }

    #[test]
    fn neither_a_slug_nor_a_file_is_not_found() {
        let (_library, catalog) = catalog();
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing.bnto.json");
        let err = read_recipe(missing.to_str().unwrap(), &catalog).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}
