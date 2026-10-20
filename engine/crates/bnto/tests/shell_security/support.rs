// Shared harness: a throwaway lab folder per test, a canary file that any
// successful injection creates, and recipe builders that wrap one
// shell-run step so each test only states the params under attack.
// Several modules are Unix- or Windows-only, so not every helper is used
// on every OS.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use crate::helpers::bnto_bin;

/// A temp folder holding the recipe, a private BNTO_HOME and the canary.
pub struct Lab {
    dir: tempfile::TempDir,
}

/// What a `bnto` invocation printed and whether it exited 0.
pub struct Outcome {
    pub ok: bool,
    /// stdout and stderr together: step output lands in progress lines.
    pub text: String,
}

impl Lab {
    pub fn new() -> Self {
        Self {
            dir: tempfile::tempdir().unwrap(),
        }
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    /// The file an injected command tries to create.
    pub fn canary(&self) -> PathBuf {
        self.path().join("canary")
    }

    pub fn canary_fired(&self) -> bool {
        self.canary().exists()
    }

    /// Write a recipe wrapping one shell-run `step` and return its path.
    pub fn recipe(&self, step: Value) -> String {
        self.recipe_with(step, json!({ "mode": "none" }))
    }

    /// Like `recipe`, for a step fed by files passed on the command line.
    pub fn file_recipe(&self, step: Value) -> String {
        self.recipe_with(step, json!({ "mode": "file-upload", "multiple": true }))
    }

    /// Like `recipe`, with the input node's params spelled out.
    pub fn recipe_with(&self, step: Value, input: Value) -> String {
        let path = self.path().join("attack.bnto.json");
        std::fs::write(&path, recipe_json(step, input).to_string()).unwrap();
        path.to_string_lossy().into_owned()
    }

    /// `bnto run <recipe> --yes` with `--param` pairs, in this lab.
    pub fn run(&self, recipe: &str, params: &[&str]) -> Outcome {
        self.run_with(recipe, params, &[], &[])
    }

    /// `bnto run` with extra input operands and env vars set on bnto itself.
    pub fn run_with(
        &self,
        recipe: &str,
        params: &[&str],
        inputs: &[&str],
        env: &[(&str, &str)],
    ) -> Outcome {
        let mut cmd = self.bnto();
        cmd.args(["run", recipe, "--yes"]);
        for param in params {
            cmd.args(["--param", param]);
        }
        // Inputs last, so a test can lead with `--` to pass a dash value.
        cmd.args(inputs).envs(env.iter().copied());
        outcome(cmd.output().unwrap())
    }

    /// `run_with`, killed after `secs`; `None` means it never finished.
    /// For tests whose regression is a hang, so CI fails instead of stalling.
    pub fn run_within(
        &self,
        secs: u64,
        recipe: &str,
        params: &[&str],
        env: &[(&str, &str)],
    ) -> Option<Outcome> {
        let mut cmd = self.bnto();
        cmd.args(["run", recipe, "--yes"]);
        for param in params {
            cmd.args(["--param", param]);
        }
        let mut child = cmd
            .envs(env.iter().copied())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
        while child.try_wait().unwrap().is_none() {
            if std::time::Instant::now() >= deadline {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Some(outcome(child.wait_with_output().unwrap()))
    }

    /// `bnto run` without `--yes`, answering the consent prompt with `answer`.
    pub fn run_answering(&self, recipe: &str, answer: &str) -> Outcome {
        self.run_answering_with(recipe, answer, &[])
    }

    /// `run_answering` with env vars set on bnto itself.
    pub fn run_answering_with(&self, recipe: &str, answer: &str, env: &[(&str, &str)]) -> Outcome {
        let mut child = self
            .bnto()
            .args(["run", recipe])
            .envs(env.iter().copied())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        use std::io::Write;
        child
            .stdin
            .take()
            .unwrap()
            .write_all(answer.as_bytes())
            .unwrap();
        outcome(child.wait_with_output().unwrap())
    }

    /// `bnto check <recipe>`: static validation, nothing runs.
    pub fn check(&self, recipe: &str) -> Outcome {
        outcome(self.bnto().args(["check", recipe]).output().unwrap())
    }

    fn bnto(&self) -> Command {
        let mut cmd = Command::new(bnto_bin());
        cmd.current_dir(self.path())
            .env("BNTO_HOME", self.path().join("home"));
        cmd
    }
}

fn outcome(output: std::process::Output) -> Outcome {
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Outcome {
        ok: output.status.success(),
        text,
    }
}

/// A shell-run step with `params` (stdout output) and string `fields`.
pub fn step(params: Value, fields: &[&str]) -> Value {
    let mut params = params;
    params["outputMode"] = json!("stdout");
    let fields: serde_json::Map<String, Value> = fields
        .iter()
        .map(|name| (name.to_string(), json!({ "type": "string", "label": name })))
        .collect();
    json!({
        "id": "attack", "type": "shell-run", "version": "1.0.0", "name": "Attack",
        "position": { "x": 250, "y": 100 }, "metadata": {},
        "parameters": params, "fields": fields,
        "inputPorts": [], "outputPorts": []
    })
}

fn io_node(id: &str, params: Value) -> Value {
    json!({
        "id": id, "type": id, "version": "1.0.0", "name": id,
        "position": { "x": 0, "y": 100 }, "metadata": {},
        "parameters": params, "inputPorts": [], "outputPorts": []
    })
}

fn recipe_json(step: Value, input: Value) -> Value {
    json!({
        "id": "attack", "type": "group", "version": "1.0.0", "name": "Attack",
        "position": { "x": 0, "y": 0 },
        "metadata": { "description": "shell-run security probe", "category": "system" },
        "parameters": {}, "inputPorts": [], "outputPorts": [],
        "settings": { "iteration": "auto" },
        "nodes": [
            io_node("input", input),
            step,
            io_node("output", json!({ "mode": "message", "label": "Output" })),
        ],
        "edges": [
            { "id": "e1", "source": "input", "target": "attack" },
            { "id": "e2", "source": "attack", "target": "output" },
        ]
    })
}

/// Classic shell-injection strings, each trying to create `canary`.
/// Covers substitution, chaining, quote breakout, newline and $IFS forms
/// (the Ultralytics branch name needed no spaces at all).
pub fn payloads(canary: &Path) -> Vec<String> {
    let c = canary.display();
    vec![
        format!("$(touch {c})"),
        format!("`touch {c}`"),
        format!("; touch {c}"),
        format!("| touch {c}"),
        format!("&& touch {c}"),
        format!("' ; touch {c} ; '"),
        format!("\" ; touch {c} ; \""),
        format!("x\ntouch {c}"),
        format!("x;touch${{IFS}}{c}"),
    ]
}
