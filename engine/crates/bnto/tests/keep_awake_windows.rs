// keep-awake on Windows — start / status / stop round-trips through the real
// CLI, driving a hidden PowerShell that holds SetThreadExecutionState. Each
// test gets its own BNTO_HOME, so it never sees or touches the user's live
// session; dropping the sandbox ends whatever session it started.
#![cfg(windows)]

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

/// A sandbox for one test: `bnto` is the BNTO_HOME, `user` the profile.
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

    /// Run keep-awake with `params`, which must succeed.
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
            .env("BNTO_TELEMETRY_DISABLED", "1")
            .env("BNTO_NO_UPDATE_CHECK", "1")
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

    fn state_file(&self) -> PathBuf {
        self.bnto.path().join("state").join("keep-awake.json")
    }

    /// The session pid the state file records.
    fn session_pid(&self) -> u32 {
        recorded_pid(&self.state_file()).expect("state records no pid")
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let pid = recorded_pid(&self.state_file());
        if self.state_file().exists() {
            let _ = self.try_run(&["action=stop"]);
        }
        if let Some(pid) = pid {
            let _ = Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .output();
        }
    }
}

fn recorded_pid(state: &Path) -> Option<u32> {
    let text = std::fs::read_to_string(state).ok()?;
    let json: serde_json::Value = serde_json::from_str(&text).ok()?;
    json["pid"].as_u64().map(|pid| pid as u32)
}

/// Whether a process with `pid` is running.
fn alive(pid: u32) -> bool {
    let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/NH", "/FO", "CSV"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stdout).contains(&format!("\"{pid}\""))
}

/// Whether `done` holds within a few seconds.
fn eventually(done: impl Fn() -> bool) -> bool {
    (0..50).any(|_| {
        if done() {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        false
    })
}

#[test]
fn status_and_stop_without_a_session() {
    let sandbox = Sandbox::new();
    assert!(
        sandbox
            .run(&["action=status"])
            .contains("No active session.")
    );
    let stopped = sandbox.run(&["action=stop"]);
    assert!(
        stopped.contains("No active session -- nothing to stop."),
        "{stopped}"
    );
}

#[test]
fn start_status_and_stop_round_trip() {
    let sandbox = Sandbox::new();
    let started = sandbox.run(&START);
    assert!(
        started.contains("Session started: staying awake for 30 minutes."),
        "{started}"
    );
    assert!(started.contains("Display: allowed to sleep"), "{started}");
    assert!(started.contains("Activity simulation: off"), "{started}");
    let pid = sandbox.session_pid();
    assert!(alive(pid), "the session process isn't running");

    let status = sandbox.run(&["action=status"]);
    assert!(
        status.contains("Session active -- 0h 30m remaining of 30 minutes."),
        "{status}"
    );

    let stopped = sandbox.run(&["action=stop"]);
    assert!(stopped.contains("Session ended"), "{stopped}");
    assert!(!sandbox.state_file().exists());
    assert!(eventually(|| !alive(pid)), "the session outlived Stop");
}

#[test]
fn a_second_start_replaces_the_first() {
    let sandbox = Sandbox::new();
    sandbox.run(&START);
    let first = sandbox.session_pid();
    let again = sandbox.run(&START);
    assert!(again.contains("Replaced previous session"), "{again}");
    assert!(
        eventually(|| !alive(first)),
        "the first session kept running"
    );
    assert_ne!(sandbox.session_pid(), first);
}

#[test]
fn keeping_the_display_on_with_activity_simulation_starts() {
    let sandbox = Sandbox::new();
    let params = [
        "action=start",
        "duration=inf",
        "display=keep-on",
        "jiggle=120",
    ];
    let started = sandbox.run(&params);
    assert!(
        started.contains("staying awake indefinitely (until you stop it)"),
        "{started}"
    );
    assert!(started.contains("Display: kept on"), "{started}");
    assert!(
        started.contains("Activity simulation: on (nudges the cursor after 120s idle)"),
        "{started}"
    );
    let status = sandbox.run(&["action=status"]);
    assert!(
        status.contains("Session active -- running indefinitely."),
        "{status}"
    );
    assert!(status.contains("Activity simulation: on"), "{status}");
}

#[test]
fn a_stale_pid_another_program_now_uses_is_left_alone() {
    let sandbox = Sandbox::new();
    let mut other = Command::new("ping")
        .args(["-n", "30", "127.0.0.1"])
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let state = sandbox.state_file();
    std::fs::create_dir_all(state.parent().unwrap()).unwrap();
    let json = format!(
        r#"{{"label":"1 hour","pid":{},"jiggle":false}}"#,
        other.id()
    );
    std::fs::write(&state, json).unwrap();

    assert!(
        sandbox
            .run(&["action=status"])
            .contains("No active session.")
    );
    sandbox.run(&["action=stop"]);
    assert!(
        other.try_wait().unwrap().is_none(),
        "Stop killed another program"
    );
    let _ = other.kill();
}
