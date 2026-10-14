// The overwrite-mode tests in `bnto/src/overwrite/tests.rs` cover ordering,
// cycles and refusals end to end; these cover what moving into folders adds.

use super::*;

fn mv(src: PathBuf, dest: PathBuf) -> Move {
    Move { src, dest }
}

#[test]
fn test_listing_shows_at_most_ten_items() {
    let items: Vec<String> = (0..12).map(|i| format!("conflict {i}")).collect();
    let text = listing(&items, ("conflict", "conflicts"));
    assert!(text.starts_with("12 conflicts\n"), "{text}");
    assert!(
        text.contains("conflict 9") && !text.contains("conflict 10"),
        "{text}"
    );
    assert!(text.ends_with("…and 2 more"), "{text}");
}

#[test]
fn test_listing_uses_the_singular_for_one_item() {
    let text = listing(&["bad".to_string()], ("problem", "problems"));
    assert_eq!(text, "1 problem\n  bad");
}

#[test]
fn test_a_move_creates_the_folders_it_lands_in() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("a.wav"), "A").unwrap();
    let dest = tmp.path().join("Pistol").join("Slide").join("a.wav");
    run_in_order(vec![mv(tmp.path().join("a.wav"), dest.clone())]).unwrap();
    assert_eq!(std::fs::read_to_string(dest).unwrap(), "A");
    assert!(!tmp.path().join("a.wav").exists());
}

#[test]
fn test_moves_between_folders_follow_the_chain_order() {
    // a/x.wav → b/x.wav while b/x.wav → c/x.wav: b's file must leave first.
    let tmp = tempfile::tempdir().unwrap();
    for (dir, body) in [("a", "A"), ("b", "B")] {
        std::fs::create_dir(tmp.path().join(dir)).unwrap();
        std::fs::write(tmp.path().join(dir).join("x.wav"), body).unwrap();
    }
    let at = |dir: &str| tmp.path().join(dir).join("x.wav");
    let moves = vec![mv(at("a"), at("b")), mv(at("b"), at("c"))];
    assert!(conflicts(&moves).is_empty());
    run_in_order(moves).unwrap();
    assert_eq!(std::fs::read_to_string(at("b")).unwrap(), "A");
    assert_eq!(std::fs::read_to_string(at("c")).unwrap(), "B");
}

#[test]
fn test_landing_on_a_file_outside_the_plan_is_a_conflict() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("a.wav"), "A").unwrap();
    std::fs::create_dir(tmp.path().join("Kept")).unwrap();
    std::fs::write(tmp.path().join("Kept").join("a.wav"), "KEEP").unwrap();
    let found = conflicts(&[mv(
        tmp.path().join("a.wav"),
        tmp.path().join("Kept").join("a.wav"),
    )]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("isn't being renamed away"), "{found:?}");
}

#[test]
fn test_platform_name_check_accepts_ordinary_names() {
    assert!(check_platform_name("Slide Rack 01.wav").is_ok());
}

#[cfg(windows)]
#[test]
fn test_platform_name_check_refuses_windows_device_names() {
    assert!(check_platform_name("CON").is_err());
    assert!(check_platform_name("aux.wav").is_err());
    assert!(check_platform_name("a:b").is_err());
}
