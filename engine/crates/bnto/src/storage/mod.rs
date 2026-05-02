// Persistent storage — paths, config, atomic writes, and migrations.
//
// Called once at startup via `ensure_ready()` before command dispatch,
// so all commands (not just TUI) get consistent storage directories
// and run any pending config migrations.

pub mod atomic;
pub mod config;
pub mod migrations;
pub mod paths;
pub mod runner;

pub use paths::BntoPaths;
pub use runner::MigrationError;

/// Initialize storage: resolve paths, ensure dirs, run pending migrations.
///
/// Called once in `main()` before command dispatch. Returns resolved
/// paths for use by logger, TUI, trust store, etc.
pub fn ensure_ready() -> Result<BntoPaths, MigrationError> {
    let paths = BntoPaths::resolve().unwrap_or_else(|| BntoPaths {
        home: std::path::PathBuf::from(".bnto"),
    });
    paths.ensure_dirs()?;
    runner::run_pending(&paths, migrations::MIGRATIONS)?;
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_paths() -> (tempfile::TempDir, BntoPaths) {
        let tmp = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            home: tmp.path().to_path_buf(),
        };
        (tmp, paths)
    }

    /// Simulate `ensure_ready()` with explicit paths (avoids env var races).
    fn ensure_ready_with(paths: &BntoPaths) -> Result<(), MigrationError> {
        paths.ensure_dirs()?;
        runner::run_pending(paths, migrations::MIGRATIONS)?;
        Ok(())
    }

    #[test]
    fn ensure_ready_creates_dirs_and_runs_migrations() {
        let (_tmp, paths) = test_paths();

        ensure_ready_with(&paths).unwrap();

        assert!(paths.recipes_dir().is_dir());
        assert!(paths.logs_dir().is_dir());
        assert!(paths.cache_dir().is_dir());
        // Config should exist after migration bumps version.
        assert!(paths.config_file().exists());
    }

    #[test]
    fn ensure_ready_is_idempotent() {
        let (_tmp, paths) = test_paths();

        ensure_ready_with(&paths).unwrap();
        ensure_ready_with(&paths).unwrap();

        assert!(paths.config_file().exists());
        // Version should still be current after second run.
        let config = config::TomlConfig::load(&paths);
        assert_eq!(config.version, 3);
    }
}
