// Recipe info — formats detailed recipe information for `bnto info <recipe>`.

use bnto_core::{Dependency, InputMode, PipelineDefinition, resolve_input_mode};
use bnto_engine::deps::collect_pipeline_dependencies;
use bnto_engine::recipes::builtin_recipe_by_slug;

/// Structured recipe details ready for display.
#[derive(Debug)]
#[allow(dead_code)] // slug used in tests via format_recipe_info
pub struct RecipeInfo {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub node_types: Vec<String>,
    pub dependencies: Vec<Dependency>,
    pub input_mode: String,
}

/// Print recipe info to stdout with colored formatting.
pub fn print_recipe_info(slug: &str, info: &RecipeInfo) {
    use colored::Colorize;
    println!("\n{}", info.name.bold());
    println!("{}\n", info.description);
    println!("  {}   {}", "Category:".dimmed(), info.category);
    println!("  {}      {}", "Input:".dimmed(), info.input_mode);
    println!(
        "  {}      {}",
        "Nodes:".dimmed(),
        info.node_types.join(", ").cyan()
    );
    print_dependencies(&info.dependencies);
    println!(
        "\n{}",
        format!("Run with: bnto run {slug} <files...>").dimmed()
    );
}

/// Print dependency details.
fn print_dependencies(deps: &[Dependency]) {
    use colored::Colorize;
    if deps.is_empty() {
        return;
    }
    println!("\n  {}", "Dependencies:".dimmed());
    for dep in deps {
        let version = if dep.version.is_empty() {
            String::new()
        } else {
            format!(" ({})", dep.version)
        };
        println!("    {}{version}", dep.binary.yellow());
        if !dep.install_hint.is_empty() {
            println!("      Install: {}", dep.install_hint);
        }
    }
}

/// Extract structured info from a built-in recipe slug.
/// Returns `None` if the slug doesn't match any built-in recipe.
pub fn get_recipe_info(slug: &str) -> Option<RecipeInfo> {
    let recipe = builtin_recipe_by_slug(slug)?;
    let def: PipelineDefinition = serde_json::from_str(recipe.definition_json).ok()?;

    let node_types = extract_node_types(&def);
    let input_mode = match resolve_input_mode(&def) {
        InputMode::FileUpload => "file-upload",
        InputMode::Url => "url",
        InputMode::Text => "text",
    };

    let registry = bnto_engine::create_registry();
    let dependencies = collect_pipeline_dependencies(&def, &registry);

    Some(RecipeInfo {
        slug: recipe.slug,
        name: recipe.name,
        description: recipe.description,
        category: recipe.category,
        node_types,
        dependencies,
        input_mode: input_mode.to_string(),
    })
}

/// Walk the definition tree and collect unique processing node types.
/// Skips I/O markers (input, output) and container types (group, loop).
fn extract_node_types(def: &PipelineDefinition) -> Vec<String> {
    let skip = ["input", "output", "group", "loop"];
    let mut types = Vec::new();
    collect_types(&def.nodes, &skip, &mut types);
    types.sort();
    types.dedup();
    types
}

