// Progressive output — copies a loop iteration's files to the output
// directory as soon as the iteration completes. Shared by the CLI reporter
// and the TUI so both write progressive files the same way.

use std::path::Path;

use bnto_core::events::ProgressOutputFile;

use crate::output_names::OutputNames;

/// Copy an iteration's output files into `output_dir`, creating parent
/// folders for names with subdirectories. `names` spans the whole run, so a
/// name an earlier iteration wrote isn't replaced, and each file lands where
/// the final write puts it. A made-up name whose file already holds the
/// bytes (`held`) is left as is. Errors are ignored: the final write after
/// the run writes every file again and reports failures.
pub fn write_progressive_files(
    output_dir: &str,
    files: &[ProgressOutputFile],
    names: &mut OutputNames,
) {
    for file in files {
        let dir = Path::new(output_dir);
        let claimed = names.claim(dir, &file.name, &file.data);
        if claimed.held {
            continue;
        }
        let path = dir.join(claimed.name);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        // Copy, never rename: a Path output may be the user's own file,
        // and the final write reads it again.
        let _ = file.data.copy_to(&path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::processor::FileData;
    use bnto_core::{PipelineFileResult, PipelineResult};

    fn output(name: &str, data: FileData) -> ProgressOutputFile {
        ProgressOutputFile {
            name: name.to_string(),
            data,
            mime_type: "text/plain".to_string(),
        }
    }

    #[test]
    fn copies_path_files_and_keeps_the_source() {
        let src_dir = tempfile::tempdir().unwrap();
        let src = src_dir.path().join("mine.txt");
        std::fs::write(&src, b"user file").unwrap();
        let out = tempfile::tempdir().unwrap();

        let files = [output("mine.txt", FileData::Path(src.clone()))];
        write_progressive_files(
            out.path().to_str().unwrap(),
            &files,
            &mut OutputNames::default(),
        );

        assert_eq!(std::fs::read(&src).unwrap(), b"user file");
        assert_eq!(
            std::fs::read(out.path().join("mine.txt")).unwrap(),
            b"user file"
        );
    }

    #[test]
    fn creates_subdirectories_for_nested_names() {
        let out = tempfile::tempdir().unwrap();

        let files = [output("sub/a.txt", FileData::Bytes(b"nested".to_vec()))];
        write_progressive_files(
            out.path().to_str().unwrap(),
            &files,
            &mut OutputNames::default(),
        );

        assert_eq!(
            std::fs::read(out.path().join("sub").join("a.txt")).unwrap(),
            b"nested"
        );
    }

    #[test]
    fn keeps_apart_names_from_separate_iterations_and_matches_the_final_write() {
        // Guards: optimize-svg writes each file as its iteration ends, so a
        // case twin replaced the earlier file before the final write ran.
        let out = tempfile::tempdir().unwrap();
        let dir = out.path().to_str().unwrap();
        let mut names = OutputNames::default();
        let iterations = [("Logo.svg", "first"), ("logo.svg", "second")];
        for (name, body) in iterations {
            let files = [output(name, FileData::Bytes(body.into()))];
            write_progressive_files(dir, &files, &mut names);
        }
        assert_eq!(
            std::fs::read(out.path().join("logo_1.svg")).unwrap(),
            b"second"
        );

        // A loop that ends the recipe hands the final write its own files.
        crate::io::write_results(&text_result(&iterations), dir).unwrap();
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 2);
    }

    #[test]
    fn a_step_after_the_loop_leaves_the_loops_suffixed_copy() {
        // Pins a deliberate change: when a step after the loop changes its
        // files, the final write finds the loop's `logo_1.svg` holding other
        // bytes and can't tell it from a user's file, so it keeps it and
        // writes `logo_2.svg`. It used to replace it (2 files, not 3).
        let out = tempfile::tempdir().unwrap();
        let dir = out.path().to_str().unwrap();
        let mut names = OutputNames::default();
        for (name, body) in [("Logo.svg", "first"), ("logo.svg", "second")] {
            let files = [output(name, FileData::Bytes(body.into()))];
            write_progressive_files(dir, &files, &mut names);
        }
        let changed = [("Logo.svg", "final-1"), ("logo.svg", "final-2")];
        crate::io::write_results(&text_result(&changed), dir).unwrap();

        let read = |name: &str| std::fs::read_to_string(out.path().join(name)).unwrap();
        assert_eq!(read("Logo.svg"), "final-1");
        assert_eq!(read("logo_1.svg"), "second");
        assert_eq!(read("logo_2.svg"), "final-2");
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 3);
    }

    /// A pipeline result of text files, each `(name, body)`.
    fn text_result(files: &[(&str, &str)]) -> PipelineResult {
        PipelineResult {
            files: files
                .iter()
                .map(|(name, body)| PipelineFileResult {
                    name: name.to_string(),
                    data: FileData::Bytes(body.as_bytes().to_vec()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                })
                .collect(),
            duration_ms: 0,
            warnings: vec![],
            node_metadata: Default::default(),
        }
    }

    #[test]
    fn a_made_up_name_skips_a_users_file_and_matches_the_final_write() {
        // RED (data loss): the second `b.txt` was copied over the user's
        // `b_1.txt`. Now it goes to `b_2.txt`, and the final write, which
        // finds that copy already there, picks `b_2.txt` too: no extra file.
        let out = tempfile::tempdir().unwrap();
        let dir = out.path().to_str().unwrap();
        std::fs::write(out.path().join("b_1.txt"), "CCCC").unwrap();
        let iterations = [("b.txt", "AAAA"), ("b.txt", "BBBB")];
        let mut names = OutputNames::default();
        for (name, body) in iterations {
            let files = [output(name, FileData::Bytes(body.into()))];
            write_progressive_files(dir, &files, &mut names);
        }
        crate::io::write_results(&text_result(&iterations), dir).unwrap();

        let read = |name: &str| std::fs::read_to_string(out.path().join(name)).unwrap();
        assert_eq!(read("b_1.txt"), "CCCC");
        assert_eq!(
            (read("b.txt"), read("b_2.txt")),
            ("AAAA".into(), "BBBB".into())
        );
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 3);
    }
}
