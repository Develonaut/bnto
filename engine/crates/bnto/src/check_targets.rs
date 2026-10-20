// `bnto check` arguments resolved to recipe files. A file is itself; a folder
// is every `.bnto.json` under it, recursively, sorted by path, so scripts on a
// shell that expands no globs (Windows) still check every recipe.

use std::path::{Path, PathBuf};

use bnto_core::slash_path;

/// What `bnto check` looks for when it walks a folder.
const RECIPE_SUFFIX: &str = ".bnto.json";

/// Folders a walk never enters: installed packages and Cargo build output
/// aren't the project's recipes. Naming one on the command line still checks it.
const SKIPPED_FOLDERS: [&str; 2] = ["node_modules", "target"];

/// One thing `bnto check` reports on.
#[derive(Debug, PartialEq)]
pub enum CheckTarget {
    /// A recipe file to check: where to read it, and the name findings print.
    Recipe { path: PathBuf, name: String },
    /// A finding about the argument itself, already prefixed with its name.
    Finding(String),
}

/// Resolve one `bnto check` argument. A folder that holds no recipes is a
/// finding naming it, so a wrong path never passes silently.
pub fn check_targets(arg: &str) -> Vec<CheckTarget> {
    let path = Path::new(arg);
    if !path.is_dir() {
        return vec![CheckTarget::Recipe {
            path: path.to_path_buf(),
            name: arg.to_string(),
        }];
    }
    let shown = arg.trim_end_matches(['/', std::path::MAIN_SEPARATOR]);
    let mut targets = Vec::new();
    walk(path, Path::new(""), shown, &mut targets);
    if targets.is_empty() {
        targets.push(CheckTarget::Finding(format!(
            "{shown}: no {RECIPE_SUFFIX} recipes in this folder"
        )));
    }
    targets
}

/// Push the recipes under `root/rel` onto `out`, entries sorted by name.
/// Symlinked folders aren't followed, so a link loop can't recurse forever.
fn walk(root: &Path, rel: &Path, shown: &str, out: &mut Vec<CheckTarget>) {
    let entries = match sorted_entries(&root.join(rel)) {
        Ok(entries) => entries,
        Err(e) => return out.push(unreadable_folder(shown, rel, &e)),
    };
    for (name, is_dir) in entries {
        let child = rel.join(&name);
        if is_dir && !SKIPPED_FOLDERS.contains(&name.as_str()) {
            walk(root, &child, shown, out);
        } else if !is_dir && name.ends_with(RECIPE_SUFFIX) {
            out.push(CheckTarget::Recipe {
                path: root.join(&child),
                name: display_name(shown, &child),
            });
        }
    }
}

/// A folder the walk can't list is a finding, so the rest still get checked.
fn unreadable_folder(shown: &str, rel: &Path, e: &std::io::Error) -> CheckTarget {
    let name = display_name(shown, rel);
    CheckTarget::Finding(format!("{name}: cannot read folder: {e}"))
}

/// `dir`'s entries as (file name, is a real folder), sorted by name so the
/// order is the same on every OS.
fn sorted_entries(dir: &Path) -> std::io::Result<Vec<(String, bool)>> {
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let is_dir = entry.file_type()?.is_dir();
        entries.push((entry.file_name().to_string_lossy().into_owned(), is_dir));
    }
    entries.sort();
    Ok(entries)
}

/// The folder argument as typed, then the path under it with `/` separators.
fn display_name(shown: &str, rel: &Path) -> String {
    if rel.as_os_str().is_empty() {
        shown.to_string()
    } else {
        format!("{shown}/{}", slash_path(rel))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(targets: &[CheckTarget]) -> Vec<&str> {
        targets
            .iter()
            .map(|t| match t {
                CheckTarget::Recipe { name, .. } => name.as_str(),
                CheckTarget::Finding(line) => line.as_str(),
            })
            .collect()
    }

    fn touch(path: &Path) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, "{}").unwrap();
    }

    #[test]
    fn test_a_file_argument_is_one_recipe_named_as_typed() {
        let targets = check_targets("some/recipe.bnto.json");
        assert_eq!(
            targets,
            [CheckTarget::Recipe {
                path: PathBuf::from("some/recipe.bnto.json"),
                name: "some/recipe.bnto.json".into(),
            }]
        );
    }

    #[test]
    fn test_a_folder_lists_nested_recipes_sorted_with_slash_names() {
        let tmp = tempfile::tempdir().unwrap();
        touch(&tmp.path().join("b.bnto.json"));
        touch(&tmp.path().join("a").join("z").join("c.bnto.json"));
        touch(&tmp.path().join("a.bnto.json"));
        touch(&tmp.path().join("readme.json"));
        let arg = tmp.path().to_string_lossy().into_owned();
        assert_eq!(
            names(&check_targets(&arg)),
            [
                format!("{arg}/a/z/c.bnto.json"),
                format!("{arg}/a.bnto.json"),
                format!("{arg}/b.bnto.json"),
            ]
        );
    }

    #[test]
    fn test_a_recipe_path_reads_from_the_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let file = tmp.path().join("sub").join("r.bnto.json");
        touch(&file);
        let arg = tmp.path().to_string_lossy().into_owned();
        let targets = check_targets(&arg);
        let [CheckTarget::Recipe { path, .. }] = targets.as_slice() else {
            panic!("{targets:?}");
        };
        assert_eq!(path, &file);
    }

    #[test]
    fn test_skipped_folders_are_not_entered() {
        let tmp = tempfile::tempdir().unwrap();
        touch(&tmp.path().join("node_modules").join("x.bnto.json"));
        touch(&tmp.path().join("deep").join("target").join("y.bnto.json"));
        touch(&tmp.path().join("targets").join("z.bnto.json"));
        let arg = tmp.path().to_string_lossy().into_owned();
        assert_eq!(
            names(&check_targets(&arg)),
            [format!("{arg}/targets/z.bnto.json")]
        );
    }

    #[test]
    fn test_a_folder_without_recipes_is_one_finding() {
        let tmp = tempfile::tempdir().unwrap();
        touch(&tmp.path().join("node_modules").join("x.bnto.json"));
        let arg = tmp.path().to_string_lossy().into_owned();
        assert_eq!(
            check_targets(&arg),
            [CheckTarget::Finding(format!(
                "{arg}: no .bnto.json recipes in this folder"
            ))]
        );
    }

    #[test]
    fn test_a_trailing_separator_is_dropped_from_names() {
        let tmp = tempfile::tempdir().unwrap();
        touch(&tmp.path().join("r.bnto.json"));
        let arg = tmp.path().to_string_lossy().into_owned();
        let typed = format!("{arg}{}", std::path::MAIN_SEPARATOR);
        assert_eq!(
            names(&check_targets(&typed)),
            [format!("{arg}/r.bnto.json")]
        );
    }
}
