// Dependencies scoped to the steps that use them, through the built binary:
// `requires` on a platform-guarded group is checked only on the OSes that
// group runs on (`bnto doctor`, `bnto install`, `bnto info`, `bnto run`'s
// pre-flight). Every command uses a temp HOME and BNTO_HOME.

mod helpers;

use std::process::{Command, Output};

use helpers::{bnto_bin, temp_output_dir};
use serde_json::{Value, json};

/// What `{{ctx.platform}}` reads on this machine.
fn host_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    }
}

/// An OS this machine isn't, with the label bnto prints for it.
fn other_platform() -> (&'static str, &'static str) {
    if cfg!(target_os = "macos") {
        ("linux", "Linux")
    } else {
        ("macos", "macOS")
    }
}

/// A group that runs on `os` only and needs `binary`.
fn os_group(id: &str, os: &str, binary: &str) -> Value {
    json!({ "id": id, "type": "group",
        "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": os },
        "requires": [{ "binary": binary }],
        "nodes": [{ "id": format!("{id}-step"), "type": "file-rename",
            "parameters": { "prefix": "x-" } }] })
}

/// A library recipe whose steps are `groups`.
fn recipe(groups: Vec<Value>) -> Value {
    let mut nodes = vec![json!({ "id": "input", "type": "input" })];
    nodes.extend(groups);
    nodes.push(json!({ "id": "output", "type": "output" }));
    json!({ "id": "scoped", "type": "group", "name": "Scoped", "nodes": nodes })
}

/// A temp HOME and BNTO_HOME, with `recipe` in the library as `scoped`.
struct Sandbox {
    root: tempfile::TempDir,
}

impl Sandbox {
    fn new(recipe: &Value) -> Self {
        let root = temp_output_dir();
        let library = root.path().join("bnto-home").join("recipes");
        std::fs::create_dir_all(&library).unwrap();
        let text = serde_json::to_string_pretty(recipe).unwrap();
        std::fs::write(library.join("scoped.bnto.json"), text).unwrap();
        Self { root }
    }

    fn bnto(&self, args: &[&str]) -> Output {
        Command::new(bnto_bin())
            .args(args)
            .env("HOME", self.root.path().join("home"))
            .env("BNTO_HOME", self.root.path().join("bnto-home"))
            .env("BNTO_TELEMETRY_DISABLED", "1")
            .env("BNTO_NO_UPDATE_CHECK", "1")
            .env("NO_COLOR", "1")
            .output()
            .unwrap()
    }

    fn stdout(&self, args: &[&str]) -> String {
        String::from_utf8_lossy(&self.bnto(args).stdout).into_owned()
    }
}

/// The line of `out` that names `binary`, or "" when none does.
fn line_naming<'a>(out: &'a str, binary: &str) -> &'a str {
    out.lines().find(|l| l.contains(binary)).unwrap_or("")
}

#[test]
fn doctor_checks_a_scoped_dependency_only_on_the_os_that_needs_it() {
    let (other, other_label) = other_platform();
    let sandbox = Sandbox::new(&recipe(vec![
        os_group("here", host_platform(), "bnto-test-host-only-tool"),
        os_group("there", other, "bnto-test-other-os-tool"),
    ]));
    let output = sandbox.bnto(&["doctor"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    let here = line_naming(&stdout, "bnto-test-host-only-tool");
    assert!(here.contains("MISSING"), "host dep is checked: {stdout}");
    let there = line_naming(&stdout, "bnto-test-other-os-tool");
    assert!(
        !there.contains("MISSING"),
        "other-OS dep isn't checked: {stdout}"
    );
    assert!(
        there.contains(&format!("{other_label} only")),
        "names its OS: {there}"
    );
    assert!(
        !output.status.success(),
        "the host's missing dep fails doctor"
    );
}

#[test]
fn doctor_tags_a_dependency_needed_here_with_its_os() {
    let sandbox = Sandbox::new(&recipe(vec![os_group(
        "here",
        host_platform(),
        "bnto-test-host-only-tool",
    )]));
    let stdout = sandbox.stdout(&["doctor"]);
    let line = line_naming(&stdout, "bnto-test-host-only-tool");
    assert!(line.contains(" only)"), "scoped dep names its OS: {line}");
}

#[test]
fn doctor_does_not_report_keep_awakes_macos_tools_missing_off_macos() {
    // Fix B's bug: caffeinate and osascript can't be installed off macOS.
    let sandbox = Sandbox::new(&recipe(vec![]));
    let stdout = sandbox.stdout(&["doctor"]);
    for binary in ["caffeinate", "osascript"] {
        let line = line_naming(&stdout, binary);
        assert!(line.contains("macOS only"), "{binary} is tagged: {stdout}");
        if cfg!(not(target_os = "macos")) {
            assert!(
                !line.contains("MISSING"),
                "{binary} isn't checked: {stdout}"
            );
        }
    }
}

#[test]
fn install_skips_a_dependency_another_os_needs() {
    let (other, _) = other_platform();
    let sandbox = Sandbox::new(&recipe(vec![os_group(
        "there",
        other,
        "bnto-test-other-os-tool",
    )]));
    let output = sandbox.bnto(&["install", "scoped", "--yes"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "stdout: {stdout}");
    assert!(stdout.contains("no external dependencies"), "got: {stdout}");
    assert!(!stdout.contains("bnto-test-other-os-tool"), "got: {stdout}");
}

#[test]
fn info_lists_every_dependency_with_the_os_that_needs_it() {
    let (other, other_label) = other_platform();
    let sandbox = Sandbox::new(&recipe(vec![
        os_group("here", host_platform(), "bnto-test-host-only-tool"),
        os_group("there", other, "bnto-test-other-os-tool"),
    ]));
    let stdout = sandbox.stdout(&["info", "scoped"]);
    assert!(stdout.contains("bnto-test-host-only-tool"), "got: {stdout}");
    let there = line_naming(&stdout, "bnto-test-other-os-tool");
    assert!(
        there.contains(&format!("{other_label} only")),
        "got: {stdout}"
    );
}

#[test]
fn run_preflight_skips_a_dependency_another_os_needs() {
    let (other, _) = other_platform();
    let sandbox = Sandbox::new(&recipe(vec![
        os_group("there", other, "bnto-test-other-os-tool"),
        json!({ "id": "rename", "type": "file-rename", "parameters": { "prefix": "y-" } }),
    ]));
    let input = sandbox.root.path().join("a.txt");
    std::fs::write(&input, "a").unwrap();
    let out = sandbox.root.path().join("out");
    let output = sandbox.bnto(&[
        "run",
        "scoped",
        input.to_str().unwrap(),
        "--yes",
        "-o",
        out.to_str().unwrap(),
    ]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");
    assert!(
        !stderr.contains("bnto-test-other-os-tool"),
        "stderr: {stderr}"
    );
}

#[test]
fn run_preflight_still_checks_a_dependency_this_os_needs() {
    let sandbox = Sandbox::new(&recipe(vec![os_group(
        "here",
        host_platform(),
        "bnto-test-host-only-tool",
    )]));
    let input = sandbox.root.path().join("a.txt");
    std::fs::write(&input, "a").unwrap();
    let out = sandbox.root.path().join("out");
    let output = sandbox.bnto(&[
        "run",
        "scoped",
        input.to_str().unwrap(),
        "--yes",
        "-o",
        out.to_str().unwrap(),
    ]);
    let all = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!output.status.success(), "output: {all}");
    assert!(all.contains("bnto-test-host-only-tool"), "output: {all}");
}
