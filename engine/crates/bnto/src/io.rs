// File I/O helpers — read input files and write pipeline results.

use std::path::{Path, PathBuf};

use bnto_core::{PipelineFile, PipelineFileResult, PipelineResult};

use crate::output_names::{self, Renamed};

/// Read a file (or directory) from disk into a PipelineFile.
///
/// For regular files, references the file on disk (`FileData::Path`) without
/// reading it; nodes that need the content read it themselves, so a multi-GB
/// file only loads when a node asks for its bytes. For directories, creates a
/// synthetic entry with the absolute path as the name and empty data —
/// processors like `file-collect` use the filename as the directory to traverse.
pub fn read_pipeline_file(path: &str) -> Result<PipelineFile, String> {
    let p = Path::new(path);

    if p.is_dir() {
        // dunce, not std: on Windows std returns the verbatim `\\?\D:\dir`,
        // which every progress line then shows. dunce drops the prefix when the
        // plain path means the same thing; std::fs handles either.
        let abs = dunce::canonicalize(p)
            .map_err(|e| format!("{e}"))?
            .to_string_lossy()
            .to_string();
        return Ok(PipelineFile {
            name: abs,
            data: bnto_core::processor::FileData::Bytes(vec![]),
            mime_type: "inode/directory".to_string(),
            metadata: serde_json::Map::new(),
        });
    }

    let name = p
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string());
    let mime_type = guess_mime(path);

    PipelineFile::input_path(name, p.to_path_buf(), mime_type).map_err(|e| format!("{e}"))
}

/// Write all result files to an output directory.
///
/// File names may contain subdirectory separators (e.g. `"subdir/video.mp4"`).
/// Parent directories are created automatically before writing.
///
/// Uses `FileData::copy_to()` which copies for path-referenced files
/// and writes bytes for in-memory files. Source files are never modified,
/// and a file already at its destination, or a made-up name's file already
/// holding the result's bytes, is left as is.
///
/// Names that would land on one file are kept apart (see `output_names`).
pub fn write_results(result: &PipelineResult, output_dir: &str) -> Result<Placed, String> {
    check_file_names(result)?;
    std::fs::create_dir_all(output_dir).map_err(|e| format!("Cannot create {output_dir}: {e}"))?;

    let (dests, renamed) = destinations(result, output_dir);
    let unchanged = already_there(result, &dests);
    for (file, dest) in result.files.iter().zip(&dests) {
        if !dest.held {
            write_one(file, &dest.path)?;
        }
    }
    Ok(Placed { renamed, unchanged })
}

/// What a write did besides writing: the files given a new name (written, or
/// held: already holding the bytes and left as is), and
/// how many were already there (the source itself, or a file holding the
/// result's bytes at its made-up name), left as is.
#[derive(Debug, Default, PartialEq)]
pub struct Placed {
    pub renamed: Vec<Renamed>,
    pub unchanged: usize,
}

/// What `write_results` would do, writing nothing (a dry run).
fn plan_results(result: &PipelineResult, output_dir: &str) -> Result<Placed, String> {
    check_file_names(result)?;
    let (dests, renamed) = destinations(result, output_dir);
    let unchanged = already_there(result, &dests);
    Ok(Placed { renamed, unchanged })
}

/// Where one result file goes, and whether the file there already holds its
/// bytes at a made-up name (`output_names::Claimed::held`), so it's not written.
struct Destination {
    path: PathBuf,
    held: bool,
}

/// Each result file's destination under `output_dir`, with the files given a
/// new name so no two land on one file and none on a file already there.
fn destinations(result: &PipelineResult, output_dir: &str) -> (Vec<Destination>, Vec<Renamed>) {
    let dir = Path::new(output_dir);
    let files = result.files.iter().map(|f| (f.name.as_str(), &f.data));
    let (claims, renamed) = output_names::plan(dir, files);
    let dests = claims
        .into_iter()
        .map(|c| Destination {
            path: dir.join(&c.name),
            held: c.held,
        })
        .collect();
    (dests, renamed)
}

/// How many result files are already there: the file at their destination,
/// e.g. an input written back into its own folder (ISS-36), which `copy_to`
/// leaves, or a held file, which isn't written.
fn already_there(result: &PipelineResult, dests: &[Destination]) -> usize {
    let files = result.files.iter().zip(dests);
    files
        .filter(|(file, d)| d.held || file.data.is_at(&d.path))
        .count()
}

