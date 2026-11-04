// project_root walks up from a folder to the nearest `.bnto/recipes/`,
// stopping where design §2 says. Every sandbox holds a `.git` marker at its
// root: under `engine/.tmp` the walk would otherwise climb into this repo.

use std::fs;
use std::path::{Path, PathBuf};

use super::project_root;

/// A sandbox whose root stops the walk, as the test harnesses' does.
fn sandbox() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir(tmp.path().join(".git")).unwrap();
    tmp
}

/// `rel` under `root`, created as a folder. On Unix each new folder is
/// 0755 whatever the umask, since a group-writable one stops the walk.
fn mkdir(root: &Path, rel: &str) -> PathBuf {
    let mut dir = root.to_path_buf();
    for part in Path::new(rel).components() {
        dir.push(part);
        if !dir.exists() {
            fs::create_dir(&dir).unwrap();
            private_mode(&dir);
        }
    }
    dir
}

#[cfg(unix)]
fn private_mode(dir: &Path) {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(dir, fs::Permissions::from_mode(0o755)).unwrap();
}

#[cfg(not(unix))]
fn private_mode(_dir: &Path) {}

/// `dir` made a project root: a real `.bnto/recipes/` in it.
fn project(root: &Path, rel: &str) -> PathBuf {
    let dir = mkdir(root, rel);
    mkdir(&dir, ".bnto/recipes");
    dir
}

/// The walk from `start`, with a home and a bnto home in the sandbox that
/// it never passes through.
fn find(sandbox: &Path, start: &Path) -> Option<PathBuf> {
    let home = mkdir(sandbox, "home");
    project_root(start, &home, &mkdir(&home, ".bnto"))
}

fn canonical(path: &Path) -> PathBuf {
    dunce::canonicalize(path).unwrap()
}

#[test]
fn found_in_the_start_folder() {
    let sb = sandbox();
    let root = project(sb.path(), "p");
    assert_eq!(find(sb.path(), &root), Some(canonical(&root)));
}

#[test]
fn found_three_levels_up() {
    let sb = sandbox();
    let root = project(sb.path(), "p");
    let start = mkdir(&root, "a/b/c");
    assert_eq!(find(sb.path(), &start), Some(canonical(&root)));
}

#[test]
fn the_nearest_project_wins() {
    let sb = sandbox();
    project(sb.path(), "outer");
    let inner = project(sb.path(), "outer/inner");
    let start = mkdir(&inner, "src");
    assert_eq!(find(sb.path(), &start), Some(canonical(&inner)));
}

#[test]
fn nothing_found_without_a_project() {
    let sb = sandbox();
    let start = mkdir(sb.path(), "a/b");
    assert_eq!(find(sb.path(), &start), None);
}

#[test]
fn stops_at_a_git_folder() {
    let sb = sandbox();
    let outer = project(sb.path(), "outer");
    mkdir(&outer, "repo/.git");
    let start = mkdir(&outer, "repo/src");
    assert_eq!(find(sb.path(), &start), None);
}

#[test]
fn stops_at_a_git_file() {
    // A worktree's or submodule's `.git` is a file.
    let sb = sandbox();
    let outer = project(sb.path(), "outer");
    let repo = mkdir(&outer, "repo");
    fs::write(repo.join(".git"), "gitdir: ../.git/worktrees/repo\n").unwrap();
    let start = mkdir(&repo, "src");
    assert_eq!(find(sb.path(), &start), None);
}

