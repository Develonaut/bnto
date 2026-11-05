// Centralized path resolution for all bnto storage directories.
//
// Single `~/.bnto/` home directory. Replaces the previous XDG-split
// model with a simpler, more discoverable layout.

use std::ffi::OsString;
use std::path::PathBuf;

use super::absolute_home::{HomeError, absolute_home};
use super::config::TomlConfig;

/// Resolved paths for all bnto storage directories.
///
/// Created once at startup via `resolve()`, then passed to all
/// subsystems that need to read or write persistent data.
///
/// Layout:
/// ```text
/// ~/.bnto/
/// ├── config.toml
/// ├── recipes/
/// ├── output/
/// ├── state/
/// │   ├── trusted.json
/// │   ├── logs/
/// │   ├── recent.json     (future)
/// │   └── history.json    (future)
/// └── cache/
/// ```
#[derive(Debug, Clone)]
pub struct BntoPaths {
    /// Single home directory (~/.bnto/ by default).
    pub home: PathBuf,
}

impl BntoPaths {
    /// Resolve the bnto home directory.
    ///
    /// Priority: `BNTO_HOME` env var > `~/.bnto/`. Refused unless absolute
    /// (ISS-126), and when the platform has no home folder.
    pub fn resolve() -> Result<Self, HomeError> {
        let home = home_from(std::env::var_os("BNTO_HOME"), dirs::home_dir())?;
        Ok(Self { home })
    }

    /// Apply a config home override if set.
    ///
    /// Call after loading config from the default home. If `paths.home`
    /// is set in the config, returns a new `BntoPaths` rooted there.
    /// All derived paths (recipes, logs, cache) follow the new root. An
    /// empty or relative `paths.home` is refused (ISS-126).
    pub fn with_config_override(self, config: &TomlConfig) -> Result<Self, HomeError> {
        let Some(home) = config.paths.home.as_deref() else {
            return Ok(self);
        };
        let setting = || format!("paths.home in {}", self.config_file().display());
        let home = absolute_home(PathBuf::from(home), setting)?;
        Ok(Self { home })
    }

    /// This home, moved to where its own `config.toml`'s `paths.home` says.
    pub fn configured(self) -> Result<Self, HomeError> {
        let config = TomlConfig::load(&self);
        self.with_config_override(&config)
    }

    /// `resolve()`, then `configured()`: the one bnto home every surface
    /// uses, so the CLI and the TUI read one trust file, one user `.env` and
    /// one output folder (ISS-127).
    pub fn resolve_configured() -> Result<Self, HomeError> {
        Self::resolve()?.configured()
    }

    /// Path to the main config file.
    pub fn config_file(&self) -> PathBuf {
        self.home.join("config.toml")
    }

    /// Path to the user's recipe library directory.
    pub fn recipes_dir(&self) -> PathBuf {
        self.home.join("recipes")
    }

    /// Path to the trusted recipes store.
    pub fn trusted_file(&self) -> PathBuf {
        self.home.join("state").join("trusted.json")
    }

    /// Path to the session logs directory.
    pub fn logs_dir(&self) -> PathBuf {
        self.home.join("state").join("logs")
    }

    /// Path to the default output directory for recipe results.
    pub fn output_dir(&self) -> PathBuf {
        self.home.join("output")
    }

    /// Path to the cache directory.
    pub fn cache_dir(&self) -> PathBuf {
        self.home.join("cache")
    }

    /// Path to execution history (future).
    #[allow(dead_code)]
    pub fn history_file(&self) -> PathBuf {
        self.home.join("state").join("history.json")
    }

    /// Path to recently used recipes/directories (future).
    #[allow(dead_code)]
    pub fn recent_file(&self) -> PathBuf {
        self.home.join("state").join("recent.json")
    }

    /// Create all directories. Called once at startup.
    pub fn ensure_dirs(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.home)?;
        std::fs::create_dir_all(self.recipes_dir())?;
        std::fs::create_dir_all(self.output_dir())?;
        std::fs::create_dir_all(self.home.join("state"))?;
        std::fs::create_dir_all(self.logs_dir())?;
        std::fs::create_dir_all(self.cache_dir())?;
        Ok(())
    }
}

