// file-collect — Traverse directories and glob-match files into the pipeline.
//
// CLI-only processor: reads files from the filesystem via ProcessContext.
// Walks the directory its input references: a disk reference's path (as
// `file-list` emits a listed folder), else the input's filename (a CLI dir
// input). Outputs each matched file as a pipeline entry, named relative to
// the walked directory.

use bnto_core::metadata::{NodeCategory, NodeMetadata, ParameterDef, ParameterType};
use bnto_core::processor::{FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};
use std::path::{Path, PathBuf};

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
        let root = walk_root(&input);
        if !root.is_dir() {
            return Err(BntoError::InvalidInput(format!(
                "Not a directory: {}",
                root.display()
            )));
        }

        let mut walker = Walker::new(&root, &input, progress);
        walker.walk(&root)?;
        Ok(NodeOutput {
            files: walker.found.into_iter().map(output_file).collect(),
            metadata: serde_json::Map::new(),
        })
    }
}

/// A matched file as a path reference (zero-copy — no file reads).
fn output_file((output_name, full_path): (String, PathBuf)) -> OutputFile {
    OutputFile {
        data: FileData::Path(full_path),
        mime_type: guess_mime_type(&output_name),
        filename: output_name,
        metadata: serde_json::Map::new(),
    }
}

fn param_str<'a>(input: &'a NodeInput, key: &str) -> Option<&'a str> {
    input.params.get(key).and_then(|v| v.as_str())
}

fn param_bool(input: &NodeInput, key: &str) -> Option<bool> {
    input.params.get(key).and_then(|v| v.as_bool())
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

/// The directory to walk: the one a disk reference points at (`file-list`
/// names it by its relative path), else the input's filename (a CLI dir
/// input carries its absolute path there, with no data).
fn walk_root(input: &NodeInput) -> PathBuf {
    match &input.data {
        FileData::Path(path) => path.clone(),
        _ => PathBuf::from(&input.filename),
    }
}

/// One directory walk: its settings, where it reports, and what it found.
struct Walker<'a> {
    /// The walked root (for computing relative paths).
    base_dir: &'a Path,
    pattern: &'a str,
    recursive: bool,
    flatten: bool,
    progress: &'a ProgressReporter,
    /// `(output name, full path)` of each matched file, in walk order.
    found: Vec<(String, PathBuf)>,
}

impl<'a> Walker<'a> {
    fn new(base_dir: &'a Path, input: &'a NodeInput, progress: &'a ProgressReporter) -> Self {
        Self {
            base_dir,
            pattern: param_str(input, "pattern").unwrap_or("*"),
            recursive: param_bool(input, "recursive").unwrap_or(true),
            flatten: param_bool(input, "flatten").unwrap_or(true),
            progress,
            found: Vec::new(),
        }
    }

