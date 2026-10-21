// The one telemetry setting: `[telemetry] enabled` in config.toml. Older
// versions kept the choice in telemetry.json; an opt-out there moves over
// once, so nobody who turned telemetry off finds it back on.

use super::config::TelemetryConfig;
use crate::storage::config::TomlConfig;
use crate::storage::paths::BntoPaths;

/// The choice after merging a telemetry.json opt-in/out from an older
/// version: off when either file says off.
pub fn merged_choice(config_toml: bool, legacy_json: Option<bool>) -> bool {
    config_toml && legacy_json != Some(false)
}

/// The choice, from config.toml, after moving over any choice telemetry.json
/// still holds. The old field is dropped only once config.toml is saved, so
/// a failed write never loses an opt-out.
pub fn settle(paths: &BntoPaths, telemetry: &mut TelemetryConfig) -> bool {
    let mut config = TomlConfig::load(paths);
    let Some(legacy) = telemetry.legacy_enabled else {
        return config.telemetry.enabled;
    };
    config.telemetry.enabled = merged_choice(config.telemetry.enabled, Some(legacy));
    if config.save(paths).is_ok() {
        telemetry.legacy_enabled = None;
        let _ = telemetry.save();
    }
    config.telemetry.enabled
}

/// Save the choice to config.toml.
pub fn set(paths: &BntoPaths, enabled: bool) -> std::io::Result<()> {
    let mut config = TomlConfig::load(paths);
    config.telemetry.enabled = enabled;
    config.save(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths() -> (tempfile::TempDir, BntoPaths) {
        let dir = tempfile::tempdir().unwrap();
        let paths = BntoPaths {
            home: dir.path().to_path_buf(),
        };
        paths.ensure_dirs().unwrap();
        (dir, paths)
    }

    fn telemetry_json(legacy: Option<bool>) -> TelemetryConfig {
        TelemetryConfig {
            legacy_enabled: legacy,
            anonymous_id: "test".into(),
            consent_shown: true,
        }
    }

    fn saved_choice(paths: &BntoPaths) -> bool {
        TomlConfig::load(paths).telemetry.enabled
    }

    #[test]
    fn an_opt_out_in_either_file_wins() {
        assert!(merged_choice(true, None));
        assert!(merged_choice(true, Some(true)));
        assert!(!merged_choice(true, Some(false)));
        assert!(!merged_choice(false, Some(true)));
        assert!(!merged_choice(false, None));
    }

    #[test]
    fn an_old_json_opt_out_moves_into_config_toml() {
        let (_dir, paths) = paths();
        let mut json = telemetry_json(Some(false));
        assert!(!settle(&paths, &mut json));
        assert!(!saved_choice(&paths), "written to config.toml");
        assert_eq!(json.legacy_enabled, None, "the old field is dropped");
    }

    #[test]
    fn an_old_json_opt_in_never_overrides_a_config_toml_opt_out() {
        let (_dir, paths) = paths();
        set(&paths, false).unwrap();
        let mut json = telemetry_json(Some(true));
        assert!(!settle(&paths, &mut json));
        assert!(!saved_choice(&paths));
    }

    #[test]
    fn without_an_old_field_config_toml_decides() {
        let (_dir, paths) = paths();
        assert!(settle(&paths, &mut telemetry_json(None)), "default on");
        set(&paths, false).unwrap();
        assert!(!settle(&paths, &mut telemetry_json(None)));
    }

    #[test]
    fn set_writes_config_toml() {
        let (_dir, paths) = paths();
        set(&paths, false).unwrap();
        assert!(!saved_choice(&paths));
        set(&paths, true).unwrap();
        assert!(saved_choice(&paths));
    }
}
