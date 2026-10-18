// Progressive output — copies a loop iteration's files to the output
// directory as soon as the iteration completes. Shared by the CLI reporter
// and the TUI so both write progressive files the same way.

use std::path::Path;

use bnto_core::events::ProgressOutputFile;

/// Copy an iteration's output files into `output_dir`, creating parent
/// folders for names with subdirectories. Errors are ignored: the final
/// write after the run writes every file again and reports failures.
pub fn write_progressive_files(output_dir: &str, files: &[ProgressOutputFile]) {
    for file in files {
        let path = Path::new(output_dir).join(&file.name);
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
        write_progressive_files(out.path().to_str().unwrap(), &files);

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
        write_progressive_files(out.path().to_str().unwrap(), &files);

        assert_eq!(
            std::fs::read(out.path().join("sub").join("a.txt")).unwrap(),
            b"nested"
        );
    }
}
