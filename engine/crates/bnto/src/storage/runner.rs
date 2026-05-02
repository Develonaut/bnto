// Migration runner — sequential numbered migrations for config evolution.
//
// Reads the `version` field from config.toml, runs all migrations with
// version > current, bumps version atomically after each success.

use std::fmt;
use std::path::Path;

use super::paths::BntoPaths;

/// A single migration step.
pub struct Migration {
    /// Target version after this migration runs.
    pub version: u32,
    /// Human-readable name for logging.
    pub name: &'static str,
    /// Apply function — receives resolved paths, returns Ok on success.
    pub apply: fn(&BntoPaths) -> Result<(), MigrationError>,
}

/// Errors that can occur during migration.
#[derive(Debug)]
pub enum MigrationError {
    Io(std::io::Error),
    Failed(String),
}

impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Failed(msg) => write!(f, "Migration failed: {msg}"),
        }
    }
}

impl From<std::io::Error> for MigrationError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// Minimal struct to read only the version field from TOML.
///
/// Avoids full deserialization — safe even when the schema has changed.
#[derive(serde::Deserialize)]
struct VersionOnly {
    #[serde(default = "default_version")]
    version: u32,
}

fn default_version() -> u32 {
    1
}

/// Read the current config version from disk. Returns 1 if no config exists.
fn read_version(config_path: &Path) -> u32 {
    let Ok(contents) = std::fs::read_to_string(config_path) else {
        return 1;
    };
    toml::from_str::<VersionOnly>(&contents)
        .map(|v| v.version)
        .unwrap_or(1)
}

/// Bump the version field in config.toml atomically.
///
/// Reads full config, updates version, writes back. Creates the file
/// with defaults if it doesn't exist.
fn bump_version(config_path: &Path, new_version: u32) -> Result<(), MigrationError> {
    let contents = std::fs::read_to_string(config_path).unwrap_or_default();
    let mut config: toml::Value =
        toml::from_str(&contents).unwrap_or(toml::Value::Table(toml::map::Map::new()));

    if let Some(table) = config.as_table_mut() {
        table.insert(
            "version".to_string(),
            toml::Value::Integer(i64::from(new_version)),
        );
    }

    let toml_str = toml::to_string_pretty(&config)
        .map_err(|e| MigrationError::Failed(format!("TOML serialize: {e}")))?;
    super::atomic::atomic_write(config_path, toml_str.as_bytes())?;
    Ok(())
}

