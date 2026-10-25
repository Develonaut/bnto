// Where a destination lands on disk, for files that don't exist yet.
//
// Two names can reach one new file through a folder symlink (`sub` → `.`
// makes `sub/b.txt` and `b.txt` one path) or a dangling file symlink. The
// existing part of the path is resolved through its links, and the rest
// (folders a write will create, the file) appended, `..` applied as
// `create_dir_all` would. Keys are case-folded, as `output_names` folds names,
// so a recipe is refused the same everywhere.

use std::path::{Component, Path, PathBuf};

/// How many symlinks a dangling-link chain may follow before giving up.
const MAX_HOPS: u8 = 16;

/// The case-folded path `path` would be written at.
pub fn landing_key(path: &Path) -> String {
    let landed = resolved(path, MAX_HOPS);
    landed.to_string_lossy().replace('\\', "/").to_lowercase()
}

/// `path` with its existing part canonicalized and the rest appended.
fn resolved(path: &Path, hops: u8) -> PathBuf {
    if let Ok(real) = dunce::canonicalize(path) {
        return real;
    }
    match dangling_target(path).filter(|_| hops > 0) {
        Some(target) => resolved(&target, hops - 1),
        None => resolved_missing(path, hops),
    }
}

/// A path that doesn't exist: its parent resolved, then its last part.
fn resolved_missing(path: &Path, hops: u8) -> PathBuf {
    let mut components = path.components();
    let Some(last) = components.next_back() else {
        return path.to_path_buf();
    };
    let rest = components.as_path();
    if rest.as_os_str().is_empty() {
        return path.to_path_buf();
    }
    let mut base = resolved(rest, hops);
    match last {
        Component::ParentDir => {
            base.pop();
        }
        Component::Normal(name) => base.push(name),
        _ => {}
    }
    base
}

/// Where a symlink whose target is missing points (a write follows it).
fn dangling_target(path: &Path) -> Option<PathBuf> {
    let meta = std::fs::symlink_metadata(path).ok()?;
    if !meta.file_type().is_symlink() {
        return None;
    }
    let target = std::fs::read_link(path).ok()?;
    Some(
        path.parent()
            .map_or(target.clone(), |dir| dir.join(&target)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separate_new_files_land_apart() {
        let out = tempfile::tempdir().unwrap();
        let key = |name: &str| landing_key(&out.path().join(name));
        assert_ne!(key("a.txt"), key("b.txt"));
        assert_ne!(key("sub/b.txt"), key("b.txt"));
    }

    #[test]
    fn test_a_new_file_and_an_existing_one_at_one_path_share_a_key() {
        let out = tempfile::tempdir().unwrap();
        std::fs::write(out.path().join("b.txt"), "B").unwrap();
        let new = out.path().join("new").join("..").join("b.txt");
        assert_eq!(landing_key(&new), landing_key(&out.path().join("b.txt")));
    }

    #[test]
    fn test_names_differing_only_in_case_share_a_key() {
        let out = tempfile::tempdir().unwrap();
        let key = |name: &str| landing_key(&out.path().join(name));
        assert_eq!(key("Sub/B.txt"), key("sub/b.txt"));
    }

    #[cfg(unix)]
    #[test]
    fn test_new_files_through_a_folder_link_share_a_key() {
        let out = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(".", out.path().join("sub")).unwrap();
        let key = |name: &str| landing_key(&out.path().join(name));
        assert_eq!(key("sub/b.txt"), key("b.txt"));
        assert_eq!(key("sub/new/b.txt"), key("new/b.txt"));
    }

    #[cfg(unix)]
    #[test]
    fn test_a_dangling_link_lands_on_its_target() {
        let out = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink("b.txt", out.path().join("l.txt")).unwrap();
        let key = |name: &str| landing_key(&out.path().join(name));
        assert_eq!(key("l.txt"), key("b.txt"));
    }

    #[cfg(unix)]
    #[test]
    fn test_a_link_loop_ends() {
        let out = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink("y", out.path().join("x")).unwrap();
        std::os::unix::fs::symlink("x", out.path().join("y")).unwrap();
        let _ = landing_key(&out.path().join("x"));
    }
}
