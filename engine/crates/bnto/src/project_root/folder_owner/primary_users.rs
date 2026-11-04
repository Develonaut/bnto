//! Accounts whose primary group is a given one, which the group's member
//! list never names. Scans the passwd database with `getpwent`, which keeps
//! one cursor per process, so bnto's scans take turns behind a lock.

use std::sync::Mutex;

/// Held while the passwd cursor is in use.
static SCAN: Mutex<()> = Mutex::new(());

/// How many accounts other than `uid` have `gid` as their primary group;
/// `None` when the scan fails, which counts as shared.
pub(super) fn others_with_primary_group(gid: u32, uid: u32) -> Option<usize> {
    let _turn = SCAN.lock().ok()?;
    // SAFETY: the lock keeps bnto's other scans off the passwd cursor.
    unsafe { libc::setpwent() };
    let others = count(gid, uid);
    // SAFETY: closes the cursor `setpwent` opened, still under the lock.
    unsafe { libc::endpwent() };
    others
}

/// Walk the open cursor to its end, counting matches.
fn count(gid: u32, uid: u32) -> Option<usize> {
    let mut others = 0;
    loop {
        // NSS modules can leave errno set on a successful call, so it's
        // cleared before each one, not once.
        errno::clear();
        // SAFETY: called under the lock; the entry is read before the next.
        let entry = unsafe { libc::getpwent() };
        if entry.is_null() {
            return errno::ended_cleanly().then_some(others);
        }
        // SAFETY: a non-null `getpwent` result is a valid passwd entry.
        let entry = unsafe { &*entry };
        if entry.pw_gid == gid && entry.pw_uid != uid {
            others += 1;
        }
    }
}

/// `getpwent` returns null both at the end and on an error; errno tells
/// them apart. ENOENT at the end is common from NSS modules and counts as
/// a clean end; any other value fails the scan.
mod errno {
    /// Set errno to 0 before a `getpwent` call.
    pub(super) fn clear() {
        if let Some(errno) = location() {
            // SAFETY: the location is this thread's errno, always valid.
            unsafe { *errno = 0 };
        }
    }

    /// Whether the null `getpwent` just returned was the end of the list;
    /// never on a platform whose errno bnto can't read.
    pub(super) fn ended_cleanly() -> bool {
        // SAFETY: as in `clear`.
        location().is_some_and(|errno| matches!(unsafe { *errno }, 0 | libc::ENOENT))
    }

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    fn location() -> Option<*mut libc::c_int> {
        // SAFETY: returns this thread's errno; no preconditions.
        Some(unsafe { libc::__error() })
    }

    #[cfg(target_os = "linux")]
    fn location() -> Option<*mut libc::c_int> {
        // SAFETY: returns this thread's errno; no preconditions.
        Some(unsafe { libc::__errno_location() })
    }

    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "linux"
    )))]
    fn location() -> Option<*mut libc::c_int> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scan_runs_to_its_end() {
        // SAFETY: geteuid and getegid take no arguments and can't fail.
        let (uid, gid) = unsafe { (libc::geteuid(), libc::getegid()) };
        assert!(others_with_primary_group(gid, uid).is_some());
    }
}
