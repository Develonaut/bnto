// Why a path a recipe writes needs the user's eye in the consent prompt: it
// names a folder outside the project, or one nobody can see until it runs.
// Read the same on every OS, so a recipe is judged as it would run anywhere.

use std::path::Path;

use crate::path_placeholders::{literal_folder, only_safe_placeholders};

/// What makes a path param worth a second look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathFlag {
    /// Starts at a root, a `\`, or a drive letter (`C:`), on any OS.
    Absolute,
    /// Holds a `..` segment.
    ParentDir,
    /// Starts with `~`, the user's home folder in a shell.
    Home,
    /// Relative, but lands outside the project root (through a link).
    LeavesProject,
    /// Holds a placeholder filled when it runs that can name any folder
    /// (`{{env.*}}`, `{{ctx.paths.*}}`, a node's output, …).
    SetWhenRun,
    /// The output node's overwrite mode: results replace the input files.
    Overwrite,
    /// A write that replaces a file already at its destination.
    ReplacesExisting,
    /// A mode filled when it runs, which may make it replace files.
    MayReplace,
    /// A segment named `.git`: a repo's hooks there run code.
    GitFolder,
    /// A segment named `.bnto`: bnto's own recipes and state.
    BntoFolder,
    /// A part of a file name holding `/` or `\\`, which puts files in other
    /// folders.
    AddsFolder,
    /// Text filled from a regex match (`$1`), which may hold a path.
    FromMatch,
    /// A find and replace over a file's whole relative path, which can
    /// build `..` or `.git` from its parts.
    EditsPath,
}

impl PathFlag {
    /// The reason, as the prompt shows it after the value.
    pub fn describe(self) -> &'static str {
        match self {
            Self::Absolute => "absolute",
            Self::ParentDir => "goes up with ..",
            Self::Home => "your home folder",
            Self::LeavesProject => "outside the project",
            Self::SetWhenRun => "set when it runs, may be any folder",
            Self::Overwrite => "replaces the input files",
            Self::ReplacesExisting => "replaces files already there",
            Self::MayReplace => "set when it runs, may replace files",
            Self::GitFolder => "inside .git (can run code)",
            Self::BntoFolder => "bnto's own folder",
            Self::AddsFolder => "puts files in other folders",
            Self::FromMatch => "filled from the match, may hold a path",
            Self::EditsPath => "edits the whole path, may move files to other folders",
        }
    }
}

/// The flag a folder param's `value` earns, if any. `root` is the project
/// root, when the recipe is a project's: the folder fixed by its literal
/// part is then followed through links on disk to see whether it stays
/// inside. A placeholder that can hold a path flags it (`path_placeholders`).
pub fn path_flag(value: &str, root: Option<&Path>) -> Option<PathFlag> {
    let value = value.trim();
    let fixed = literal_folder(value);
    leaves_folder(value)
        .or_else(|| value.starts_with('~').then_some(PathFlag::Home))
        .or_else(|| {
            let outside = root.is_some_and(|r| !fixed.is_empty() && leaves_root(r, fixed));
            outside.then_some(PathFlag::LeavesProject)
        })
        .or_else(|| (!only_safe_placeholders(value, false)).then_some(PathFlag::SetWhenRun))
}

/// The flag an output name param's `value` (a file name, which may hold
/// folders) earns: one that can put a file outside the output folder. The
/// file's own name tokens (`{{name}}`) don't flag it.
pub fn name_flag(value: &str) -> Option<PathFlag> {
    let value = value.trim();
    leaves_folder(value)
        .or_else(|| (!only_safe_placeholders(value, true)).then_some(PathFlag::SetWhenRun))
}

/// Absolute (on any OS), climbing with `..`, or into `.git` or `.bnto`
/// (any case: macOS and Windows read `.GIT` as `.git`).
fn leaves_folder(value: &str) -> Option<PathFlag> {
    let segment = |name: &str| {
        value
            .split(['/', '\\'])
            .any(|s| s.eq_ignore_ascii_case(name))
    };
    if is_absolute_anywhere(value) {
        Some(PathFlag::Absolute)
    } else if segment("..") {
        Some(PathFlag::ParentDir)
    } else if segment(".git") {
        Some(PathFlag::GitFolder)
    } else if segment(".bnto") {
        Some(PathFlag::BntoFolder)
    } else {
        None
    }
}

