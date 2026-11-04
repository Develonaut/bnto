//! The user's private group: named after the user, their primary group, with
//! no other listed members and no other account's primary group ("user
//! private groups", the Debian, Ubuntu and Fedora default). Its write bit
//! lets no one else in. Any lookup error counts as not private.
//!
//! An account whose primary group this is never shows in `gr_mem`, so the
//! passwd database is scanned for one (`primary_users`). With NSS or LDAP
//! that scan can be incomplete; that's accepted: consent still asks before
//! a project recipe runs, and step 7e skips recipe files others own.
//!
//! Lookups use one fixed 16 KiB buffer with no retry on ERANGE, on purpose:
//! a private group's entries never come near it, and a bigger one fails
//! closed.

use std::ffi::CStr;
use std::ptr;

use super::primary_users;

/// Room for one passwd or group entry; a bigger entry fails the lookup.
const ENTRY_BUFFER: usize = 16 * 1024;

/// Whether `gid` is the private group of the user `uid`. The passwd scan
/// runs only for a group that passes every other test.
pub(crate) fn is_private_group(gid: u32, uid: u32) -> bool {
    let (Some(user), Some(group)) = (user(uid), group(gid)) else {
        return false;
    };
    decide(&user, gid, &group, 0)
        && primary_users::others_with_primary_group(gid, uid)
            .is_some_and(|others| decide(&user, gid, &group, others))
}

/// A passwd entry's name and primary group.
struct User {
    name: String,
    gid: u32,
}

/// A group entry's name and listed members.
struct Group {
    name: String,
    members: Vec<String>,
}

/// The rule, on entries already read: the user's primary group, named as
/// the user, listing no one but the user, and no other account's primary
/// group (`other_primary_users` of them).
fn decide(user: &User, gid: u32, group: &Group, other_primary_users: usize) -> bool {
    gid == user.gid
        && group.name == user.name
        && group.members.iter().all(|m| *m == user.name)
        && other_primary_users == 0
}

fn user(uid: u32) -> Option<User> {
    let mut buf = vec![0 as libc::c_char; ENTRY_BUFFER];
    // SAFETY: an all-zero passwd (null pointers, zero ids) is valid to fill.
    let mut entry: libc::passwd = unsafe { std::mem::zeroed() };
    let mut found = ptr::null_mut();
    // SAFETY: every pointer is valid for the call and `buf.len()` is its size.
    let rc = unsafe { libc::getpwuid_r(uid, &mut entry, buf.as_mut_ptr(), buf.len(), &mut found) };
    if rc != 0 || found.is_null() {
        return None;
    }
    // SAFETY: on success `pw_name` is a NUL-terminated string inside `buf`.
    let name = unsafe { CStr::from_ptr(entry.pw_name) }.to_str().ok()?;
    Some(User {
        name: name.to_owned(),
        gid: entry.pw_gid,
    })
}

fn group(gid: u32) -> Option<Group> {
    let mut buf = vec![0 as libc::c_char; ENTRY_BUFFER];
    // SAFETY: an all-zero group (null pointers, zero id) is valid to fill.
    let mut entry: libc::group = unsafe { std::mem::zeroed() };
    let mut found = ptr::null_mut();
    // SAFETY: every pointer is valid for the call and `buf.len()` is its size.
    let rc = unsafe { libc::getgrgid_r(gid, &mut entry, buf.as_mut_ptr(), buf.len(), &mut found) };
    if rc != 0 || found.is_null() {
        return None;
    }
    // SAFETY: on success `gr_name` and `gr_mem` point into `buf`, still alive.
    let name = unsafe { CStr::from_ptr(entry.gr_name) }.to_str().ok()?;
    // SAFETY: `gr_mem` is null or a NULL-terminated list in `buf`, alive
    // for this call, which is what `members` asks.
    let members = unsafe { members(entry.gr_mem) }?;
    Some(Group {
        name: name.to_owned(),
        members,
    })
}

/// The names in a NULL-terminated `gr_mem` list (none when it's null).
///
/// # Safety
/// `list` is null or a `gr_mem` from a successful `getgrgid_r` whose buffer
/// is still alive.
unsafe fn members(list: *mut *mut libc::c_char) -> Option<Vec<String>> {
    let mut names = Vec::new();
    for i in 0.. {
        // SAFETY: the caller's list is NULL-terminated, so `i` stays in it.
        let item = if list.is_null() {
            ptr::null_mut()
        } else {
            unsafe { *list.add(i) }
        };
        if item.is_null() {
            break;
        }
        // SAFETY: each entry before the NULL is a NUL-terminated string.
        names.push(unsafe { CStr::from_ptr(item) }.to_str().ok()?.to_owned());
    }
    Some(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ryan() -> User {
        User {
            name: "ryan".into(),
            gid: 1000,
        }
    }

    fn group_of(name: &str, members: &[&str]) -> Group {
        Group {
            name: name.into(),
            members: members.iter().map(|m| (*m).to_owned()).collect(),
        }
    }

    #[test]
    fn the_users_own_primary_group_is_private() {
        assert!(decide(&ryan(), 1000, &group_of("ryan", &[]), 0));
        assert!(decide(&ryan(), 1000, &group_of("ryan", &["ryan"]), 0));
    }

    #[test]
    fn a_shared_or_borrowed_group_is_not_private() {
        // macOS's `staff`, a group with another member, another user's group.
        assert!(!decide(&ryan(), 20, &group_of("staff", &[]), 0));
        assert!(!decide(
            &ryan(),
            1000,
            &group_of("ryan", &["ryan", "eve"]),
            0
        ));
        assert!(!decide(&ryan(), 1001, &group_of("ryan", &[]), 0));
        assert!(!decide(&ryan(), 1000, &group_of("devs", &[]), 0));
    }

    #[test]
    fn a_group_another_account_has_as_primary_is_not_private() {
        // `gr_mem` never lists such an account, so only the scan finds it.
        assert!(!decide(&ryan(), 1000, &group_of("ryan", &[]), 1));
    }

    #[test]
    fn the_running_users_entries_can_be_read() {
        // SAFETY: geteuid takes no arguments, can't fail and touches no memory.
        let me = user(unsafe { libc::geteuid() }).expect("a passwd entry");
        assert!(!me.name.is_empty());
        assert!(group(me.gid).is_some(), "no group entry for gid {}", me.gid);
    }
}
