// The check on a run's result names before the output write plans anything.
// A result's name is its path under the output folder, and a recipe builds
// names (`file-render`'s `filename`, a `file-rename`), so one that would
// leave the folder (ISS-128), or holds a part this OS can't hold as a name
// (`CON`, a trailing dot or space on Windows, as `file-move` and overwrite
// mode refuse), refuses the write. Shared by the CLI and the TUI.

use std::path::Path;

use bnto_core::PipelineResult;
use bnto_file::relocate;

use crate::ordered_write;

/// Fail on a result with no name, or one the output folder can't take
/// (`bad_name`), for a real write and a dry run alike, before anything is
/// written. Files a progressive loop already wrote into `output_dir` are
/// named, since the run did write them.
pub fn check_result_names(result: &PipelineResult, output_dir: &Path) -> Result<(), String> {
    if result.files.iter().any(|f| f.name.is_empty()) {
        return Err(
            "Pipeline produced a file with an empty name — cannot write to output directory"
                .to_string(),
        );
    }
    let bad: Vec<String> = result
        .files
        .iter()
        .filter_map(|f| bad_name(&f.name))
        .collect();
    if bad.is_empty() {
        return Ok(());
    }
    let listing = relocate::listing(&bad, ("name", "names"));
    Err(ordered_write::refusal_in(&listing, output_dir))
}

/// Whether a progressive copy may write `name`: one the final write would
/// refuse is skipped, leaving the refusal to it.
pub fn may_write(name: &str) -> bool {
    bad_name(name).is_none()
}

/// Why the output folder can't take `name`, or `None`; escaped where it's
/// built (`name_refusal`).
fn bad_name(name: &str) -> Option<String> {
    relocate::name_refusal(name, "the output folder")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_text::display_text;
    use bnto_core::PipelineFileResult;
    use bnto_core::processor::FileData;

    fn named(names: &[&str]) -> PipelineResult {
        PipelineResult {
            files: names
                .iter()
                .map(|name| PipelineFileResult {
                    name: name.to_string(),
                    data: FileData::Bytes(b"x".to_vec()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                })
                .collect(),
            duration_ms: 0,
            warnings: vec![],
            node_metadata: Default::default(),
        }
    }

    fn refusal(names: &[&str]) -> String {
        let dir = tempfile::tempdir().unwrap();
        check_result_names(&named(names), dir.path()).unwrap_err()
    }

    #[test]
    fn plain_and_dotted_names_pass() {
        let dir = tempfile::tempdir().unwrap();
        let ok = named(&["a.txt", "./sub/..hidden.txt", "sub//b.txt"]);
        assert!(check_result_names(&ok, dir.path()).is_ok());
    }

    #[test]
    fn a_name_that_leaves_is_refused_and_nothing_was_written() {
        let err = refusal(&["ok.txt", "../x.txt", "/abs.txt"]);
        assert!(
            err.starts_with("Output refused, nothing was written"),
            "{err}"
        );
        assert!(err.contains("\"../x.txt\" would leave"), "{err}");
        assert!(err.contains("\"/abs.txt\" would leave"), "{err}");
        assert!(!err.contains("ok.txt"), "{err}");
    }

    #[test]
    fn a_name_of_only_dot_segments_is_refused() {
        assert!(refusal(&["./"]).contains("has no file name"));
    }

    #[test]
    fn a_name_this_os_cant_hold_is_refused() {
        let err = refusal(&["sub/a\0b.txt"]);
        assert!(err.contains("\"a\\0b.txt\""), "{err}");
    }

    #[cfg(windows)]
    #[test]
    fn a_windows_device_or_trailing_dot_name_is_refused_on_windows() {
        for name in ["sub/CON.txt", "sub/COM1", "a./b.txt", "x/.. /y.txt"] {
            assert!(refusal(&[name]).contains("Output refused"), "{name}");
        }
    }

    #[test]
    fn a_progressive_loops_files_are_named_in_the_refusal() {
        let dir = tempfile::tempdir().unwrap();
        let written = dir.path().join("a.txt");
        ordered_write::note_progressive(&written);
        let err = check_result_names(&named(&["../b.txt"]), dir.path()).unwrap_err();
        ordered_write::forget_progressive(dir.path());
        assert!(err.contains("the final write wrote nothing"), "{err}");
        assert!(err.contains("Loop iterations already wrote"), "{err}");
        assert!(err.contains(&written.display().to_string()), "{err}");
    }

    #[test]
    fn a_name_is_escaped_once() {
        let err = refusal(&["../a\nb\\c"]);
        assert!(err.contains("\"../a\\nb\\c\""), "{err}");
        let again = |line: &str| display_text(line) == line;
        assert!(err.lines().all(again), "a second pass changed it: {err}");
    }

    #[test]
    fn progressive_copies_skip_what_the_final_write_refuses() {
        assert!(may_write("sub/a.txt"));
        assert!(!may_write("../a.txt"));
        assert!(!may_write("a\0b"));
    }
}