/// The flag a part of a file name earns (`file-rename`'s `prefix`, `suffix`,
/// `separator`, `extension` or `replace`): any name flag, a separator,
/// which moves the file into another folder, or for a `replacement`, a `$`
/// group reference, filled from the matched path.
pub fn fragment_flag(value: &str, replacement: bool) -> Option<PathFlag> {
    let group = |w: &[char]| w[0] == '$' && (w[1].is_alphanumeric() || w[1] == '{' || w[1] == '_');
    let chars: Vec<char> = value.chars().collect();
    let separator = value.contains(['/', '\\']);
    let climbs = value.split(['/', '\\']).any(|s| s == "..");
    climbs
        .then_some(PathFlag::ParentDir)
        .or_else(|| separator.then_some(PathFlag::AddsFolder))
        .or_else(|| name_flag(value))
        .or_else(|| (replacement && chars.windows(2).any(group)).then_some(PathFlag::FromMatch))
}

/// Whether `value` is a path under the folder a step runs in: not absolute,
/// not under `~`, and not filled when it runs.
pub fn is_relative(value: &str) -> bool {
    let value = value.trim();
    !is_absolute_anywhere(value) && !value.starts_with('~') && !value.starts_with("{{")
}

/// `/x`, `\x`, `C:x` or `C:\x`, whichever OS reads it.
fn is_absolute_anywhere(value: &str) -> bool {
    let bytes = value.as_bytes();
    let drive = bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
    value.starts_with(['/', '\\']) || drive || Path::new(value).is_absolute()
}

