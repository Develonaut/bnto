//! Finds the project root (design §2 of `project-recipes.md`, step 7d): the
//! nearest folder at or above the working folder holding a real
//! `.bnto/recipes/`, never passing a `.git`, the user's home, a folder
//! someone else could have planted, or anything that can't be read.
//!
//! Choices the design left open: the start is canonicalized first, so the
//! walk climbs real folders, not a symlinked path's spelling. Folders are
//! compared by identity (`same-file`), which settles case, `/var` vs
//! `/private/var` and trailing slashes alike; a folder that can't be opened
//! for that stops the walk. A `.bnto` or `.bnto/recipes` that isn't a real
//! folder (a file, a symlink) is passed over and the walk goes on up; one
//! that isn't safe (`folder_owner`) stops it, as an unsafe visited folder
//! does. The filesystem root is never checked, on any OS, and on Unix the
//! walk never leaves the start folder's filesystem (git's default).

mod folder_owner;

use std::io;
use std::path::{Path, PathBuf};

use same_file::Handle;

use folder_owner::Role;

/// The project root for a run started in `start`, given the user's home
/// folder and the bnto home; `None` when the walk stops first.
pub(crate) fn project_root(start: &Path, home: &Path, bnto_home: &Path) -> Option<PathBuf> {
    let mut dir = dunce::canonicalize(start).ok()?;
    let walk = Walk::new(&dir, home, bnto_home)?;
    loop {
        match walk.visit(&dir) {
            Visit::Root => return Some(dir),
            Visit::Stop => return None,
            Visit::Up => dir = dir.parent()?.to_path_buf(),
        }
    }
}

/// What one folder of the walk decides.
#[derive(Debug, PartialEq)]
enum Visit {
    /// This folder is the project root.
    Root,
    /// Not here; try the parent.
    Up,
    /// Load nothing from here or above.
    Stop,
}

/// The folders the walk must recognize, opened once.
struct Walk {
    /// The start folder's filesystem: the walk never leaves it.
    device: u64,
    home: Handle,
    bnto_home: Handle,
    /// `None` when the bnto home has no `recipes` folder yet.
    library: Option<Handle>,
}

impl Walk {
    /// `None` when home or the bnto home can't be opened: the walk couldn't
    /// tell them apart from the folders it visits, so it finds nothing.
    fn new(start: &Path, home: &Path, bnto_home: &Path) -> Option<Self> {
        Some(Self {
            device: device_of(start).ok()?,
            home: Handle::from_path(home).ok()?,
            bnto_home: Handle::from_path(bnto_home).ok()?,
            library: Handle::from_path(bnto_home.join("recipes")).ok(),
        })
    }

    /// Check `dir` as a project root, then whether the walk may go past it.
    fn visit(&self, dir: &Path) -> Visit {
        if self.is_boundary(dir) {
            return Visit::Stop;
        }
        match self.candidate(dir) {
            Visit::Up if holds_no_git(dir) => Visit::Up,
            Visit::Up => Visit::Stop,
            decided => decided,
        }
    }

    /// The filesystem root, another filesystem than the start's, home, a
    /// folder that can't be opened, or one someone else could have planted
    /// a `.bnto` in.
    fn is_boundary(&self, dir: &Path) -> bool {
        dir.parent().is_none()
            || device_of(dir).ok() != Some(self.device)
            || is(Some(&self.home), dir) != Some(false)
            || !folder_owner::is_safe_folder(dir, Role::Visited)
    }

    /// Whether `dir`'s `.bnto/recipes` makes it the project root.
    fn candidate(&self, dir: &Path) -> Visit {
        let dot = dir.join(".bnto");
        match real_folder(&dot) {
            Ok(true) if is(Some(&self.bnto_home), &dot) == Some(false) => {}
            Ok(false) => return Visit::Up,
            _ => return Visit::Stop,
        }
        let recipes = dot.join("recipes");
        match real_folder(&recipes) {
            Ok(true) if is(self.library.as_ref(), &recipes) == Some(false) => Visit::Root,
            Ok(false) => Visit::Up,
            _ => Visit::Stop,
        }
    }
}

/// Whether `path` is the folder `target` holds; `None` when `path` can't be
/// opened to tell.
fn is(target: Option<&Handle>, path: &Path) -> Option<bool> {
    let handle = Handle::from_path(path).ok()?;
    Some(target == Some(&handle))
}

/// Whether `path` is a real folder (not a symlink, not a file; missing is
/// `false`). One someone else could have planted is an error: it stops the
/// walk rather than being passed over.
fn real_folder(path: &Path) -> io::Result<bool> {
    match std::fs::symlink_metadata(path) {
        Ok(meta) if !meta.is_dir() => Ok(false),
        Ok(_) if folder_owner::is_safe_folder(path, Role::Loaded) => Ok(true),
        Ok(_) => Err(io::Error::from(io::ErrorKind::PermissionDenied)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

/// The filesystem `path` is on (`st_dev`), as git compares it when it won't
/// discover across filesystems.
#[cfg(unix)]
fn device_of(path: &Path) -> io::Result<u64> {
    use std::os::unix::fs::MetadataExt;
    Ok(std::fs::symlink_metadata(path)?.dev())
}

/// Windows' volume serial number is unstable in std (`volume_serial_number`),
/// so every folder counts as on the start's volume.
#[cfg(not(unix))]
fn device_of(_path: &Path) -> io::Result<u64> {
    Ok(0)
}

/// Whether `dir` has no `.git` (a folder, or a file for worktrees and
/// submodules) and nothing stopped the check.
fn holds_no_git(dir: &Path) -> bool {
    matches!(
        std::fs::symlink_metadata(dir.join(".git")),
        Err(e) if e.kind() == io::ErrorKind::NotFound
    )
}

#[cfg(test)]
mod tests;
