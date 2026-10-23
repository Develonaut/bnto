// What a run tells telemetry about its recipe. Only a bundled recipe is
// named: a library recipe's slug, a recipe file's path and a custom
// recipe's field names are the user's own words, so they stay on the machine.

use crate::catalog::RecipeCatalog;

/// The slug sent for any recipe bnto doesn't ship.
pub const CUSTOM_SLUG: &str = "custom";

/// The recipe facts a `cli_recipe_run` event carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeLabel {
    /// The bundled recipe's slug, or [`CUSTOM_SLUG`].
    pub slug: String,
    /// The `-p` override names, for a bundled recipe only.
    pub param_names: Vec<String>,
    /// How many `-p` overrides the run had, for any recipe.
    pub param_count: usize,
}

/// Label the recipe `recipe_arg` (a slug or a path, as typed) runs. A path
/// is always custom, even when it points at a copy of a bundled recipe.
pub fn recipe_label(
    catalog: &RecipeCatalog,
    recipe_arg: &str,
    param_names: Vec<String>,
) -> RecipeLabel {
    let param_count = param_names.len();
    if catalog.is_bundled(recipe_arg) {
        return RecipeLabel {
            slug: recipe_arg.to_string(),
            param_names,
            param_count,
        };
    }
    RecipeLabel {
        slug: CUSTOM_SLUG.to_string(),
        param_names: Vec::new(),
        param_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|n| n.to_string()).collect()
    }

    /// A catalog whose library holds `slugs`, each a one-node recipe.
    fn catalog_with_library(slugs: &[&str]) -> (tempfile::TempDir, RecipeCatalog) {
        let dir = tempfile::tempdir().unwrap();
        for slug in slugs {
            let json = format!(r#"{{"id":"{slug}","name":"{slug}","nodes":[]}}"#);
            std::fs::write(dir.path().join(format!("{slug}.bnto.json")), json).unwrap();
        }
        let catalog = RecipeCatalog::load(dir.path());
        (dir, catalog)
    }

    #[test]
    fn a_bundled_slug_and_its_param_names_are_kept() {
        let (_dir, catalog) = catalog_with_library(&[]);
        let label = recipe_label(&catalog, "compress-images", names(&["quality"]));
        assert_eq!(label.slug, "compress-images");
        assert_eq!(label.param_names, names(&["quality"]));
        assert_eq!(label.param_count, 1);
    }

    #[test]
    fn a_library_slug_is_custom() {
        let (_dir, catalog) = catalog_with_library(&["acme-invoices"]);
        let label = recipe_label(&catalog, "acme-invoices", vec![]);
        assert_eq!(label.slug, CUSTOM_SLUG);
    }

    #[test]
    fn a_library_recipe_overriding_a_bundled_slug_is_custom() {
        let (_dir, catalog) = catalog_with_library(&["compress-images"]);
        let label = recipe_label(&catalog, "compress-images", vec![]);
        assert_eq!(label.slug, CUSTOM_SLUG);
    }

    #[test]
    fn a_path_is_custom_even_to_a_bundled_recipe() {
        let (_dir, catalog) = catalog_with_library(&[]);
        for path in [
            "./compress-images.bnto.json",
            "/Users/someone/clients/acme.bnto.json",
            r"C:\Users\someone\acme.bnto.json",
        ] {
            let label = recipe_label(&catalog, path, vec![]);
            assert_eq!(label.slug, CUSTOM_SLUG, "{path}");
        }
    }

    #[test]
    fn a_custom_recipe_keeps_the_param_count_but_not_the_names() {
        let (_dir, catalog) = catalog_with_library(&["acme-invoices"]);
        let label = recipe_label(&catalog, "acme-invoices", names(&["clientName", "vatId"]));
        assert!(label.param_names.is_empty());
        assert_eq!(label.param_count, 2);
    }
}
