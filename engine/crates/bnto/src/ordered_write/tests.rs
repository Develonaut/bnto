use super::*;

fn result(name: &str, data: FileData) -> PipelineFileResult {
    PipelineFileResult {
        name: name.to_string(),
        data,
        mime_type: "text/plain".to_string(),
        metadata: serde_json::Map::new(),
    }
}

/// A folder holding each `(name, content)` file, and the files' paths.
fn folder(files: &[(&str, &str)]) -> (tempfile::TempDir, Vec<PathBuf>) {
    let dir = tempfile::tempdir().unwrap();
    let paths = files
        .iter()
        .map(|(name, body)| {
            let path = dir.path().join(name);
            std::fs::write(&path, body).unwrap();
            path
        })
        .collect();
    (dir, paths)
}

/// Results reading each source, in order.
fn reading(sources: &[PathBuf]) -> Vec<PipelineFileResult> {
    sources
        .iter()
        .map(|s| result("unused", FileData::Path(s.clone())))
        .collect()
}

fn read(dir: &Path, name: &str) -> String {
    std::fs::read_to_string(dir.join(name)).unwrap()
}

fn names_in(dir: &Path) -> usize {
    std::fs::read_dir(dir).unwrap().count()
}

#[test]
fn test_a_write_onto_a_later_source_reads_that_source_first() {
    // RED (ISS-48): a → b.txt ran first and replaced b.txt before b → b_1.txt
    // read it, so BBBB was lost.
    let (dir, src) = folder(&[("a.txt", "AAAA"), ("b.txt", "BBBB")]);
    let paths = [dir.path().join("b.txt"), dir.path().join("b_1.txt")];
    write_all(&reading(&src), &paths, dir.path()).unwrap();
    assert_eq!(read(dir.path(), "a.txt"), "AAAA");
    assert_eq!(read(dir.path(), "b.txt"), "AAAA");
    assert_eq!(read(dir.path(), "b_1.txt"), "BBBB");
}

#[test]
fn test_a_swap_swaps_and_leaves_no_staging_folder() {
    // RED: a → b.txt and b → a.txt; either order alone loses one file.
    let (dir, src) = folder(&[("a.txt", "AAAA"), ("b.txt", "BBBB")]);
    let paths = [src[1].clone(), src[0].clone()];
    write_all(&reading(&src), &paths, dir.path()).unwrap();
    assert_eq!(read(dir.path(), "a.txt"), "BBBB");
    assert_eq!(read(dir.path(), "b.txt"), "AAAA");
    assert_eq!(names_in(dir.path()), 2, "the staging folder is removed");
}

#[test]
fn test_a_three_way_rotation_keeps_every_file() {
    // RED: a → b.txt, b → c.txt, c → a.txt.
    let (dir, src) = folder(&[("a.txt", "A"), ("b.txt", "B"), ("c.txt", "C")]);
    let paths = [src[1].clone(), src[2].clone(), src[0].clone()];
    write_all(&reading(&src), &paths, dir.path()).unwrap();
    assert_eq!(read(dir.path(), "a.txt"), "C");
    assert_eq!(read(dir.path(), "b.txt"), "A");
    assert_eq!(read(dir.path(), "c.txt"), "B");
}

#[test]
fn test_a_case_only_swap_keeps_both_contents() {
    // RED on a case-insensitive filesystem: a → `B.txt` is a write onto
    // b.txt, which b → a.txt still reads. On a case-sensitive one `B.txt` is
    // a new file and only the a.txt write has to wait. Both keep each file.
    let (dir, src) = folder(&[("a.txt", "AAAA"), ("b.txt", "BBBB")]);
    let paths = [dir.path().join("B.txt"), src[0].clone()];
    write_all(&reading(&src), &paths, dir.path()).unwrap();
    assert_eq!(read(dir.path(), "B.txt"), "AAAA");
    assert_eq!(read(dir.path(), "a.txt"), "BBBB");
}

#[test]
fn test_bytes_and_untouched_sources_write_as_before() {
    // Guard: no waiting, no staging when nothing reads a destination.
    let (dir, src) = folder(&[("a.txt", "AAAA")]);
    let out = tempfile::tempdir().unwrap();
    let files = [
        result("a.txt", FileData::Path(src[0].clone())),
        result("new.txt", FileData::Bytes(b"new".to_vec())),
    ];
    let paths = [
        out.path().join("a.txt"),
        out.path().join("sub").join("new.txt"),
    ];
    write_all(&files, &paths, out.path()).unwrap();
    assert_eq!(read(out.path(), "a.txt"), "AAAA");
    assert_eq!(read(&out.path().join("sub"), "new.txt"), "new");
    assert_eq!(read(dir.path(), "a.txt"), "AAAA");
    assert_eq!(names_in(out.path()), 2);
}

