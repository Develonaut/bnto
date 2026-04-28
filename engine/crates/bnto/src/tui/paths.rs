// Centralized path resolution for all bnto storage directories.
//
// One struct, resolved once at startup, shared everywhere. Replaces
// scattered `dirs::config_dir()` calls with XDG-compliant paths.

use std::path::PathBuf;

/// Resolved paths for all bnto storage directories.
///
/// Created once at startup via `resolve()`, then passed to all
/// subsystems that need to read or write persistent data.
#[derive(Debug, Clone)]
pub struct BntoPaths {
    /// User-editable config (~/.config/bnto/ on Linux/macOS).
    pub config: PathBuf,
    /// App-managed persistent data (~/.local/share/bnto/ on Linux).
    pub data: PathBuf,
    /// Ephemeral state that survives restarts (~/.local/state/bnto/ on Linux).
    pub state: PathBuf,
    /// Fully disposable cache (~/.cache/bnto/ on Linux).
    pub cache: PathBuf,
}

impl BntoPaths {
    /// Resolve paths from environment and platform defaults.
    ///
    /// Priority: `BNTO_HOME` > `BNTO_CONFIG_DIR` > XDG env vars > platform defaults.
    /// Returns `None` only if the platform can't determine any base directory.
    pub fn resolve() -> Option<Self> {
        if let Some(home) = std::env::var_os("BNTO_HOME") {
            let root = PathBuf::from(home);
            return Some(Self {
                config: root.join("config"),
                data: root.join("data"),
                state: root.join("state"),
                cache: root.join("cache"),
            });
        }

        let config = resolve_config_dir()?;
        let data = resolve_data_dir()?;
        let state = resolve_state_dir()?;
        let cache = resolve_cache_dir()?;

        Some(Self {
            config,
            data,
            state,
            cache,
        })
    }

    /// Path to the main config file (config.toml).
    pub fn config_file(&self) -> PathBuf {
        self.config.join("config.toml")
    }

    /// Path to the user's recipe library directory.
    pub fn recipes_dir(&self) -> PathBuf {
        self.data.join("recipes")
    }

    /// Path to execution history.
    #[allow(dead_code)]
    pub fn history_file(&self) -> PathBuf {
        self.state.join("history.json")
    }

    /// Path to recently used recipes/directories.
    #[allow(dead_code)]
    pub fn recent_file(&self) -> PathBuf {
        self.state.join("recent.json")
    }

    /// Path to the trusted recipes store.
    pub fn trusted_file(&self) -> PathBuf {
        self.state.join("trusted.json")
    }

    /// Path to the session logs directory.
    pub fn logs_dir(&self) -> PathBuf {
        self.state.join("logs")
    }

    /// Create all directories. Called once at startup.
    pub fn ensure_dirs(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.config)?;
        std::fs::create_dir_all(&self.data)?;
        std::fs::create_dir_all(&self.state)?;
        std::fs::create_dir_all(&self.cache)?;
        std::fs::create_dir_all(self.recipes_dir())?;
        std::fs::create_dir_all(self.logs_dir())?;
        Ok(())
    }
}

/// Config dir: BNTO_CONFIG_DIR > XDG_CONFIG_HOME > platform default.
///
/// macOS exception: CLI tools use ~/.config/bnto/ (XDG-style) instead
/// of ~/Library/Application Support/bnto/. This matches Helix, bat, starship.
fn resolve_config_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("BNTO_CONFIG_DIR") {
        return Some(PathBuf::from(dir));
    }
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg).join("bnto"));
    }
    // macOS: use ~/.config/bnto/ (CLI convention, not ~/Library/...)
    if cfg!(target_os = "macos") {
        return dirs::home_dir().map(|h| h.join(".config").join("bnto"));
    }
    dirs::config_dir().map(|d| d.join("bnto"))
}

/// Data dir: XDG_DATA_HOME > platform default.
fn resolve_data_dir() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
        return Some(PathBuf::from(xdg).join("bnto"));
    }
    dirs::data_dir().map(|d| d.join("bnto"))
}

/// State dir: XDG_STATE_HOME > platform default (macOS: data/state/).
fn resolve_state_dir() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_STATE_HOME") {
        return Some(PathBuf::from(xdg).join("bnto"));
    }
    // macOS has no native state dir — use data_dir/state/ as fallback.
    if cfg!(target_os = "macos") {
        return dirs::data_dir().map(|d| d.join("bnto").join("state"));
    }
    // Linux: ~/.local/state/bnto/
    dirs::state_dir().map(|d| d.join("bnto"))
}

