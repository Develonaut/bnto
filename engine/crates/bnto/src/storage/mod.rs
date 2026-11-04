// Persistent storage — paths, config, atomic writes, and migrations.
//
// `main` resolves the bnto home (refusing a relative one, ISS-126), then
// calls `ensure_ready()` before command dispatch, so all commands (not just
// TUI) get consistent storage directories and run any pending migrations.

mod absolute_home;
pub mod atomic;
pub mod config;
pub mod migrations;
pub mod paths;
pub mod runner;

pub use absolute_home::HomeError;
pub use paths::BntoPaths;
pub use runner::MigrationError;

/// Initialize storage in a resolved home: ensure dirs, run pending
/// migrations. Called once in `main()` before command dispatch.
pub fn ensure_ready(paths: &BntoPaths) -> Result<(), MigrationError> {
    paths.ensure_dirs()?;
    runner::run_pending(paths, migrations::MIGRATIONS)?;
    Ok(())
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

    #[test]
    fn ensure_ready_creates_dirs_and_runs_migrations() {
        let (_tmp, paths) = test_paths();

        ensure_ready(&paths).unwrap();

        assert!(paths.recipes_dir().is_dir());
        assert!(paths.logs_dir().is_dir());
        assert!(paths.cache_dir().is_dir());
        // Config should exist after migration bumps version.
        assert!(paths.config_file().exists());
    }

    #[test]
    fn ensure_ready_is_idempotent() {
        let (_tmp, paths) = test_paths();

        ensure_ready(&paths).unwrap();
        ensure_ready(&paths).unwrap();

        assert!(paths.config_file().exists());
        // Version should still be current after second run.
        let config = config::TomlConfig::load(&paths);
        assert_eq!(config.version, 3);
    }
}
