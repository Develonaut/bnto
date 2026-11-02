// The "newer bnto available" notice, seen the way a user sees it: the real
// binary in a pseudo-terminal (the notice only shows when stderr is one).
// Every case seeds a fresh cached check, so nothing reaches the network.

use std::io::Read;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use tempfile::TempDir;

const NOTICE: &str = "is available (you have";

/// Env vars any of which marks a run as CI (`telemetry::ci::is_ci`).
const CI_VARS: [&str; 7] = [
    "CI",
    "GITHUB_ACTIONS",
    "GITLAB_CI",
    "CIRCLECI",
    "TRAVIS",
    "BUILDKITE",
    "JENKINS_URL",
];

/// A sandbox whose cached check says 99.0.0 is out, checked just now.
fn sandbox() -> (TempDir, TempDir) {
    let (user, bnto) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let cache = bnto.path().join("cache");
    std::fs::create_dir_all(&cache).unwrap();
    let json = format!(r#"{{"checkedAt":{now},"latest":"99.0.0"}}"#);
    std::fs::write(cache.join("update-check.json"), json).unwrap();
    (user, bnto)
}

/// `bnto list` in a terminal; everything it printed.
fn run_in_terminal(user: &Path, bnto: &Path, extra_env: &[(&str, &str)]) -> String {
    let size = PtySize {
        rows: 40,
        cols: 140,
        pixel_width: 0,
        pixel_height: 0,
    };
    let pair = native_pty_system().openpty(size).unwrap();
    let mut cmd = CommandBuilder::new(env!("CARGO_BIN_EXE_bnto"));
    cmd.arg("list");
    cmd.env("HOME", user);
    cmd.env("XDG_CONFIG_HOME", user.join(".config"));
    cmd.env("BNTO_HOME", bnto);
    cmd.env("BNTO_TELEMETRY_DISABLED", "1");
    for var in CI_VARS {
        cmd.env_remove(var);
    }
    for (key, value) in extra_env {
        cmd.env(key, value);
    }
    let mut child = pair.slave.spawn_command(cmd).unwrap();
    drop(pair.slave);
    let mut reader = pair.master.try_clone_reader().unwrap();
    let output = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = reader.read_to_end(&mut bytes);
        String::from_utf8_lossy(&bytes).into_owned()
    });
    child.wait().unwrap();
    drop(pair.master);
    output.join().unwrap()
}

#[test]
#[cfg_attr(windows, ignore = "ConPTY runs not yet stable in CI")]
fn a_newer_cached_release_is_mentioned() {
    let (user, bnto) = sandbox();
    let out = run_in_terminal(user.path(), bnto.path(), &[]);
    let expected = format!(
        "bnto 99.0.0 is available (you have {}). Run `bnto upgrade`.",
        env!("CARGO_PKG_VERSION")
    );
    assert!(out.contains(&expected), "{out}");
}

#[test]
#[cfg_attr(windows, ignore = "ConPTY runs not yet stable in CI")]
fn the_env_var_opts_out() {
    let (user, bnto) = sandbox();
    let out = run_in_terminal(user.path(), bnto.path(), &[("BNTO_NO_UPDATE_CHECK", "1")]);
    assert!(!out.contains(NOTICE), "{out}");
}

#[test]
#[cfg_attr(windows, ignore = "ConPTY runs not yet stable in CI")]
fn the_config_opts_out() {
    let (user, bnto) = sandbox();
    std::fs::write(
        bnto.path().join("config.toml"),
        "[updates]\ncheck = false\n",
    )
    .unwrap();
    let out = run_in_terminal(user.path(), bnto.path(), &[]);
    assert!(!out.contains(NOTICE), "{out}");
}

#[test]
#[cfg_attr(windows, ignore = "ConPTY runs not yet stable in CI")]
fn ci_never_sees_it() {
    let (user, bnto) = sandbox();
    let out = run_in_terminal(user.path(), bnto.path(), &[("CI", "true")]);
    assert!(!out.contains(NOTICE), "{out}");
}

#[test]
fn piped_output_never_sees_it() {
    let (user, bnto) = sandbox();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_bnto"))
        .arg("list")
        .env("HOME", user.path())
        .env("BNTO_HOME", bnto.path())
        .env("BNTO_TELEMETRY_DISABLED", "1")
        .output()
        .unwrap();
    let all = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!all.contains(NOTICE), "{all}");
}
