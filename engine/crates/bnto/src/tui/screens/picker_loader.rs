// File picker loading — read filesystem entries filtered by extension.
//
// Extracted from picker.rs to keep I/O separate from pure state logic.

use std::path::Path;

use super::picker::FileEntry;

/// Read directory entries, filter files by extension, sort dirs-first then alpha.
pub fn load_entries(dir: &Path, extensions: &[String], show_hidden: bool) -> Vec<FileEntry> {
    let read_dir = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return Vec::new(),
    };

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    for entry in read_dir.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !show_hidden && name.starts_with('.') {
            continue;
        }

        let path = entry.path();
        let is_dir = path.is_dir();

        if is_dir {
            dirs.push(FileEntry {
                name,
                is_dir,
                path,
                size: None,
            });
        } else if matches_extensions(&name, extensions) {
            let size = std::fs::metadata(&path).ok().map(|m| m.len());
            files.push(FileEntry {
                name,
                is_dir: false,
                path,
                size,
            });
        }
    }

    dirs.sort_by_key(|a| a.name.to_lowercase());
    files.sort_by_key(|a| a.name.to_lowercase());

    dirs.extend(files);
    dirs
}

/// Check if a filename matches any of the allowed extensions.
fn matches_extensions(name: &str, extensions: &[String]) -> bool {
    if extensions.is_empty() {
        return true;
    }
    let lower = name.to_lowercase();
    extensions
        .iter()
        .any(|ext| lower.ends_with(&format!(".{ext}")))
}

/// Collect accepted extensions from a recipe definition via the engine registry.
///
/// Walks the recipe's node list, finds processor nodes, resolves their
/// `metadata().accepts` MIME types, and converts to file extensions.
pub fn extensions_for_recipe(
    slug: &str,
    registry: &bnto_core::registry::NodeRegistry,
) -> Vec<String> {
    let recipe = match bnto_engine::recipes::builtin_recipe_by_slug(slug) {
        Some(r) => r,
        None => return Vec::new(),
    };
    let def: serde_json::Value = match serde_json::from_str(recipe.definition_json) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let nodes = match def["nodes"].as_array() {
        Some(n) => n,
        None => return Vec::new(),
    };

    let mut mimes = Vec::new();
    for node in nodes {
        let node_type = node["type"].as_str().unwrap_or_default();
        if node_type == "input" || node_type == "output" {
            continue;
        }
        let node_params = node["parameters"].as_object();
        let empty = serde_json::Map::new();
        let params = node_params.unwrap_or(&empty);
        if let Some(processor) = registry.resolve(node_type, params) {
            for mime in &processor.metadata().accepts {
                mimes.push(mime.clone());
            }
        }
    }

    extensions_from_mimes(&mimes)
}

/// Resolve a MIME type to file extensions.
pub fn mime_to_extensions(mime: &str) -> &'static [&'static str] {
    match mime {
        "image/jpeg" => &["jpg", "jpeg"],
        "image/png" => &["png"],
        "image/webp" => &["webp"],
        "image/gif" => &["gif"],
        "image/bmp" => &["bmp"],
        "image/tiff" => &["tiff", "tif"],
        "image/svg+xml" => &["svg"],
        "text/csv" => &["csv"],
        "application/json" => &["json"],
        "text/plain" => &["txt"],
        "video/mp4" => &["mp4"],
        "video/webm" => &["webm"],
        "audio/mpeg" => &["mp3"],
        _ => &[],
    }
}

/// Collect unique extensions from a list of MIME types.
pub fn extensions_from_mimes(mimes: &[String]) -> Vec<String> {
    let mut exts: Vec<String> = mimes
        .iter()
        .flat_map(|m| mime_to_extensions(m))
        .map(|s| s.to_string())
        .collect();
    exts.sort();
    exts.dedup();
    exts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_extensions_with_valid_ext() {
        assert!(matches_extensions(
            "photo.jpg",
            &["jpg".into(), "png".into()]
        ));
        assert!(matches_extensions(
            "photo.PNG",
            &["jpg".into(), "png".into()]
        ));
    }

    #[test]
    fn matches_extensions_rejects_non_matching() {
        assert!(!matches_extensions(
            "readme.txt",
            &["jpg".into(), "png".into()]
        ));
    }

    #[test]
    fn matches_extensions_allows_all_when_empty() {
        assert!(matches_extensions("anything.xyz", &[]));
    }

    #[test]
    fn mime_to_extensions_known_types() {
        assert_eq!(mime_to_extensions("image/jpeg"), &["jpg", "jpeg"]);
        assert_eq!(mime_to_extensions("image/png"), &["png"]);
        assert_eq!(mime_to_extensions("text/csv"), &["csv"]);
    }

    #[test]
    fn mime_to_extensions_unknown_returns_empty() {
        assert!(mime_to_extensions("application/x-unknown").is_empty());
    }

    #[test]
    fn extensions_from_mimes_deduplicates_and_sorts() {
        let mimes = vec!["image/jpeg".into(), "image/png".into(), "image/jpeg".into()];
        let exts = extensions_from_mimes(&mimes);
        assert_eq!(exts, vec!["jpeg", "jpg", "png"]);
    }

    #[test]
    fn extensions_for_compress_images() {
        let registry = bnto_engine::create_registry();
        let exts = extensions_for_recipe("compress-images", &registry);
        assert!(
            exts.contains(&"jpg".to_string()) || exts.contains(&"jpeg".to_string()),
            "compress-images should accept JPEG, got: {exts:?}"
        );
        assert!(
            exts.contains(&"png".to_string()),
            "compress-images should accept PNG, got: {exts:?}"
        );
    }

    #[test]
    fn extensions_for_unknown_recipe_returns_empty() {
        let registry = bnto_engine::create_registry();
        let exts = extensions_for_recipe("nonexistent-recipe", &registry);
        assert!(exts.is_empty());
    }
}