/// Cache dir: XDG_CACHE_HOME > platform default.
fn resolve_cache_dir() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_CACHE_HOME") {
        return Some(PathBuf::from(xdg).join("bnto"));
    }
    dirs::cache_dir().map(|d| d.join("bnto"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: construct BntoPaths directly from a temp dir root,
    /// mimicking what BNTO_HOME would produce without touching env vars.
    fn paths_from_root(root: &std::path::Path) -> BntoPaths {
        BntoPaths {
            config: root.join("config"),
            data: root.join("data"),
            state: root.join("state"),
            cache: root.join("cache"),
        }
    }

    #[test]
    fn resolve_returns_some() {
        // Platform defaults should always produce Some on macOS/Linux/Windows.
        // Use a separate unsafe block to clear any stale overrides.
        unsafe {
            std::env::remove_var("BNTO_HOME");
            std::env::remove_var("BNTO_CONFIG_DIR");
        }
        assert!(BntoPaths::resolve().is_some());
    }

    #[test]
    fn bnto_home_overrides_all() {
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("BNTO_HOME", tmp.path().as_os_str()) };
        let paths = BntoPaths::resolve().expect("resolve should succeed");
        unsafe { std::env::remove_var("BNTO_HOME") };

        assert_eq!(paths.config, tmp.path().join("config"));
        assert_eq!(paths.data, tmp.path().join("data"));
        assert_eq!(paths.state, tmp.path().join("state"));
        assert_eq!(paths.cache, tmp.path().join("cache"));
    }

    #[test]
    fn bnto_config_dir_overrides_config_only() {
        let tmp = tempfile::tempdir().unwrap();
        let custom_config = tmp.path().join("my-config");

        unsafe {
            std::env::remove_var("BNTO_HOME");
            std::env::set_var("BNTO_CONFIG_DIR", custom_config.as_os_str());
        }
        let paths = BntoPaths::resolve().expect("resolve should succeed");
        unsafe { std::env::remove_var("BNTO_CONFIG_DIR") };

        assert_eq!(paths.config, custom_config);
        // Data/state/cache should NOT be under custom_config.
        assert_ne!(paths.data.parent(), Some(custom_config.as_path()));
    }

    #[test]
    fn config_file_path() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(
            paths.config_file(),
            tmp.path().join("config").join("config.toml")
        );
    }

    #[test]
    fn recipes_dir_path() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(paths.recipes_dir(), tmp.path().join("data").join("recipes"));
    }

    #[test]
    fn history_file_path() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(
            paths.history_file(),
            tmp.path().join("state").join("history.json")
        );
    }

    #[test]
    fn recent_file_path() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(
            paths.recent_file(),
            tmp.path().join("state").join("recent.json")
        );
    }

    #[test]
    fn trusted_file_path() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(
            paths.trusted_file(),
            tmp.path().join("state").join("trusted.json")
        );
    }

    #[test]
    fn logs_dir_path() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        assert_eq!(paths.logs_dir(), tmp.path().join("state").join("logs"));
    }

    #[test]
    fn ensure_dirs_creates_all() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_from_root(tmp.path());
        paths.ensure_dirs().expect("ensure_dirs should succeed");

        assert!(paths.config.is_dir());
        assert!(paths.data.is_dir());
        assert!(paths.state.is_dir());
        assert!(paths.cache.is_dir());
        assert!(paths.recipes_dir().is_dir());
        assert!(paths.logs_dir().is_dir());
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
    fn bnto_home_priority_over_xdg() {
        let tmp = tempfile::tempdir().unwrap();
        let xdg_dir = tmp.path().join("xdg");

        unsafe {
            std::env::set_var("BNTO_HOME", tmp.path().join("bnto-home").as_os_str());
            std::env::set_var("XDG_CONFIG_HOME", xdg_dir.as_os_str());
        }
        let paths = BntoPaths::resolve().expect("resolve should succeed");
        unsafe {
            std::env::remove_var("BNTO_HOME");
            std::env::remove_var("XDG_CONFIG_HOME");
        }

        // BNTO_HOME should win over XDG.
        assert_eq!(paths.config, tmp.path().join("bnto-home").join("config"));
    }
}
