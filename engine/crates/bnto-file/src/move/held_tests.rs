// file-move against the rest of the run (ISS-82): a batch that would move a
// file a later step still reads (another loop iteration's, a branch's) is
// refused before anything moves.

use super::time_tests::{dir_with, read, run};
use crate::held_by::HeldBy;

const TWO: [(&str, &str); 2] = [("a.wav", "Pack/a.wav"), ("b.wav", "Pack/b.wav")];

#[test]
fn test_a_move_of_a_file_a_later_step_reads_is_refused_before_any_move() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    let b = tmp.path().join("b.wav").canonicalize().unwrap();
    let ctx = HeldBy::file(&b, "b.wav");

    let err = run(tmp.path(), &TWO, &ctx)
        .err()
        .expect("the move is refused");

    let text = err.to_string();
    assert!(text.contains("File move refused, nothing moved"), "{text}");
    assert!(
        text.contains(&format!(
            "{}: a later step still reads it as b.wav",
            b.display()
        )),
        "{text}"
    );
    assert_eq!(read(&tmp.path().join("a.wav")), "A", "a.wav never moved");
    assert_eq!(read(&b), "B");
}

#[test]
fn test_a_file_already_in_place_is_not_a_move() {
    let tmp = dir_with(&[("dest/Pack/a.wav", "A")]);
    let a = tmp.path().join("dest/Pack/a.wav").canonicalize().unwrap();
    let ctx = HeldBy::file(&a, "a.wav");

    let moved = run(tmp.path(), &[("dest/Pack/a.wav", "Pack/a.wav")], &ctx);

    assert!(moved.is_ok(), "nothing moves, so nothing is taken");
}