fn write_one(file: &PipelineFileResult, out_path: &Path) -> Result<(), String> {
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Cannot create {}: {e}", parent.display()))?;
    }
    file.data
        .copy_to(out_path)
        .map_err(|e| format!("Cannot write {}: {e}", out_path.display()))
}

/// Fail on a result file with no name, for a real write and a dry run alike.
fn check_file_names(result: &PipelineResult) -> Result<(), String> {
    if result.files.iter().any(|f| f.name.is_empty()) {
        return Err(
            "Pipeline produced a file with an empty name — cannot write to output directory"
                .to_string(),
        );
    }
    Ok(())
}

// =============================================================================
// Mode-aware output
// =============================================================================

/// Outcome of a mode-aware write operation.
#[derive(Debug, PartialEq)]
pub enum WriteOutcome {
    /// Files were copied to an output directory; `renamed` lists those given
    /// a new name so they didn't replace one another, including held ones
    /// (already holding the bytes, left as is and not written).
    /// `count` excludes the `unchanged` files, already there and left as is.
    Written {
        dir: String,
        count: usize,
        renamed: Vec<Renamed>,
        unchanged: usize,
    },
    /// Files were renamed in place (overwrite mode).
    Overwritten { count: usize },
    /// No files written — summary message only.
    Message { summary: String },
    /// No output action taken.
    None,
}

/// Write pipeline results based on the output node's mode.
///
/// - `write`: Copy files to `output_dir` (default behavior).
/// - `overwrite`: Rename source files in place (requires `FileData::Path`).
/// - `message`: No file writes, returns a summary.
/// - `none`: No output action.
///
/// On a `dry_run` nothing is written or renamed; the outcome is what a real
/// run would report.
#[expect(clippy::too_many_lines)]
pub fn write_results_with_mode(
    result: &PipelineResult,
    mode: &str,
    output_dir: &str,
    dry_run: bool,
) -> Result<WriteOutcome, String> {
    match mode {
        "write" => {
            let placed = if dry_run {
                plan_results(result, output_dir)?
            } else {
                write_results(result, output_dir)?
            };
            Ok(WriteOutcome::Written {
                dir: output_dir.to_string(),
                count: result.files.len() - placed.unchanged,
                renamed: placed.renamed,
                unchanged: placed.unchanged,
            })
        }
        "overwrite" => crate::overwrite::overwrite_in_place(result, dry_run)
            .map(|count| WriteOutcome::Overwritten { count }),
        "message" => {
            let summary = if result.files.is_empty() {
                "No output files produced.".to_string()
            } else {
                let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
                format!("{} file(s): {}", names.len(), names.join(", "))
            };
            Ok(WriteOutcome::Message { summary })
        }
        _ => Ok(WriteOutcome::None),
    }
}

/// Names of result files that `file-copy` left because the destination copy
/// was newer (its `skipped_newer` metadata flag is 1).
pub fn files_left_as_newer(result: &PipelineResult) -> Vec<&str> {
    result
        .files
        .iter()
        .filter(|f| f.metadata.get("skipped_newer").and_then(|v| v.as_i64()) == Some(1))
        .map(|f| f.name.as_str())
        .collect()
}

/// Per-file `file-copy` outcomes summed: (copied, up to date, newer at the
/// destination). `None` when no file carries any of the three flags.
pub fn copy_totals(result: &PipelineResult) -> Option<(usize, usize, usize)> {
    let keys = ["copied", "up_to_date", "skipped_newer"];
    let count = |key: &str| {
        result
            .files
            .iter()
            .filter(|f| f.metadata.get(key).and_then(|v| v.as_i64()) == Some(1))
            .count()
    };
    let seen = result
        .files
        .iter()
        .any(|f| keys.iter().any(|k| f.metadata.contains_key(*k)));
    seen.then(|| (count(keys[0]), count(keys[1]), count(keys[2])))
}

/// Print one `Copy:` line from `copy_totals`; a dry run says `would be copied`.
pub fn print_copy_totals(result: &PipelineResult, dry_run: bool) {
    if let Some((copied, up_to_date, newer)) = copy_totals(result) {
        let verb = if dry_run { "would be copied" } else { "copied" };
        eprintln!(
            "  Copy: {copied} {verb}, {up_to_date} up to date, {newer} newer at the destination"
        );
    }
}

/// Print one warning line naming the files `files_left_as_newer` finds.
pub fn warn_files_left_as_newer(result: &PipelineResult) {
    use colored::Colorize;

    let newer = files_left_as_newer(result);
    if !newer.is_empty() {
        let warning = "Warning:".yellow().bold();
        eprintln!("  {warning} {}", newer_warning_text(&newer));
    }
}

