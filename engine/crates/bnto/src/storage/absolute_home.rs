//! The bnto home must be an absolute path (ISS-126): a relative one would
//! make the current folder's `.bnto/` the home, so a cloned repo's recipes
//! would load as library recipes and its `state/trusted.json` would hold
//! the approvals.

use std::fmt;
use std::path::PathBuf;

/// Why no bnto home could be used.
#[derive(Debug, PartialEq)]
pub enum HomeError {
    /// `BNTO_HOME` isn't set and the platform has no home folder.
    NoHome,
    /// A setting names an empty or relative home.
    NotAbsolute {
        /// Where the value came from (`BNTO_HOME`, `paths.home in …`).
        setting: String,
        /// The value as given, which bnto never resolves.
        value: PathBuf,
    },
}

impl fmt::Display for HomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoHome => write!(f, "no home folder found; set BNTO_HOME to an absolute path"),
            Self::NotAbsolute { setting, value } => {
                let path = value.display().to_string();
                write!(f, "{setting} must be an absolute path, got \"{path}\"")?;
                if path.starts_with('~') {
                    write!(f, " (`~` isn't expanded: write the full path)")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for HomeError {}

/// `value`, refused unless absolute; `setting` names where it came from.
pub fn absolute_home(
    value: PathBuf,
    setting: impl FnOnce() -> String,
) -> Result<PathBuf, HomeError> {
    if value.is_absolute() {
        return Ok(value);
    }
    Err(HomeError::NotAbsolute {
        setting: setting(),
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bnto_home() -> String {
        "BNTO_HOME".into()
    }

    #[test]
    fn a_relative_home_is_refused() {
        let err = absolute_home(PathBuf::from(".bnto"), bnto_home).unwrap_err();
        assert_eq!(
            err.to_string(),
            r#"BNTO_HOME must be an absolute path, got ".bnto""#
        );
    }

    #[test]
    fn an_empty_home_is_refused() {
        let err = absolute_home(PathBuf::new(), bnto_home).unwrap_err();
        assert_eq!(
            err.to_string(),
            r#"BNTO_HOME must be an absolute path, got """#
        );
    }

    #[test]
    fn an_absolute_home_is_kept() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(
            absolute_home(tmp.path().to_path_buf(), bnto_home),
            Ok(tmp.path().to_path_buf())
        );
    }

    #[test]
    fn a_home_starting_with_a_tilde_says_it_is_not_expanded() {
        let err = absolute_home(PathBuf::from("~/bnto"), bnto_home).unwrap_err();
        assert_eq!(
            err.to_string(),
            r#"BNTO_HOME must be an absolute path, got "~/bnto" (`~` isn't expanded: write the full path)"#
        );
    }

    #[cfg(windows)]
    #[test]
    fn a_windows_path_without_a_drive_is_refused() {
        for value in [r"\Users\me\.bnto", r"C:bnto"] {
            assert!(
                absolute_home(PathBuf::from(value), bnto_home).is_err(),
                "{value}"
            );
        }
    }
}
