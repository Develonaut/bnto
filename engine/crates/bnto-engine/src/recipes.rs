//! Built-in recipe catalog — engine-owned recipe definitions.
//!
//! Each recipe is a `.bnto.json` embedded at compile time via `include_str!()`.
//! The engine is the source of truth for recipes. TypeScript catalogs
//! are generated from the engine's catalog snapshot.

/// A built-in recipe with metadata extracted from the definition JSON.
pub struct BuiltinRecipe {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub definition_json: &'static str,
}

/// All built-in recipe definition strings, embedded at compile time.
const RECIPE_DEFINITIONS: &[&str] = &[
    include_str!("../../../recipes/compress-images.bnto.json"),
    include_str!("../../../recipes/resize-images.bnto.json"),
    include_str!("../../../recipes/convert-image-format.bnto.json"),
    include_str!("../../../recipes/rename-files.bnto.json"),
    include_str!("../../../recipes/clean-csv.bnto.json"),
    include_str!("../../../recipes/rename-csv-columns.bnto.json"),
    include_str!("../../../recipes/csv-to-json.bnto.json"),
    include_str!("../../../recipes/merge-csv.bnto.json"),
    include_str!("../../../recipes/optimize-images-for-web.bnto.json"),
    include_str!("../../../recipes/generate-thumbnails.bnto.json"),
    include_str!("../../../recipes/compress-and-rename.bnto.json"),
    include_str!("../../../recipes/standardize-csv.bnto.json"),
    include_str!("../../../recipes/strip-exif.bnto.json"),
    include_str!("../../../recipes/watermark-images.bnto.json"),
    include_str!("../../../recipes/download-video.bnto.json"),
];

/// Returns all built-in recipes, embedded at compile time.
pub fn builtin_recipes() -> Vec<BuiltinRecipe> {
    RECIPE_DEFINITIONS
        .iter()
        .map(|json| {
            let val: serde_json::Value =
                serde_json::from_str(json).expect("built-in recipe JSON must be valid");
            BuiltinRecipe {
                slug: val["id"].as_str().unwrap_or_default().to_string(),
                name: val["name"].as_str().unwrap_or_default().to_string(),
                description: val["metadata"]["description"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                category: val["metadata"]["category"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                definition_json: json,
            }
        })
        .collect()
}

/// Find a built-in recipe by slug.
pub fn builtin_recipe_by_slug(slug: &str) -> Option<BuiltinRecipe> {
    builtin_recipes().into_iter().find(|r| r.slug == slug)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::PipelineDefinition;

    #[test]
    fn test_builtin_recipes_count() {
        assert_eq!(builtin_recipes().len(), 15);
    }

    #[test]
    fn test_builtin_recipes_all_have_slugs() {
        for recipe in builtin_recipes() {
            assert!(!recipe.slug.is_empty(), "Recipe has empty slug");
        }
    }

    #[test]
    fn test_builtin_recipes_all_have_categories() {
        for recipe in builtin_recipes() {
            assert!(
                !recipe.category.is_empty(),
                "Recipe '{}' has empty category",
                recipe.slug,
            );
        }
    }

    #[test]
    fn test_builtin_recipes_all_parse_as_pipelines() {
        for recipe in builtin_recipes() {
            let _def: PipelineDefinition = serde_json::from_str(recipe.definition_json)
                .unwrap_or_else(|e| panic!("Recipe '{}' failed to parse: {e}", recipe.slug));
        }
    }

    #[test]
    fn test_builtin_recipe_by_slug() {
        let recipe =
            builtin_recipe_by_slug("compress-images").expect("compress-images should exist");
        assert_eq!(recipe.name, "Compress Images");
        assert_eq!(recipe.category, "image");
    }

    #[test]
    fn test_builtin_recipe_by_slug_not_found() {
        assert!(builtin_recipe_by_slug("nonexistent").is_none());
    }

    #[test]
    fn test_builtin_recipes_unique_slugs() {
        let recipes = builtin_recipes();
        let mut slugs: Vec<&str> = recipes.iter().map(|r| r.slug.as_str()).collect();
        slugs.sort();
        slugs.dedup();
        assert_eq!(slugs.len(), recipes.len(), "Duplicate slugs found");
    }

    #[test]
    fn test_builtin_recipes_expected_categories() {
        let recipes = builtin_recipes();
        let mut categories: Vec<&str> = recipes.iter().map(|r| r.category.as_str()).collect();
        categories.sort();
        categories.dedup();
        assert_eq!(categories, ["file", "image", "spreadsheet", "video"]);
    }
}
