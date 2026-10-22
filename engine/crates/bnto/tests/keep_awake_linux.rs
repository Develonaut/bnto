// keep-awake on Linux — start / status / stop round-trips through the real
// CLI, driving `systemd-inhibit`. Each test gets its own BNTO_HOME and HOME
// and no desktop session variables, so it never sees or touches the user's
// live session.
//
// Outside a login session (a CI runner), polkit refuses a sleep inhibitor;
// the round trip then checks the refusal is clear and leaves nothing
// behind. BNTO_REQUIRE_KEEP_AWAKE=1 makes a refusal fail instead.
#![cfg(target_os = "linux")]

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

/// Desktop variables the recipe passes to its script; cleared so a test
/// behaves the same on a desktop and on a headless runner.
const DESKTOP_VARS: [&str; 5] = [
    "DISPLAY",
    "XAUTHORITY",
    "WAYLAND_DISPLAY",
    "DBUS_SESSION_BUS_ADDRESS",
    "XDG_RUNTIME_DIR",
];

/// A sandbox for one test: `bnto` is the BNTO_HOME, `user` the HOME.
/// Dropping it ends any session a failed assertion left running.
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
        for name in DESKTOP_VARS {
            cmd.env_remove(name);
        }
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

    /// Start a session, or `None` when systemd-inhibit is missing or polkit
    /// refused it (asserting the refusal is clear and left nothing behind).
    fn start(&self, params: &[&str]) -> Option<String> {
        if !Path::new("/usr/bin/systemd-inhibit").exists()
            && !Path::new("/bin/systemd-inhibit").exists()
        {
            return None;
        }
        let (ok, text) = self.try_run(params);
        if ok {
            return Some(text);
        }
        assert!(
            text.contains("systemd refused to keep this computer awake"),
            "start failed some other way: {text}"
        );
        assert!(!self.state_file().exists(), "a refused start wrote state");
        let required = std::env::var("BNTO_REQUIRE_KEEP_AWAKE").as_deref() == Ok("1");
        assert!(!required, "BNTO_REQUIRE_KEEP_AWAKE=1 but: {text}");
        None
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        if self.state_file().exists() {
            let _ = self.try_run(&["action=stop"]);
        }
        for pid in recorded_pids(&self.state_file()) {
            let _ = Command::new("kill").arg(pid.to_string()).status();
        }
    }
}

/// The pids a state file records.
fn recorded_pids(state: &Path) -> Vec<u32> {
    let Ok(text) = std::fs::read_to_string(state) else {
        return Vec::new();
    };
    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
    ["pid", "displayPid", "jigglePid"]
        .iter()
        .filter_map(|key| json[key].as_u64().map(|pid| pid as u32))
        .collect()
}

fn alive(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
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
    let Some(started) = sandbox.start(&START) else {
        return;
    };
    assert!(
        started.contains("Session started: staying awake for 30 minutes."),
        "{started}"
    );
    assert!(started.contains("Display: allowed to sleep"), "{started}");
    let pid = recorded_pids(&sandbox.state_file())[0];
    assert!(alive(pid), "the inhibitor isn't running");

    let status = sandbox.run(&["action=status"]);
    assert!(
        status.contains("Session active -- 0h 30m remaining of 30 minutes."),
        "{status}"
    );

    let stopped = sandbox.run(&["action=stop"]);
    assert!(stopped.contains("Session ended"), "{stopped}");
    assert!(!sandbox.state_file().exists());
    assert!(eventually(|| !alive(pid)), "the inhibitor outlived Stop");
}

#[test]
fn activity_simulation_without_x11_starts_anyway_with_a_notice() {
    let sandbox = Sandbox::new();
    let params = [
        "action=start",
        "duration=1800",
        "display=keep-on",
        "jiggle=120",
    ];
    let Some(started) = sandbox.start(&params) else {
        return;
    };
    assert!(started.contains("Session started"), "{started}");
    assert!(started.contains("Activity simulation: off"), "{started}");
    assert!(
        started.contains("NOTICE: activity simulation needs an X11"),
        "{started}"
    );
}

#[test]
fn a_second_start_replaces_the_first() {
    let sandbox = Sandbox::new();
    let Some(_) = sandbox.start(&START) else {
        return;
    };
    let first = recorded_pids(&sandbox.state_file())[0];
    let again = sandbox.run(&START);
    assert!(again.contains("Replaced previous session"), "{again}");
    assert!(
        eventually(|| !alive(first)),
        "the first inhibitor kept running"
    );
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
