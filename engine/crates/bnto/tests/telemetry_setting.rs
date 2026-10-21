// The one telemetry setting, `[telemetry] enabled` in config.toml, through
// the built binary: `bnto telemetry disable` writes it, and an opt-out an
// older bnto left in telemetry.json moves into it. Every run gets fresh
// HOME / BNTO_HOME / XDG_CONFIG_HOME temp dirs.

mod helpers;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use helpers::bnto_bin;

/// Where the binary keeps telemetry.json under a sandboxed `home`.
fn telemetry_json(home: &Path) -> PathBuf {
    let config = if cfg!(target_os = "macos") {
        home.join("Library").join("Application Support")
    } else {
        home.join(".config")
    };
    config.join("bnto").join("telemetry.json")
}

fn bnto(home: &Path, args: &[&str]) -> Output {
    Command::new(bnto_bin())
        .args(args)
        .env("HOME", home)
        .env("BNTO_HOME", home.join(".bnto"))
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("BNTO_NO_UPDATE_CHECK", "1")
        .env_remove("BNTO_TELEMETRY_DISABLED")
        .env_remove("DO_NOT_TRACK")
        .output()
        .unwrap()
}

fn config_toml(home: &Path) -> String {
    std::fs::read_to_string(home.join(".bnto").join("config.toml")).unwrap_or_default()
}

fn status(home: &Path) -> String {
    let output = bnto(home, &["telemetry", "status"]);
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
#[cfg_attr(windows, ignore = "config dir can't be redirected on Windows")]
fn disable_writes_config_toml() {
    let home = tempfile::tempdir().unwrap();
    let output = bnto(home.path(), &["telemetry", "disable"]);
    assert!(output.status.success(), "{output:?}");
    assert!(config_toml(home.path()).contains("enabled = false"));
    assert!(status(home.path()).contains("Config enabled: false"));

    bnto(home.path(), &["telemetry", "enable"]);
    assert!(config_toml(home.path()).contains("enabled = true"));
    assert!(status(home.path()).contains("Config enabled: true"));
}

#[test]
#[cfg_attr(windows, ignore = "config dir can't be redirected on Windows")]
fn an_old_telemetry_json_opt_out_moves_into_config_toml() {
    let home = tempfile::tempdir().unwrap();
    let json = telemetry_json(home.path());
    std::fs::create_dir_all(json.parent().unwrap()).unwrap();
    let old = r#"{"enabled":false,"anonymous_id":"old-id","consent_shown":true}"#;
    std::fs::write(&json, old).unwrap();

    assert!(bnto(home.path(), &["list"]).status.success());

    assert!(config_toml(home.path()).contains("enabled = false"));
    let rewritten = std::fs::read_to_string(&json).unwrap();
    assert!(!rewritten.contains("enabled"), "{rewritten}");
    assert!(rewritten.contains("old-id"), "the anonymous id is kept");
    // Turning it back on sticks: the old field can't switch it off again.
    bnto(home.path(), &["telemetry", "enable"]);
    assert!(status(home.path()).contains("Config enabled: true"));
}
