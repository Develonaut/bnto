// `bnto dry-run` prints each shell command, and the `bnto run` footer, quoted
// so a pasted line passes the same arguments the run would (ISS-50).

mod helpers;

use std::process::{Output, Stdio};

use helpers::Sandbox;

/// A shell step printing its `msg` field, one argument, on its own line.
const PRINT_MSG: &str = r#"{ "id": "print-msg", "name": "Print Msg",
  "nodes": [{ "id": "input", "type": "input", "parameters": { "mode": "none" } },
    { "id": "say", "type": "shell-run",
      "fields": { "msg": { "type": "string", "label": "Message" } },
      "parameters": { "command": "printf", "args": ["%s\\n", "{{fields.msg}}"] } }] }"#;

/// Every char sh would split on, expand or glob, and a quote.
const AWKWARD: &str = "a b $HOME;*'x";

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// `bnto dry-run <recipe> -p msg=<AWKWARD>` on PRINT_MSG saved as `file_name`.
fn dry_run_print_msg(sandbox: &Sandbox, file_name: &str) -> String {
    std::fs::write(sandbox.work_dir().join(file_name), PRINT_MSG).unwrap();
    let output = sandbox
        .command()
        .args(["dry-run", file_name, "-p", &format!("msg={AWKWARD}")])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {err}");
    stdout(&output)
}

#[cfg(unix)]
#[test]
fn a_shown_command_pasted_into_sh_passes_the_same_argument() {
    let sandbox = Sandbox::new();
    let out = dry_run_print_msg(&sandbox, "print.bnto.json");
    let line = out
        .lines()
        .find_map(|l| l.trim_start().strip_prefix("$ "))
        .unwrap_or_else(|| panic!("no command line: {out}"));
    let pasted = sandbox.sh(line).stdin(Stdio::null()).output().unwrap();
    assert!(pasted.status.success(), "{line}");
    assert_eq!(stdout(&pasted), format!("{AWKWARD}\n"), "{line}");
}

#[test]
fn the_run_with_footer_quotes_the_recipe_path_and_params() {
    let sandbox = Sandbox::new();
    let out = dry_run_print_msg(&sandbox, "my print.bnto.json");
    // sh closes, escapes and reopens a quote; PowerShell doubles it.
    let msg = if cfg!(windows) {
        "'msg=a b $HOME;*''x'"
    } else {
        r"'msg=a b $HOME;*'\''x'"
    };
    let footer = format!("Run with: bnto run 'my print.bnto.json' -p {msg}");
    assert!(out.contains(&footer), "{out}");
}