/// Run all pending migrations where version > current config version.
///
/// Bumps the config version atomically after each successful migration.
/// If migration N succeeds but N+1 fails, next startup retries from N+1.
pub fn run_pending(paths: &BntoPaths, migrations: &[Migration]) -> Result<(), MigrationError> {
    let config_path = paths.config_file();
    let current = read_version(&config_path);

    for migration in migrations {
        if migration.version <= current {
            continue;
        }
        (migration.apply)(paths)?;
        bump_version(&config_path, migration.version)?;
    }

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
        paths.ensure_dirs().unwrap();
        (tmp, paths)
    }

    /// Write a config.toml with only a version field.
    fn write_version(paths: &BntoPaths, version: u32) {
        let toml = format!("version = {version}\n");
        std::fs::write(paths.config_file(), toml).unwrap();
    }

    /// Read version from disk (using the runner's own read function).
    fn current_version(paths: &BntoPaths) -> u32 {
        read_version(&paths.config_file())
    }

    /// Append a marker to applied.txt to track which migrations ran.
    fn append_marker(paths: &BntoPaths, marker: &str) {
        let tracker = paths.home.join("applied.txt");
        let mut contents = std::fs::read_to_string(&tracker).unwrap_or_default();
        contents.push_str(marker);
        contents.push('\n');
        std::fs::write(&tracker, contents).unwrap();
    }

    fn apply_v2(paths: &BntoPaths) -> Result<(), MigrationError> {
        append_marker(paths, "2");
        Ok(())
    }

    fn apply_v3(paths: &BntoPaths) -> Result<(), MigrationError> {
        append_marker(paths, "3");
        Ok(())
    }

    fn apply_v4(paths: &BntoPaths) -> Result<(), MigrationError> {
        append_marker(paths, "4");
        Ok(())
    }

    fn apply_v3_fail(_paths: &BntoPaths) -> Result<(), MigrationError> {
        Err(MigrationError::Failed("intentional".into()))
    }

    fn read_tracker(paths: &BntoPaths) -> Vec<String> {
        let tracker = paths.home.join("applied.txt");
        std::fs::read_to_string(&tracker)
            .unwrap_or_default()
            .lines()
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect()
    }

    // --- RED tests ---

    #[test]
    fn skips_migrations_at_or_below_current_version() {
        let (_tmp, paths) = test_paths();
        write_version(&paths, 3);

        let migrations = &[
            Migration {
                version: 2,
                name: "v2",
                apply: apply_v2,
            },
            Migration {
                version: 3,
                name: "v3",
                apply: apply_v3,
            },
            Migration {
                version: 4,
                name: "v4",
                apply: apply_v4,
            },
        ];

        run_pending(&paths, migrations).unwrap();

        let applied = read_tracker(&paths);
        assert_eq!(applied, vec!["4"], "only v4 should run");
        assert_eq!(current_version(&paths), 4);
    }

    #[test]
    fn applies_pending_in_order() {
        let (_tmp, paths) = test_paths();
        write_version(&paths, 1);

        let migrations = &[
            Migration {
                version: 2,
                name: "v2",
                apply: apply_v2,
            },
            Migration {
                version: 3,
                name: "v3",
                apply: apply_v3,
            },
        ];

        run_pending(&paths, migrations).unwrap();

        let applied = read_tracker(&paths);
        assert_eq!(applied, vec!["2", "3"], "must apply in order");
    }

    #[test]
    fn bumps_version_after_each_success() {
        let (_tmp, paths) = test_paths();
        write_version(&paths, 1);

        let migrations = &[Migration {
            version: 2,
            name: "v2",
            apply: apply_v2,
        }];
        run_pending(&paths, migrations).unwrap();

        assert_eq!(current_version(&paths), 2);
    }

    #[test]
    fn stops_on_first_failure() {
        let (_tmp, paths) = test_paths();
        write_version(&paths, 1);

        let migrations = &[
            Migration {
                version: 2,
                name: "v2",
                apply: apply_v2,
            },
            Migration {
                version: 3,
                name: "v3_fail",
                apply: apply_v3_fail,
            },
            Migration {
                version: 4,
                name: "v4",
                apply: apply_v4,
            },
        ];

        let result = run_pending(&paths, migrations);
        assert!(result.is_err(), "should fail on v3");

        let applied = read_tracker(&paths);
        assert_eq!(applied, vec!["2"], "v2 applied, v4 not reached");
        assert_eq!(current_version(&paths), 2, "version stuck at 2");
    }

    #[test]
    fn idempotent_rerun() {
        let (_tmp, paths) = test_paths();
        write_version(&paths, 1);

        let migrations = &[
            Migration {
                version: 2,
                name: "v2",
                apply: apply_v2,
            },
            Migration {
                version: 3,
                name: "v3",
                apply: apply_v3,
            },
        ];

        run_pending(&paths, migrations).unwrap();
        // Reset tracker to detect re-runs.
        std::fs::remove_file(paths.home.join("applied.txt")).ok();

        run_pending(&paths, migrations).unwrap();

        let applied = read_tracker(&paths);
        assert!(applied.is_empty(), "second run should apply nothing");
    }

    #[test]
    fn missing_config_treats_as_version_1() {
        let (_tmp, paths) = test_paths();
        // No config.toml — version defaults to 1.

        let migrations = &[
            Migration {
                version: 2,
                name: "v2",
                apply: apply_v2,
            },
            Migration {
                version: 3,
                name: "v3",
                apply: apply_v3,
            },
        ];

        run_pending(&paths, migrations).unwrap();

        let applied = read_tracker(&paths);
        assert_eq!(applied, vec!["2", "3"], "all migrations should run");
    }

    #[test]
    fn creates_config_if_missing() {
        let (_tmp, paths) = test_paths();
        assert!(!paths.config_file().exists());

        let migrations = &[Migration {
            version: 2,
            name: "v2",
            apply: apply_v2,
        }];
        run_pending(&paths, migrations).unwrap();

        assert!(
            paths.config_file().exists(),
            "config.toml should be created"
        );
        assert_eq!(current_version(&paths), 2);
    }
}
