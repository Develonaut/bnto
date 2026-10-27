// file-move failing partway (ISS-87): a rename that fails after others ran
// hands over the files now under the destination, as the time limit does
// (time_tests.rs), so a closing step still sees what was moved.

use bnto_core::processor::FileData;

use super::time_tests::{dir_with, names, read, run};
use crate::changes_mid_run::ChangesMidRun;

const TWO: [(&str, &str); 2] = [("a.wav", "Pack/a.wav"), ("b.wav", "Pack/b.wav")];

#[test]
fn test_a_rename_that_finds_its_target_taken_hands_over_the_first_file() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    let taken = tmp.path().join("dest/Pack/b.wav");
    let ctx = ChangesMidRun::before_file(2, || std::fs::write(&taken, "X").unwrap());

    let err = run(tmp.path(), &TWO, &ctx)
        .err()
        .expect("the second rename fails");

    assert!(!err.is_timeout(), "{err}");
    assert!(err.to_string().contains("now exists"), "{err}");
    assert_eq!(read(&taken), "X", "never replaced");
    let handed = err.into_parts().1.expect("the moved file is handed over");
    assert_eq!(names(&handed), ["Pack/a.wav"]);
    assert!(matches!(&handed.files[0].data, FileData::Path(p) if p.ends_with("dest/Pack/a.wav")));
}

#[test]
fn test_a_first_rename_that_fails_hands_over_nothing() {
    let tmp = dir_with(&[("a.wav", "A"), ("b.wav", "B")]);
    let taken = tmp.path().join("dest/Pack/a.wav");
    let ctx = ChangesMidRun::before_file(1, || {
        std::fs::create_dir_all(taken.parent().unwrap()).unwrap();
        std::fs::write(&taken, "X").unwrap();
    });

    let err = run(tmp.path(), &TWO, &ctx)
        .err()
        .expect("the first rename fails");

    assert!(err.to_string().contains("now exists"), "{err}");
    assert!(err.into_parts().1.is_none(), "nothing was moved");
}
