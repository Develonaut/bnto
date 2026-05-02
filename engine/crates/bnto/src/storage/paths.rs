// Centralized path resolution for all bnto storage directories.
//
// Single `~/.bnto/` home directory. Replaces the previous XDG-split
// model with a simpler, more discoverable layout.

use std::path::PathBuf;

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
    /// Priority: `BNTO_HOME` env var > `~/.bnto/`.
    /// Returns `None` only if the platform can't determine a home directory.
    pub fn resolve() -> Option<Self> {
        let home = std::env::var_os("BNTO_HOME")
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|h| h.join(".bnto")))?;
        Some(Self { home })
    }

    /// Apply a config home override if set.
    ///
    /// Call after loading config from the default home. If `paths.home`
    /// is set in the config, returns a new `BntoPaths` rooted there.
    /// All derived paths (recipes, logs, cache) follow the new root.
    pub fn with_config_override(self, config: &super::config::TomlConfig) -> Self {
        match config.paths.home.as_deref().filter(|s| !s.is_empty()) {
            Some(home) => Self {
                home: PathBuf::from(home),
            },
            None => self,
        }
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
        std::fs::create_dir_all(self.home.join("state"))?;
        std::fs::create_dir_all(self.logs_dir())?;
        std::fs::create_dir_all(self.cache_dir())?;
        Ok(())
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
    fn resolve_returns_some() {
        unsafe {
            std::env::remove_var("BNTO_HOME");
        }
        assert!(BntoPaths::resolve().is_some());
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
    fn with_config_override_applies_custom_home() {
        let original = paths_from_root(std::path::Path::new("/default"));
        let mut config = crate::storage::config::TomlConfig::default();
        config.paths.home = Some("/custom/bnto".into());
        let overridden = original.with_config_override(&config);
        assert_eq!(overridden.home, std::path::PathBuf::from("/custom/bnto"));
        assert_eq!(
            overridden.recipes_dir(),
            std::path::PathBuf::from("/custom/bnto/recipes")
        );
    }

    #[test]
    fn with_config_override_keeps_default_when_none() {
        let original = paths_from_root(std::path::Path::new("/default"));
        let config = crate::storage::config::TomlConfig::default();
        let same = original.with_config_override(&config);
        assert_eq!(same.home, std::path::PathBuf::from("/default"));
    }

    #[test]
    fn with_config_override_ignores_empty_string() {
        let original = paths_from_root(std::path::Path::new("/default"));
        let mut config = crate::storage::config::TomlConfig::default();
        config.paths.home = Some(String::new());
        let same = original.with_config_override(&config);
        assert_eq!(same.home, std::path::PathBuf::from("/default"));
    }
}
