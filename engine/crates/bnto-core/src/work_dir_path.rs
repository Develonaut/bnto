//! A path a recipe names in a param, placed under the run's working folder.
//!
//! Nodes resolve relative params against `ProcessContext::work_dir`, never
//! the process's own folder, so a run whose working folder isn't the current
//! one (a project recipe run from a subfolder) reads and writes where the
//! recipe means.

use std::path::{Component, Path, PathBuf};

use crate::context::ProcessContext;
use crate::display_text::display_text;
use crate::errors::BntoError;
use crate::leaves_folder::is_absolute_anywhere;

/// How a path param is placed.
#[derive(Debug, PartialEq)]
enum Placement {
    /// A full path on this OS, used as written.
    AsWritten,
    /// Relative: joined onto the run's working folder.
    UnderWorkDir,
    /// Starts at a root or names a drive without being a full path here
    /// (`\x`, `C:x`, `/x` on Windows): joining it would replace or leave the
    /// working folder, and `C:x` reads against the process's folder on C.
    Refused,
}

fn placement(value: &str) -> Placement {
    if Path::new(value).is_absolute() {
        Placement::AsWritten
    } else if is_absolute_anywhere(value) {
        Placement::Refused
    } else {
        Placement::UnderWorkDir
    }
}

/// The `param` value `value` as written when it's a full path, else joined
/// onto the run's working folder, less its `.` segments (`.` is the working
/// folder itself, not `<work>/.`, so paths built on it read as they did when
/// they were relative). One that starts at a root or names a drive but isn't
/// a full path on this OS (Windows' `\x` and `C:x`, read the same on every
/// OS) is refused, naming `param`. Only a relative value asks for the working
/// folder, so an absolute one works in a context that has none (the browser,
/// tests).
pub fn in_work_dir(
    param: &str,
    value: &str,
    ctx: &dyn ProcessContext,
) -> Result<PathBuf, BntoError> {
    let path = Path::new(value);
    match placement(value) {
        Placement::AsWritten => Ok(path.to_path_buf()),
        Placement::Refused => Err(BntoError::InvalidInput(format!(
            "'{param}' \"{}\" starts at a root or names a drive but isn't a full \
             path here; give a full path, or one relative to the working folder",
            display_text(value)
        ))),
        Placement::UnderWorkDir => {
            let mut joined = ctx.work_dir()?.to_path_buf();
            joined.extend(path.components().filter(|c| *c != Component::CurDir));
            Ok(joined)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::NoopContext;

    struct InFolder(PathBuf);

    impl ProcessContext for InFolder {
        fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
            NoopContext.run_command(cmd, args)
        }
        fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
            NoopContext.temp_file(suffix)
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Ok(&self.0)
        }
    }

    #[test]
    fn a_relative_value_is_under_the_work_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let ctx = InFolder(tmp.path().to_path_buf());
        assert_eq!(
            in_work_dir("p", "out/sub", &ctx).unwrap(),
            tmp.path().join("out/sub")
        );
    }

    #[test]
    fn a_dot_is_the_work_dir_itself() {
        let tmp = tempfile::tempdir().unwrap();
        let ctx = InFolder(tmp.path().to_path_buf());
        assert_eq!(in_work_dir("p", ".", &ctx).unwrap(), tmp.path());
        assert_eq!(
            in_work_dir("p", "./out", &ctx).unwrap(),
            tmp.path().join("out")
        );
    }

    #[test]
    fn an_absolute_value_is_kept_without_asking_for_the_work_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let abs = tmp.path().join("x");
        assert_eq!(
            in_work_dir("p", abs.to_str().unwrap(), &NoopContext).unwrap(),
            abs
        );
    }

    #[test]
    fn root_and_drive_relative_values_are_refused_on_every_os() {
        for value in ["\\x", "\\out\\sub", "C:x", "c:out/sub"] {
            assert_eq!(placement(value), Placement::Refused, "{value}");
        }
    }

    #[test]
    fn a_refusal_names_the_param() {
        let tmp = tempfile::tempdir().unwrap();
        let ctx = InFolder(tmp.path().to_path_buf());
        let err = in_work_dir("destination", "C:x", &ctx).unwrap_err();
        assert!(err.to_string().contains("'destination' \"C:x\""), "{err}");
    }

    #[test]
    fn a_refused_value_is_escaped_where_the_error_is_built() {
        let tmp = tempfile::tempdir().unwrap();
        let ctx = InFolder(tmp.path().to_path_buf());
        let err = in_work_dir("p", "\\x\n\u{1b}[2Kfake", &ctx).unwrap_err();
        let shown = err.to_string();
        assert!(shown.contains("\"\\x\\n\\e[2Kfake\""), "{shown}");
        assert!(
            !shown.contains('\n') && !shown.contains('\u{1b}'),
            "{shown}"
        );
    }

    #[test]
    fn plain_relative_values_go_under_the_work_dir() {
        for value in ["out", "./out", "a..b/c", ".env"] {
            assert_eq!(placement(value), Placement::UnderWorkDir, "{value}");
        }
    }

    #[cfg(windows)]
    #[test]
    fn on_windows_a_full_path_is_kept_and_a_rooted_one_refused() {
        assert_eq!(placement(r"C:\x"), Placement::AsWritten);
        assert_eq!(placement(r"\\server\share\x"), Placement::AsWritten);
        assert_eq!(placement("/x"), Placement::Refused);
        let tmp = tempfile::tempdir().unwrap();
        let ctx = InFolder(tmp.path().to_path_buf());
        assert!(in_work_dir("p", r"\x", &ctx).is_err());
        assert!(in_work_dir("p", "/x", &ctx).is_err());
        assert!(in_work_dir("p", "C:x", &ctx).is_err());
    }

    #[cfg(not(windows))]
    #[test]
    fn elsewhere_a_slash_path_is_kept_and_a_windows_one_refused() {
        assert_eq!(placement("/x"), Placement::AsWritten);
        assert_eq!(placement(r"C:\x"), Placement::Refused);
    }

    #[test]
    fn a_relative_value_with_no_work_dir_is_an_error() {
        assert!(in_work_dir("p", "out", &NoopContext).is_err());
    }
}
