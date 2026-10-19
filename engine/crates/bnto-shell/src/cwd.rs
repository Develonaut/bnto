// `cwd` — the folder a shell-command's command runs in.
//
// A folder under the work dir, so a tool that reads config from where it
// starts (rustup's rust-toolchain.toml, a package.json) sees that folder's.
// Absolute paths, drive letters and ".." are refused: a recipe can only move
// its command deeper into the project, never out of it. `search_path` and
// `{{output_dir}}` don't change; relative args are now relative to `cwd`.

use std::path::{Path, PathBuf};

use serde_json::Value;

/// `cwd` as written, or `None` when it's unset or empty (run in the work dir).
pub fn read_cwd(value: Option<&Value>) -> Result<Option<&str>, String> {
    let folder = match value {
        None | Some(Value::Null) => return Ok(None),
        Some(Value::String(s)) if s.trim().is_empty() => return Ok(None),
        Some(Value::String(s)) => s.as_str(),
        Some(_) => return Err("'cwd' must be a folder path".to_string()),
    };
    if leaves_work_dir(folder) {
        return Err(format!(
            "'cwd' must be a folder under the working directory, not '{folder}' \
             (absolute, a drive, or \"..\")"
        ));
    }
    Ok(Some(folder))
}

/// `folder` under `work_dir`, which must be an existing folder.
pub fn resolve_cwd(folder: &str, work_dir: &Path) -> Result<PathBuf, String> {
    let dir = work_dir.join(folder);
    if dir.is_dir() {
        Ok(dir)
    } else {
        Err(format!(
            "'cwd' folder '{folder}' doesn't exist under {}",
            work_dir.display()
        ))
    }
}

/// Absolute paths, drive letters (on every OS, so a recipe means the same
/// thing wherever it runs) and `..` segments all leave the work dir.
fn leaves_work_dir(folder: &str) -> bool {
    let bytes = folder.as_bytes();
    let drive = bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
    folder.starts_with(['/', '\\'])
        || drive
        || Path::new(folder).is_absolute()
        || folder.split(['/', '\\']).any(|s| s == "..")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_unset_null_or_empty_runs_in_the_work_dir() {
        assert_eq!(read_cwd(None), Ok(None));
        assert_eq!(read_cwd(Some(&Value::Null)), Ok(None));
        assert_eq!(read_cwd(Some(&json!(""))), Ok(None));
        assert_eq!(read_cwd(Some(&json!("  "))), Ok(None));
    }

    #[test]
    fn test_a_folder_under_the_work_dir_is_read_as_written() {
        assert_eq!(read_cwd(Some(&json!("Tools"))), Ok(Some("Tools")));
        assert_eq!(read_cwd(Some(&json!("a/b\\c"))), Ok(Some("a/b\\c")));
        assert_eq!(read_cwd(Some(&json!("./Tools"))), Ok(Some("./Tools")));
    }

    #[test]
    fn test_a_dotted_name_is_not_a_climb() {
        assert_eq!(read_cwd(Some(&json!("..hidden"))), Ok(Some("..hidden")));
        assert_eq!(read_cwd(Some(&json!("a..b/c"))), Ok(Some("a..b/c")));
    }

    #[test]
    fn test_folders_that_leave_the_work_dir_are_refused() {
        for folder in [
            "/etc",
            "\\share",
            "C:\\Tools",
            "c:Tools",
            "..",
            "a/../b",
            "a\\..\\b",
        ] {
            let err = read_cwd(Some(&json!(folder))).expect_err(folder);
            assert!(
                err.contains("under the working directory"),
                "{folder}: {err}"
            );
        }
    }

    #[test]
    fn test_a_non_text_cwd_is_refused() {
        let err = read_cwd(Some(&json!(["Tools"]))).unwrap_err();
        assert_eq!(err, "'cwd' must be a folder path");
    }

    #[test]
    fn test_resolve_joins_an_existing_folder_to_the_work_dir() {
        let work = tempfile::tempdir().unwrap();
        std::fs::create_dir(work.path().join("Tools")).unwrap();
        assert_eq!(
            resolve_cwd("Tools", work.path()),
            Ok(work.path().join("Tools"))
        );
    }

    #[test]
    fn test_resolve_refuses_a_missing_folder() {
        let work = tempfile::tempdir().unwrap();
        let err = resolve_cwd("Tools", work.path()).unwrap_err();
        assert!(err.contains("'cwd' folder 'Tools' doesn't exist"), "{err}");
    }

    #[test]
    fn test_resolve_refuses_a_file() {
        let work = tempfile::tempdir().unwrap();
        std::fs::write(work.path().join("Tools"), b"").unwrap();
        assert!(resolve_cwd("Tools", work.path()).is_err());
    }
}
