// Progressive output — copies a loop iteration's files to the output
// directory as soon as the iteration completes. Shared by the CLI reporter
// and the TUI so both write progressive files the same way.

use std::path::Path;

use bnto_core::events::ProgressOutputFile;

use crate::ordered_write::{GATE, WriteGate};
use crate::output_names::OutputNames;

/// Copy an iteration's output files into `output_dir`, creating parent
/// folders for names with subdirectories. `names` spans the whole run, so a
/// name an earlier iteration wrote isn't replaced, and each file lands where
/// the final write puts it. A made-up name whose file already holds the
/// bytes (`held`) is left as is, here and in the final write. Any other
/// destination that already holds a file is left for the final write: it may
/// be a later iteration's source, not read yet (ISS-48), and only the final
/// write sees every source. Errors are ignored: the final write after the run
/// writes every file again (all but the held ones) and reports failures.
/// Each copy goes through `ordered_write::GATE`, so Ctrl+C or quitting the
/// TUI waits for the file being copied and copies no more.
pub fn write_progressive_files(
    output_dir: &str,
    files: &[ProgressOutputFile],
    names: &mut OutputNames,
) {
    write_progressive_through(&GATE, Path::new(output_dir), files, names);
}

/// `write_progressive_files` through `gate`.
fn write_progressive_through(
    gate: &WriteGate,
    dir: &Path,
    files: &[ProgressOutputFile],
    names: &mut OutputNames,
) {
    let Some(_writing) = gate.enter() else { return };
    for file in files {
        if gate.stopping() {
            return;
        }
        copy_one(dir, file, names);
    }
}

/// Copy one file to its claimed name, unless the name is held or occupied.
fn copy_one(dir: &Path, file: &ProgressOutputFile, names: &mut OutputNames) {
    let claimed = names.claim(dir, &file.name, &file.data);
    let path = dir.join(claimed.name);
    if claimed.held || std::fs::symlink_metadata(&path).is_ok() {
        return;
    }
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    // Copy, never rename: a Path output may be the user's own file, and the
    // final write reads it again.
    if file.data.copy_to(&path).is_ok() {
        crate::ordered_write::note_progressive(&path);
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
    fn leaves_a_made_up_names_file_already_holding_the_bytes_as_is() {
        // RED: deleting the `held` skip copied over the identical `b_1.txt`,
        // giving it a new mtime. (`Bytes` data is written with `fs::write`,
        // which keeps the mode, so only the mtime check proves this.)
        let out = tempfile::tempdir().unwrap();
        let held = out.path().join("b_1.txt");
        std::fs::write(&held, "BBBB").unwrap();
        let old = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_577_836_800);
        let file = std::fs::File::options().write(true).open(&held).unwrap();
        file.set_modified(old).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&held, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        let mut names = OutputNames::default();
        let dir = out.path().to_str().unwrap();
        for body in ["AAAA", "BBBB"] {
            let files = [output("b.txt", FileData::Bytes(body.into()))];
            write_progressive_files(dir, &files, &mut names);
        }

        let meta = std::fs::metadata(&held).unwrap();
        assert_eq!(meta.modified().unwrap(), old, "b_1.txt was rewritten");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(meta.permissions().mode() & 0o777, 0o600);
        }
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 2);
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
    fn leaves_a_file_already_at_a_destination_for_the_final_write() {
        // RED (ISS-48): iteration 1 renames `a` to `b.txt`; copying it then
        // replaced `b.txt` before iteration 2 read it. A later iteration's
        // source can't be known yet, so an occupied destination waits for
        // the final write, which orders its writes (see `ordered_write`).
        let out = tempfile::tempdir().unwrap();
        let (a, b) = (out.path().join("a.txt"), out.path().join("b.txt"));
        std::fs::write(&a, "AAAA").unwrap();
        std::fs::write(&b, "BBBB").unwrap();
        let dir = out.path().to_str().unwrap();
        let mut names = OutputNames::default();

        write_progressive_files(dir, &[output("b.txt", FileData::Path(a))], &mut names);
        assert_eq!(std::fs::read_to_string(&b).unwrap(), "BBBB");

        write_progressive_files(dir, &[output("b.txt", FileData::Path(b))], &mut names);
        let written = out.path().join("b_1.txt");
        assert_eq!(std::fs::read_to_string(written).unwrap(), "BBBB");
    }

    #[test]
    fn copies_nothing_once_bnto_is_quitting() {
        // Progressive copies used to skip the write gate, so Ctrl+C or a TUI
        // quit could end the process mid-copy; now a quitting gate stops them.
        let out = tempfile::tempdir().unwrap();
        let gate = WriteGate::new();
        gate.stop_and_wait(|| {});
        let files = [output("a.txt", FileData::Bytes(b"A".to_vec()))];
        write_progressive_through(&gate, out.path(), &files, &mut OutputNames::default());
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 0);
    }

    #[test]
    fn a_held_name_and_an_occupied_name_are_both_skipped() {
        // #759's skip (a made-up name already holding the bytes) and ISS-48's
        // (any file already at the name) both apply.
        let out = tempfile::tempdir().unwrap();
        std::fs::write(out.path().join("a.txt"), "USER").unwrap();
        std::fs::write(out.path().join("a_1.txt"), "SAME").unwrap();
        let dir = out.path().to_str().unwrap();
        let files = [
            output("a.txt", FileData::Bytes(b"NEW".to_vec())),
            output("a.txt", FileData::Bytes(b"SAME".to_vec())),
        ];
        write_progressive_files(dir, &files, &mut OutputNames::default());
        assert_eq!(
            std::fs::read_to_string(out.path().join("a.txt")).unwrap(),
            "USER"
        );
        assert_eq!(
            std::fs::read_to_string(out.path().join("a_1.txt")).unwrap(),
            "SAME"
        );
        assert_eq!(std::fs::read_dir(out.path()).unwrap().count(), 2);
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
