// Recipe info — formats detailed recipe information for `bnto info <recipe>`.

use bnto_core::os_support::{describe_os, recipe_os};
use bnto_core::pipeline::{is_container_node, is_io_node};
use bnto_core::{HostPlatform, InputMode, PipelineDefinition, resolve_input_mode};
use bnto_engine::deps::{ScopedDependency, scoped_pipeline_dependencies};

use crate::catalog::RecipeCatalog;

/// Structured recipe details ready for display.
#[derive(Debug)]
#[allow(dead_code)] // slug used in tests via format_recipe_info
pub struct RecipeInfo {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub node_types: Vec<String>,
    /// Every dependency, tagged with the OSes that need it.
    pub dependencies: Vec<ScopedDependency>,
    pub input_mode: InputMode,
    /// The operating systems it has steps for.
    pub os: Vec<HostPlatform>,
}

/// Print recipe info to stdout with colored formatting.
pub fn print_recipe_info(slug: &str, info: &RecipeInfo) {
    use colored::Colorize;
    println!("\n{}", info.name.bold());
    println!("{}\n", info.description);
    println!("  {}   {}", "Category:".dimmed(), info.category);
    println!("  {}      {}", "Input:".dimmed(), info.input_mode);
    println!("  {}    {}", "Runs on:".dimmed(), describe_os(&info.os));
    println!(
        "  {}      {}",
        "Nodes:".dimmed(),
        info.node_types.join(", ").cyan()
    );
    print_dependencies(&info.dependencies);
    println!("\n{}", run_hint(slug, info.input_mode).dimmed());
}

/// Usage hint tailored to the recipe's input mode — a `none` recipe takes
/// no trailing argument at all, so suggesting `<files...>` is misleading.
fn run_hint(slug: &str, input_mode: InputMode) -> String {
    match input_mode {
        InputMode::None => format!("Run with: bnto run {slug}"),
        InputMode::Url => format!("Run with: bnto run {slug} <url>"),
        InputMode::Text => format!("Run with: bnto run {slug} <text>"),
        InputMode::FileUpload => format!("Run with: bnto run {slug} <files...>"),
    }
}

/// Print dependency details. A dependency only some OSes need says which.
fn print_dependencies(deps: &[ScopedDependency]) {
    use colored::Colorize;
    if deps.is_empty() {
        return;
    }
    println!("\n  {}", "Dependencies:".dimmed());
    for scoped in deps {
        println!(
            "    {}{}",
            scoped.dependency.binary.yellow(),
            dependency_notes(scoped)
        );
        println!(
            "      Install: {}",
            scoped.install_hint(HostPlatform::current())
        );
    }
}

/// " (>=6.0)", " (macOS only)", both, or nothing.
fn dependency_notes(scoped: &ScopedDependency) -> String {
    let version = &scoped.dependency.version;
    [
        (!version.is_empty()).then(|| version.clone()),
        scoped.os_note(),
    ]
    .into_iter()
    .flatten()
    .map(|note| format!(" ({note})"))
    .collect()
}

/// Extract structured info from a catalog recipe.
/// Returns `None` if the slug doesn't match any recipe.
pub fn get_recipe_info(slug: &str, catalog: &RecipeCatalog) -> Option<RecipeInfo> {
    let entry = catalog.resolve(slug)?;
    let def: PipelineDefinition = serde_json::from_str(&entry.definition_json).ok()?;

    let node_types = extract_node_types(&def);
    let input_mode = resolve_input_mode(&def);

    let registry = bnto_engine::create_registry();
    let dependencies = scoped_pipeline_dependencies(&def, &registry);

    Some(RecipeInfo {
        slug: entry.slug.clone(),
        name: entry.name.clone(),
        description: entry.description.clone(),
        category: entry.category.clone(),
        node_types,
        dependencies,
        input_mode,
        os: recipe_os(&def),
    })
}

/// Walk the definition tree and collect unique processing node types,
/// leaving out I/O markers and containers.
fn extract_node_types(def: &PipelineDefinition) -> Vec<String> {
    let mut types = Vec::new();
    collect_types(&def.nodes, &mut types);
    types.sort();
    types.dedup();
    types
}

