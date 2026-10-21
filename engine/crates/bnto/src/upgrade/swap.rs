// Windows won't overwrite a running .exe, and `bnto upgrade` is itself the
// running bnto. Renaming a running .exe is allowed, though: move it aside to
// `bnto.old.exe`, let cargo write the new one, and delete the old one on the
// next start. Plain renames, so the logic is tested on every OS.

use std::path::{Path, PathBuf};

/// Where the running binary is moved during an upgrade.
pub fn old_path(exe: &Path) -> PathBuf {
    let name = match exe.extension() {
        Some(ext) => format!("bnto.old.{}", ext.to_string_lossy()),
        None => "bnto.old".to_string(),
    };
    exe.with_file_name(name)
}

/// Move `exe` out of the way, returning where it went.
pub fn move_aside(exe: &Path) -> std::io::Result<PathBuf> {
    let old = old_path(exe);
    let _ = std::fs::remove_file(&old);
    std::fs::rename(exe, &old)?;
    Ok(old)
}

/// Put `old` back as `exe` after a failed upgrade, unless cargo already
/// wrote a new binary there.
pub fn restore(old: &Path, exe: &Path) {
    if !exe.exists() {
        let _ = std::fs::rename(old, exe);
    }
}

/// Delete a binary a previous upgrade moved aside. Called at startup; quiet
/// when there's nothing to remove.
pub fn remove_leftover(exe: &Path) {
    let _ = std::fs::remove_file(old_path(exe));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_old_binary_keeps_its_extension() {
        assert_eq!(
            old_path(Path::new("/c/bin/bnto.exe")),
            Path::new("/c/bin/bnto.old.exe")
        );
        assert_eq!(
            old_path(Path::new("/c/bin/bnto")),
            Path::new("/c/bin/bnto.old")
        );
    }

    #[test]
    fn moving_aside_frees_the_path_and_restore_puts_it_back() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("bnto.exe");
        std::fs::write(&exe, b"v1").unwrap();
        let old = move_aside(&exe).unwrap();
        assert!(!exe.exists());
        restore(&old, &exe);
        assert_eq!(std::fs::read(&exe).unwrap(), b"v1");
    }

    #[test]
    fn restore_never_clobbers_a_new_binary() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("bnto.exe");
        std::fs::write(&exe, b"v1").unwrap();
        let old = move_aside(&exe).unwrap();
        std::fs::write(&exe, b"v2").unwrap();
        restore(&old, &exe);
        assert_eq!(std::fs::read(&exe).unwrap(), b"v2");
    }

    #[test]
    fn a_leftover_old_binary_is_removed() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("bnto.exe");
        std::fs::write(old_path(&exe), b"v1").unwrap();
        remove_leftover(&exe);
        assert!(!old_path(&exe).exists());
        remove_leftover(&exe); // nothing left: still quiet
    }
}