#[cfg(unix)]
#[test]
fn stops_at_a_symlinked_git() {
    let sb = sandbox();
    let outer = project(sb.path(), "outer");
    let repo = mkdir(&outer, "repo");
    let elsewhere = mkdir(sb.path(), "elsewhere");
    std::os::unix::fs::symlink(&elsewhere, repo.join(".git")).unwrap();
    let start = mkdir(&repo, "src");
    assert_eq!(find(sb.path(), &start), None);
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "mounts a disk image with hdiutil; run by hand"]
fn a_disk_image_mounted_without_owners_finds_nothing() {
    // macOS mounts disk images `noowners` by default: every file shows as
    // the mounting user's, so the owner check alone would pass.
    use std::process::Command;
    let sb = sandbox();
    let image = sb.path().join("vol.dmg");
    let mnt = mkdir(sb.path(), "mnt");
    let run = |args: &[&str]| {
        Command::new("hdiutil")
            .args(args)
            .status()
            .unwrap()
            .success()
    };
    let created = run(&[
        "create",
        "-size",
        "8m",
        "-fs",
        "APFS",
        "-volname",
        "v",
        image.to_str().unwrap(),
    ]);
    assert!(created);
    assert!(run(&[
        "attach",
        "-owners",
        "off",
        "-nobrowse",
        "-mountpoint",
        mnt.to_str().unwrap(),
        image.to_str().unwrap()
    ]));
    // A project inside the volume: the walk never leaves its filesystem, so
    // only the filesystem check can stop it.
    let root = mnt.join("proj");
    fs::create_dir_all(root.join(".bnto").join("recipes")).unwrap();
    let found = find(sb.path(), &root);
    run(&["detach", mnt.to_str().unwrap()]);
    assert_eq!(found, None);
}

#[test]
fn the_folder_holding_git_is_checked_itself() {
    let sb = sandbox();
    let repo = project(sb.path(), "repo");
    mkdir(&repo, ".git");
    let start = mkdir(&repo, "src");
    assert_eq!(find(sb.path(), &start), Some(canonical(&repo)));
}

#[test]
fn home_is_never_a_project_root() {
    let sb = sandbox();
    let home = project(sb.path(), "home");
    let start = mkdir(&home, "work");
    let bnto_home = mkdir(sb.path(), "bnto-home");
    assert_eq!(project_root(&start, &home, &bnto_home), None);
    assert_eq!(project_root(&home, &home, &bnto_home), None);
}

#[test]
fn a_project_inside_home_is_found() {
    let sb = sandbox();
    let home = project(sb.path(), "home");
    let root = project(&home, "code/p");
    let start = mkdir(&root, "src");
    let bnto_home = mkdir(sb.path(), "bnto-home");
    assert_eq!(
        project_root(&start, &home, &bnto_home),
        Some(canonical(&root))
    );
}

#[test]
fn home_with_a_trailing_slash_is_still_home() {
    let sb = sandbox();
    let home = project(sb.path(), "home");
    let start = mkdir(&home, "work");
    let slashed = PathBuf::from(format!("{}{}", home.display(), std::path::MAIN_SEPARATOR));
    let bnto_home = mkdir(sb.path(), "bnto-home");
    assert_eq!(project_root(&start, &slashed, &bnto_home), None);
}

#[cfg(any(target_os = "macos", windows))]
#[test]
fn home_in_another_case_is_still_home() {
    // Both OSes' default file systems ignore case.
    let sb = sandbox();
    let home = project(sb.path(), "home");
    let start = mkdir(&home, "work");
    let other_case = sb.path().join("HOME");
    let bnto_home = mkdir(sb.path(), "bnto-home");
    assert_eq!(project_root(&start, &other_case, &bnto_home), None);
}

#[cfg(unix)]
#[test]
fn home_through_a_symlink_is_still_home() {
    // macOS's `/var` is a link to `/private/var`: home may be spelled
    // through a link while the walk sees the real folders.
    let sb = sandbox();
    let home = project(sb.path(), "home");
    let start = mkdir(&home, "work");
    let link = sb.path().join("home-link");
    std::os::unix::fs::symlink(&home, &link).unwrap();
    let bnto_home = mkdir(sb.path(), "bnto-home");
    assert_eq!(project_root(&start, &link, &bnto_home), None);
    // A start spelled through the link climbs the real folders too.
    assert_eq!(project_root(&link.join("work"), &home, &bnto_home), None);
}

#[test]
fn stops_at_a_project_whose_bnto_folder_is_the_bnto_home() {
    let sb = sandbox();
    let outer = project(sb.path(), "outer");
    let inner = project(&outer, "inner");
    let home = mkdir(sb.path(), "home");
    let start = mkdir(&inner, "src");
    assert_eq!(project_root(&start, &home, &inner.join(".bnto")), None);
}

#[cfg(unix)]
#[test]
fn stops_at_a_project_whose_recipes_are_the_library() {
    // A bnto home whose `recipes` links to the project's recipes folder.
    let sb = sandbox();
    let outer = project(sb.path(), "outer");
    let inner = project(&outer, "inner");
    let bnto_home = mkdir(sb.path(), "bnto-home");
    let recipes = inner.join(".bnto").join("recipes");
    std::os::unix::fs::symlink(&recipes, bnto_home.join("recipes")).unwrap();
    let home = mkdir(sb.path(), "home");
    assert_eq!(project_root(&inner, &home, &bnto_home), None);
}

