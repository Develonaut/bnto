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
    pub tags: Vec<String>,
    pub definition_json: &'static str,
}

/// All built-in recipe definition strings, embedded at compile time.
const RECIPE_DEFINITIONS: &[&str] = &[
    include_str!("../recipes/compress-images.bnto.json"),
    include_str!("../recipes/resize-images.bnto.json"),
    include_str!("../recipes/convert-image-format.bnto.json"),
    include_str!("../recipes/rename-files.bnto.json"),
    include_str!("../recipes/clean-csv.bnto.json"),
    include_str!("../recipes/rename-csv-columns.bnto.json"),
    include_str!("../recipes/csv-to-json.bnto.json"),
    include_str!("../recipes/merge-csv.bnto.json"),
    include_str!("../recipes/optimize-images-for-web.bnto.json"),
    include_str!("../recipes/generate-thumbnails.bnto.json"),
    include_str!("../recipes/compress-and-rename.bnto.json"),
    include_str!("../recipes/standardize-csv.bnto.json"),
    include_str!("../recipes/strip-exif.bnto.json"),
    include_str!("../recipes/watermark-images.bnto.json"),
    include_str!("../recipes/download-video.bnto.json"),
    include_str!("../recipes/svg-to-png.bnto.json"),
    include_str!("../recipes/svg-to-jpeg.bnto.json"),
    include_str!("../recipes/optimize-svg.bnto.json"),
    include_str!("../recipes/number-files.bnto.json"),
    include_str!("../recipes/sanitize-filenames.bnto.json"),
    include_str!("../recipes/flatten-folders.bnto.json"),
    include_str!("../recipes/keep-awake.bnto.json"),
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
                tags: val["metadata"]["tags"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default(),
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
    use bnto_core::{HostPlatform, PipelineDefinition};

    #[test]
    fn test_builtin_recipes_count() {
        assert_eq!(builtin_recipes().len(), 22);
    }

    #[test]
    fn every_bundled_dependency_installs_on_every_os_that_needs_it() {
        // `bnto doctor` checks a dependency on every host that needs it, so
        // one with no install route there fails doctor for good.
        let registry = crate::create_registry();
        for recipe in builtin_recipes() {
            let def: PipelineDefinition = serde_json::from_str(recipe.definition_json).unwrap();
            for scoped in crate::deps::scoped_pipeline_dependencies(&def, &registry) {
                for os in scoped.os {
                    assert!(
                        scoped.dependency.install_hints.for_platform(os).is_some(),
                        "{} needs {}, which has no install hint for {os:?}",
                        recipe.slug,
                        scoped.dependency.binary
                    );
                }
            }
        }
    }

    #[test]
    fn keep_awake_needs_each_oss_tools_only_there() {
        let recipe = builtin_recipe_by_slug("keep-awake").unwrap();
        let def: PipelineDefinition = serde_json::from_str(recipe.definition_json).unwrap();
        let scoped = crate::deps::scoped_pipeline_dependencies(&def, &crate::create_registry());
        let needs: Vec<(&str, &[HostPlatform])> = scoped
            .iter()
            .map(|s| (s.dependency.binary.as_str(), s.os.as_slice()))
            .collect();
        let mac: &[HostPlatform] = &[HostPlatform::MacOs];
        let linux: &[HostPlatform] = &[HostPlatform::Linux];
        assert_eq!(
            needs,
            [
                ("caffeinate", mac),
                ("osascript", mac),
                ("systemd-inhibit", linux)
            ]
        );
    }

    #[test]
    fn keep_awakes_fields_live_once_on_the_session_group_both_oss_read() {
        let recipe = builtin_recipe_by_slug("keep-awake").unwrap();
        let def: serde_json::Value = serde_json::from_str(recipe.definition_json).unwrap();
        let session = def["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|n| n["id"] == "session")
            .expect("a session group");
        assert_eq!(session["type"], "group");
        let fields: Vec<&str> = session["fields"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(fields, ["action", "display", "duration", "jiggle"]);
        let steps: Vec<&serde_json::Value> = session["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|os| &os["nodes"][0])
            .collect();
        assert_eq!(steps.len(), 2, "a macOS and a Linux branch");
        for step in &steps {
            assert!(
                step.get("fields").is_none(),
                "{} declares fields",
                step["id"]
            );
        }
        assert_eq!(
            steps[0]["parameters"]["args"],
            steps[1]["parameters"]["args"]
        );
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

    /// Every recipe's root edges are a valid graph whose execution order is
    /// today's processing-node order, so reading edges changes no run.
    #[test]
    fn test_builtin_recipes_root_graph_keeps_node_order() {
        for recipe in builtin_recipes() {
            let def: PipelineDefinition = serde_json::from_str(recipe.definition_json).unwrap();
            let flow = bnto_core::level_flow(&def.nodes, &def.edges)
                .unwrap_or_else(|e| panic!("Recipe '{}' edges refused: {e}", recipe.slug));
            let bnto_core::Flow::Graph(graph) = flow else {
                panic!("Recipe '{}' has no root edges", recipe.slug);
            };
            let order: Vec<&str> = graph.order.iter().map(|n| n.id.as_str()).collect();
            let today: Vec<&str> = def
                .nodes
                .iter()
                .filter(|n| !bnto_core::pipeline::is_io_node(&n.node_type))
                .map(|n| n.id.as_str())
                .collect();
            assert_eq!(order, today, "Recipe '{}'", recipe.slug);
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
    fn test_builtin_recipes_all_have_tags() {
        for recipe in builtin_recipes() {
            assert!(
                !recipe.tags.is_empty(),
                "Recipe '{}' has no tags",
                recipe.slug,
            );
        }
    }

    #[test]
    fn test_builtin_recipes_expected_categories() {
        let recipes = builtin_recipes();
        let mut categories: Vec<&str> = recipes.iter().map(|r| r.category.as_str()).collect();
        categories.sort();
        categories.dedup();
        assert_eq!(
            categories,
            ["file", "image", "spreadsheet", "system", "vector", "video"]
        );
    }
}
