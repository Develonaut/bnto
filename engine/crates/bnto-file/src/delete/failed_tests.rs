// file-delete failing partway (ISS-87): a delete that fails after others ran
// hands over the items finished before it, flagged, as the time limit does
// (time_tests.rs), so a closing step still sees what was deleted.

use serde_json::json;

use super::time_tests::{names, run, three_files};
use crate::changes_mid_run::ChangesMidRun;

const ALL: [&str; 3] = ["a.txt", "b.txt", "c.txt"];

#[test]
fn test_a_delete_that_fails_hands_over_the_items_before_it() {
    let tmp = three_files();
    let base = tmp.path().join("base");
    let b = base.join("b.txt");
    let ctx = ChangesMidRun::before_file(2, || std::fs::remove_file(&b).unwrap());

    let err = run(&base, &ALL, &ctx).err().expect("deleting b.txt fails");

    assert!(!err.is_timeout(), "{err}");
    assert!(err.to_string().contains("Failed to delete b.txt"), "{err}");
    assert!(
        base.join("c.txt").exists(),
        "nothing deleted after the failure"
    );
    let handed = err.into_parts().1.expect("the deleted item is handed over");
    assert_eq!(names(&handed), ["a.txt"]);
    assert_eq!(handed.files[0].metadata["deleted"], json!(1));
    assert_eq!(handed.metadata["deleted"], json!(1));
}

#[test]
fn test_a_first_delete_that_fails_hands_over_nothing() {
    let tmp = three_files();
    let base = tmp.path().join("base");
    let a = base.join("a.txt");
    let ctx = ChangesMidRun::before_file(1, || std::fs::remove_file(&a).unwrap());

    let err = run(&base, &ALL, &ctx).err().expect("deleting a.txt fails");

    assert!(err.to_string().contains("Failed to delete a.txt"), "{err}");
    assert!(err.into_parts().1.is_none(), "nothing was deleted");
}