#[test]
fn stops_at_the_bnto_home_even_without_its_recipes_folder() {
    // Tells the bnto-home stop apart from the library one: this home has no
    // `recipes`, so only its own check keeps the walk from the outer project.
    let sb = sandbox();
    let outer = project(sb.path(), "outer");
    let inner = mkdir(&outer, "inner");
    let bnto_home = mkdir(&inner, ".bnto");
    let home = mkdir(sb.path(), "home");
    assert_eq!(project_root(&inner, &home, &bnto_home), None);
}

#[test]
fn a_home_that_cannot_be_opened_finds_nothing() {
    // Unopened, home couldn't be told apart from the folders the walk
    // visits, so its stop would never fire: fail closed instead.
    let sb = sandbox();
    let root = project(sb.path(), "p");
    let bnto_home = mkdir(sb.path(), "bnto-home");
    let missing = sb.path().join("missing");
    assert_eq!(project_root(&root, &missing, &bnto_home), None);
    let home = mkdir(sb.path(), "home");
    assert_eq!(project_root(&root, &home, &missing), None);
}

#[test]
fn ignores_recipes_without_the_dot() {
    let sb = sandbox();
    let start = mkdir(sb.path(), "p/recipes");
    assert_eq!(find(sb.path(), &start), None);
    assert_eq!(find(sb.path(), &sb.path().join("p")), None);
}

#[test]
fn ignores_a_bnto_file() {
    let sb = sandbox();
    let outer = project(sb.path(), "outer");
    let inner = mkdir(&outer, "inner");
    fs::write(inner.join(".bnto"), "").unwrap();
    assert_eq!(find(sb.path(), &inner), Some(canonical(&outer)));
}

#[cfg(unix)]
#[test]
fn ignores_a_bnto_folder_that_is_a_symlink() {
    let sb = sandbox();
    let outer = project(sb.path(), "outer");
    let elsewhere = project(sb.path(), "elsewhere");
    let inner = mkdir(&outer, "inner");
    std::os::unix::fs::symlink(elsewhere.join(".bnto"), inner.join(".bnto")).unwrap();
    assert_eq!(find(sb.path(), &inner), Some(canonical(&outer)));
}

#[cfg(unix)]
#[test]
fn ignores_a_recipes_folder_that_is_a_symlink() {
    let sb = sandbox();
    let outer = project(sb.path(), "outer");
    let elsewhere = mkdir(sb.path(), "elsewhere");
    let inner = mkdir(&outer, "inner/.bnto");
    std::os::unix::fs::symlink(&elsewhere, inner.join("recipes")).unwrap();
    let start = outer.join("inner");
    assert_eq!(find(sb.path(), &start), Some(canonical(&outer)));
}