/// How many names the warning lists before folding the rest into `…and N more`.
const MAX_NAMES_LISTED: usize = 10;

/// `N file(s) newer at the destination, left: a, b`, names capped.
fn newer_warning_text(newer: &[&str]) -> String {
    format!(
        "{} newer at the destination, left: {}",
        count_files(newer.len()),
        capped_names(newer)
    )
}

/// Names of result files that `spreadsheet-lookup` found no row for (its
/// `unmatched` metadata flag is 1).
pub fn files_without_a_row(result: &PipelineResult) -> Vec<&str> {
    result
        .files
        .iter()
        .filter(|f| f.metadata.get("unmatched").and_then(|v| v.as_i64()) == Some(1))
        .map(|f| f.name.as_str())
        .collect()
}

/// Print one warning line naming the files `files_without_a_row` finds.
pub fn warn_files_without_a_row(result: &PipelineResult) {
    use colored::Colorize;

    let unmatched = files_without_a_row(result);
    if !unmatched.is_empty() {
        let warning = "Warning:".yellow().bold();
        eprintln!("  {warning} {}", no_row_warning_text(&unmatched));
    }
}

/// `N file(s) had no row in the lookup table: a, b`, names capped.
fn no_row_warning_text(unmatched: &[&str]) -> String {
    format!(
        "{} had no row in the lookup table: {}",
        count_files(unmatched.len()),
        capped_names(unmatched)
    )
}

/// Print a warning line naming the files written under a new name, and one
/// naming those left as is under one (held), each only when there are any.
pub fn warn_renamed(outcome: &WriteOutcome, dry_run: bool) {
    use colored::Colorize;

    if let WriteOutcome::Written { renamed, .. } = outcome {
        for line in renamed_warning_lines(renamed, dry_run) {
            eprintln!("  {} {line}", "Warning:".yellow().bold());
        }
    }
}

/// `N file(s) written under a new name …: a → a_1`, then `N file(s) already
/// there under a new name, holding the same bytes, left as is: …`.
fn renamed_warning_lines(renamed: &[Renamed], dry_run: bool) -> Vec<String> {
    let (held, written): (Vec<&Renamed>, Vec<&Renamed>) = renamed.iter().partition(|r| r.held);
    let (write, left) = if dry_run {
        ("would be written", "would be left")
    } else {
        ("written", "left")
    };
    let clash = format!("{write} under a new name, since another has the same name (case ignored)");
    let kept = format!("already there under a new name, holding the same bytes, {left} as is");
    [(written, clash), (held, kept)]
        .into_iter()
        .filter(|(files, _)| !files.is_empty())
        .map(|(files, what)| format!("{} {what}: {}", count_files(files.len()), pairs(&files)))
        .collect()
}

/// `a → a_1, b → b_1`, names capped.
fn pairs(renamed: &[&Renamed]) -> String {
    let pairs: Vec<String> = renamed
        .iter()
        .map(|r| format!("{} → {}", r.from, r.to))
        .collect();
    capped_names(&pairs.iter().map(String::as_str).collect::<Vec<_>>())
}

/// `1 file` / `N files`.
fn count_files(n: usize) -> String {
    let noun = if n == 1 { "file" } else { "files" };
    format!("{n} {noun}")
}

/// Names joined by `, `, folding past `MAX_NAMES_LISTED` into `…and N more`.
fn capped_names(names: &[&str]) -> String {
    let mut listed = names
        .iter()
        .take(MAX_NAMES_LISTED)
        .copied()
        .collect::<Vec<_>>()
        .join(", ");
    if names.len() > MAX_NAMES_LISTED {
        listed.push_str(&format!(", …and {} more", names.len() - MAX_NAMES_LISTED));
    }
    listed
}

