// `search_path` — the folders a shell-run looks in for its command.
//
// `command` stays a bare name (validate.rs refuses paths and shells). When a
// recipe sets `search_path`, the command is found in those folders only, in
// order, and run by its full path; PATH is not consulted, so a wrong folder
// fails loudly instead of running some other copy. The child's environment
// (PATH included) is untouched. Relative folders are under the work dir.

use std::path::{Path, PathBuf};

use serde_json::Value;

/// `search_path` as a list of folders: a list of text, or one string.
/// Missing or empty means "use PATH as usual".
pub fn read_search_path(value: Option<&Value>) -> Result<Vec<String>, String> {
    let refused = || "'search_path' must be a list of folder paths".to_string();
    let folders = match value {
        None | Some(Value::Null) => return Ok(Vec::new()),
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Array(items)) => items
            .iter()
            .map(|v| v.as_str().map(str::to_string).ok_or_else(refused))
            .collect::<Result<_, _>>()?,
        Some(_) => return Err(refused()),
    };
    if folders.iter().any(|f| f.trim().is_empty()) {
        return Err("'search_path' has an empty folder".to_string());
    }
    Ok(folders)
}

/// The full path of `command` in the first of `folders` that holds it, or
/// `None` when `folders` is empty (resolve through PATH as usual).
pub fn resolve_command(
    command: &str,
    folders: &[String],
    work_dir: Option<&Path>,
) -> Result<Option<PathBuf>, String> {
    if folders.is_empty() {
        return Ok(None);
    }
    let dirs: Vec<PathBuf> = folders.iter().map(|f| under(work_dir, f)).collect();
    dirs.iter()
        .flat_map(|dir| candidates(command).map(move |name| dir.join(name)))
        .find(|path| path.is_file())
        .map(Some)
        .ok_or_else(|| not_found(command, &dirs))
}

/// `folder`, or `folder` under the work dir when it's relative.
fn under(work_dir: Option<&Path>, folder: &str) -> PathBuf {
    match work_dir {
        Some(base) if Path::new(folder).is_relative() => base.join(folder),
        _ => PathBuf::from(folder),
    }
}

/// The file names `command` may have: itself, and on Windows, when it has
/// no extension, the program extensions Windows would try.
fn candidates(command: &str) -> impl Iterator<Item = String> + '_ {
    let bare = cfg!(windows) && Path::new(command).extension().is_none();
    let extensions: &[&str] = if bare {
        WINDOWS_PROGRAM_EXTENSIONS
    } else {
        &[]
    };
    std::iter::once(command.to_string())
        .chain(extensions.iter().map(move |ext| format!("{command}{ext}")))
}

/// What Windows' default PATHEXT tries for a bare program name, in order.
const WINDOWS_PROGRAM_EXTENSIONS: &[&str] = &[".com", ".exe", ".bat", ".cmd"];

fn not_found(command: &str, dirs: &[PathBuf]) -> String {
    let listed: Vec<String> = dirs.iter().map(|d| d.display().to_string()).collect();
    format!(
        "'{command}' was not found in search_path: {}",
        listed.join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn touch(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        std::fs::write(&path, b"").unwrap();
        path
    }

    fn folders(dirs: &[&Path]) -> Vec<String> {
        dirs.iter()
            .map(|d| d.to_string_lossy().into_owned())
            .collect()
    }

    // --- Reading the param ---

    #[test]
    fn test_read_accepts_a_list_or_one_string() {
        assert_eq!(read_search_path(None).unwrap(), Vec::<String>::new());
        assert_eq!(read_search_path(Some(&json!("a"))).unwrap(), ["a"]);
        assert_eq!(
            read_search_path(Some(&json!(["a", "b"]))).unwrap(),
            ["a", "b"]
        );
    }

    #[test]
    fn test_read_refuses_anything_but_text_folders() {
        for bad in [json!(3), json!([1]), json!(["a", ""]), json!({ "a": 1 })] {
            let err = read_search_path(Some(&bad)).unwrap_err();
            assert!(err.contains("search_path"), "{bad}: {err}");
        }
    }

    // --- Resolving ---

    #[test]
    fn test_no_folders_means_path_as_usual() {
        assert_eq!(resolve_command("git", &[], None).unwrap(), None);
    }

    #[test]
    fn test_finds_the_command_in_the_first_folder_that_has_it() {
        let (a, b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let in_b = touch(b.path(), "tool.bat");
        let found = resolve_command("tool.bat", &folders(&[a.path(), b.path()]), None);
        assert_eq!(found.unwrap(), Some(in_b));
        let in_a = touch(a.path(), "tool.bat");
        let found = resolve_command("tool.bat", &folders(&[a.path(), b.path()]), None);
        assert_eq!(found.unwrap(), Some(in_a));
    }

    #[test]
    fn test_a_command_in_no_folder_fails_naming_it_and_the_folders() {
        let a = tempfile::tempdir().unwrap();
        let err = resolve_command("tool.bat", &folders(&[a.path()]), None).unwrap_err();
        assert!(err.contains("tool.bat"), "{err}");
        assert!(
            err.contains(&a.path().to_string_lossy().into_owned()),
            "{err}"
        );
    }

    #[test]
    fn test_a_folder_named_like_the_command_is_not_a_match() {
        let a = tempfile::tempdir().unwrap();
        std::fs::create_dir(a.path().join("tool.bat")).unwrap();
        assert!(resolve_command("tool.bat", &folders(&[a.path()]), None).is_err());
    }

    #[test]
    fn test_a_relative_folder_is_under_the_work_dir() {
        let work = tempfile::tempdir().unwrap();
        std::fs::create_dir(work.path().join("bin")).unwrap();
        let tool = touch(&work.path().join("bin"), "tool.bat");
        let found = resolve_command("tool.bat", &["bin".to_string()], Some(work.path()));
        assert_eq!(found.unwrap(), Some(tool));
    }

    #[cfg(windows)]
    #[test]
    fn test_a_bare_name_finds_its_windows_program_file() {
        let a = tempfile::tempdir().unwrap();
        let bat = touch(a.path(), "tool.bat");
        let found = resolve_command("tool", &folders(&[a.path()]), None);
        assert_eq!(found.unwrap(), Some(bat));
    }

    #[cfg(unix)]
    #[test]
    fn test_a_bare_name_matches_only_that_file_on_unix() {
        let a = tempfile::tempdir().unwrap();
        touch(a.path(), "tool.sh");
        assert!(resolve_command("tool", &folders(&[a.path()]), None).is_err());
        let tool = touch(a.path(), "tool");
        assert_eq!(
            resolve_command("tool", &folders(&[a.path()]), None).unwrap(),
            Some(tool)
        );
    }
}
