// keep-awake recipe — start / status / stop round-trips through the real CLI.
// macOS only: the recipe drives `caffeinate` and `osascript`. Each test gets
// its own BNTO_HOME and HOME, so it never sees or touches the user's live
// session, even if the script's home path breaks.
#![cfg(target_os = "macos")]

mod helpers;

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tempfile::TempDir;

use helpers::{bnto_bin, recipe_path, temp_output_dir};

const START: [&str; 4] = [
    "action=start",
    "duration=1800",
    "display=allow-sleep",
    "jiggle=off",
];

/// Arguments that make osascript look like the recipe's cursor nudger.
const NUDGER_LOOKALIKE: [&str; 7] = [
    "-l",
    "JavaScript",
    "-e",
    "function run(argv) { delay(30) }",
    "loop",
    "60",
    "0",
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
        let output = self.command(params).output().unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        (output.status.success(), text)
    }

    /// `bnto run keep-awake --yes` with `params`, inside this sandbox.
    fn command(&self, params: &[&str]) -> Command {
        let mut cmd = Command::new(bnto_bin());
        cmd.env("BNTO_HOME", self.bnto.path())
            .env("HOME", self.user.path())
            .args(["run", "keep-awake", "--yes"]);
        for param in params {
            cmd.args(["--param", param]);
        }
        cmd
    }

    /// The caffeinate pid the session's state file records.
    fn session_pid(&self) -> u32 {
        recorded_pid(&self.state_file()).expect("state records no pid")
    }

    fn state_file(&self) -> PathBuf {
        state_file(self.bnto.path())
    }

    /// Write a state file as if an earlier session had left it.
    fn write_state(&self, json: &str) {
        std::fs::create_dir_all(self.bnto.path().join("state")).unwrap();
        std::fs::write(self.state_file(), json).unwrap();
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        // Kill what the state records directly, rather than trusting the
        // Stop under test. A broken home path would have put it under $HOME.
        let leaked = state_file(&self.user.path().join(".bnto"));
        let mut pids = recorded_pids(&self.state_file());
        pids.extend(recorded_pids(&leaked));
        if self.state_file().exists() {
            let _ = self.try_run(&["action=stop"]);
        }
        for pid in pids {
            let _ = Command::new("kill").arg(pid.to_string()).status();
        }
    }
}

/// A process the test starts itself, killed when dropped.
struct Spawned(Child);

impl Spawned {
    fn new(program: &str, args: &[&str]) -> Self {
        let child = Command::new(program)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        Self(child)
    }

    fn pid(&self) -> u32 {
        self.0.id()
    }

    fn running(&mut self) -> bool {
        self.0.try_wait().unwrap().is_none()
    }

    /// Whether it exits within a few seconds.
    fn exits(&mut self) -> bool {
        eventually(|| !self.running())
    }
}

