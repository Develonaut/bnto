//! Unix: a folder is safe when the user bnto runs as owns it and nobody
//! else can write in it. Group-write counts as others (macOS puts every
//! user in `staff`) unless the group is the user's private group. On a
//! folder the walk only climbs through, the sticky bit (as on `/tmp`) makes
//! shared write fine; on `.bnto` and `.bnto/recipes` it doesn't, since
//! others could still add files there. A filesystem that makes owners up
//! (`filesystem.rs`) is never safe.

use std::os::unix::fs::MetadataExt;
use std::path::Path;

use super::Role;
use super::filesystem::records_ownership;
use super::private_group::is_private_group;

const GROUP_WRITE: u32 = 0o020;
const WORLD_WRITE: u32 = 0o002;
/// Only an entry's owner may rename or delete it.
const STICKY: u32 = 0o1000;

/// Whether `path` (not followed if a symlink) is the user's and not shared.
pub(crate) fn is_safe_folder(path: &Path, role: Role) -> bool {
    // SAFETY: geteuid takes no arguments, can't fail and touches no memory.
    let me = unsafe { libc::geteuid() };
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return false;
    };
    let private = || is_private_group(meta.gid(), me);
    owned_by(meta.uid(), me)
        && writers_allowed(meta.mode(), role, private)
        && records_ownership(path)
}

/// Whether an entry owned by `owner` belongs to the user `me`.
fn owned_by(owner: u32, me: u32) -> bool {
    owner == me
}

/// Whether a folder with `mode` keeps others from adding or replacing what
/// it holds; `group_is_private` is asked only for a group-writable one.
fn writers_allowed(mode: u32, role: Role, group_is_private: impl FnOnce() -> bool) -> bool {
    let sticky_ok = role == Role::Visited && mode & STICKY != 0;
    if mode & WORLD_WRITE != 0 {
        return sticky_ok;
    }
    if mode & GROUP_WRITE != 0 {
        return sticky_ok || group_is_private();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn another_users_folder_is_not_owned() {
        assert!(owned_by(501, 501));
        assert!(!owned_by(0, 501));
        assert!(!owned_by(502, 501));
    }

    #[test]
    fn a_folder_only_its_owner_writes_is_fine() {
        for role in [Role::Visited, Role::Loaded] {
            assert!(writers_allowed(0o40755, role, || false));
            assert!(writers_allowed(0o700, role, || false));
        }
    }

    #[test]
    fn world_write_is_fine_only_on_a_sticky_visited_folder() {
        assert!(writers_allowed(0o41777, Role::Visited, || true));
        assert!(!writers_allowed(0o40777, Role::Visited, || true));
        assert!(!writers_allowed(0o41777, Role::Loaded, || true));
        assert!(!writers_allowed(0o40757, Role::Loaded, || true));
    }

    #[test]
    fn group_write_needs_the_private_group_or_a_sticky_visited_folder() {
        assert!(writers_allowed(0o40775, Role::Loaded, || true));
        assert!(!writers_allowed(0o40775, Role::Loaded, || false));
        assert!(!writers_allowed(0o41775, Role::Loaded, || false));
        assert!(writers_allowed(0o41775, Role::Visited, || false));
        assert!(!writers_allowed(0o40775, Role::Visited, || false));
    }
}
