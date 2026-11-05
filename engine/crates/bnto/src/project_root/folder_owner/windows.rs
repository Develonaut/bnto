//! Windows: a folder is safe when its owner is the user bnto runs as, as
//! git checks (CVE-2022-24765). A folder owned by the Administrators group
//! counts only when this process holds that group enabled (elevated), as
//! git allows. Any error reading either side counts as not owned.

use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

use super::Role;

use windows_sys::Win32::Foundation::{CloseHandle, ERROR_SUCCESS, HANDLE, LocalFree};
use windows_sys::Win32::Security::Authorization::{GetNamedSecurityInfoW, SE_FILE_OBJECT};
use windows_sys::Win32::Security::{
    CheckTokenMembership, EqualSid, GetTokenInformation, IsWellKnownSid,
    OWNER_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, PSID, TOKEN_QUERY, TOKEN_USER, TokenUser,
    WinBuiltinAdministratorsSid,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Whether the user bnto runs as owns `path`. The role doesn't matter
/// here: like git, Windows checks the owner, not who else may write.
pub(crate) fn is_safe_folder(path: &Path, _role: Role) -> bool {
    owner_match(path).is_some()
}

/// Which side of the owner rule `path` meets, if any.
fn owner_match(path: &Path) -> Option<OwnerMatch> {
    let owner = FolderOwner::read(path)?;
    let user = TokenUserSid::current()?;
    // SAFETY: both SIDs point into buffers their owners keep alive here.
    let is_user = unsafe { EqualSid(owner.sid, user.sid()) } != 0;
    if is_user {
        return Some(OwnerMatch::User);
    }
    accepts_owner(false, || owner.is_admins_held()).then_some(OwnerMatch::Administrators)
}

/// How a folder's owner was accepted.
#[derive(Debug, PartialEq)]
enum OwnerMatch {
    /// Owned by the user bnto runs as.
    User,
    /// Owned by the Administrators group, which this process holds.
    Administrators,
}

/// The owner rule: the user, or the Administrators group while this
/// process holds it (`admins_held` is asked only for a non-user owner).
fn accepts_owner(owner_is_user: bool, admins_held: impl FnOnce() -> bool) -> bool {
    owner_is_user || admins_held()
}

/// A folder's owner SID, inside the security descriptor that holds it.
struct FolderOwner {
    descriptor: PSECURITY_DESCRIPTOR,
    sid: PSID,
}

impl FolderOwner {
    fn read(path: &Path) -> Option<Self> {
        let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        let mut owner = Self {
            descriptor: ptr::null_mut(),
            sid: ptr::null_mut(),
        };
        // SAFETY: `wide` is NUL-terminated and outlives the call; the two
        // out pointers are valid, and the outputs not asked for are null, as
        // the API allows. `sid` points into `descriptor`, freed on drop.
        let status = unsafe {
            GetNamedSecurityInfoW(
                wide.as_ptr(),
                SE_FILE_OBJECT,
                OWNER_SECURITY_INFORMATION,
                &mut owner.sid,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut owner.descriptor,
            )
        };
        (status == ERROR_SUCCESS && !owner.sid.is_null()).then_some(owner)
    }

    /// Whether the owner is the Administrators group and this process's
    /// token holds it enabled (elevated); a filtered token doesn't.
    fn is_admins_held(&self) -> bool {
        // SAFETY: `self.sid` is a valid SID while `self` lives.
        if unsafe { IsWellKnownSid(self.sid, WinBuiltinAdministratorsSid) } == 0 {
            return false;
        }
        let mut member = 0;
        // A null token reads this thread's token, while the owner check
        // reads the process token: they agree because bnto never
        // impersonates.
        // SAFETY: a null token means this thread's own; `member` is a valid
        // out pointer and `self.sid` a valid SID.
        let ok = unsafe { CheckTokenMembership(ptr::null_mut(), self.sid, &mut member) };
        ok != 0 && member != 0
    }
}

impl Drop for FolderOwner {
    fn drop(&mut self) {
        if !self.descriptor.is_null() {
            // SAFETY: GetNamedSecurityInfoW allocated it with LocalAlloc.
            unsafe { LocalFree(self.descriptor) };
        }
    }
}

/// The user SID of this process's token, in the buffer that holds it.
struct TokenUserSid {
    /// A `TOKEN_USER` and the SID after it; `u64`s keep it pointer-aligned.
    buffer: Vec<u64>,
}

impl TokenUserSid {
    fn current() -> Option<Self> {
        let token = ProcessToken::open()?;
        let mut len = 0u32;
        // SAFETY: a size query: a null buffer of length 0, as documented.
        unsafe { GetTokenInformation(token.0, TokenUser, ptr::null_mut(), 0, &mut len) };
        let mut buffer = vec![0u64; (len as usize).div_ceil(8)];
        // SAFETY: `buffer` holds at least `len` bytes, 8-byte aligned.
        let ok = unsafe {
            GetTokenInformation(
                token.0,
                TokenUser,
                buffer.as_mut_ptr().cast(),
                len,
                &mut len,
            )
        };
        (ok != 0 && len > 0).then_some(Self { buffer })
    }

    fn sid(&self) -> PSID {
        // SAFETY: GetTokenInformation filled the buffer with a TOKEN_USER.
        unsafe { (*self.buffer.as_ptr().cast::<TOKEN_USER>()).User.Sid }
    }
}

/// This process's access token, closed on drop.
struct ProcessToken(HANDLE);

impl ProcessToken {
    fn open() -> Option<Self> {
        let mut handle: HANDLE = ptr::null_mut();
        // SAFETY: GetCurrentProcess returns a pseudo-handle that needs no
        // closing; `handle` is a valid out pointer.
        let ok = unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) };
        (ok != 0).then_some(Self(handle))
    }
}

impl Drop for ProcessToken {
    fn drop(&mut self) {
        // SAFETY: OpenProcessToken gave this handle, closed only here.
        unsafe { CloseHandle(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_folder_just_made_is_owned_by_this_user() {
        // GitHub's Windows runners run elevated, and an elevated process's
        // new folders belong to the Administrators group: either branch
        // may match. The run's output says which one did.
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("mine");
        std::fs::create_dir(&dir).unwrap();
        let matched = owner_match(&dir).expect("owned by this user or held Administrators");
        eprintln!("owner matched as {matched:?}");
        assert!(is_safe_folder(&dir, Role::Loaded));
    }

    #[test]
    fn a_missing_folder_is_not_safe() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(!is_safe_folder(&tmp.path().join("missing"), Role::Loaded));
    }

    #[test]
    fn another_owner_counts_only_as_held_administrators() {
        assert!(accepts_owner(true, || false));
        assert!(accepts_owner(false, || true));
        assert!(!accepts_owner(false, || false));
    }
}
