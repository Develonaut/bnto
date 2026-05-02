// v1 → v2: Migrate from XDG-split layout to unified ~/.bnto/ home.
//
// Copies (not moves) recipes, config values, and trust store from
// the old XDG locations into the new unified directory.
//
// Old layout (macOS):
//   Config:  ~/.config/bnto/config.toml
//   Data:    ~/Library/Application Support/bnto/recipes/
//   State:   ~/Library/Application Support/bnto/trusted.json (fallback)
//
// New layout:
//   ~/.bnto/config.toml
//   ~/.bnto/recipes/
//   ~/.bnto/state/trusted.json

use std::path::PathBuf;

use super::super::paths::BntoPaths;
use super::super::runner::MigrationError;

/// Apply the v1 → v2 migration.
pub fn apply(paths: &BntoPaths) -> Result<(), MigrationError> {
    let old = OldPaths::resolve();
    let Some(old) = old else {
        // No home dir — nothing to migrate.
        return Ok(());
    };

    copy_recipes(&old, paths)?;
    migrate_config_values(&old, paths)?;
    copy_trusted_json(&old, paths)?;

    Ok(())
}

/// Old XDG-split paths (v1 layout).
struct OldPaths {
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl OldPaths {
    fn resolve() -> Option<Self> {
        let home = dirs::home_dir()?;

        // Config: macOS used ~/.config/bnto/ (XDG convention for CLIs).
        let config_dir = if cfg!(target_os = "macos") {
            home.join(".config").join("bnto")
        } else {
            dirs::config_dir()?.join("bnto")
        };

        // Data: platform default (~/Library/Application Support/bnto/ on macOS).
        let data_dir = dirs::data_dir()?.join("bnto");

        Some(Self {
            config_dir,
            data_dir,
        })
    }

    fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    fn recipes_dir(&self) -> PathBuf {
        self.data_dir.join("recipes")
    }

    fn trusted_file(&self) -> PathBuf {
        self.data_dir.join("trusted.json")
    }
}

/// Copy recipe files from the old data dir. Skips files that already exist.
fn copy_recipes(old: &OldPaths, paths: &BntoPaths) -> Result<(), MigrationError> {
    let old_dir = old.recipes_dir();
    if !old_dir.is_dir() {
        return Ok(());
    }

    let new_dir = paths.recipes_dir();
    std::fs::create_dir_all(&new_dir)?;

    let entries = std::fs::read_dir(&old_dir)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }
        let dest = new_dir.join(entry.file_name());
        if dest.exists() {
            continue; // Don't overwrite existing recipes.
        }
        std::fs::copy(entry.path(), &dest)?;
    }

    Ok(())
}

/// Migrate config values from the old v1 TOML format.
///
/// Old format had `[output] dir = "..."`, `[picker] default_path = "..."`,
/// and `[telemetry] enabled = bool`. The new v2 format uses `[paths]`
/// with `output` and `recipes` fields.
fn migrate_config_values(old: &OldPaths, paths: &BntoPaths) -> Result<(), MigrationError> {
    let old_config = old.config_file();
    if !old_config.is_file() {
        return Ok(());
    }

    let new_config = paths.config_file();
    if new_config.is_file() {
        // Config already exists — don't overwrite.
        return Ok(());
    }

    let old_contents = std::fs::read_to_string(&old_config)?;
    let old_parsed: V1Config = toml::from_str(&old_contents).unwrap_or_default();

    // Build v2 config from old values.
    let mut new = super::super::config::TomlConfig::default();

    if let Some(theme) = old_parsed.tui.and_then(|t| t.theme) {
        new.tui.theme = theme;
    }
    if let Some(dir) = old_parsed.output.and_then(|o| o.dir) {
        new.paths.output = Some(dir);
    }
    if let Some(enabled) = old_parsed.telemetry.and_then(|t| t.enabled) {
        new.telemetry.enabled = enabled;
    }

    new.save(paths)
        .map_err(|e| MigrationError::Failed(format!("Failed to save migrated config: {e}")))?;

    Ok(())
}

/// Copy trusted.json from the old state dir if it exists.
fn copy_trusted_json(old: &OldPaths, paths: &BntoPaths) -> Result<(), MigrationError> {
    let old_trusted = old.trusted_file();
    if !old_trusted.is_file() {
        return Ok(());
    }

    let new_trusted = paths.trusted_file();
    if new_trusted.exists() {
        return Ok(()); // Don't overwrite existing trust store.
    }

    // Ensure parent dir exists.
    if let Some(parent) = new_trusted.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(&old_trusted, &new_trusted)?;

    Ok(())
}