/// The bnto home `BNTO_HOME` names, else `.bnto` in the platform's home
/// folder; either must be absolute (`dirs` takes a relative `$HOME` as is).
fn home_from(bnto_home: Option<OsString>, platform: Option<PathBuf>) -> Result<PathBuf, HomeError> {
    match bnto_home {
        Some(value) => absolute_home(PathBuf::from(value), || "BNTO_HOME".into()),
        None => {
            let home = platform.ok_or(HomeError::NoHome)?;
            // The error shows the value to fix (`$HOME`), not the joined path.
            let home = absolute_home(home, || "the home folder ($HOME)".into())?;
            Ok(home.join(".bnto"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: construct BntoPaths from a temp dir root.
    fn paths_from_root(root: &std::path::Path) -> BntoPaths {
        BntoPaths {
            home: root.to_path_buf(),
        }
    }

    #[test]
    fn resolve_returns_a_home() {
        unsafe {
            std::env::remove_var("BNTO_HOME");
        }
        assert!(BntoPaths::resolve().is_ok());
    }

    #[test]
    fn resolve_defaults_to_dot_bnto() {
        unsafe {
            std::env::remove_var("BNTO_HOME");
        }
        let paths = BntoPaths::resolve().unwrap();
        let home = dirs::home_dir().unwrap();
        assert_eq!(paths.home, home.join(".bnto"));
    }

    #[test]
    fn bnto_home_overrides_default() {
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("BNTO_HOME", tmp.path().as_os_str()) };
        let paths = BntoPaths::resolve().expect("resolve should succeed");
        unsafe { std::env::remove_var("BNTO_HOME") };

        assert_eq!(paths.home, tmp.path());
    }

    #[test]
    fn config_file_at_home_root() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(paths.config_file(), tmp.path().join("config.toml"));
    }

    #[test]
    fn recipes_dir_under_home() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(paths.recipes_dir(), tmp.path().join("recipes"));
    }

    #[test]
    fn state_files_under_state_subdir() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(
            paths.trusted_file(),
            tmp.path().join("state").join("trusted.json")
        );
        assert_eq!(paths.logs_dir(), tmp.path().join("state").join("logs"));
        assert_eq!(
            paths.history_file(),
            tmp.path().join("state").join("history.json")
        );
        assert_eq!(
            paths.recent_file(),
            tmp.path().join("state").join("recent.json")
        );
    }

    #[test]
    fn output_dir_under_home() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(paths.output_dir(), tmp.path().join("output"));
    }

    #[test]
    fn cache_dir_under_home() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(paths.cache_dir(), tmp.path().join("cache"));
    }

    #[test]
    fn ensure_dirs_creates_all() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        paths.ensure_dirs().expect("ensure_dirs should succeed");

        assert!(paths.home.is_dir());
        assert!(paths.recipes_dir().is_dir());
        assert!(paths.output_dir().is_dir());
        assert!(paths.home.join("state").is_dir());
        assert!(paths.logs_dir().is_dir());
        assert!(paths.cache_dir().is_dir());
    }

    #[test]
    fn ensure_dirs_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        paths.ensure_dirs().unwrap();
        paths
            .ensure_dirs()
            .expect("second call should also succeed");
    }

    #[test]
    fn configured_follows_the_homes_own_config() {
        let default = tempfile::tempdir().unwrap();
        let moved = tempfile::tempdir().unwrap();
        let config = format!("[paths]\nhome = '{}'\n", moved.path().display());
        std::fs::write(default.path().join("config.toml"), config).unwrap();
        let paths = paths_from_root(default.path()).configured().unwrap();
        assert_eq!(paths.home, moved.path());
        assert_eq!(
            paths.trusted_file(),
            moved.path().join("state").join("trusted.json")
        );
    }

    #[test]
    fn configured_keeps_a_home_without_an_override() {
        let home = tempfile::tempdir().unwrap();
        assert_eq!(
            paths_from_root(home.path()).configured().unwrap().home,
            home.path()
        );
    }

    #[test]
    fn with_config_override_applies_custom_home() {
        let original = paths_from_root(std::path::Path::new("/default"));
        let custom = tempfile::tempdir().unwrap();
        let mut config = crate::storage::config::TomlConfig::default();
        config.paths.home = Some(custom.path().to_string_lossy().into_owned());
        let overridden = original.with_config_override(&config).unwrap();
        assert_eq!(overridden.home, custom.path());
        assert_eq!(overridden.recipes_dir(), custom.path().join("recipes"));
    }

    #[test]
    fn with_config_override_keeps_default_when_none() {
        let original = paths_from_root(std::path::Path::new("/default"));
        let config = crate::storage::config::TomlConfig::default();
        let same = original.with_config_override(&config).unwrap();
        assert_eq!(same.home, std::path::PathBuf::from("/default"));
    }

    #[test]
    fn with_config_override_refuses_an_empty_or_relative_home() {
        // ISS-126: either would resolve against the current folder.
        let tmp = tempfile::tempdir().unwrap();
        for home in ["", ".bnto", "rel/bnto"] {
            let mut config = crate::storage::config::TomlConfig::default();
            config.paths.home = Some(home.into());
            let err = paths_from_root(tmp.path())
                .with_config_override(&config)
                .unwrap_err();
            let message = err.to_string();
            assert!(message.starts_with("paths.home in "), "{message}");
            assert!(message.contains("must be an absolute path"), "{message}");
        }
    }

    #[test]
    fn a_relative_or_empty_bnto_home_is_refused() {
        let platform = Some(PathBuf::from("/Users/me"));
        for value in [".bnto", ""] {
            let err = home_from(Some(value.into()), platform.clone()).unwrap_err();
            assert!(err.to_string().starts_with("BNTO_HOME must be"), "{err}");
        }
    }

    #[test]
    fn an_absolute_bnto_home_is_used() {
        let tmp = tempfile::tempdir().unwrap();
        let home = home_from(Some(tmp.path().into()), None).unwrap();
        assert_eq!(home, tmp.path());
    }

    #[test]
    fn without_bnto_home_the_platform_home_is_used() {
        let tmp = tempfile::tempdir().unwrap();
        let home = home_from(None, Some(tmp.path().into())).unwrap();
        assert_eq!(home, tmp.path().join(".bnto"));
    }

    #[test]
    fn no_home_at_all_is_refused_not_made_relative() {
        assert_eq!(home_from(None, None), Err(HomeError::NoHome));
        let relative = home_from(None, Some(PathBuf::from("me"))).unwrap_err();
        assert_eq!(
            relative.to_string(),
            r#"the home folder ($HOME) must be an absolute path, got "me""#
        );
    }
}
