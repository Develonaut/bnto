// file-delete against the rest of the run (ISS-82): a batch that would delete
// a file a later step still reads (another loop iteration's, a branch's) is
// refused before anything is deleted.

use super::time_tests::{run, three_files};
use crate::held_by::HeldBy;

#[test]
fn test_a_delete_of_a_file_a_later_step_reads_is_refused_before_any_delete() {
    let tmp = three_files();
    let base = tmp.path().join("base");
    let c = base.join("c.txt").canonicalize().unwrap();
    let ctx = HeldBy::file(&c, "c.txt");

    let err = run(&base, &["a.txt", "b.txt", "c.txt"], &ctx)
        .err()
        .expect("refused");

    let text = err.to_string();
    assert!(
        text.contains("File delete refused, nothing deleted"),
        "{text}"
    );
    assert!(
        text.contains("a later step still reads it as c.txt"),
        "{text}"
    );
    for name in ["a.txt", "b.txt", "c.txt"] {
        assert!(base.join(name).exists(), "{name} never deleted");
    }
}

#[test]
fn test_a_missing_item_is_not_a_delete() {
    let tmp = three_files();
    let base = tmp.path().join("base");
    let gone = base.join("gone.txt");
    let ctx = HeldBy::file(&gone, "gone.txt");

    let deleted = run(&base, &["gone.txt"], &ctx);

    assert!(deleted.is_ok(), "nothing is deleted, so nothing is taken");
}