impl Drop for Spawned {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn state_file(home: &Path) -> PathBuf {
    home.join("state").join("keep-awake.json")
}

fn recorded_pid(state: &Path) -> Option<u32> {
    read_state(state)?["pid"].as_u64().map(|pid| pid as u32)
}

/// The session and nudger pids a state file records.
fn recorded_pids(state: &Path) -> Vec<u32> {
    let Some(json) = read_state(state) else {
        return Vec::new();
    };
    ["pid", "jigglePid"]
        .iter()
        .filter_map(|key| json[key].as_u64().map(|pid| pid as u32))
        .collect()
}

fn read_state(state: &Path) -> Option<serde_json::Value> {
    let text = std::fs::read_to_string(state).ok()?;
    serde_json::from_str(&text).ok()
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Whether `done` holds within a few seconds.
fn eventually(mut done: impl FnMut() -> bool) -> bool {
    for _ in 0..50 {
        if done() {
            return true;
        }
        sleep(Duration::from_millis(100));
    }
    false
}

/// Whether `pid`, which the test didn't start, exits within a few seconds.
fn exits(pid: u32) -> bool {
    eventually(|| command_line(pid).is_empty())
}

/// Every `duration` option of the recipe: (value, label).
fn duration_options() -> Vec<(String, String)> {
    let text = std::fs::read_to_string(recipe_path("keep-awake")).unwrap();
    let recipe: serde_json::Value = serde_json::from_str(&text).unwrap();
    // The session step, inside the macOS group.
    let session = &recipe["nodes"][1]["nodes"][0];
    let options = &session["fields"]["duration"]["options"];
    options
        .as_array()
        .unwrap()
        .iter()
        .map(|o| {
            (
                o["value"].as_str().unwrap().into(),
                o["label"].as_str().unwrap().into(),
            )
        })
        .collect()
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
fn status_ends_with_the_session_log_not_the_output_file_name() {
    let sandbox = Sandbox::new();
    let output = sandbox.command(&["action=status"]).output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    let lines: Vec<&str> = stderr.lines().filter(|l| !l.trim().is_empty()).collect();
    let summary = &lines[lines.len() - 3..];
    assert_eq!(summary[0], "Session Log:", "{stderr}");
    assert_eq!(summary[1], "  No active session.", "{stderr}");
    assert!(summary[2].starts_with("Done. in "), "{stderr}");
    assert!(!stderr.contains("sh-output"), "{stderr}");
}

#[test]
fn status_json_reports_the_message() {
    let sandbox = Sandbox::new();
    let output = sandbox
        .command(&["action=status"])
        .arg("--json")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let summary: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(summary["status"], "pass", "{stdout}");
    assert_eq!(summary["message"], "No active session.", "{stdout}");
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
    let first = sandbox.session_pid();
    let again = sandbox.run(&START);
    assert!(again.contains("Replaced previous session"), "{again}");
    assert!(exits(first), "the first caffeinate is still running");
    assert_ne!(sandbox.session_pid(), first);
}

#[test]
fn an_indefinite_session_runs_until_stopped() {
    let sandbox = Sandbox::new();
    sandbox.run(&[
        "action=start",
        "duration=inf",
        "display=allow-sleep",
        "jiggle=off",
    ]);
    let args = command_line(sandbox.session_pid());
    assert!(args.contains("caffeinate -i -s -t 315360000"), "{args}");
    let status = sandbox.run(&["action=status"]);
    assert!(status.contains("running indefinitely"), "{status}");
}

#[test]
fn every_session_length_is_labelled_as_its_option() {
    let sandbox = Sandbox::new();
    for (value, label) in duration_options() {
        let duration = format!("duration={value}");
        let out = sandbox.run(&[
            "action=start",
            &duration,
            "display=allow-sleep",
            "jiggle=off",
        ]);
        let expected = if value == "inf" {
            "staying awake indefinitely".to_string()
        } else {
            format!("staying awake for {}.", label.to_lowercase())
        };
        assert!(out.contains(&expected), "{value}: {out}");
    }
}

#[test]
fn a_fresh_session_reports_its_full_length_remaining() {
    let sandbox = Sandbox::new();
    sandbox.run(&START);
    let status = sandbox.run(&["action=status"]);
    assert!(status.contains("0h 30m remaining"), "{status}");
}

#[test]
fn activity_simulation_starts_or_says_why_not() {
    // Whether it can start depends on this terminal's Accessibility
    // permission, so either outcome passes; Stop must end what started.
    let sandbox = Sandbox::new();
    let out = sandbox.run(&[
        "action=start",
        "duration=1800",
        "display=allow-sleep",
        "jiggle=60",
    ]);
    let nudger = read_state(&sandbox.state_file()).unwrap()["jigglePid"].as_u64();
    match nudger {
        None => assert!(out.contains("needs Accessibility permission"), "{out}"),
        Some(pid) => {
            assert!(command_line(pid as u32).contains("loop 60"), "{out}");
            sandbox.run(&["action=stop"]);
            assert!(exits(pid as u32), "the nudger outlived Stop");
        }
    }
}

#[test]
fn stop_ends_a_nudger_whose_session_already_ended() {
    // caffeinate died (say, `killall caffeinate`) but the nudger lives on.
    let sandbox = Sandbox::new();
    let mut nudger = Spawned::new("/usr/bin/osascript", &NUDGER_LOOKALIKE);
    sandbox.write_state(&format!(
        r#"{{"label": "indefinitely", "pid": 999999, "jigglePid": {}}}"#,
        nudger.pid()
    ));
    let status = sandbox.run(&["action=status"]);
    assert!(
        status.contains("Activity simulation is still running"),
        "{status}"
    );
    let out = sandbox.run(&["action=stop"]);
    assert!(out.contains("Session ended"), "{out}");
    assert!(nudger.exits(), "the orphaned nudger is still running");
}

#[test]
fn stop_leaves_processes_that_reused_recorded_pids_alone() {
    // A stale state file whose pids now belong to other programs.
    let sandbox = Sandbox::new();
    let mut other_caffeinate = Spawned::new("/usr/bin/caffeinate", &["-i", "-t", "30"]);
    let mut other_osascript = Spawned::new("/usr/bin/osascript", &["-e", "delay 30"]);
    sandbox.write_state(&format!(
        r#"{{"label": "1 hour", "pid": {}, "jigglePid": {}, "until": {}}}"#,
        other_caffeinate.pid(),
        other_osascript.pid(),
        now() + 3600
    ));
    let status = sandbox.run(&["action=status"]);
    assert!(status.contains("No active session"), "{status}");
    let out = sandbox.run(&["action=stop"]);
    assert!(out.contains("nothing to stop"), "{out}");
    sleep(Duration::from_millis(300));
    assert!(other_caffeinate.running(), "Stop killed another caffeinate");
    assert!(other_osascript.running(), "Stop killed another osascript");
}

#[test]
fn a_session_past_its_end_time_reads_as_none() {
    // A timed caffeinate exits at its end, so one still at that pid
    // afterwards isn't the session, even with the same command line.
    let sandbox = Sandbox::new();
    let mut lookalike = Spawned::new("/usr/bin/caffeinate", &["-i", "-s", "-t", "30"]);
    sandbox.write_state(&format!(
        r#"{{"label": "1 hour", "pid": {}, "until": {}}}"#,
        lookalike.pid(),
        now() - 60
    ));
    let status = sandbox.run(&["action=status"]);
    assert!(status.contains("No active session"), "{status}");
    sandbox.run(&["action=stop"]);
    sleep(Duration::from_millis(300));
    assert!(
        lookalike.running(),
        "Stop killed a process past the session's end"
    );
}

#[test]
fn stale_state_from_a_dead_session_reads_as_no_session() {
    // A crash or reboot leaves state pointing at a pid that's gone.
    let sandbox = Sandbox::new();
    sandbox.write_state(r#"{"pid": 999999, "label": "1 hour"}"#);
    let out = sandbox.run(&["action=status"]);
    assert!(out.contains("No active session"), "{out}");
}
