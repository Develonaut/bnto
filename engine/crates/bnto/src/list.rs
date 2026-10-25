// Recipe listing — formats recipes for `bnto list` output.

use std::collections::BTreeMap;

use bnto_core::HostPlatform;
use bnto_core::os_support::recipe_os;

use crate::catalog::CatalogEntry;
use crate::host_support::os_badge;

/// A group of recipes sharing a category, ready for display.
#[derive(Debug)]
pub struct RecipeGroup {
    pub category: String,
    pub recipes: Vec<RecipeEntry>,
}

/// A single recipe entry for display.
#[derive(Debug)]
#[allow(dead_code)] // name used in tests via format_recipe_list
pub struct RecipeEntry {
    pub slug: String,
    pub name: String,
    pub description: String,
    /// The operating systems it has steps for.
    pub os: Vec<HostPlatform>,
}

/// Print grouped recipes to stdout with colored formatting.
pub fn print_recipe_list(groups: &[RecipeGroup]) {
    use colored::Colorize;
    println!("{}\n", "Available recipes:".bold());
    for (i, group) in groups.iter().enumerate() {
        if i > 0 {
            println!();
        }
        println!("  {}:", group.category.yellow());
        for entry in &group.recipes {
            let badge = os_badge(&entry.os).map(|b| format!("{} ", b.yellow()));
            println!(
                "    {:<30} {}{}",
                entry.slug.cyan(),
                badge.unwrap_or_default(),
                entry.description.dimmed()
            );
        }
    }
    println!("\n{}", "Run with: bnto run <recipe> <files...>".dimmed());
}

/// Group recipes by category, sorted alphabetically within each group.
/// Categories are sorted alphabetically. Returns groups ready for display.
pub fn group_recipes(entries: &[CatalogEntry]) -> Vec<RecipeGroup> {
    let mut by_category: BTreeMap<String, Vec<RecipeEntry>> = BTreeMap::new();

    for entry in entries {
        by_category
            .entry(entry.category.clone())
            .or_default()
            .push(RecipeEntry {
                slug: entry.slug.clone(),
                name: entry.name.clone(),
                description: entry.description.clone(),
                os: entry_os(entry),
            });
    }

    by_category
        .into_iter()
        .map(|(category, mut recipes)| {
            recipes.sort_by(|a, b| a.slug.cmp(&b.slug));
            RecipeGroup { category, recipes }
        })
        .collect()
}