#[cfg(unix)]
mod unix_permissions {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};

    use super::{canonical, find, mkdir, project, sandbox};

    /// `path`'s mode set to `mode` until dropped, so the sandbox can go.
    struct Mode(PathBuf);

    impl Mode {
        fn set(path: &Path, mode: u32) -> Self {
            fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
            Self(path.to_path_buf())
        }
    }

    impl Drop for Mode {
        fn drop(&mut self) {
            let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o755));
        }
    }

    #[test]
    fn stops_at_a_world_writable_folder() {
        // Anyone may plant a `.bnto` there, as another user's folder: chown
        // needs root, so this stands in for "not owned by the current user".
        let sb = sandbox();
        let root = project(sb.path(), "p");
        let _mode = Mode::set(&root, 0o777);
        assert_eq!(find(sb.path(), &root), None);
    }

    #[test]
    fn a_walk_from_inside_a_world_writable_folder_stops_there() {
        // The `/tmp/x` case: a `.bnto` above the shared folder isn't reached.
        let sb = sandbox();
        let outer = project(sb.path(), "outer");
        let shared = mkdir(&outer, "shared");
        let start = mkdir(&shared, "mine");
        let _mode = Mode::set(&shared, 0o777);
        assert_eq!(find(sb.path(), &start), None);
    }

    #[test]
    fn a_sticky_world_writable_folder_does_not_stop_the_walk() {
        // Like `/tmp`: others can't replace what the owner made there.
        let sb = sandbox();
        let root = project(sb.path(), "p");
        let _mode = Mode::set(&root, 0o1777);
        assert_eq!(find(sb.path(), &root), Some(canonical(&root)));
    }

    #[test]
    fn stops_at_an_unreadable_parent() {
        let sb = sandbox();
        let root = project(sb.path(), "p");
        let start = mkdir(&root, "src");
        let _mode = Mode::set(&root, 0o311);
        if fs::read_dir(&root).is_ok() {
            eprintln!("skipped: the folder stays readable for this user");
            return;
        }
        assert_eq!(find(sb.path(), &start), None);
    }

    #[test]
    fn stops_at_an_unreadable_bnto_folder() {
        // An error other than NotFound stops the walk: it doesn't go on up
        // to the outer project.
        let sb = sandbox();
        let outer = project(sb.path(), "outer");
        let inner = project(&outer, "inner");
        let _mode = Mode::set(&inner.join(".bnto"), 0o000);
        if fs::read_dir(inner.join(".bnto")).is_ok() {
            eprintln!("skipped: the folder stays readable for this user");
            return;
        }
        assert_eq!(find(sb.path(), &inner), None);
    }

    #[test]
    fn stops_at_a_world_writable_bnto_folder() {
        // Not passed over: someone else may have planted it.
        let sb = sandbox();
        let outer = project(sb.path(), "outer");
        let inner = project(&outer, "inner");
        let _mode = Mode::set(&inner.join(".bnto"), 0o777);
        assert_eq!(find(sb.path(), &inner), None);
    }

    #[test]
    fn stops_at_a_world_writable_recipes_folder() {
        let sb = sandbox();
        let outer = project(sb.path(), "outer");
        let inner = project(&outer, "inner");
        let _mode = Mode::set(&inner.join(".bnto").join("recipes"), 0o777);
        assert_eq!(find(sb.path(), &inner), None);
    }

    /// Whether `dir`'s group is the user's private group (`user`/`user`,
    /// no other members), which may write in a project's folders.
    fn has_private_group(dir: &Path) -> bool {
        use std::os::unix::fs::MetadataExt;
        let gid = fs::metadata(dir).unwrap().gid();
        // SAFETY: geteuid takes no arguments, can't fail and touches no memory.
        let uid = unsafe { libc::geteuid() };
        crate::project_root::folder_owner::private_group::is_private_group(gid, uid)
    }

    #[test]
    fn a_group_writable_folder_stops_the_walk_unless_its_group_is_private() {
        // macOS puts every user in `staff`: group-writable is shared there.
        let sb = sandbox();
        let outer = project(sb.path(), "outer");
        let shared = mkdir(&outer, "shared");
        let start = mkdir(&shared, "mine");
        let _mode = Mode::set(&shared, 0o775);
        let expected = has_private_group(&shared).then(|| canonical(&outer));
        assert_eq!(find(sb.path(), &start), expected);
    }

    #[test]
    fn a_group_writable_bnto_folder_stops_the_walk_unless_its_group_is_private() {
        let sb = sandbox();
        let root = project(sb.path(), "p");
        let _mode = Mode::set(&root.join(".bnto"), 0o775);
        let expected = has_private_group(&root.join(".bnto")).then(|| canonical(&root));
        assert_eq!(find(sb.path(), &root), expected);
    }

    #[test]
    fn a_sticky_world_writable_bnto_folder_still_stops_the_walk() {
        // Sticky stops others replacing files, not adding them, and these
        // are the folders whose files load.
        let sb = sandbox();
        let outer = project(sb.path(), "outer");
        let inner = project(&outer, "inner");
        let _mode = Mode::set(&inner.join(".bnto"), 0o1777);
        assert_eq!(find(sb.path(), &inner), None);
    }

    #[test]
    fn a_sticky_world_writable_recipes_folder_still_stops_the_walk() {
        let sb = sandbox();
        let outer = project(sb.path(), "outer");
        let inner = project(&outer, "inner");
        let _mode = Mode::set(&inner.join(".bnto").join("recipes"), 0o1777);
        assert_eq!(find(sb.path(), &inner), None);
    }
}
