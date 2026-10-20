// The destination folder file-move moves under, and the checks tied to it:
// a target's folder must stay inside it, and a source must share its drive.

use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};

/// The destination as a canonical path (it may not exist yet).
pub(super) struct Root {
    pub path: PathBuf,
    create_dirs: bool,
    volume: Option<String>,
    /// Folder → its check result, so thousands of files in one folder cost
    /// one walk.
    folders: HashMap<PathBuf, Result<(), String>>,
}

impl Root {
    /// Resolve `destination`. A missing one is fine when `createDirs` is on:
    /// the nearest folder above it that exists is canonicalized instead.
    pub(super) fn resolve(destination: &str, create_dirs: bool) -> Result<Self, String> {
        let abs = std::path::absolute(destination)
            .map_err(|e| format!("destination {destination} can't be resolved: {e}"))?;
        let path = if abs.exists() {
            if !abs.is_dir() {
                return Err(format!("destination {} is not a folder", abs.display()));
            }
            canonical(&abs)?
        } else if create_dirs {
            canonical_missing(&abs)?
        } else {
            return Err(format!(
                "destination {} doesn't exist and createDirs is off",
                abs.display()
            ));
        };
        Ok(Self {
            volume: volume(&path),
            path,
            create_dirs,
            folders: HashMap::new(),
        })
    }

    /// Fail when `src` is on another drive: a move there would be a copy.
    pub(super) fn check_volume(&self, src: &Path) -> Result<(), String> {
        match (&self.volume, volume(src)) {
            (Some(root), Some(src_volume)) if *root != src_volume => Err(format!(
                "{} is on a different drive from {}; file-move only renames, it never copies",
                src.display(),
                self.path.display()
            )),
            _ => Ok(()),
        }
    }

    /// Fail when `dest`'s folder can't be used: it would be created with
    /// `createDirs` off, a file sits where a folder should, or a link or
    /// junction inside the destination leads outside it.
    pub(super) fn check_folder(&mut self, dest: &Path) -> Result<(), String> {
        let folder = dest.parent().unwrap_or(&self.path).to_path_buf();
        if let Some(cached) = self.folders.get(&folder) {
            return cached.clone();
        }
        let result = self.walk_folder(&folder);
        self.folders.insert(folder, result.clone());
        result
    }

    fn walk_folder(&self, folder: &Path) -> Result<(), String> {
        // The nearest part of the path that exists, at or below the root.
        let existing = folder
            .ancestors()
            .take_while(|a| a.starts_with(&self.path))
            .find(|a| a.exists());
        let Some(existing) = existing else {
            return Ok(()); // the root itself is new, so is everything under it
        };
        if !existing.is_dir() {
            return Err(format!("{} is a file, not a folder", existing.display()));
        }
        if existing != folder && !self.create_dirs {
            return Err(format!(
                "folder {} doesn't exist and createDirs is off",
                folder.display()
            ));
        }
        if !canonical(existing)?.starts_with(&self.path) {
            return Err(format!(
                "{} leads outside {} (a link or junction)",
                existing.display(),
                self.path.display()
            ));
        }
        Ok(())
    }
}

fn canonical(path: &Path) -> Result<PathBuf, String> {
    std::fs::canonicalize(path).map_err(|e| format!("{} can't be resolved: {e}", path.display()))
}

/// Canonicalize the nearest folder above `abs` that exists and re-append the
/// missing rest, which may not climb with "..".
fn canonical_missing(abs: &Path) -> Result<PathBuf, String> {
    let base = abs
        .ancestors()
        .find(|a| a.exists())
        .ok_or_else(|| format!("no part of {} exists", abs.display()))?;
    if !base.is_dir() {
        return Err(format!("{} is a file, not a folder", base.display()));
    }
    let rest = abs.strip_prefix(base).unwrap_or(Path::new(""));
    let mut path = canonical(base)?;
    for part in rest.components() {
        match part {
            Component::Normal(name) => path.push(name),
            _ => return Err(format!("destination {} climbs with \"..\"", abs.display())),
        }
    }
    Ok(path)
}

/// The drive a canonical path is on: its prefix on Windows (`D`, or
/// `\\server\share`). A folder mounted inside a drive isn't seen here; a move
/// across one fails at rename and never copies.
#[cfg(windows)]
fn volume(path: &Path) -> Option<String> {
    use std::path::Prefix;
    let Some(Component::Prefix(prefix)) = path.components().next() else {
        return None;
    };
    Some(match prefix.kind() {
        Prefix::Disk(d) | Prefix::VerbatimDisk(d) => (d as char).to_ascii_uppercase().to_string(),
        Prefix::UNC(server, share) | Prefix::VerbatimUNC(server, share) => format!(
            r"\\{}\{}",
            server.to_string_lossy(),
            share.to_string_lossy()
        )
        .to_lowercase(),
        _ => prefix.as_os_str().to_string_lossy().to_lowercase(),
    })
}

/// The device of the nearest part of `path` that exists.
#[cfg(unix)]
fn volume(path: &Path) -> Option<String> {
    use std::os::unix::fs::MetadataExt;
    let existing = path.ancestors().find(|a| a.exists())?;
    std::fs::metadata(existing)
        .ok()
        .map(|m| m.dev().to_string())
}

#[cfg(not(any(unix, windows)))]
fn volume(_path: &Path) -> Option<String> {
    None
}