/// The OSes a catalog entry runs on; everywhere when its definition doesn't
/// parse (`bnto run` reports that itself).
fn entry_os(entry: &CatalogEntry) -> Vec<HostPlatform> {
    serde_json::from_str(&entry.definition_json)
        .map(|def| recipe_os(&def))
        .unwrap_or_else(|_| HostPlatform::ALL.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{RecipeCatalog, RecipeSource};
    use std::fmt::Write;

    /// Format grouped recipes as plain text (no ANSI colors) for test assertions.
    fn format_recipe_list(groups: &[RecipeGroup]) -> String {
        let mut out = String::new();
        for (i, group) in groups.iter().enumerate() {
            if i > 0 {
                writeln!(out).ok();
            }
            writeln!(out, "  {}:", group.category).ok();
            for entry in &group.recipes {
                let badge = os_badge(&entry.os).map(|b| b + " ");
                let description = &entry.description;
                let line = format!("{}{description}", badge.unwrap_or_default());
                writeln!(out, "    {:<30} {line}", entry.slug).ok();
            }
        }
        out
    }

    fn make_entry(slug: &str, name: &str, desc: &str, category: &str) -> CatalogEntry {
        CatalogEntry {
            slug: slug.to_string(),
            name: name.to_string(),
            description: desc.to_string(),
            category: category.to_string(),
            tags: vec![],
            definition_json: "{}".to_string(),
            source: RecipeSource::Bundled,
        }
    }

    // --- group_recipes ---

    #[test]
    fn test_group_recipes_groups_by_category() {
        let entries = vec![
            make_entry(
                "compress-images",
                "Compress Images",
                "Compress image files",
                "image",
            ),
            make_entry("clean-csv", "Clean CSV", "Clean CSV files", "spreadsheet"),
            make_entry(
                "resize-images",
                "Resize Images",
                "Resize image files",
                "image",
            ),
        ];
        let groups = group_recipes(&entries);

        assert_eq!(groups.len(), 2, "Should have 2 categories");
        assert_eq!(groups[0].category, "image");
        assert_eq!(groups[1].category, "spreadsheet");
    }

    #[test]
    fn test_group_recipes_sorts_within_category() {
        let entries = vec![
            make_entry("resize-images", "Resize Images", "Resize", "image"),
            make_entry("compress-images", "Compress Images", "Compress", "image"),
        ];
        let groups = group_recipes(&entries);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].recipes[0].slug, "compress-images");
        assert_eq!(groups[0].recipes[1].slug, "resize-images");
    }

    #[test]
    fn test_group_recipes_categories_sorted_alphabetically() {
        let entries = vec![
            make_entry("dl-video", "Download Video", "Download", "video"),
            make_entry("clean-csv", "Clean CSV", "Clean", "spreadsheet"),
            make_entry("rename-files", "Rename Files", "Rename", "file"),
            make_entry("compress-images", "Compress Images", "Compress", "image"),
        ];
        let groups = group_recipes(&entries);
        let cats: Vec<&str> = groups.iter().map(|g| g.category.as_str()).collect();
        assert_eq!(cats, ["file", "image", "spreadsheet", "video"]);
    }

    #[test]
    fn test_group_recipes_empty_input() {
        let groups = group_recipes(&[]);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_group_recipes_preserves_all_entries() {
        let entries = vec![
            make_entry("a", "A", "desc a", "cat1"),
            make_entry("b", "B", "desc b", "cat1"),
            make_entry("c", "C", "desc c", "cat2"),
        ];
        let groups = group_recipes(&entries);
        let total: usize = groups.iter().map(|g| g.recipes.len()).sum();
        assert_eq!(total, 3);
    }

    // --- format_recipe_list ---

    #[test]
    fn test_format_contains_category_headers() {
        let groups = vec![RecipeGroup {
            category: "image".to_string(),
            recipes: vec![RecipeEntry {
                slug: "compress-images".to_string(),
                name: "Compress Images".to_string(),
                description: "Compress image files".to_string(),
                os: HostPlatform::ALL.to_vec(),
            }],
        }];
        let output = format_recipe_list(&groups);
        assert!(output.contains("image"), "Should contain category header");
    }

    #[test]
    fn test_format_contains_recipe_slugs() {
        let groups = vec![RecipeGroup {
            category: "image".to_string(),
            recipes: vec![RecipeEntry {
                slug: "compress-images".to_string(),
                name: "Compress Images".to_string(),
                description: "Compress image files".to_string(),
                os: HostPlatform::ALL.to_vec(),
            }],
        }];
        let output = format_recipe_list(&groups);
        assert!(
            output.contains("compress-images"),
            "Should contain recipe slug"
        );
    }

    #[test]
    fn test_format_contains_descriptions() {
        let groups = vec![RecipeGroup {
            category: "image".to_string(),
            recipes: vec![RecipeEntry {
                slug: "compress-images".to_string(),
                name: "Compress Images".to_string(),
                description: "Compress image files".to_string(),
                os: HostPlatform::ALL.to_vec(),
            }],
        }];
        let output = format_recipe_list(&groups);
        assert!(
            output.contains("Compress image files"),
            "Should contain description"
        );
    }

    #[test]
    fn test_format_marks_a_recipe_that_runs_on_some_oses() {
        let library = tempfile::tempdir().unwrap();
        let mac_only = r#"{ "id": "mac-only", "type": "group", "name": "Mac Only",
            "metadata": { "description": "Only on a Mac.", "category": "system" },
            "nodes": [{ "id": "g", "type": "group",
              "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": "macos" },
              "nodes": [{ "id": "s", "type": "shell-run", "parameters": { "command": "true" } }] }] }"#;
        std::fs::write(library.path().join("mac-only.bnto.json"), mac_only).unwrap();
        let catalog = RecipeCatalog::load(library.path());
        let output = format_recipe_list(&group_recipes(catalog.all()));
        let line = |slug: &str| {
            output
                .lines()
                .find(|l| l.trim_start().starts_with(&format!("{slug} ")))
                .unwrap_or_else(|| panic!("no {slug} line:\n{output}"))
                .to_string()
        };
        assert!(
            line("mac-only").contains(" [macOS] Only on a Mac."),
            "{output}"
        );
        assert!(!line("keep-awake").contains('['));
        assert!(!line("compress-images").contains('['));
    }

    #[test]
    fn test_format_empty_groups() {
        let output = format_recipe_list(&[]);
        assert!(output.is_empty() || output.trim().is_empty());
    }

    // --- Integration: group then format all catalog recipes ---

    #[test]
    fn test_all_catalog_recipes_appear_in_output() {
        let catalog = RecipeCatalog::bundled_only();
        let groups = group_recipes(catalog.all());
        let total: usize = groups.iter().map(|g| g.recipes.len()).sum();
        assert_eq!(
            total,
            catalog.all().len(),
            "All recipes must appear in groups"
        );

        let output = format_recipe_list(&groups);
        assert!(!output.is_empty(), "Formatted output must not be empty");
    }
}
