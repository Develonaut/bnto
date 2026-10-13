//! In-process PATH lookup that answers "would `Command::new(name)` find it?"
//!
//! Dependency checks used to spawn `which` / `where`, but neither matches
//! how `std::process::Command` resolves programs: `where` honours PATHEXT
//! (so `.cmd`/`.bat` shims look installed yet fail to spawn) and searches
//! the CWD first, while `which` is missing on minimal Linux images. This
//! module mirrors std's spawn rules instead, with no child process.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Characters that never appear in a bare program name. Separators would
/// bypass the PATH search; the rest are wildcards or drive markers to
/// Windows lookup tools.
const REJECTED_CHARS: [char; 5] = ['/', '\\', '*', '?', ':'];

/// Resolve `name` against a PATH-style variable the way `Command::new` would.
///
/// Returns the first runnable candidate in PATH order. Only bare names are
/// accepted (no separators or wildcards) and empty PATH entries are
/// skipped, so the result never depends on the current directory.
/// Pure apart from filesystem metadata reads — callers supply `path_var`.
pub fn find_on_path(name: &str, path_var: &OsStr) -> Option<PathBuf> {
    if !is_bare_name(name) {
        return None;
    }
    let file_name = spawn_file_name(name);
    std::env::split_paths(path_var)
        .filter(|dir| !dir.as_os_str().is_empty())
        .map(|dir| dir.join(&file_name))
        .find(|candidate| is_runnable(candidate))
}

fn is_bare_name(name: &str) -> bool {
    !name.is_empty() && name != "." && name != ".." && !name.contains(REJECTED_CHARS)
}

/// On Windows, std appends `.exe` unless the name already ends in it
/// (case-insensitively). PATHEXT is not consulted, so `.cmd` / `.bat`
/// shims are not spawnable by bare name.
#[cfg(windows)]
fn spawn_file_name(name: &str) -> String {
    if name.to_ascii_lowercase().ends_with(".exe") {
        name.to_string()
    } else {
        format!("{name}.exe")
    }
}

#[cfg(not(windows))]
fn spawn_file_name(name: &str) -> String {
    name.to_string()
}

/// Windows: std checks existence without following reparse points, so App
/// Execution Aliases under `WindowsApps` count, including the Microsoft
/// Store `python3` stub, matching what `Command::new` would launch.
#[cfg(windows)]
fn is_runnable(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|m| !m.is_dir())
}

/// Unix: `execvp` skips entries that are not executable regular files.
#[cfg(unix)]
fn is_runnable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

/// Targets without processes (WASM) can never spawn anything.
#[cfg(not(any(unix, windows)))]
fn is_runnable(_path: &Path) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn path_of(dirs: &[&Path]) -> std::ffi::OsString {
        std::env::join_paths(dirs).unwrap()
    }

    /// Create a file that the host can spawn: `<name>.exe` on Windows,
    /// a mode-0755 file on unix.
    fn make_runnable(dir: &Path, name: &str) -> PathBuf {
        let file = dir.join(if cfg!(windows) {
            format!("{name}.exe")
        } else {
            name.to_string()
        });
        std::fs::write(&file, b"").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        file
    }

    #[test]
    fn finds_runnable_binary_on_path() {
        let dir = tempfile::tempdir().unwrap();
        let exe = make_runnable(dir.path(), "tool");
        let found = find_on_path("tool", &path_of(&[dir.path()]));
        assert_eq!(found, Some(exe));
    }

    #[test]
    fn returns_none_when_not_on_path() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(find_on_path("tool", &path_of(&[dir.path()])), None);
    }

    #[test]
    fn returns_first_match_in_path_order() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        make_runnable(second.path(), "tool");
        let expected = make_runnable(first.path(), "tool");
        let path = path_of(&[first.path(), second.path()]);
        assert_eq!(find_on_path("tool", &path), Some(expected));
    }

    #[test]
    fn skips_directories_with_the_binary_name() {
        let dir = tempfile::tempdir().unwrap();
        let name = if cfg!(windows) { "tool.exe" } else { "tool" };
        std::fs::create_dir(dir.path().join(name)).unwrap();
        assert_eq!(find_on_path("tool", &path_of(&[dir.path()])), None);
    }

    #[test]
    fn rejects_names_with_path_separators() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("sub")).unwrap();
        make_runnable(&dir.path().join("sub"), "tool");
        let path = path_of(&[dir.path()]);
        assert_eq!(find_on_path("sub/tool", &path), None);
        assert_eq!(find_on_path("sub\\tool", &path), None);
    }

    #[test]
    fn rejects_empty_and_wildcard_names() {
        let dir = tempfile::tempdir().unwrap();
        make_runnable(dir.path(), "tool");
        let path = path_of(&[dir.path()]);
        for name in ["", ".", "..", "to*", "to?l", "c:tool"] {
            assert_eq!(find_on_path(name, &path), None, "{name:?}");
        }
    }

    #[test]
    fn ignores_empty_path_entries() {
        // An empty entry means "current directory" to some shells; a
        // dependency check must never resolve relative to the CWD.
        assert_eq!(find_on_path("tool", OsStr::new("")), None);
    }

    #[cfg(windows)]
    #[test]
    fn windows_ignores_cmd_and_bat_shims() {
        // `Command::new("npm")` only tries `npm.exe`, so a `.cmd` shim on
        // PATH must not count as found even though `where` reports it.
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("npm.cmd"), b"").unwrap();
        std::fs::write(dir.path().join("npm.bat"), b"").unwrap();
        assert_eq!(find_on_path("npm", &path_of(&[dir.path()])), None);
    }

    #[cfg(windows)]
    #[test]
    fn windows_accepts_explicit_exe_suffix_case_insensitively() {
        let dir = tempfile::tempdir().unwrap();
        let exe = make_runnable(dir.path(), "tool");
        let path = path_of(&[dir.path()]);
        assert_eq!(find_on_path("tool.exe", &path), Some(exe.clone()));
        assert!(find_on_path("TOOL.EXE", &path).is_some());
    }

    #[cfg(windows)]
    #[test]
    fn windows_ignores_extensionless_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("tool"), b"").unwrap();
        assert_eq!(find_on_path("tool", &path_of(&[dir.path()])), None);
    }

    #[cfg(unix)]
    #[test]
    fn unix_skips_non_executable_file() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("tool");
        std::fs::write(&file, b"").unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert_eq!(find_on_path("tool", &path_of(&[dir.path()])), None);
    }

    #[cfg(unix)]
    #[test]
    fn unix_follows_symlinks_to_executables() {
        let dir = tempfile::tempdir().unwrap();
        let real = make_runnable(dir.path(), "real-tool");
        std::os::unix::fs::symlink(&real, dir.path().join("tool")).unwrap();
        assert!(find_on_path("tool", &path_of(&[dir.path()])).is_some());
    }
}
