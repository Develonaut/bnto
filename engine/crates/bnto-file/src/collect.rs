// file-collect — Traverse directories and glob-match files into the pipeline.
//
// CLI-only processor: reads files from the filesystem via ProcessContext.
// Takes a directory path as its "input file" (the filename IS the path),
// outputs individual matched files as pipeline entries.

use bnto_core::metadata::{NodeCategory, NodeMetadata, ParameterDef, ParameterType};
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use std::path::Path;

/// File-collect processor — directory traversal + glob matching.
pub struct FileCollect;

impl FileCollect {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileCollect {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileCollect {
    fn name(&self) -> &str {
        "file-collect"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-collect".to_string(),
            name: "Collect Files".to_string(),
            description:
                "Traverse a directory and collect files matching a glob pattern into the pipeline."
                    .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_collect_params(),
            input_cardinality: Default::default(), // PerFile — directory path is the "file"
            requires: vec![],
        }
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        progress.report(0, "Collecting files...");

        // The input "filename" is the directory path to traverse.
        let dir_path = Path::new(&input.filename);

        // Verify the directory exists.
        if !dir_path.is_dir() {
            return Err(BntoError::InvalidInput(format!(
                "Not a directory: {}",
                input.filename
            )));
        }

        // Read params.
        let pattern = input
            .params
            .get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("*");

        let recursive = input
            .params
            .get("recursive")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let flatten = input
            .params
            .get("flatten")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        // Walk the directory and collect matching files.
        let mut matched_files = Vec::new();
        walk_directory(
            dir_path,
            dir_path,
            pattern,
            recursive,
            flatten,
            &mut matched_files,
        )?;

        progress.report(50, &format!("Found {} files", matched_files.len()));

        // Produce output with path references (zero-copy — no file reads).
        let total = matched_files.len();
        let mut output_files = Vec::with_capacity(total);

        for (i, (output_name, full_path)) in matched_files.into_iter().enumerate() {
            let pct = 50 + ((i as u32 * 50) / (total as u32).max(1));
            progress.report(pct, &format!("Collecting {}/{total}...", i + 1));

            let mime_type = guess_mime_type(&output_name);

            output_files.push(OutputFile {
                data: FileData::Path(full_path),
                filename: output_name,
                mime_type,
                metadata: serde_json::Map::new(),
            });
        }

        progress.report(100, "Done");

        Ok(NodeOutput {
            files: output_files,
            metadata: serde_json::Map::new(),
        })
    }
}

// --- Parameter Definitions ---

fn build_collect_params() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "pattern".to_string(),
            label: "Pattern".to_string(),
            description: "Glob pattern to match files (e.g., \"*.jpg\", \"**/*.svg\"). Default: \"*\" (all files).".to_string(),
            param_type: ParameterType::String,
            default: Some(serde_json::json!("*")),
            placeholder: Some("*.jpg".to_string()),
            ..Default::default()
        },
        ParameterDef {
            name: "recursive".to_string(),
            label: "Recursive".to_string(),
            description: "Traverse subdirectories when collecting files.".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            ..Default::default()
        },
        ParameterDef {
            name: "flatten".to_string(),
            label: "Flatten".to_string(),
            description: "Strip directory structure from output filenames. When true, all files appear as if in the same directory.".to_string(),
            param_type: ParameterType::Boolean,
            default: Some(serde_json::json!(true)),
            ..Default::default()
        },
    ]
}

// --- Core Logic ---

/// Recursively walk a directory and collect files matching the glob pattern.
///
/// `base_dir` is the root directory (for computing relative paths).
/// `current_dir` is the directory being scanned right now.
fn walk_directory(
    base_dir: &Path,
    current_dir: &Path,
    pattern: &str,
    recursive: bool,
    flatten: bool,
    results: &mut Vec<(String, std::path::PathBuf)>,
) -> Result<(), BntoError> {
    let entries = std::fs::read_dir(current_dir).map_err(|e| {
        BntoError::ProcessingFailed(format!(
            "Failed to read directory {}: {e}",
            current_dir.display()
        ))
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to read directory entry: {e}"))
        })?;
        let path = entry.path();

        if path.is_dir() {
            // Recurse into subdirectories if enabled.
            if recursive {
                walk_directory(base_dir, &path, pattern, recursive, flatten, results)?;
            }
        } else if path.is_file() {
            // Check if the filename matches the glob pattern.
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if glob_matches(pattern, file_name) {
                // Determine the output filename.
                let output_name = if flatten {
                    file_name.to_string()
                } else {
                    // Preserve the path relative to the base directory.
                    path.strip_prefix(base_dir)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .to_string()
                };

                results.push((output_name, path));
            }
        }
    }

    Ok(())
}

