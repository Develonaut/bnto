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
        let path = Path::new(output_dir).join(names.claim(&file.name));
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
        for (name, body) in [("Logo.svg", "first"), ("logo.svg", "second")] {
            let files = [output(name, FileData::Bytes(body.into()))];
            write_progressive_files(dir, &files, &mut names);
        }
        assert_eq!(
            std::fs::read(out.path().join("logo_1.svg")).unwrap(),
            b"second"
        );

        let result = PipelineResult {
            files: ["Logo.svg", "logo.svg"]
                .into_iter()
                .map(|name| PipelineFileResult {
                    name: name.to_string(),
                    data: FileData::Bytes(b"final".to_vec()),
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
}
