// SEC-11..12: templates inside interpreter code (`python3 -c`, `node -e`, ...).
// The denylist stops `sh -c "{{x}}"` but not `python3 -c "...{{x}}..."`,
// which is the Budibase CVE-2026-25044 / Airflow CVE-2020-11978 bug class:
// data interpolated into source text that a parser then runs.

use serde_json::json;

use crate::support::{Lab, step};

/// (command, code flag) pairs whose next argument is program text.
const INTERPRETERS: &[(&str, &str)] = &[
    ("python3", "-c"),
    ("python", "-c"),
    ("node", "-e"),
    ("node", "--eval"),
    ("perl", "-e"),
    ("ruby", "-e"),
    ("osascript", "-e"),
];

/// SEC-11: a template in the code argument is refused by `bnto check` and
/// by `bnto run`, before anything is spawned.
#[test]
fn sec11_templates_in_interpreter_code_are_refused() {
    for (command, flag) in INTERPRETERS {
        let lab = Lab::new();
        let code = "print('{{fields.v}}')";
        let recipe = lab.recipe(step(
            json!({ "command": command, "args": [flag, code] }),
            &["v"],
        ));
        let check = lab.check(&recipe);
        assert!(!check.ok, "check accepted {command} {flag}: {}", check.text);
        let payload = format!("'); open('{}', 'w'); print('", lab.canary().display());
        let run = lab.run(&recipe, &[&format!("v={payload}")]);
        assert!(!run.ok, "run accepted {command} {flag}: {}", run.text);
        assert!(run.text.contains("template"), "{command}: {}", run.text);
        assert!(!lab.canary_fired(), "{command} ran injected code");
    }
}

/// SEC-12: the safe form — literal code reading data from trailing argv
/// (`sys.argv[1]`) — stays allowed, so SEC-11's fix can't over-block.
#[test]
fn sec12_data_as_trailing_interpreter_args_is_allowed() {
    for (command, flag) in INTERPRETERS {
        let lab = Lab::new();
        let params = json!({
            "command": command,
            "args": [flag, "import sys; print(sys.argv[1])", "{{fields.v}}"]
        });
        let check = lab.check(&lab.recipe(step(params, &["v"])));
        assert!(check.ok, "{command} {flag}: {}", check.text);
    }
}