fn collect_types(nodes: &[bnto_core::PipelineNode], types: &mut Vec<String>) {
    for node in nodes {
        if !is_io_node(&node.node_type) && !is_container_node(&node.node_type) {
            types.push(node.node_type.clone());
        }
        if let Some(children) = &node.children {
            collect_types(children, types);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;
    use std::path::Path;

    fn catalog() -> RecipeCatalog {
        RecipeCatalog::load(Path::new("/nonexistent"))
    }

    /// Format recipe info as plain text (no ANSI colors) for test assertions.
    fn format_recipe_info(info: &RecipeInfo) -> String {
        let mut out = String::new();
        writeln!(out, "  {}", info.name).ok();
        writeln!(out, "  {}", info.description).ok();
        writeln!(out).ok();
        writeln!(out, "  Category:   {}", info.category).ok();
        writeln!(out, "  Input:      {}", info.input_mode).ok();
        writeln!(out, "  Runs on:    {}", describe_os(&info.os)).ok();
        writeln!(out, "  Nodes:      {}", info.node_types.join(", ")).ok();
        if !info.dependencies.is_empty() {
            writeln!(out).ok();
            writeln!(out, "  Dependencies:").ok();
            for scoped in &info.dependencies {
                let notes = dependency_notes(scoped);
                writeln!(out, "    {}{notes}", scoped.dependency.binary).ok();
                let hint = scoped.install_hint(HostPlatform::current());
                writeln!(out, "      Install: {hint}").ok();
            }
        }
        out
    }

    // --- get_recipe_info ---

    #[test]
    fn test_get_info_known_recipe() {
        let c = catalog();
        let info = get_recipe_info("compress-images", &c);
        assert!(info.is_some(), "compress-images should exist");

        let info = info.unwrap();
        assert_eq!(info.slug, "compress-images");
        assert_eq!(info.name, "Compress Images");
        assert_eq!(info.category, "image");
        assert!(!info.description.is_empty());
    }

    #[test]
    fn test_get_info_unknown_recipe() {
        let c = catalog();
        let info = get_recipe_info("nonexistent-recipe", &c);
        assert!(info.is_none());
    }

    #[test]
    fn test_get_info_extracts_node_types() {
        let c = catalog();
        let info = get_recipe_info("compress-images", &c).unwrap();
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
        let c = catalog();

        // file-upload recipes
        let info = get_recipe_info("compress-images", &c).unwrap();
        assert_eq!(info.input_mode, InputMode::FileUpload);

        // url-mode recipe
        let info = get_recipe_info("download-video", &c).unwrap();
        assert_eq!(info.input_mode, InputMode::Url);
    }

    #[test]
    fn test_get_info_video_has_dependencies() {
        let c = catalog();
        let info = get_recipe_info("download-video", &c).unwrap();
        assert!(
            !info.dependencies.is_empty(),
            "download-video requires external dependencies"
        );
        let dep_names: Vec<&str> = info
            .dependencies
            .iter()
            .map(|d| d.dependency.binary.as_str())
            .collect();
        assert!(dep_names.contains(&"yt-dlp"));
    }

    #[test]
    fn keep_awake_lists_each_oss_tools_with_its_os() {
        let info = get_recipe_info("keep-awake", &catalog()).unwrap();
        let notes: Vec<String> = info
            .dependencies
            .iter()
            .map(|d| format!("{}{}", d.dependency.binary, dependency_notes(d)))
            .collect();
        assert_eq!(
            notes,
            [
                "caffeinate (macOS only)",
                "osascript (macOS only)",
                "systemd-inhibit (Linux only)"
            ]
        );
    }

    #[test]
    fn test_get_info_image_recipe_no_dependencies() {
        let c = catalog();
        let info = get_recipe_info("compress-images", &c).unwrap();
        assert!(
            info.dependencies.is_empty(),
            "Browser-safe image recipes have no external dependencies"
        );
    }

    #[test]
    fn test_get_info_multi_node_recipe() {
        let c = catalog();
        let info = get_recipe_info("optimize-images-for-web", &c).unwrap();
        assert!(
            info.node_types.len() >= 2,
            "optimize-images-for-web is a multi-node pipeline"
        );
    }

    #[test]
    fn test_get_info_reports_where_it_runs() {
        let c = catalog();
        let info = get_recipe_info("keep-awake", &c).unwrap();
        assert_eq!(info.os, HostPlatform::ALL);
        let info = get_recipe_info("compress-images", &c).unwrap();
        assert_eq!(info.os, HostPlatform::ALL);
    }

    #[test]
    fn test_node_types_skip_every_container() {
        let def: PipelineDefinition = serde_json::from_value(serde_json::json!({ "nodes": [
            { "id": "input", "type": "input" },
            { "id": "p", "type": "parallel", "nodes": [{ "id": "a", "type": "file-rename" }] },
            { "id": "g", "type": "group", "nodes": [{ "id": "b", "type": "file-sort" }] },
        ] }))
        .unwrap();
        assert_eq!(extract_node_types(&def), ["file-rename", "file-sort"]);
    }

    // --- format_recipe_info ---

    #[test]
    fn test_format_shows_where_it_runs() {
        let info = RecipeInfo {
            slug: "keep-awake".to_string(),
            name: "Keep Awake".to_string(),
            description: "Keep your Mac awake".to_string(),
            category: "system".to_string(),
            node_types: vec!["shell-run".to_string()],
            dependencies: vec![],
            input_mode: InputMode::None,
            os: vec![HostPlatform::MacOs],
        };
        assert!(format_recipe_info(&info).contains("  Runs on:    macOS\n"));
    }

    #[test]
    fn test_format_contains_name() {
        let info = RecipeInfo {
            slug: "compress-images".to_string(),
            name: "Compress Images".to_string(),
            description: "Compress image files".to_string(),
            category: "image".to_string(),
            node_types: vec!["image-compress".to_string()],
            dependencies: vec![],
            input_mode: InputMode::FileUpload,
            os: HostPlatform::ALL.to_vec(),
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
            input_mode: InputMode::FileUpload,
            os: HostPlatform::ALL.to_vec(),
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
            input_mode: InputMode::FileUpload,
            os: HostPlatform::ALL.to_vec(),
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
            node_types: vec!["shell-run".to_string()],
            dependencies: vec![ScopedDependency {
                dependency: bnto_core::Dependency {
                    binary: "yt-dlp".to_string(),
                    version: String::new(),
                    install_hint: "brew install yt-dlp".to_string(),
                    install_hints: Default::default(),
                    homepage: "https://github.com/yt-dlp/yt-dlp".to_string(),
                },
                os: HostPlatform::ALL.to_vec(),
            }],
            input_mode: InputMode::Url,
            os: HostPlatform::ALL.to_vec(),
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
            node_types: vec!["shell-run".to_string()],
            dependencies: vec![],
            input_mode: InputMode::Url,
            os: HostPlatform::ALL.to_vec(),
        };
        let output = format_recipe_info(&info);
        assert!(output.contains("  Input:      url\n"));
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
            input_mode: InputMode::FileUpload,
            os: HostPlatform::ALL.to_vec(),
        };
        let output = format_recipe_info(&info);
        assert!(
            !output.contains("yt-dlp"),
            "Should not mention deps when none exist"
        );
    }

    #[test]
    fn run_hint_omits_argument_for_none_mode() {
        assert_eq!(
            run_hint("keep-awake", InputMode::None),
            "Run with: bnto run keep-awake"
        );
    }

    #[test]
    fn run_hint_uses_url_for_url_mode() {
        assert_eq!(
            run_hint("download-video", InputMode::Url),
            "Run with: bnto run download-video <url>"
        );
    }

    #[test]
    fn run_hint_uses_text_for_text_mode() {
        assert_eq!(
            run_hint("echo", InputMode::Text),
            "Run with: bnto run echo <text>"
        );
    }

    #[test]
    fn run_hint_uses_files_for_file_upload_mode() {
        assert_eq!(
            run_hint("compress-images", InputMode::FileUpload),
            "Run with: bnto run compress-images <files...>"
        );
    }
}
