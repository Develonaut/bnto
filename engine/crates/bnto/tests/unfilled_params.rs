// `bnto run` and `bnto dry-run` refuse a required param with no value before
// consent and before the `-o` folder is made, naming the `-p` that fills it
// (ISS-74). The engine's pre-flight refuses the same run later.

mod helpers;

use helpers::{Sandbox, fixture_image, recipe_path};

/// What Watermark Images says with its overlay left empty.
const OVERLAY_HINT: &str = "Node 'overlay': param \"overlay\" (Overlay Image) is required: \
                            pass -p overlay:overlay=<path>";

/// A shell step (which needs consent) before an overlay step with no image.
const SHELL_THEN_OVERLAY: &str = r#"{ "id": "shell-then-overlay", "name": "Shell Then Overlay",
  "nodes": [{ "id": "input", "type": "input" },
    { "id": "say", "type": "shell-run", "parameters": { "command": "echo", "args": ["hi"] } },
    { "id": "stamp", "type": "image-overlay", "parameters": { "overlay": "" } },
    { "id": "output", "type": "output" }] }"#;

/// One shell step and nothing else.
const SAY: &str = r#"{ "id": "say", "name": "Say",
  "nodes": [{ "id": "input", "type": "input", "parameters": { "mode": "none" } },
    { "id": "say", "type": "shell-run", "parameters": { "command": "echo", "args": ["hi"] } }] }"#;

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn run_refuses_an_empty_required_param_before_making_the_output_folder() {
    let sandbox = Sandbox::new();
    let out = sandbox.bnto_home().with_file_name("out");
    let output = sandbox
        .command()
        .args(["run", &recipe_path("watermark-images"), "--yes"])
        .arg(fixture_image("small.jpg"))
        .arg("-o")
        .arg(&out)
        .output()
        .unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(err.contains(OVERLAY_HINT), "{err}");
    assert!(!out.exists(), "refused before the output folder is made");
}

#[test]
fn run_refuses_an_empty_required_param_before_the_consent_prompt() {
    let sandbox = Sandbox::new();
    let recipe = sandbox
        .bnto_home()
        .with_file_name("shell-then-overlay.bnto.json");
    std::fs::write(&recipe, SHELL_THEN_OVERLAY).unwrap();
    let output = sandbox
        .command()
        .arg("run")
        .arg(&recipe)
        .arg(fixture_image("small.jpg"))
        .output()
        .unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(
        err.contains("Node 'stamp': param \"overlay\" (Overlay Image) is required: pass -p stamp:overlay=<path>"),
        "{err}"
    );
    assert!(
        !err.contains("Allow execution?"),
        "asked for consent first: {err}"
    );
}

#[test]
fn dry_run_refuses_an_empty_required_param() {
    let sandbox = Sandbox::new();
    let with_input = sandbox
        .command()
        .args(["dry-run", "watermark-images"])
        .arg(fixture_image("small.jpg"))
        .output()
        .unwrap();
    let without = sandbox
        .command()
        .args(["dry-run", "watermark-images"])
        .output()
        .unwrap();
    for output in [with_input, without] {
        let err = stderr(&output);
        assert_eq!(output.status.code(), Some(1), "{err}");
        assert!(err.contains(OVERLAY_HINT), "{err}");
    }
}

#[test]
fn dry_run_with_the_param_given_passes_the_check() {
    let sandbox = Sandbox::new();
    let overlay = format!("overlay:overlay={}", fixture_image("small.png"));
    let output = sandbox
        .command()
        .args(["dry-run", "watermark-images", "-p", &overlay])
        .output()
        .unwrap();
    let err = stderr(&output);
    assert!(output.status.success(), "{err}");
}

#[test]
fn dry_run_refuses_a_p_naming_a_param_a_shell_step_lacks() {
    // The dry run doesn't execute a shell recipe, so before this only `bnto
    // run` refused it, in pre-flight.
    let sandbox = Sandbox::new();
    let recipe = sandbox.bnto_home().with_file_name("say.bnto.json");
    std::fs::write(&recipe, SAY).unwrap();
    let output = sandbox
        .command()
        .arg("dry-run")
        .arg(&recipe)
        .args(["-p", "say:timout=30"])
        .output()
        .unwrap();
    let err = stderr(&output);
    assert_eq!(output.status.code(), Some(1), "{err}");
    assert!(
        err.contains("Node 'say': shell-run has no param \"timout\""),
        "{err}"
    );
}

#[test]
fn a_param_the_input_fills_is_not_refused() {
    // Download Video's step takes its `url` from the run's input.
    let sandbox = Sandbox::new();
    let output = sandbox
        .command()
        .args(["dry-run", "download-video"])
        .output()
        .unwrap();
    let err = stderr(&output);
    assert!(output.status.success(), "{err}");
}