/// Guess MIME type from file extension.
fn guess_mime(path: &str) -> String {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "csv" => "text/csv",
        "json" => "application/json",
        "txt" => "text/plain",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::PipelineFileResult;
    use bnto_core::processor::FileData;

    fn make_result_with_path(src: std::path::PathBuf, name: &str) -> PipelineResult {
        PipelineResult {
            files: vec![PipelineFileResult {
                name: name.to_string(),
                data: FileData::Path(src),
                mime_type: "text/plain".to_string(),
                metadata: serde_json::Map::new(),
            }],
            duration_ms: 0,
            warnings: vec![],
            node_metadata: Default::default(),
        }
    }

    fn make_result_with_bytes(data: &[u8], name: &str) -> PipelineResult {
        PipelineResult {
            files: vec![PipelineFileResult {
                name: name.to_string(),
                data: FileData::Bytes(data.to_vec()),
                mime_type: "text/plain".to_string(),
                metadata: serde_json::Map::new(),
            }],
            duration_ms: 0,
            warnings: vec![],
            node_metadata: Default::default(),
        }
    }

    // --- files_left_as_newer tests ---

    fn result_with_flags(files: &[(&str, Option<i64>)]) -> PipelineResult {
        let mut result = make_result_with_bytes(b"", "unused");
        result.files = files
            .iter()
            .map(|(name, flag)| {
                let mut f = make_result_with_bytes(b"", name).files.remove(0);
                if let Some(v) = flag {
                    f.metadata.insert("skipped_newer".into(), (*v).into());
                }
                f
            })
            .collect();
        result
    }

    #[test]
    fn test_copy_totals_none_without_flags() {
        assert_eq!(copy_totals(&result_with_flags(&[("a", None)])), None);
        assert_eq!(copy_totals(&result_with_flags(&[])), None);
    }

    #[test]
    fn test_copy_totals_sums_mixed_flags() {
        let mut result = result_with_flags(&[("a", Some(1)), ("b", Some(0)), ("c", None)]);
        result.files[1].metadata.insert("copied".into(), 1.into());
        result.files[2].metadata.insert("copied".into(), 1.into());
        result.files[2]
            .metadata
            .insert("up_to_date".into(), 1.into());
        assert_eq!(copy_totals(&result), Some((2, 1, 1)));
    }

    #[test]
    fn test_dry_run_write_mode_errors_on_empty_filename() {
        let result = make_result_with_bytes(b"data", "");
        let err = write_results_with_mode(&result, "write", "out", true).unwrap_err();
        assert!(err.contains("empty name"), "Error: {err}");
    }

    #[test]
    fn test_files_left_as_newer_empty_result() {
        assert!(files_left_as_newer(&result_with_flags(&[])).is_empty());
    }

    #[test]
    fn test_files_left_as_newer_one_flagged() {
        let r = result_with_flags(&[("a.txt", Some(0)), ("b.txt", Some(1))]);
        assert_eq!(files_left_as_newer(&r), vec!["b.txt"]);
    }

    #[test]
    fn test_files_left_as_newer_several_flagged_in_order() {
        let r = result_with_flags(&[("a.txt", Some(1)), ("b.txt", Some(0)), ("c.txt", Some(1))]);
        assert_eq!(files_left_as_newer(&r), vec!["a.txt", "c.txt"]);
    }

    #[test]
    fn test_files_left_as_newer_ignores_files_without_the_key() {
        let r = result_with_flags(&[("a.txt", None), ("b.txt", None)]);
        assert!(files_left_as_newer(&r).is_empty());
    }

    #[test]
    fn test_newer_warning_text_singular_and_plural() {
        assert_eq!(
            newer_warning_text(&["a.txt"]),
            "1 file newer at the destination, left: a.txt"
        );
        assert_eq!(
            newer_warning_text(&["a.txt", "b.txt"]),
            "2 files newer at the destination, left: a.txt, b.txt"
        );
    }

    #[test]
    fn test_newer_warning_text_caps_the_name_list() {
        let names: Vec<String> = (1..=12).map(|i| format!("f{i}.txt")).collect();
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        let text = newer_warning_text(&refs);
        assert!(text.starts_with("12 files newer at the destination, left: f1.txt"));
        assert!(text.contains("f10.txt"));
        assert!(!text.contains("f11.txt"));
        assert!(text.ends_with(", …and 2 more"));
    }

    #[test]
    fn test_newer_warning_text_cap_boundary() {
        let names: Vec<String> = (1..=11).map(|i| format!("f{i}.txt")).collect();
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        assert!(newer_warning_text(&refs[..10]).ends_with("f10.txt"));
        assert!(newer_warning_text(&refs).ends_with("f10.txt, …and 1 more"));
    }

    fn result_with_unmatched(names: &[(&str, bool)]) -> PipelineResult {
        let mut result = result_with_flags(&[]);
        for (name, unmatched) in names {
            let mut file = make_result_with_bytes(b"x", name).files.remove(0);
            if *unmatched {
                file.metadata.insert("unmatched".into(), 1.into());
            }
            result.files.push(file);
        }
        result
    }

    #[test]
    fn test_files_without_a_row_lists_flagged_files_in_order() {
        let r = result_with_unmatched(&[("a.wav", true), ("b.wav", false), ("c.wav", true)]);
        assert_eq!(files_without_a_row(&r), vec!["a.wav", "c.wav"]);
    }

    #[test]
    fn test_no_row_warning_text_singular_plural_and_cap() {
        assert_eq!(
            no_row_warning_text(&["a.wav"]),
            "1 file had no row in the lookup table: a.wav"
        );
        assert_eq!(
            no_row_warning_text(&["a.wav", "b.wav"]),
            "2 files had no row in the lookup table: a.wav, b.wav"
        );
        let names: Vec<String> = (1..=12).map(|i| format!("f{i}.wav")).collect();
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        assert!(no_row_warning_text(&refs).ends_with("f10.wav, …and 2 more"));
    }

    // --- write_results_with_mode tests ---

    #[test]
    fn test_write_mode_copies_files_to_output_dir() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let src_file = tmp.join("source.txt");
        std::fs::write(&src_file, b"hello").unwrap();

        let out_dir = tmp.join("output");
        let result = make_result_with_path(src_file.clone(), "output.txt");

        let outcome =
            write_results_with_mode(&result, "write", out_dir.to_str().unwrap(), false).unwrap();

        assert!(matches!(outcome, WriteOutcome::Written { count: 1, .. }));
        assert_eq!(std::fs::read(out_dir.join("output.txt")).unwrap(), b"hello");
        // Source still exists (write = copy semantics).
        assert!(src_file.exists());
    }

    #[test]
    fn test_overwrite_mode_renames_source_file_in_place() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let old_file = tmp.join("old-name.txt");
        std::fs::write(&old_file, b"important data").unwrap();

        let result = make_result_with_path(old_file.clone(), "new-name.txt");
        let outcome = write_results_with_mode(&result, "overwrite", "", false).unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(!old_file.exists(), "Old file should be gone");
        assert_eq!(
            std::fs::read(tmp.join("new-name.txt")).unwrap(),
            b"important data"
        );
    }

    #[test]
    fn test_overwrite_mode_noop_when_name_unchanged() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let file = tmp.join("same.txt");
        std::fs::write(&file, b"content").unwrap();

        let result = make_result_with_path(file.clone(), "same.txt");
        let outcome = write_results_with_mode(&result, "overwrite", "", false).unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(file.exists(), "File should still exist");
        assert_eq!(std::fs::read(&file).unwrap(), b"content");
    }

    #[test]
    fn test_overwrite_mode_errors_on_bytes_data() {
        let result = make_result_with_bytes(b"data", "anything.txt");
        let err = write_results_with_mode(&result, "overwrite", "", false).unwrap_err();
        assert!(
            err.contains("Overwrite mode requires FileData::Path"),
            "Error: {err}"
        );
    }

    #[test]
    fn test_message_mode_writes_no_files() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let result = make_result_with_bytes(b"data", "file.txt");
        let outcome =
            write_results_with_mode(&result, "message", tmp.to_str().unwrap(), false).unwrap();

        assert!(matches!(outcome, WriteOutcome::Message { .. }));
        // No files should have been written.
        let entries: Vec<_> = std::fs::read_dir(&tmp).unwrap().collect();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_none_mode_writes_no_files() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let result = make_result_with_bytes(b"data", "file.txt");
        let outcome =
            write_results_with_mode(&result, "none", tmp.to_str().unwrap(), false).unwrap();

        assert_eq!(outcome, WriteOutcome::None);
        let entries: Vec<_> = std::fs::read_dir(&tmp).unwrap().collect();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_overwrite_mode_strips_directory_from_filename() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        let src = tmp.join("original.txt");
        std::fs::write(&src, b"nested content").unwrap();

        // Pipeline output has a directory prefix — overwrite should only use the leaf.
        let result = make_result_with_path(src.clone(), "group/renamed.txt");
        let outcome = write_results_with_mode(&result, "overwrite", "", false).unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(!src.exists());
        assert_eq!(
            std::fs::read(tmp.join("renamed.txt")).unwrap(),
            b"nested content"
        );
    }

    #[test]
    fn test_dry_run_write_mode_writes_nothing_and_counts() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();
        let src = tmp.join("source.txt");
        std::fs::write(&src, b"hello").unwrap();
        let out_dir = tmp.join("output");
        let result = make_result_with_path(src, "output.txt");

        let outcome =
            write_results_with_mode(&result, "write", out_dir.to_str().unwrap(), true).unwrap();

        assert!(matches!(outcome, WriteOutcome::Written { count: 1, .. }));
        assert!(!out_dir.exists(), "dry run must not create the output dir");
    }

    // --- a file already at its destination (ISS-36) ---

    #[test]
    fn test_write_mode_into_the_source_folder_leaves_the_file_as_is() {
        let temp = tempfile::tempdir().unwrap();
        let src = temp.path().join("a.txt");
        std::fs::write(&src, b"important").unwrap();
        let result = make_result_with_path(src.clone(), "a.txt");

        let outcome =
            write_results_with_mode(&result, "write", temp.path().to_str().unwrap(), false)
                .unwrap();

        assert_eq!(std::fs::read(&src).unwrap(), b"important");
        assert!(
            matches!(
                outcome,
                WriteOutcome::Written {
                    count: 0,
                    unchanged: 1,
                    ..
                }
            ),
            "{outcome:?}"
        );
    }

    #[test]
    fn test_dry_run_write_mode_into_the_source_folder_counts_it_as_left() {
        let temp = tempfile::tempdir().unwrap();
        let src = temp.path().join("a.txt");
        std::fs::write(&src, b"important").unwrap();
        let result = make_result_with_path(src, "a.txt");

        let outcome =
            write_results_with_mode(&result, "write", temp.path().to_str().unwrap(), true).unwrap();

        assert!(
            matches!(
                outcome,
                WriteOutcome::Written {
                    count: 0,
                    unchanged: 1,
                    ..
                }
            ),
            "{outcome:?}"
        );
    }

    // --- output names that clash ---

    fn make_result_with_paths(files: &[(std::path::PathBuf, &str)]) -> PipelineResult {
        let mut result = make_result_with_bytes(b"", "unused");
        result.files = files
            .iter()
            .map(|(src, name)| make_result_with_path(src.clone(), name).files.remove(0))
            .collect();
        result
    }

    /// Two sources whose pipeline names differ only in case.
    fn case_twins(tmp: &Path) -> PipelineResult {
        let (a, b) = (tmp.join("a.svg"), tmp.join("b.svg"));
        std::fs::write(&a, b"first").unwrap();
        std::fs::write(&b, b"second").unwrap();
        make_result_with_paths(&[(a, "Logo.svg"), (b, "logo.svg")])
    }

    fn renamed(from: &str, to: &str) -> Renamed {
        Renamed {
            from: from.into(),
            to: to.into(),
            held: false,
        }
    }

    #[test]
    fn test_write_mode_keeps_both_files_whose_names_differ_only_in_case() {
        // Guards: on Windows the second copy replaced the first, while the
        // summary still counted both.
        let temp = tempfile::tempdir().unwrap();
        let out_dir = temp.path().join("output");
        let result = case_twins(temp.path());

        let outcome =
            write_results_with_mode(&result, "write", out_dir.to_str().unwrap(), false).unwrap();

        let WriteOutcome::Written {
            count, renamed: r, ..
        } = outcome
        else {
            panic!("expected Written, got {outcome:?}");
        };
        assert_eq!(r, [renamed("logo.svg", "logo_1.svg")]);
        assert_eq!(count, std::fs::read_dir(&out_dir).unwrap().count());
        assert_eq!(std::fs::read(out_dir.join("Logo.svg")).unwrap(), b"first");
        assert_eq!(
            std::fs::read(out_dir.join("logo_1.svg")).unwrap(),
            b"second"
        );
    }

    #[test]
    fn test_dry_run_write_mode_reports_the_new_name_and_writes_nothing() {
        let temp = tempfile::tempdir().unwrap();
        let out_dir = temp.path().join("output");
        let result = case_twins(temp.path());

        let outcome =
            write_results_with_mode(&result, "write", out_dir.to_str().unwrap(), true).unwrap();

        let expected = WriteOutcome::Written {
            dir: out_dir.to_str().unwrap().to_string(),
            count: 2,
            renamed: vec![renamed("logo.svg", "logo_1.svg")],
            unchanged: 0,
        };
        assert_eq!(outcome, expected);
        assert!(!out_dir.exists());
    }

    #[test]
    fn test_renamed_warning_text_names_each_file_and_its_new_name() {
        let one = [renamed("logo.svg", "logo_1.svg")];
        assert_eq!(
            renamed_warning_lines(&one, false),
            [
                "1 file written under a new name, since another has the same name \
              (case ignored): logo.svg → logo_1.svg"
            ]
        );
        let two = [renamed("a.svg", "a_1.svg"), renamed("A.svg", "A_2.svg")];
        assert_eq!(
            renamed_warning_lines(&two, true),
            [
                "2 files would be written under a new name, since another has the same \
              name (case ignored): a.svg → a_1.svg, A.svg → A_2.svg"
            ]
        );
    }

    #[test]
    fn test_a_held_file_is_named_as_left_as_is_not_written() {
        // RED: a held file was listed as "written under a new name" while
        // the summary said it was left as is.
        let held = Renamed {
            held: true,
            ..renamed("b.txt", "b_2.txt")
        };
        let both = [renamed("a.svg", "a_1.svg"), held];
        assert_eq!(
            renamed_warning_lines(&both, false),
            [
                "1 file written under a new name, since another has the same name \
                 (case ignored): a.svg → a_1.svg",
                "1 file already there under a new name, holding the same bytes, left \
                 as is: b.txt → b_2.txt",
            ]
        );
        assert_eq!(
            renamed_warning_lines(&both[1..], true),
            [
                "1 file already there under a new name, holding the same bytes, would \
              be left as is: b.txt → b_2.txt"
            ]
        );
    }

    #[test]
    fn test_dry_run_overwrite_mode_leaves_source_and_counts() {
        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();
        let old_file = tmp.join("old-name.txt");
        std::fs::write(&old_file, b"data").unwrap();
        let result = make_result_with_path(old_file.clone(), "new-name.txt");

        let outcome = write_results_with_mode(&result, "overwrite", "", true).unwrap();

        assert_eq!(outcome, WriteOutcome::Overwritten { count: 1 });
        assert!(old_file.exists(), "dry run must not rename the source");
        assert!(!tmp.join("new-name.txt").exists());
    }

    #[test]
    fn test_dry_run_overwrite_mode_still_errors_on_bytes_data() {
        let result = make_result_with_bytes(b"data", "anything.txt");
        let err = write_results_with_mode(&result, "overwrite", "", true).unwrap_err();
        assert!(
            err.contains("Overwrite mode requires FileData::Path"),
            "Error: {err}"
        );
    }

    // --- Existing tests ---

    #[test]
    fn test_read_pipeline_file_directory() {
        let temp = tempfile::Builder::new()
            .prefix("bnto-test-dir-input")
            .tempdir()
            .unwrap();
        let dir = temp.path().to_path_buf();

        let pf = read_pipeline_file(dir.to_str().unwrap()).unwrap();
        assert!(pf.name.contains("bnto-test-dir-input"));
        assert_eq!(pf.mime_type, "inode/directory");
        assert!(
            matches!(&pf.data, bnto_core::processor::FileData::Bytes(b) if b.is_empty()),
            "Directory input should have empty bytes"
        );
    }

    /// The Windows verbatim path prefix, as in `\\?\D:\_ToDelete`.
    const VERBATIM: &str = r"\\?\";

    #[test]
    fn test_directory_input_is_named_by_the_users_path_not_a_verbatim_one() {
        // Windows canonical paths carry the verbatim prefix, which the
        // progress line printed. The name is the plain absolute path.
        let temp = tempfile::tempdir().unwrap();

        let pf = read_pipeline_file(temp.path().to_str().unwrap()).unwrap();

        assert!(
            !pf.name.starts_with(VERBATIM),
            "verbatim prefix: {}",
            pf.name
        );
        assert!(Path::new(&pf.name).is_absolute(), "{}", pf.name);
        assert!(Path::new(&pf.name).is_dir(), "{}", pf.name);
    }

    #[test]
    fn test_relative_directory_input_is_named_by_its_absolute_path() {
        let pf = read_pipeline_file("src").unwrap();

        assert!(
            !pf.name.starts_with(VERBATIM),
            "verbatim prefix: {}",
            pf.name
        );
        assert!(Path::new(&pf.name).is_absolute(), "{}", pf.name);
        assert!(pf.name.ends_with("src"), "{}", pf.name);
    }

    #[test]
    fn test_read_pipeline_file_regular_file_is_a_path_reference() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("clip.jpg");
        std::fs::write(&path, b"twelve bytes").unwrap();

        let pf = read_pipeline_file(path.to_str().unwrap()).unwrap();

        assert_eq!(pf.name, "clip.jpg");
        assert_eq!(pf.mime_type, "image/jpeg");
        assert_eq!(pf.data, FileData::Path(path));
        assert_eq!(pf.metadata[bnto_core::INPUT_SIZE_KEY], 12);
    }

    #[test]
    fn test_read_pipeline_file_missing_file_errors() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("gone.jpg");
        assert!(read_pipeline_file(path.to_str().unwrap()).is_err());
    }

    #[test]
    fn test_bytes_node_reads_an_argument_file_reference() {
        let fixture =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../test-fixtures/images/small.jpg");
        let recipe = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../bnto-engine/recipes/compress-images.bnto.json");
        let pf = read_pipeline_file(fixture.to_str().unwrap()).unwrap();
        assert!(matches!(pf.data, FileData::Path(_)));

        let result = bnto_engine::run_pipeline(
            &std::fs::read_to_string(recipe).unwrap(),
            vec![pf],
            &bnto_core::PipelineReporter::new_noop(),
            &bnto_core::NoopContext,
        )
        .unwrap();

        let bytes = result.files[0].data.to_bytes().unwrap();
        assert_eq!(&bytes[..3], &[0xFF, 0xD8, 0xFF], "JPEG magic bytes");
    }

    #[test]
    fn test_guess_mime_jpeg() {
        assert_eq!(guess_mime("photo.jpg"), "image/jpeg");
        assert_eq!(guess_mime("photo.jpeg"), "image/jpeg");
        assert_eq!(guess_mime("photo.JPG"), "image/jpeg");
    }

    #[test]
    fn test_guess_mime_png() {
        assert_eq!(guess_mime("image.png"), "image/png");
    }

    #[test]
    fn test_guess_mime_csv() {
        assert_eq!(guess_mime("data.csv"), "text/csv");
    }

    #[test]
    fn test_guess_mime_unknown() {
        assert_eq!(guess_mime("file.xyz"), "application/octet-stream");
        assert_eq!(guess_mime("noext"), "application/octet-stream");
    }

    #[test]
    fn test_write_results_preserves_source_for_path_refs() {
        use bnto_core::PipelineFileResult;
        use bnto_core::processor::FileData;

        let temp = tempfile::tempdir().unwrap();
        let tmp = temp.path().to_path_buf();

        // Create a source file that the pipeline references by path.
        let src_file = tmp.join("original.txt");
        std::fs::write(&src_file, b"keep me").unwrap();

        let out_dir = tmp.join("output");
        let result = PipelineResult {
            files: vec![PipelineFileResult {
                name: "renamed.txt".to_string(),
                data: FileData::Path(src_file.clone()),
                mime_type: "text/plain".to_string(),
                metadata: serde_json::Map::new(),
            }],
            duration_ms: 0,
            warnings: vec![],
            node_metadata: Default::default(),
        };

        write_results(&result, out_dir.to_str().unwrap()).unwrap();

        // Output file exists with correct content.
        assert_eq!(
            std::fs::read(out_dir.join("renamed.txt")).unwrap(),
            b"keep me"
        );
        // Source file is preserved — not moved or deleted.
        assert!(src_file.exists(), "Source file must survive write_results");
    }

    #[test]
    fn test_write_results_creates_subdirectories() {
        use bnto_core::PipelineFileResult;
        use bnto_core::processor::FileData;

        // write_results must create the output dir itself, so it starts absent.
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().join("out");

        let result = PipelineResult {
            files: vec![
                PipelineFileResult {
                    name: "top.txt".to_string(),
                    data: FileData::Bytes(b"top-level".to_vec()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                },
                PipelineFileResult {
                    name: "group/nested.mp4".to_string(),
                    data: FileData::Bytes(b"video-data".to_vec()),
                    mime_type: "video/mp4".to_string(),
                    metadata: serde_json::Map::new(),
                },
                PipelineFileResult {
                    name: "a/b/deep.txt".to_string(),
                    data: FileData::Bytes(b"deep-data".to_vec()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                },
            ],
            duration_ms: 0,
            warnings: vec![],
            node_metadata: Default::default(),
        };

        write_results(&result, dir.to_str().unwrap()).unwrap();

        assert!(dir.join("top.txt").exists());
        assert!(dir.join("group").join("nested.mp4").exists());
        assert!(dir.join("a").join("b").join("deep.txt").exists());
        assert_eq!(
            std::fs::read(dir.join("group/nested.mp4")).unwrap(),
            b"video-data"
        );
    }
}
