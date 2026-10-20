// keep-awake recipe — start / status / stop round-trips through the real CLI.
// macOS only: the recipe drives `caffeinate` and `osascript`. Each test gets
// its own BNTO_HOME and HOME, so it never sees or touches the user's live
// session, even if the script's home path breaks.
#![cfg(target_os = "macos")]

mod helpers;

use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

use helpers::{bnto_bin, temp_output_dir};

const START: [&str; 4] = [
    "action=start",
    "duration=1800",
    "display=allow-sleep",
    "jiggle=off",
];

/// A sandbox for one test: `bnto` is the BNTO_HOME, `user` the HOME.
/// Dropping it stops any session a failed assertion left running.
struct Sandbox {
    bnto: TempDir,
    user: TempDir,
}

impl Sandbox {
    fn new() -> Self {
        Self {
            bnto: temp_output_dir(),
            user: temp_output_dir(),
        }
    }

    /// Run keep-awake with `params`, returning stdout + stderr.
    fn run(&self, params: &[&str]) -> String {
        let (ok, text) = self.try_run(params);
        assert!(ok, "keep-awake {params:?} failed: {text}");
        text
    }

    /// Run keep-awake with `params`: whether it succeeded, and its output.
    fn try_run(&self, params: &[&str]) -> (bool, String) {
        let mut cmd = Command::new(bnto_bin());
        cmd.env("BNTO_HOME", self.bnto.path())
            .env("HOME", self.user.path())
            .args(["run", "keep-awake", "--yes"]);
        for param in params {
            cmd.args(["--param", param]);
        }
        let output = cmd.output().unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        (output.status.success(), text)
    }

    /// The caffeinate pid the session's state file records.
    fn session_pid(&self) -> u32 {
        recorded_pid(&self.state_file()).expect("state records no pid")
    }

    fn state_file(&self) -> PathBuf {
        state_file(self.bnto.path())
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        if self.state_file().exists() {
            let _ = self.try_run(&["action=stop"]);
        }
        // A broken home path would have put the session under $HOME.
        let leaked = state_file(&self.user.path().join(".bnto"));
        if let Some(pid) = recorded_pid(&leaked) {
            let _ = Command::new("kill").arg(pid.to_string()).status();
        }
    }
}

fn state_file(home: &Path) -> PathBuf {
    home.join("state").join("keep-awake.json")
}

fn recorded_pid(state: &Path) -> Option<u32> {
    let text = std::fs::read_to_string(state).ok()?;
    let json: serde_json::Value = serde_json::from_str(&text).ok()?;
    json["pid"].as_u64().map(|pid| pid as u32)
}

/// The command line `pid` runs, as `ps` reports it.
fn command_line(pid: u32) -> String {
    let out = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "args="])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

#[test]
fn stop_without_a_session_says_nothing_to_stop() {
    let sandbox = Sandbox::new();
    let out = sandbox.run(&["action=stop"]);
    assert!(out.contains("nothing to stop"), "{out}");
}

#[test]
fn status_without_a_session_says_none_active() {
    let sandbox = Sandbox::new();
    let out = sandbox.run(&["action=status"]);
    assert!(out.contains("No active session"), "{out}");
}

#[test]
fn start_then_status_then_stop_round_trips() {
    // A fresh home has no state dir: start must create it.
    let sandbox = Sandbox::new();
    let start = sandbox.run(&START);
    assert!(start.contains("Session started"), "{start}");
    assert!(sandbox.state_file().exists(), "start must write state");

    let status = sandbox.run(&["action=status"]);
    assert!(status.contains("Session active"), "{status}");
    assert!(status.contains("Activity simulation: off"), "{status}");

    let stop = sandbox.run(&["action=stop"]);
    assert!(stop.contains("Session ended"), "{stop}");
    assert!(!sandbox.state_file().exists(), "stop must clear state");
}

#[test]
fn state_lives_in_bnto_home_not_the_user_home() {
    // Script mode clears the environment, so the script can't read
    // BNTO_HOME itself. It must get bnto's home from the engine.
    let sandbox = Sandbox::new();
    sandbox.run(&START);
    assert!(
        sandbox.state_file().exists(),
        "state missing from BNTO_HOME"
    );
    assert!(
        !state_file(&sandbox.user.path().join(".bnto")).exists(),
        "state leaked into $HOME/.bnto"
    );
}

#[test]
fn keeping_the_display_on_passes_caffeinate_its_display_flags() {
    let sandbox = Sandbox::new();
    sandbox.run(&[
        "action=start",
        "duration=1800",
        "display=keep-on",
        "jiggle=off",
    ]);
    let args = command_line(sandbox.session_pid());
    assert!(args.contains("caffeinate -du -i -s -t 1800"), "{args}");
}

#[test]
fn allowing_display_sleep_omits_the_display_flags() {
    let sandbox = Sandbox::new();
    sandbox.run(&START);
    let args = command_line(sandbox.session_pid());
    assert!(args.contains("caffeinate -i -s -t 1800"), "{args}");
}

#[test]
fn an_unlisted_display_value_is_refused_before_anything_runs() {
    // SEC-13: the old flag-shaped value is no longer an option.
    let sandbox = Sandbox::new();
    let (ok, out) = sandbox.try_run(&["action=start", "display=-du", "jiggle=off"]);
    assert!(!ok, "{out}");
    assert!(!sandbox.state_file().exists(), "a session started: {out}");
}

#[test]
fn starting_again_replaces_the_running_session() {
    let sandbox = Sandbox::new();
    sandbox.run(&START);
    let again = sandbox.run(&START);
    assert!(again.contains("Replaced previous session"), "{again}");
}

#[test]
fn stale_state_from_a_dead_session_reads_as_no_session() {
    // A crash or reboot leaves state pointing at a pid that's gone.
    let sandbox = Sandbox::new();
    std::fs::create_dir_all(sandbox.bnto.path().join("state")).unwrap();
    std::fs::write(
        sandbox.state_file(),
        r#"{"pid": 999999, "label": "1 hour"}"#,
    )
    .unwrap();
    let out = sandbox.run(&["action=status"]);
    assert!(out.contains("No active session"), "{out}");
}