/// Simple glob matching (same logic as file-filter's glob_match).
fn glob_matches(pattern: &str, text: &str) -> bool {
    let mut regex_str = String::from("^");
    for ch in pattern.chars() {
        match ch {
            '*' => regex_str.push_str(".*"),
            '?' => regex_str.push('.'),
            '.' | '+' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '\\' => {
                regex_str.push('\\');
                regex_str.push(ch);
            }
            _ => regex_str.push(ch),
        }
    }
    regex_str.push('$');

    regex::Regex::new(&regex_str)
        .map(|re| re.is_match(text))
        .unwrap_or(false)
}

/// Guess MIME type from file extension.
fn guess_mime_type(filename: &str) -> String {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "webp" => "image/webp".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "gif" => "image/gif".to_string(),
        "csv" => "text/csv".to_string(),
        "json" => "application/json".to_string(),
        "txt" | "md" => "text/plain".to_string(),
        "pdf" => "application/pdf".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::context::NoopContext;
    use bnto_core::progress::ProgressReporter;
    use std::fs;
    use tempfile::TempDir;

    /// Helper: create a test directory with known files.
    fn setup_test_dir() -> TempDir {
        let dir = TempDir::new().unwrap();
        let root = dir.path();

        // Create files in root.
        fs::write(root.join("photo.jpg"), b"jpeg data").unwrap();
        fs::write(root.join("document.txt"), b"text data").unwrap();
        fs::write(root.join("data.csv"), b"a,b\n1,2\n").unwrap();

        // Create a subdirectory with files.
        let sub = root.join("subfolder");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("image.png"), b"png data").unwrap();
        fs::write(sub.join("readme.txt"), b"readme").unwrap();

        dir
    }

    /// Helper: create a NodeInput pointing to a directory.
    fn dir_input(dir: &Path, params: serde_json::Map<String, serde_json::Value>) -> NodeInput {
        NodeInput {
            data: FileData::Bytes(vec![]), // No data for directory input.
            filename: dir.to_string_lossy().to_string(),
            mime_type: None,
            params,
        }
    }

    fn params(pairs: &[(&str, serde_json::Value)]) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v.clone());
        }
        map
    }

    // --- Trait basics ---

    #[test]
    fn test_name_returns_correct_key() {
        assert_eq!(FileCollect::new().name(), "file-collect");
    }

    #[test]
    fn test_metadata_is_cli_only() {
        let meta = FileCollect::new().metadata();
        assert_eq!(meta.category, NodeCategory::File);
        assert!(!meta.platforms.contains(&"browser".to_string()));
        assert!(meta.platforms.contains(&"cli".to_string()));
    }

    // --- Glob matching ---

    #[test]
    fn test_collect_all_files() {
        let test_dir = setup_test_dir();
        let processor = FileCollect::new();
        let progress = ProgressReporter::new_noop();

        let input = dir_input(
            test_dir.path(),
            params(&[("pattern", serde_json::json!("*"))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        // Should find all 5 files (3 root + 2 subfolder), recursive by default.
        assert_eq!(
            output.files.len(),
            5,
            "Should collect all files recursively"
        );
    }

    #[test]
    fn test_collect_glob_extension_filter() {
        let test_dir = setup_test_dir();
        let processor = FileCollect::new();
        let progress = ProgressReporter::new_noop();

        let input = dir_input(
            test_dir.path(),
            params(&[("pattern", serde_json::json!("*.txt"))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        // Should find document.txt + readme.txt
        assert_eq!(output.files.len(), 2, "Should match only .txt files");
        let names: Vec<&str> = output.files.iter().map(|f| f.filename.as_str()).collect();
        assert!(names.contains(&"document.txt"));
        assert!(names.contains(&"readme.txt"));
    }

    // --- Recursive vs flat ---

    #[test]
    fn test_collect_non_recursive() {
        let test_dir = setup_test_dir();
        let processor = FileCollect::new();
        let progress = ProgressReporter::new_noop();

        let input = dir_input(
            test_dir.path(),
            params(&[
                ("pattern", serde_json::json!("*")),
                ("recursive", serde_json::json!(false)),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        // Should find only 3 root files, not the 2 in subfolder.
        assert_eq!(
            output.files.len(),
            3,
            "Non-recursive should only find root files"
        );
    }

    // --- Flatten behavior ---

    #[test]
    fn test_collect_flatten_strips_paths() {
        let test_dir = setup_test_dir();
        let processor = FileCollect::new();
        let progress = ProgressReporter::new_noop();

        let input = dir_input(
            test_dir.path(),
            params(&[
                ("pattern", serde_json::json!("*.png")),
                ("flatten", serde_json::json!(true)),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(
            output.files[0].filename, "image.png",
            "Flatten should strip directory"
        );
    }

    #[test]
    fn test_collect_no_flatten_preserves_paths() {
        let test_dir = setup_test_dir();
        let processor = FileCollect::new();
        let progress = ProgressReporter::new_noop();

        let input = dir_input(
            test_dir.path(),
            params(&[
                ("pattern", serde_json::json!("*.png")),
                ("flatten", serde_json::json!(false)),
            ]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files.len(), 1);
        assert!(
            output.files[0].filename.contains("subfolder"),
            "No-flatten should preserve relative path, got: {}",
            output.files[0].filename
        );
    }

    // --- No matches ---

    #[test]
    fn test_collect_no_matches() {
        let test_dir = setup_test_dir();
        let processor = FileCollect::new();
        let progress = ProgressReporter::new_noop();

        let input = dir_input(
            test_dir.path(),
            params(&[("pattern", serde_json::json!("*.xyz"))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(
            output.files.len(),
            0,
            "No matches should produce empty output"
        );
    }

    // --- Error handling ---

    #[test]
    fn test_collect_nonexistent_dir_fails() {
        let processor = FileCollect::new();
        let progress = ProgressReporter::new_noop();

        let input = dir_input(
            Path::new("/nonexistent/path/that/does/not/exist"),
            serde_json::Map::new(),
        );
        let result = processor.process(input, &progress, &NoopContext);

        assert!(result.is_err(), "Nonexistent directory should fail");
    }

    // --- File data reading ---

    #[test]
    fn test_collect_outputs_path_references() {
        let test_dir = setup_test_dir();
        let processor = FileCollect::new();
        let progress = ProgressReporter::new_noop();

        let input = dir_input(
            test_dir.path(),
            params(&[("pattern", serde_json::json!("*.csv"))]),
        );
        let output = processor.process(input, &progress, &NoopContext).unwrap();

        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].filename, "data.csv");

        // Verify output is a path reference, not in-memory bytes.
        assert!(
            matches!(&output.files[0].data, FileData::Path(_)),
            "Collect should output FileData::Path for zero-copy moves"
        );

        // Path should resolve to actual file content.
        let bytes = output.files[0].data.clone().into_bytes().unwrap();
        let content = String::from_utf8_lossy(&bytes);
        assert!(
            content.contains("a,b"),
            "Path should resolve to file content"
        );
    }

    // --- MIME type guessing ---

    #[test]
    fn test_mime_type_guessing() {
        assert_eq!(guess_mime_type("photo.jpg"), "image/jpeg");
        assert_eq!(guess_mime_type("icon.png"), "image/png");
        assert_eq!(guess_mime_type("data.csv"), "text/csv");
        assert_eq!(guess_mime_type("config.json"), "application/json");
        assert_eq!(guess_mime_type("unknown.xyz"), "application/octet-stream");
    }

    // --- Glob matching ---

    #[test]
    fn test_glob_matches_star() {
        assert!(glob_matches("*.txt", "hello.txt"));
        assert!(!glob_matches("*.txt", "hello.jpg"));
        assert!(glob_matches("*", "anything"));
    }

    #[test]
    fn test_glob_matches_question() {
        assert!(glob_matches("file?.txt", "file1.txt"));
        assert!(!glob_matches("file?.txt", "file12.txt"));
    }
}
