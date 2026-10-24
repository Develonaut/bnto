// Progressive output — copies a loop iteration's files to the output
// directory as soon as the iteration completes. Shared by the CLI reporter
// and the TUI so both write progressive files the same way.

use std::path::Path;

use bnto_core::events::ProgressOutputFile;

use crate::output_names::OutputNames;

/// Copy an iteration's output files into `output_dir`, creating parent
/// folders for names with subdirectories. `names` spans the whole run, so a
/// name an earlier iteration wrote isn't replaced, and each file lands where
/// the final write puts it. Errors are ignored: the final write after the run
/// writes every file again and reports failures.
pub fn write_progressive_files(
    output_dir: &str,
    files: &[ProgressOutputFile],
    names: &mut OutputNames,
) {
    for file in files {
        let dir = Path::new(output_dir);
        let path = dir.join(names.claim(dir, &file.name, &file.data));
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

        // The final write gets the iterations' own files, as a run does.
        let result = PipelineResult {
            files: iterations
                .into_iter()
                .map(|(name, body)| PipelineFileResult {
                    name: name.to_string(),
                    data: FileData::Bytes(body.into()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                })
                .collect(),
            duration_ms: 0,
            warnings: vec![],
            node_metadata: Default::default(),
        };
        crate::io::write_results(&result, dir).unwrap();
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 2);
    }

    #[test]
    fn a_made_up_name_skips_a_users_file_and_matches_the_final_write() {
        // RED (data loss): the second `b.txt` was copied over the user's
        // `b_1.txt`. Now it goes to `b_2.txt`, and the final write, which
        // finds that copy already there, picks `b_2.txt` too: no extra file.
        let out = tempfile::tempdir().unwrap();
        let dir = out.path().to_str().unwrap();
        std::fs::write(out.path().join("b_1.txt"), "CCCC").unwrap();
        let bodies = ["AAAA", "BBBB"];
        let mut names = OutputNames::default();
        for body in bodies {
            let files = [output("b.txt", FileData::Bytes(body.into()))];
            write_progressive_files(dir, &files, &mut names);
        }
        let result = PipelineResult {
            files: bodies
                .into_iter()
                .map(|body| PipelineFileResult {
                    name: "b.txt".to_string(),
                    data: FileData::Bytes(body.into()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                })
                .collect(),
            duration_ms: 0,
            warnings: vec![],
            node_metadata: Default::default(),
        };
        crate::io::write_results(&result, dir).unwrap();

        let read = |name: &str| std::fs::read_to_string(out.path().join(name)).unwrap();
        assert_eq!(read("b_1.txt"), "CCCC");
        assert_eq!(
            (read("b.txt"), read("b_2.txt")),
            ("AAAA".into(), "BBBB".into())
        );
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 3);
    }
}
