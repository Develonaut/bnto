// First-run telemetry notice — env opt-out must suppress it.
//
// Each run gets fresh HOME / BNTO_HOME / XDG_CONFIG_HOME temp dirs, so the
// binary sees a first run and never touches the developer's real config.

mod helpers;

use std::process::Command;

use helpers::bnto_bin;

const NOTICE: &str = "collects anonymous usage data";

/// Run `bnto list` against an isolated first-run home, returning stderr.
fn first_run_stderr(env: &[(&str, &str)]) -> String {
    let home = tempfile::tempdir().unwrap();
    let mut cmd = Command::new(bnto_bin());
    cmd.arg("list")
        .env("HOME", home.path())
        .env("BNTO_HOME", home.path().join(".bnto"))
        .env("XDG_CONFIG_HOME", home.path().join(".config"))
        .env_remove("BNTO_TELEMETRY_DISABLED")
        .env_remove("DO_NOT_TRACK");
    for (key, value) in env {
        cmd.env(key, value);
    }
    let output = cmd.output().unwrap();
    assert!(output.status.success(), "bnto list failed");
    String::from_utf8_lossy(&output.stderr).into_owned()
}

// Windows resolves the config dir via the Known Folder API, ignoring env vars,
// so a fresh first run can't be isolated there.
#[test]
#[cfg_attr(windows, ignore = "config dir can't be redirected on Windows")]
fn notice_shows_on_first_run() {
    let stderr = first_run_stderr(&[]);
    assert!(stderr.contains(NOTICE), "expected notice, got: {stderr}");
}

#[test]
fn bnto_telemetry_disabled_suppresses_notice() {
    let stderr = first_run_stderr(&[("BNTO_TELEMETRY_DISABLED", "1")]);
    assert!(!stderr.contains(NOTICE), "unexpected notice: {stderr}");
}

#[test]
fn do_not_track_suppresses_notice() {
    let stderr = first_run_stderr(&[("DO_NOT_TRACK", "1")]);
    assert!(!stderr.contains(NOTICE), "unexpected notice: {stderr}");
}