/// The `.bnto-staged-*` folders in `dir`.
fn staging_folders(dir: &Path) -> Vec<PathBuf> {
    let entries = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path());
    let staged = |p: &PathBuf| {
        let name = p.file_name().unwrap().to_string_lossy().into_owned();
        name.starts_with(bnto_file::STAGED_COPIES_PREFIX)
    };
    entries.filter(staged).collect()
}

#[test]
fn test_a_write_failing_after_staging_keeps_the_staged_copy_and_names_it() {
    // The swap stages b.txt, writes a → b.txt, then fails writing the staged
    // b → a.txt (read-only). The staged copy is now the only BBBB: it must
    // survive, and the error must say where it is.
    let (dir, src) = folder(&[("a.txt", "AAAA"), ("b.txt", "BBBB")]);
    let mut readonly = std::fs::metadata(&src[0]).unwrap().permissions();
    readonly.set_readonly(true);
    std::fs::set_permissions(&src[0], readonly).unwrap();
    let paths = [src[1].clone(), src[0].clone()];

    let error = write_all(&reading(&src), &paths, dir.path()).unwrap_err();

    let folders = staging_folders(dir.path());
    assert_eq!(folders.len(), 1, "{error}");
    assert_eq!(
        std::fs::read_to_string(folders[0].join("0-b.txt")).unwrap(),
        "BBBB"
    );
    assert!(error.contains("Cannot write"), "{error}");
    assert!(error.contains(&folders[0].display().to_string()), "{error}");
    assert_eq!(read(dir.path(), "a.txt"), "AAAA");
    assert_eq!(read(dir.path(), "b.txt"), "AAAA");
    allow_removal(&src[0]);
}

/// Windows can't delete a read-only file, so the temp dir couldn't clean up.
#[cfg(windows)]
#[allow(clippy::permissions_set_readonly_false)]
fn allow_removal(path: &Path) {
    let mut writable = std::fs::metadata(path).unwrap().permissions();
    writable.set_readonly(false);
    std::fs::set_permissions(path, writable).unwrap();
}

#[cfg(not(windows))]
fn allow_removal(_: &Path) {}

#[test]
fn test_destinations_that_are_one_file_are_refused_before_any_write() {
    // l.txt is a hard link to b.txt, which b's write leaves as is.
    let (dir, src) = folder(&[("a.txt", "AAAA"), ("b.txt", "BBBB")]);
    let link = dir.path().join("l.txt");
    std::fs::hard_link(&src[1], &link).unwrap();
    let paths = [link, src[1].clone()];

    let error = write_all(&reading(&src), &paths, dir.path()).unwrap_err();

    assert!(
        error.starts_with("Output refused, nothing was written: 1 conflict"),
        "{error}"
    );
    assert!(error.contains("are one file on disk"), "{error}");
    assert_eq!(read(dir.path(), "b.txt"), "BBBB");
    assert_eq!(read(dir.path(), "l.txt"), "BBBB");
}

// --- a quitting process stops writes between steps ---

#[test]
fn test_a_write_after_quitting_writes_nothing() {
    let (dir, src) = folder(&[("a.txt", "AAAA"), ("b.txt", "BBBB")]);
    let gate = WriteGate::new();
    gate.stop_and_wait();
    let paths = [src[1].clone(), src[0].clone()];

    let error = write_through(&gate, &reading(&src), &paths, dir.path()).unwrap_err();

    assert_eq!(error, STOPPED);
    assert_eq!(read(dir.path(), "a.txt"), "AAAA");
    assert_eq!(read(dir.path(), "b.txt"), "BBBB");
    assert_eq!(names_in(dir.path()), 2, "no staging folder");
}

#[test]
fn test_a_swap_stopped_between_steps_keeps_the_only_copy_for_the_report() {
    // What a stop after `Write(0)` of a swap leaves: b.txt already holds
    // a's content, so the staged copy is the only BBBB. Dropping the run (as
    // `write_through` does on a stop) must keep it and leave it registered.
    let (dir, src) = folder(&[("a.txt", "AAAA"), ("b.txt", "BBBB")]);
    let paths = [src[1].clone(), src[0].clone()];
    let files = reading(&src);
    let mut run = Run::new(&files, &paths, dir.path());
    let steps = order::order(&readers::waits(&files, &paths));
    for step in steps.into_iter().take(2) {
        run.step(step).unwrap();
    }
    drop(run);

    let folders = staging_folders(dir.path());
    assert_eq!(folders.len(), 1);
    assert_eq!(read(&folders[0], "0-b.txt"), "BBBB");
    assert_eq!(read(dir.path(), "b.txt"), "AAAA");
    assert!(staging::is_registered(&folders[0]));
    staging::forget(&folders[0]);
}
