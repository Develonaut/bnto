use serde_json::json;

use super::*;

fn problems(command: &str, args: serde_json::Value) -> Vec<String> {
    code_problems(command, Some(&args))
}

#[test]
fn data_in_code_is_refused_for_each_interpreter() {
    for (command, flag) in [
        ("python3", "-c"),
        ("python", "-c"),
        ("python3.12", "-c"),
        ("Python.exe", "-c"),
        ("node", "-e"),
        ("node", "--eval"),
        ("node", "-p"),
        ("bun", "-e"),
        ("perl", "-e"),
        ("perl", "-E"),
        ("ruby", "-e"),
        ("php", "-r"),
        ("osascript", "-e"),
        ("Rscript", "-e"),
        ("lua", "-e"),
        ("deno", "eval"),
    ] {
        let found = problems(command, json!([flag, "print('{{fields.v}}')"]));
        assert_eq!(found.len(), 1, "{command} {flag}");
        assert!(found[0].contains("template"), "{}", found[0]);
        assert!(found[0].contains("{{fields.v}}"), "{}", found[0]);
    }
}

#[test]
fn clustered_and_attached_flags_are_caught() {
    assert_eq!(
        problems("perl", json!(["-ne", "print '{{item.x}}'"])).len(),
        1
    );
    assert_eq!(
        problems("perl", json!(["-lne", "print '{{item.x}}'"])).len(),
        1
    );
    assert_eq!(problems("node", json!(["-pe", "'{{url}}'"])).len(), 1);
    assert_eq!(problems("python3", json!(["-uc", "x='{{path}}'"])).len(), 1);
    assert_eq!(problems("python3", json!(["-cx='{{env.K}}'"])).len(), 1);
    assert_eq!(problems("node", json!(["--eval={{node.a.b}}"])).len(), 1);
}

#[test]
fn options_that_take_a_value_do_not_end_option_parsing() {
    let jxa = json!(["-l", "JavaScript", "-e", "ObjC.import('{{fields.v}}')"]);
    assert_eq!(problems("osascript", jxa).len(), 1);
    assert_eq!(
        problems("python3", json!(["-W", "ignore", "-c", "'{{item.k}}'"])).len(),
        1
    );
    assert_eq!(
        problems("node", json!(["-r", "dotenv/config", "-e", "'{{url}}'"])).len(),
        1
    );
}

#[test]
fn repeated_code_flags_are_each_checked() {
    let args = json!(["-e", "set x to 1", "-e", "display dialog \"{{fields.v}}\""]);
    assert_eq!(problems("osascript", args).len(), 1);
}

#[test]
fn code_inside_groups_is_checked() {
    let args = json!([["-c", "print('{{fields.v}}')"], { "each": ["{{path}}"] }]);
    assert_eq!(problems("python3", args).len(), 1);
}

#[test]
fn literal_code_with_data_as_trailing_args_is_allowed() {
    let args = json!([
        "-c",
        "import sys; print(sys.argv[1])",
        "{{fields.v}}",
        "{{item.x}}"
    ]);
    assert!(problems("python3", args).is_empty());
    let args = json!(["-e", "console.log(process.argv[1])", "{{path}}"]);
    assert!(problems("node", args).is_empty());
}

#[test]
fn text_that_only_looks_like_a_template_is_allowed() {
    // A literal mention of the syntax in a program, as in download-patreon.
    let args = json!(["-c", "print('fill {{fields.*}} later')"]);
    assert!(problems("python3", args).is_empty());
}

#[test]
fn run_vars_in_code_are_allowed() {
    assert!(problems("python3", json!(["-c", "import {{vars.Mod}}"])).is_empty());
}

#[test]
fn non_interpreters_and_other_flags_are_ignored() {
    assert!(problems("ffmpeg", json!(["-c", "{{fields.codec}}"])).is_empty());
    // After the script name, flags are the script's own (`-c config.toml`).
    assert!(problems("python3", json!(["tool.py", "-c", "{{fields.config}}"])).is_empty());
    assert!(problems("python3", json!(["-m", "mod", "-c", "{{fields.v}}"])).is_empty());
    assert!(problems("node", json!(["--", "-e", "{{fields.v}}"])).is_empty());
    assert!(problems("perl", json!(["--help", "{{fields.v}}"])).is_empty());
    assert!(problems("python3", json!(["-m", "http.server", "{{fields.port}}"])).is_empty());
}

#[test]
fn python_names_are_recognised_without_overmatching() {
    assert!(is_python("python3.12"));
    assert!(is_python("pypy3"));
    assert!(!is_python("python-config"));
    assert!(!is_python("pythonista"));
}