/// Minimal v1 config structure — just enough to extract values we care about.
#[derive(serde::Deserialize, Default)]
struct V1Config {
    tui: Option<V1Tui>,
    output: Option<V1Output>,
    #[allow(dead_code)]
    picker: Option<V1Picker>,
    telemetry: Option<V1Telemetry>,
}

#[derive(serde::Deserialize)]
struct V1Tui {
    theme: Option<String>,
}

#[derive(serde::Deserialize)]
struct V1Output {
    dir: Option<String>,
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct V1Picker {
    default_path: Option<String>,
}

#[derive(serde::Deserialize)]
struct V1Telemetry {
    enabled: Option<bool>,
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

    /// Set up a fake "old" directory structure and return old + new paths.
    fn setup_old_and_new() -> (tempfile::TempDir, OldPaths, tempfile::TempDir, BntoPaths) {
        let old_tmp = tempfile::tempdir().unwrap();
        let old = OldPaths {
            config_dir: old_tmp.path().join("config"),
            data_dir: old_tmp.path().join("data"),
        };
        std::fs::create_dir_all(&old.config_dir).unwrap();
        std::fs::create_dir_all(old.recipes_dir()).unwrap();

        let (_new_tmp, new_paths) = test_paths();
        (old_tmp, old, _new_tmp, new_paths)
    }

    #[test]
    fn copies_recipes_from_old_data_dir() {
        let (_old_tmp, old, _new_tmp, new_paths) = setup_old_and_new();

        // Create a recipe in the old location.
        std::fs::write(
            old.recipes_dir().join("my-recipe.bnto.json"),
            r#"{"name":"test"}"#,
        )
        .unwrap();

        copy_recipes(&old, &new_paths).unwrap();

        let dest = new_paths.recipes_dir().join("my-recipe.bnto.json");
        assert!(dest.exists(), "recipe should be copied");
        assert_eq!(
            std::fs::read_to_string(&dest).unwrap(),
            r#"{"name":"test"}"#
        );

        // Originals should still exist.
        assert!(
            old.recipes_dir().join("my-recipe.bnto.json").exists(),
            "originals untouched"
        );
    }

    #[test]
    fn does_not_overwrite_existing_recipes() {
        let (_old_tmp, old, _new_tmp, new_paths) = setup_old_and_new();

        // Old recipe.
        std::fs::write(old.recipes_dir().join("recipe.json"), "old-content").unwrap();

        // Pre-existing recipe in new location.
        std::fs::write(new_paths.recipes_dir().join("recipe.json"), "new-content").unwrap();

        copy_recipes(&old, &new_paths).unwrap();

        assert_eq!(
            std::fs::read_to_string(new_paths.recipes_dir().join("recipe.json")).unwrap(),
            "new-content",
            "existing recipe should not be overwritten"
        );
    }

    #[test]
    fn migrates_v1_config_values() {
        let (_old_tmp, old, _new_tmp, new_paths) = setup_old_and_new();

        let v1_toml = r#"
version = 1

[tui]
theme = "tokyo"

[output]
dir = "/my/output"

[telemetry]
enabled = false
"#;
        std::fs::write(old.config_file(), v1_toml).unwrap();

        migrate_config_values(&old, &new_paths).unwrap();

        let config = super::super::super::config::TomlConfig::load(&new_paths);
        assert_eq!(config.tui.theme, "tokyo");
        assert_eq!(config.paths.output, Some("/my/output".to_string()));
        assert!(!config.telemetry.enabled);
    }

    #[test]
    fn copies_trusted_json() {
        let (_old_tmp, old, _new_tmp, new_paths) = setup_old_and_new();

        // Create trusted.json in old location.
        std::fs::write(old.trusted_file(), r#"{"hashes":["abc"]}"#).unwrap();

        copy_trusted_json(&old, &new_paths).unwrap();

        let dest = new_paths.trusted_file();
        assert!(dest.exists(), "trusted.json should be copied");
        assert_eq!(
            std::fs::read_to_string(&dest).unwrap(),
            r#"{"hashes":["abc"]}"#
        );
    }

    #[test]
    fn parses_v1_config_format() {
        let v1_toml = r#"
version = 1

[tui]
theme = "monaco"

[output]
dir = "/old-output"

[picker]
default_path = "/old-default"

[telemetry]
enabled = false
"#;
        let config: V1Config = toml::from_str(v1_toml).unwrap();
        assert_eq!(config.tui.unwrap().theme, Some("monaco".to_string()));
        assert_eq!(config.output.unwrap().dir, Some("/old-output".to_string()));
        assert_eq!(
            config.picker.unwrap().default_path,
            Some("/old-default".to_string())
        );
        assert_eq!(config.telemetry.unwrap().enabled, Some(false));
    }
}
