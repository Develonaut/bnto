// SEC-09..10: what a recipe's `env` can put in a command's environment.
// Probed with `env`, which prints exactly what the child received.
#![cfg(unix)]

use serde_json::{Map, Value, json};

use crate::support::{Lab, step};

const EVIL: &str = "/bnto-evil";

/// Run `env` with the recipe setting every name in `names` to EVIL; return
/// the names that reached the child (or None if the run was refused).
fn names_that_reached_child(names: &[&str]) -> Option<Vec<String>> {
    let lab = Lab::new();
    let env: Map<String, Value> = names.iter().map(|n| (n.to_string(), json!(EVIL))).collect();
    let recipe = lab.recipe(step(
        json!({ "command": "env", "args": [], "env": env }),
        &[],
    ));
    let out = lab.run(&recipe, &[]);
    if !out.ok {
        return None;
    }
    let leaked = names
        .iter()
        .filter(|n| reached(&out.text, n))
        .map(|n| n.to_string())
        .collect();
    Some(leaked)
}

/// True when `env` printed `name=EVIL` as its own token (progress lines
/// carry a spinner prefix, and `ENV` must not match `BASH_ENV`).
fn reached(text: &str, name: &str) -> bool {
    let assignment = format!("{name}={EVIL}");
    text.split_whitespace().any(|token| token == assignment)
}

/// SEC-09: loader and lookup variables can't be set by a recipe, in any
/// letter case (Windows env names are case-insensitive).
#[test]
fn sec09_recipe_env_cannot_hijack_loader_or_lookup() {
    let names = [
        "PATH",
        "LD_PRELOAD",
        "LD_LIBRARY_PATH",
        "DYLD_INSERT_LIBRARIES",
        "DYLD_LIBRARY_PATH",
        "DYLD_FRAMEWORK_PATH",
        "DYLD_ANYTHING",
        "HOME",
        "SHELL",
        "TMPDIR",
        "path",
        "Ld_Preload",
    ];
    let leaked = names_that_reached_child(&names).expect("run refused");
    assert_eq!(leaked, Vec::<String>::new());
}

/// SEC-10: shell startup variables run code before a script's first line:
/// SHELLOPTS=xtrace + PS4='$(cmd)' executes under bash and macOS /bin/sh,
/// BASH_FUNC_x%% replaces commands, BASH_ENV / ZDOTDIR source files, and
/// ksh FPATH autoloads functions. Env values are templated, so letting a
/// recipe set these turns data back into code. They must never reach the
/// child — stripped or refused.
#[test]
fn sec10_recipe_env_cannot_set_shell_startup_variables() {
    let names = [
        "BASH_ENV",
        "ENV",
        "SHELLOPTS",
        "BASHOPTS",
        "PS1",
        "PS2",
        "PS3",
        "PS4",
        "PROMPT_COMMAND",
        "BASH_FUNC_ls%%",
        "IFS",
        "CDPATH",
        "GLOBIGNORE",
        "ZDOTDIR",
        "FPATH",
        "bash_env",
        "Shellopts",
    ];
    if let Some(leaked) = names_that_reached_child(&names) {
        assert_eq!(leaked, Vec::<String>::new());
    }
}
