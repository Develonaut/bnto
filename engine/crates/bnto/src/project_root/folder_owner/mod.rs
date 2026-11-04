//! Whether a folder on the walk could hold a `.bnto` someone else planted:
//! git's `safe.directory` lesson (CVE-2022-24765). Each OS reads who owns
//! the folder; any error counts as unsafe, so the user just gets no project
//! recipes.

#[cfg(unix)]
mod filesystem;
#[cfg(unix)]
mod primary_users;
#[cfg(unix)]
pub(super) mod private_group;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub(super) use unix::is_safe_folder;
#[cfg(windows)]
pub(super) use windows::is_safe_folder;

/// What a folder is to the walk, which decides whether the sticky bit
/// excuses others' write access (Unix only; Windows checks the owner alone).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Role {
    /// A folder the walk climbs through: sticky stops others replacing a
    /// `.bnto` the user made, so shared write is fine there.
    Visited,
    /// `.bnto` or `.bnto/recipes`, whose files load: sticky still lets
    /// others add files, so any write but the owner's (or the private
    /// group's) is unsafe.
    Loaded,
}

/// No owner to read here: never safe.
#[cfg(not(any(unix, windows)))]
pub(super) fn is_safe_folder(_path: &std::path::Path, _role: Role) -> bool {
    false
}
