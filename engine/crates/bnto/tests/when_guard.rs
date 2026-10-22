// `when` guards through the built binary (CLI journeys CJ1-CJ3, CJ6-CJ8 in
// .claude/journeys/cli.md): a guarded step runs or is skipped with its files
// passed through. Every run uses a temp HOME and BNTO_HOME.

mod helpers;

use std::path::Path;
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

/// A `file-rename` step adding `prefix`, guarded by `when` when given.
fn rename(id: &str, prefix: &str, when: Option<Value>) -> Value {
    let mut node = json!({ "id": id, "type": "file-rename", "parameters": { "prefix": prefix } });
    if let Some(when) = when {
        node["when"] = when;
    }
    node
}

fn guard(value: &str, op: &str, expected: &str) -> Value {
    json!({ "value": value, "op": op, "expected": expected })
}

/// A recipe running `steps` between an input and an output step.
fn recipe(steps: Vec<Value>) -> Value {
    let mut nodes = vec![json!({ "id": "input", "type": "input" })];
    nodes.extend(steps);
    nodes.push(json!({ "id": "output", "type": "output" }));
    json!({ "id": "guarded", "type": "group", "name": "Guarded", "nodes": nodes })
}

/// A sandbox holding `recipe` as `guarded.bnto.json` and the given inputs.
struct Sandbox {
    root: tempfile::TempDir,
}

impl Sandbox {
    fn new(recipe: &Value, inputs: &[&str]) -> Self {
        let root = temp_output_dir();
        let text = serde_json::to_string_pretty(recipe).unwrap();
        std::fs::write(root.path().join("guarded.bnto.json"), text).unwrap();
        std::fs::create_dir_all(root.path().join("in")).unwrap();
        for name in inputs {
            std::fs::write(root.path().join("in").join(name), name).unwrap();
        }
        Self { root }
    }

    fn path(&self, rel: &str) -> String {
        self.root.path().join(rel).to_string_lossy().into_owned()
    }

    fn bnto(&self, args: &[&str]) -> Output {
        Command::new(bnto_bin())
            .args(args)
            .env("HOME", self.root.path().join("home"))
            .env("BNTO_HOME", self.root.path().join("bnto-home"))
            .env("BNTO_TELEMETRY_DISABLED", "1")
            .env("BNTO_NO_UPDATE_CHECK", "1")
            .output()
            .unwrap()
    }

    /// `bnto run` on every input; the names written to the output folder.
    fn run(&self) -> Vec<String> {
        self.run_with_stderr().0
    }

    /// Like `run`, with what the run printed on stderr.
    fn run_with_stderr(&self) -> (Vec<String>, String) {
        let output = self.run_raw(&[]);
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        assert!(output.status.success(), "stderr: {stderr}");
        (names_in(&self.root.path().join("out")), stderr)
    }

    /// `bnto run` on every input with `extra` args, into a fresh output folder.
    fn run_raw(&self, extra: &[&str]) -> Output {
        let _ = std::fs::remove_dir_all(self.root.path().join("out"));
        let inputs: Vec<String> = std::fs::read_dir(self.root.path().join("in"))
            .unwrap()
            .map(|e| e.unwrap().path().to_string_lossy().into_owned())
            .collect();
        let (recipe, out) = (self.path("guarded.bnto.json"), self.path("out"));
        let mut args = vec!["run", recipe.as_str()];
        args.extend(inputs.iter().map(String::as_str));
        args.extend(["--yes", "-o", out.as_str()]);
        args.extend(extra);
        self.bnto(&args)
    }

    /// `bnto run` with `extra` args; the names written to the output folder.
    fn run_args(&self, extra: &[&str]) -> Vec<String> {
        let output = self.run_raw(extra);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "stderr: {stderr}");
        names_in(&self.root.path().join("out"))
    }
}