/// Whether `root/value`, through any link on the way, lands outside `root`.
/// Only the part that exists can be followed; the rest is taken as written.
/// A root that can't be read says nothing.
fn leaves_root(root: &Path, value: &str) -> bool {
    let Ok(root) = std::fs::canonicalize(root) else {
        return false;
    };
    let target = root.join(value);
    let Some(existing) = target.ancestors().find(|a| a.exists()) else {
        return false;
    };
    std::fs::canonicalize(existing).is_ok_and(|real| !real.starts_with(&root))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_relative_folder_is_not_flagged() {
        for value in ["assets", "./build/out", "a..b/c", "{{ctx.date}}-out"] {
            assert_eq!(path_flag(value, None), None, "{value}");
        }
    }

    #[test]
    fn absolute_paths_are_flagged_on_every_os() {
        for value in ["/Users/me", "\\\\server\\share", "C:\\Users", "c:tools"] {
            assert_eq!(path_flag(value, None), Some(PathFlag::Absolute), "{value}");
        }
    }

    #[test]
    fn a_parent_segment_is_flagged() {
        for value in ["..", "../x", "a/../../b", "a\\..\\b"] {
            assert_eq!(path_flag(value, None), Some(PathFlag::ParentDir), "{value}");
        }
    }

    #[test]
    fn a_leading_tilde_is_flagged() {
        assert_eq!(path_flag("~", None), Some(PathFlag::Home));
        assert_eq!(path_flag("~/Documents", None), Some(PathFlag::Home));
    }

    #[test]
    fn a_value_filled_from_outside_the_recipe_is_flagged() {
        for value in ["{{env.HOME}}", "{{ fields.folder }}/x", "{{vars.out}}"] {
            assert_eq!(
                path_flag(value, None),
                Some(PathFlag::SetWhenRun),
                "{value}"
            );
        }
    }

    #[test]
    fn any_placeholder_that_can_hold_a_path_is_flagged_wherever_it_sits() {
        for value in [
            "{{ctx.paths.home_dir}}",
            "{{ctx.paths.temp_dir}}/x",
            "{{ctx.paths.work_dir}}",
            "{{ctx.paths.output_dir}}",
            "{{ctx.work_dir}}",
            "{{node.list.path}}",
            "{{item.dir}}",
            "{{loop.folder}}",
            "{{entry.path}}",
            "assets/{{env.X}}",
            "out-{{ctx.date}}/{{fields.f}}",
            "{{name}}",
            "{{unclosed",
        ] {
            assert_eq!(
                path_flag(value, None),
                Some(PathFlag::SetWhenRun),
                "{value}"
            );
        }
    }

    #[test]
    fn placeholders_that_cant_hold_a_separator_are_not_flagged() {
        for value in [
            "{{ctx.date}}-out",
            "out/{{ ctx.time }}",
            "{{ctx.timestamp}}_{{ctx.platform}}",
            "runs/{{ctx.started}}",
        ] {
            assert_eq!(path_flag(value, None), None, "{value}");
        }
    }

    #[test]
    fn an_output_name_is_flagged_only_when_it_can_leave_its_folder() {
        assert_eq!(name_flag("compressed-{{name}}"), None);
        assert_eq!(name_flag("{{name}}.{{ext}}"), None);
        assert_eq!(name_flag("sub/{{ctx.date}}.csv"), None);
        assert_eq!(name_flag("../../home/zshrc"), Some(PathFlag::ParentDir));
        assert_eq!(name_flag("/etc/x"), Some(PathFlag::Absolute));
        assert_eq!(name_flag("C:x"), Some(PathFlag::Absolute));
        assert_eq!(name_flag("{{env.X}}-{{name}}"), Some(PathFlag::SetWhenRun));
        assert_eq!(name_flag("{{item.path}}"), Some(PathFlag::SetWhenRun));
    }

    #[test]
    fn a_path_into_git_or_bntos_own_folder_is_flagged() {
        for value in [".git/hooks", "a/.GIT/config", "x\\.Git"] {
            assert_eq!(path_flag(value, None), Some(PathFlag::GitFolder), "{value}");
            assert_eq!(name_flag(value), Some(PathFlag::GitFolder), "{value}");
        }
        for value in [".bnto/recipes", "a/.BNTO"] {
            assert_eq!(
                path_flag(value, None),
                Some(PathFlag::BntoFolder),
                "{value}"
            );
        }
        assert_eq!(path_flag(".github/x", None), None);
        assert_eq!(path_flag("a.git/x", None), None);
    }

    #[test]
    fn a_name_fragment_holding_a_separator_is_flagged() {
        assert_eq!(fragment_flag("_v2", false), None);
        assert_eq!(fragment_flag("sub/", false), Some(PathFlag::AddsFolder));
        assert_eq!(fragment_flag("a\\b", false), Some(PathFlag::AddsFolder));
        assert_eq!(fragment_flag("../x_", false), Some(PathFlag::ParentDir));
        assert_eq!(fragment_flag("$1-x", true), Some(PathFlag::FromMatch));
        assert_eq!(fragment_flag("${name}", true), Some(PathFlag::FromMatch));
        assert_eq!(fragment_flag("$1-x", false), None);
    }

    #[test]
    fn a_folder_inside_the_project_is_not_flagged() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir(root.path().join("assets")).unwrap();
        assert_eq!(path_flag("assets", Some(root.path())), None);
        assert_eq!(path_flag("assets/new/deeper", Some(root.path())), None);
        assert_eq!(path_flag("missing", Some(root.path())), None);
    }

    #[cfg(unix)]
    #[test]
    fn a_link_out_of_the_project_is_flagged() {
        let root = tempfile::tempdir().unwrap();
        let elsewhere = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(elsewhere.path(), root.path().join("out")).unwrap();
        assert_eq!(
            path_flag("out", Some(root.path())),
            Some(PathFlag::LeavesProject)
        );
        assert_eq!(
            path_flag("out/sub", Some(root.path())),
            Some(PathFlag::LeavesProject)
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_templated_value_still_has_its_literal_folder_followed() {
        let root = tempfile::tempdir().unwrap();
        let elsewhere = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(elsewhere.path(), root.path().join("out")).unwrap();
        for value in ["out/{{ctx.date}}", "out/{{env.X}}", "out/x-{{ctx.date}}"] {
            assert_eq!(
                path_flag(value, Some(root.path())),
                Some(PathFlag::LeavesProject),
                "{value}"
            );
        }
    }
}
