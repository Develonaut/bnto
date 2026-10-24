// How a recipe argument on the command line becomes recipe JSON, the one
// rule `bnto run`, `bnto dry-run` and `bnto install` share.

use crate::catalog::RecipeCatalog;

/// A slug in the catalog, else a `.bnto.json` path read from disk. A slug
/// wins over a file of the same name.
pub fn read_recipe(arg: &str, catalog: &RecipeCatalog) -> std::io::Result<String> {
    match catalog.resolve(arg) {
        Some(entry) => Ok(entry.definition_json.clone()),
        None => std::fs::read_to_string(arg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog() -> (tempfile::TempDir, RecipeCatalog) {
        let library = tempfile::tempdir().unwrap();
        let catalog = RecipeCatalog::load(library.path());
        (library, catalog)
    }

    #[test]
    fn a_slug_reads_the_catalog_definition() {
        let (_library, catalog) = catalog();
        let json = read_recipe("compress-images", &catalog).unwrap();
        let expected = &catalog.resolve("compress-images").unwrap().definition_json;
        assert_eq!(&json, expected);
    }

    #[test]
    fn a_path_reads_the_file() {
        let (_library, catalog) = catalog();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mine.bnto.json");
        std::fs::write(&path, r#"{"name":"Mine"}"#).unwrap();
        let json = read_recipe(path.to_str().unwrap(), &catalog).unwrap();
        assert_eq!(json, r#"{"name":"Mine"}"#);
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