fn names_in(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

// --- CJ1: a guard runs or skips its step ---

#[test]
fn cj1_a_holding_guard_runs_its_step() {
    let when = guard("{{ctx.platform}}", "equals", host_platform());
    let sandbox = Sandbox::new(&recipe(vec![rename("tag", "on-", Some(when))]), &["a.txt"]);
    assert_eq!(sandbox.run(), ["on-a.txt"]);
}

#[test]
fn cj1_a_failing_guard_passes_the_file_through_untouched() {
    let when = guard("{{ctx.platform}}", "not-equals", host_platform());
    let sandbox = Sandbox::new(&recipe(vec![rename("tag", "on-", Some(when))]), &["a.txt"]);
    let (names, stderr) = sandbox.run_with_stderr();
    assert_eq!(names, ["a.txt"]);
    // The step is reported skipped, with the guard and what it read.
    let line = stderr
        .lines()
        .find(|l| l.contains("file-rename"))
        .unwrap_or_else(|| panic!("no step line: {stderr}"));
    assert!(line.contains("skipped"), "{line}");
    assert!(line.contains("when {{ctx.platform}} not-equals"), "{line}");
}

// --- CJ3: per-OS groups ---

// --- CJ2: an if/else pair on a container's field, flipped with --param ---

/// A `session` group declaring `action` (start or stop), holding a rename
/// for each value guarded on it, plus `extra` steps after the group.
fn if_else_recipe(extra: Vec<Value>) -> Value {
    let on = |action: &str| guard("{{fields.action}}", "equals", action);
    let mut steps = vec![json!({ "id": "session", "type": "group",
        "fields": { "action": { "type": "enum", "label": "Action", "default": "start",
            "options": [{ "value": "start", "label": "Start" }, { "value": "stop", "label": "Stop" }] } },
        "nodes": [rename("start", "start-", Some(on("start"))), rename("stop", "stop-", Some(on("stop")))] })];
    steps.extend(extra);
    recipe(steps)
}

#[test]
fn cj2_param_flips_an_if_else_pair_on_a_groups_field() {
    let sandbox = Sandbox::new(&if_else_recipe(Vec::new()), &["a.txt"]);
    assert_eq!(sandbox.run(), ["start-a.txt"]);
    assert_eq!(sandbox.run_args(&["-p", "action=stop"]), ["stop-a.txt"]);
    assert_eq!(
        sandbox.run_args(&["-p", "session:action=stop"]),
        ["stop-a.txt"]
    );
}

#[test]
fn cj2_a_field_two_nodes_declare_needs_its_node_named() {
    let mut again = rename("again", "x-", None);
    again["fields"] = json!({ "action": { "type": "string", "label": "Action" } });
    let sandbox = Sandbox::new(&if_else_recipe(vec![again]), &["a.txt"]);
    let output = sandbox.run_raw(&["-p", "action=stop"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr: {stderr}");
    assert!(stderr.contains("\"session\", \"again\""), "{stderr}");
    assert!(!sandbox.root.path().join("out").exists(), "nothing ran");
}

#[test]
fn cj3_only_the_hosts_group_runs() {
    let groups = ["macos", "linux", "windows"].map(|os| {
        json!({ "id": os, "type": "group",
                "when": guard("{{ctx.platform}}", "equals", os),
                "nodes": [rename(&format!("{os}-tag"), &format!("{os}-"), None)] })
    });
    let sandbox = Sandbox::new(&recipe(groups.to_vec()), &["a.txt"]);
    assert_eq!(sandbox.run(), [format!("{}-a.txt", host_platform())]);
}

// --- CJ6: bnto check ---

#[test]
fn cj6_check_refuses_an_unknown_op_and_a_guard_on_an_io_node() {
    let mut bad = recipe(vec![rename("tag", "on-", Some(guard("a", "is", "a")))]);
    bad["nodes"][0]["when"] = guard("a", "equals", "a");
    let sandbox = Sandbox::new(&bad, &[]);
    let output = sandbox.bnto(&["check", &sandbox.path("guarded.bnto.json")]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success(), "{stdout}");
    assert!(
        stdout.contains("node \"input\" (input): \"when\" has no effect"),
        "{stdout}"
    );
    assert!(stdout.contains("node \"tag\" (file-rename)"), "{stdout}");
    assert!(stdout.contains("'op' \"is\" is not one of"), "{stdout}");
    assert!(stdout.contains("Check: FAIL (2 findings)"), "{stdout}");
}

// --- CJ7: the dry run shows what this host skips ---

#[test]
fn cj7_dry_run_marks_the_other_oses_commands_skipped() {
    let groups = ["macos", "linux", "windows"].map(|os| {
        json!({ "id": os, "type": "group",
                "when": guard("{{ctx.platform}}", "equals", os),
                "nodes": [{ "id": format!("{os}-say"), "type": "shell-run",
                            "parameters": { "command": format!("say-{os}") } }] })
    });
    let sandbox = Sandbox::new(&recipe(groups.to_vec()), &[]);
    let library = sandbox.root.path().join("bnto-home").join("recipes");
    std::fs::create_dir_all(&library).unwrap();
    std::fs::copy(
        sandbox.path("guarded.bnto.json"),
        library.join("guarded.bnto.json"),
    )
    .unwrap();

    let output = sandbox.bnto(&["dry-run", "guarded"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{stdout}");
    let skipped = stdout.matches("Skipped on this host").count();
    assert_eq!(skipped, 2, "the other two OSes' commands:\n{stdout}");
    let host_block = stdout
        .split(&format!("say-{}", host_platform()))
        .nth(1)
        .unwrap_or_else(|| panic!("no host command:\n{stdout}"));
    let next_line = host_block.lines().take(3).collect::<Vec<_>>().join("\n");
    assert!(!next_line.contains("Skipped"), "{next_line}");
}

// --- CJ8: a guard on each file inside a loop ---

#[test]
fn cj8_a_guard_on_item_handles_matching_files_and_passes_the_rest() {
    let each = json!({ "id": "each", "type": "loop", "nodes": [
        rename("tag", "photo-", Some(guard("{{item.extension}}", "equals", "jpg")))
    ] });
    let inspect = json!({ "id": "inspect", "type": "file-inspect" });
    let sandbox = Sandbox::new(&recipe(vec![inspect, each]), &["a.jpg", "b.txt"]);
    assert_eq!(sandbox.run(), ["b.txt", "photo-a.jpg"]);
}