    /// Collect the files in `dir` that match the pattern, and those in its
    /// subdirectories when recursive.
    ///
    /// Reports the running count after every entry, so a walk of a huge
    /// tree keeps moving; the reporter throttles what reaches the user.
    fn walk(&mut self, dir: &Path) -> Result<(), BntoError> {
        let entries = std::fs::read_dir(dir).map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to read directory {}: {e}", dir.display()))
        })?;
        let label = self.folder_label(dir);
        for entry in entries {
            let entry = entry.map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to read directory entry: {e}"))
            })?;
            self.visit(entry.path())?;
            self.report(&label);
        }
        Ok(())
    }

    fn visit(&mut self, path: PathBuf) -> Result<(), BntoError> {
        if path.is_dir() {
            if self.recursive {
                self.walk(&path)?;
            }
        } else if path.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if glob_matches(self.pattern, file_name) {
                let output_name = if self.flatten {
                    file_name.to_string()
                } else {
                    bnto_core::slash_path(path.strip_prefix(self.base_dir).unwrap_or(&path))
                };
                self.found.push((output_name, path));
            }
        }
        Ok(())
    }

    /// `dir` relative to the walked root, `/`-separated; empty for the root.
    fn folder_label(&self, dir: &Path) -> String {
        bnto_core::slash_path(dir.strip_prefix(self.base_dir).unwrap_or(dir))
    }

    fn report(&self, label: &str) {
        let count = self.found.len();
        let message = if label.is_empty() {
            format!("Found {count} files")
        } else {
            format!("Found {count} files · {label}")
        };
        self.progress.report_count(count as u64, &message);
    }
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
    use bnto_core::progress::{LiveProgress, ProgressReporter};
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

    // --- Live progress ---

    type Reports = std::rc::Rc<std::cell::RefCell<Vec<(LiveProgress, String)>>>;

    /// Run a collect over the test dir, recording every live update.
    fn collect_reports(throttle_clock: Option<fn() -> u64>) -> Reports {
        let test_dir = setup_test_dir();
        let reports: Reports = Default::default();
        let sink = std::rc::Rc::clone(&reports);
        let mut progress = ProgressReporter::with_live(
            move |p, m| sink.borrow_mut().push((p, m.to_string())),
            |_| {},
        );
        if let Some(clock) = throttle_clock {
            progress = progress.throttled(clock, 100);
        }
        let input = dir_input(test_dir.path(), params(&[]));
        FileCollect::new()
            .process(input, &progress, &NoopContext)
            .unwrap();
        reports
    }

    fn counts(reports: &Reports) -> Vec<u64> {
        reports
            .borrow()
            .iter()
            .map(|(p, _)| match p {
                LiveProgress::Count(n) => *n,
                LiveProgress::Percent(p) => panic!("expected counts only, got {p}%"),
            })
            .collect()
    }

    #[test]
    fn test_collect_reports_a_growing_count_while_it_walks() {
        let counts = counts(&collect_reports(None));
        assert!(counts.windows(2).all(|w| w[0] <= w[1]), "{counts:?}");
        assert!(
            counts.contains(&1),
            "reports before the walk ends: {counts:?}"
        );
        assert_eq!(counts.last(), Some(&5));
    }

    #[test]
    fn test_collect_progress_names_the_folder_being_walked() {
        let reports = collect_reports(None);
        let messages: Vec<String> = reports.borrow().iter().map(|r| r.1.clone()).collect();
        assert!(
            messages.iter().any(|m| m.contains("subfolder")),
            "{messages:?}"
        );
        assert!(
            messages.iter().all(|m| m.starts_with("Found ")),
            "{messages:?}"
        );
    }

    #[test]
    fn test_collect_progress_is_throttled_by_the_reporter() {
        // A clock that never moves lets only the first report through.
        assert_eq!(counts(&collect_reports(Some(|| 0))).len(), 1);
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
        // `/` on every OS, so name filters downstream split it the same way.
        assert_eq!(output.files[0].filename, "subfolder/image.png");
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

    // --- Disk-reference input ---

    /// An input that references `dir` on disk under a different name, as
    /// `file-list` emits a listed folder.
    fn reference_input(
        dir: &Path,
        name: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> NodeInput {
        NodeInput {
            data: FileData::Path(dir.to_path_buf()),
            filename: name.to_string(),
            mime_type: None,
            params,
        }
    }

    #[test]
    fn test_collect_walks_the_referenced_dir() {
        let test_dir = setup_test_dir();
        let input = reference_input(
            test_dir.path(),
            "Some/Folder",
            params(&[("flatten", serde_json::json!(false))]),
        );
        let output = FileCollect::new()
            .process(input, &ProgressReporter::new_noop(), &NoopContext)
            .unwrap();

        let mut names: Vec<&str> = output.files.iter().map(|f| f.filename.as_str()).collect();
        names.sort_unstable();
        assert_eq!(
            names,
            [
                "data.csv",
                "document.txt",
                "photo.jpg",
                "subfolder/image.png",
                "subfolder/readme.txt"
            ],
            "names are relative to the walked folder, as for a CLI dir"
        );
    }

    #[test]
    fn test_collect_missing_referenced_dir_fails() {
        let test_dir = setup_test_dir();
        let gone = test_dir.path().join("gone");
        let input = reference_input(&gone, "gone", serde_json::Map::new());
        let Err(err) =
            FileCollect::new().process(input, &ProgressReporter::new_noop(), &NoopContext)
        else {
            panic!("a missing referenced dir should fail");
        };

        assert!(
            err.to_string().contains(&gone.display().to_string()),
            "the error names the walked path: {err}"
        );
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