fn collect_types(nodes: &[bnto_core::PipelineNode], skip: &[&str], types: &mut Vec<String>) {
    for node in nodes {
        if !skip.contains(&node.node_type.as_str()) {
            types.push(node.node_type.clone());
        }
        if let Some(children) = &node.children {
            collect_types(children, skip, types);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    /// Format recipe info as plain text (no ANSI colors) for test assertions.
    fn format_recipe_info(info: &RecipeInfo) -> String {
        let mut out = String::new();
        writeln!(out, "  {}", info.name).ok();
        writeln!(out, "  {}", info.description).ok();
        writeln!(out).ok();
        writeln!(out, "  Category:   {}", info.category).ok();
        writeln!(out, "  Input:      {}", info.input_mode).ok();
        writeln!(out, "  Nodes:      {}", info.node_types.join(", ")).ok();
        if !info.dependencies.is_empty() {
            writeln!(out).ok();
            writeln!(out, "  Dependencies:").ok();
            for dep in &info.dependencies {
                write!(out, "    {} ", dep.binary).ok();
                if !dep.version.is_empty() {
                    write!(out, "({})", dep.version).ok();
                }
                writeln!(out).ok();
                if !dep.install_hint.is_empty() {
                    writeln!(out, "      Install: {}", dep.install_hint).ok();
                }
            }
        }
        out
    }

    // --- get_recipe_info ---

    #[test]
    fn test_get_info_known_recipe() {
        let info = get_recipe_info("compress-images");
        assert!(info.is_some(), "compress-images should exist");

        let info = info.unwrap();
        assert_eq!(info.slug, "compress-images");
        assert_eq!(info.name, "Compress Images");
        assert_eq!(info.category, "image");
        assert!(!info.description.is_empty());
    }

    #[test]
    fn test_get_info_unknown_recipe() {
        let info = get_recipe_info("nonexistent-recipe");
        assert!(info.is_none());
    }

    #[test]
    fn test_get_info_extracts_node_types() {
        let info = get_recipe_info("compress-images").unwrap();
        assert!(
            !info.node_types.is_empty(),
            "Should extract processing node types"
        );
        assert!(
            info.node_types.contains(&"image-compress".to_string()),
            "compress-images should use image-compress processor"
        );
    }

    #[test]
    fn test_get_info_extracts_input_mode() {
        // file-upload recipes
        let info = get_recipe_info("compress-images").unwrap();
        assert_eq!(info.input_mode, "file-upload");

        // url-mode recipe
        let info = get_recipe_info("download-video").unwrap();
        assert_eq!(info.input_mode, "url");
    }

    #[test]
    fn test_get_info_video_has_dependencies() {
        let info = get_recipe_info("download-video").unwrap();
        assert!(
            !info.dependencies.is_empty(),
            "download-video requires external dependencies"
        );
        let dep_names: Vec<&str> = info
            .dependencies
            .iter()
            .map(|d| d.binary.as_str())
            .collect();
        assert!(dep_names.contains(&"yt-dlp"));
    }

    #[test]
    fn test_get_info_image_recipe_no_dependencies() {
        let info = get_recipe_info("compress-images").unwrap();
        assert!(
            info.dependencies.is_empty(),
            "Browser-safe image recipes have no external dependencies"
        );
    }

    #[test]
    fn test_get_info_multi_node_recipe() {
        let info = get_recipe_info("optimize-images-for-web").unwrap();
        assert!(
            info.node_types.len() >= 2,
            "optimize-images-for-web is a multi-node pipeline"
        );
    }

    // --- format_recipe_info ---

    #[test]
    fn test_format_contains_name() {
        let info = RecipeInfo {
            slug: "compress-images".to_string(),
            name: "Compress Images".to_string(),
            description: "Compress image files".to_string(),
            category: "image".to_string(),
            node_types: vec!["image-compress".to_string()],
            dependencies: vec![],
            input_mode: "file-upload".to_string(),
        };
        let output = format_recipe_info(&info);
        assert!(output.contains("Compress Images"));
    }

    #[test]
    fn test_format_contains_category() {
        let info = RecipeInfo {
            slug: "compress-images".to_string(),
            name: "Compress Images".to_string(),
            description: "Compress image files".to_string(),
            category: "image".to_string(),
            node_types: vec!["image-compress".to_string()],
            dependencies: vec![],
            input_mode: "file-upload".to_string(),
        };
        let output = format_recipe_info(&info);
        assert!(output.contains("image"));
    }

    #[test]
    fn test_format_contains_node_types() {
        let info = RecipeInfo {
            slug: "compress-images".to_string(),
            name: "Compress Images".to_string(),
            description: "Compress image files".to_string(),
            category: "image".to_string(),
            node_types: vec!["image-compress".to_string()],
            dependencies: vec![],
            input_mode: "file-upload".to_string(),
        };
        let output = format_recipe_info(&info);
        assert!(output.contains("image-compress"));
    }

    #[test]
    fn test_format_shows_dependencies_when_present() {
        let info = RecipeInfo {
            slug: "download-video".to_string(),
            name: "Download Video".to_string(),
            description: "Download videos".to_string(),
            category: "video".to_string(),
            node_types: vec!["shell-command".to_string()],
            dependencies: vec![Dependency {
                binary: "yt-dlp".to_string(),
                version: String::new(),
                install_hint: "brew install yt-dlp".to_string(),
                homepage: "https://github.com/yt-dlp/yt-dlp".to_string(),
            }],
            input_mode: "url".to_string(),
        };
        let output = format_recipe_info(&info);
        assert!(output.contains("yt-dlp"));
    }

    #[test]
    fn test_format_shows_input_mode() {
        let info = RecipeInfo {
            slug: "download-video".to_string(),
            name: "Download Video".to_string(),
            description: "Download videos".to_string(),
            category: "video".to_string(),
            node_types: vec!["shell-command".to_string()],
            dependencies: vec![],
            input_mode: "url".to_string(),
        };
        let output = format_recipe_info(&info);
        assert!(output.contains("url"));
    }

    #[test]
    fn test_format_no_deps_section_when_empty() {
        let info = RecipeInfo {
            slug: "compress-images".to_string(),
            name: "Compress Images".to_string(),
            description: "Compress image files".to_string(),
            category: "image".to_string(),
            node_types: vec!["image-compress".to_string()],
            dependencies: vec![],
            input_mode: "file-upload".to_string(),
        };
        let output = format_recipe_info(&info);
        // Should not contain a "Dependencies" section with content when empty
        assert!(
            !output.contains("yt-dlp"),
            "Should not mention deps when none exist"
        );
    }
}
