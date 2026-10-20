use serde_json::json;

use super::*;

fn problems(command: &str, args: serde_json::Value) -> Vec<String> {
    wrapper_problems(command, Some(&args))
}

#[test]
fn wrapped_shells_are_refused() {
    for (command, args) in [
        ("env", json!(["sh", "-c", "true"])),
        ("env", json!(["-i", "FOO=1", "--", "bash", "-c", "true"])),
        ("xargs", json!(["-I", "{}", "sh", "-c", "echo {}"])),
        ("nohup", json!(["bash", "-c", "true"])),
        ("nice", json!(["-n", "10", "sh", "-c", "true"])),
        ("timeout", json!(["5", "sh", "-c", "true"])),
        ("timeout", json!(["-s", "KILL", "5", "zsh", "-c", "true"])),
        ("stdbuf", json!(["-oL", "dash", "-c", "true"])),
        ("setsid", json!(["ksh", "-c", "true"])),
        ("sudo", json!(["-u", "root", "sh"])),
        ("busybox", json!(["sh", "-c", "true"])),
        ("caffeinate", json!(["-i", "bash", "-c", "true"])),
        (
            "find",
            json!([".", "-name", "x", "-exec", "sh", "-c", "true", ";"]),
        ),
        ("env", json!(["powershell.exe", "-Command", "x"])),
    ] {
        let found = problems(command, args.clone());
        assert_eq!(found.len(), 1, "{command} {args}");
        assert!(found[0].contains("shell"), "{}", found[0]);
    }
}

#[test]
fn wrappers_that_start_a_shell_themselves_are_refused() {
    for (command, args) in [
        ("script", json!(["-c", "ls", "/dev/null"])),
        ("watch", json!(["ls"])),
        ("sudo", json!(["-s"])),
        ("sudo", json!(["-i"])),
        ("env", json!(["-S", "sh -c true"])),
        ("chroot", json!(["/srv/root"])),
    ] {
        assert_eq!(problems(command, args.clone()).len(), 1, "{command} {args}");
    }
}

#[test]
fn a_wrapped_program_chosen_by_data_is_refused() {
    for (command, args) in [
        ("env", json!(["{{fields.tool}}", "--version"])),
        ("timeout", json!(["5", "{{item.bin}}"])),
        ("find", json!([".", "-exec", "{{fields.p}}", "{}", ";"])),
    ] {
        let found = problems(command, args.clone());
        assert_eq!(found.len(), 1, "{command} {args}");
        assert!(found[0].contains("data"), "{}", found[0]);
    }
}

#[test]
fn data_in_wrapped_interpreter_code_is_refused() {
    // SEC-11 must not be routed around through a wrapper.
    let found = problems("env", json!(["python3", "-c", "print('{{fields.v}}')"]));
    assert_eq!(found.len(), 1);
    assert!(found[0].contains("template"), "{}", found[0]);
    assert_eq!(
        problems("timeout", json!(["5", "node", "-e", "'{{url}}'"])).len(),
        1
    );
    let safe = json!([
        "5",
        "python3",
        "-c",
        "import sys; print(sys.argv[1])",
        "{{path}}"
    ]);
    assert!(problems("timeout", safe).is_empty());
}

#[test]
fn nested_wrappers_are_unwrapped() {
    assert_eq!(
        problems("nice", json!(["-n", "5", "nohup", "sh", "-c", "x"])).len(),
        1
    );
    assert_eq!(
        problems("env", json!(["timeout", "5", "env", "bash"])).len(),
        1
    );
}

#[test]
fn wrappers_running_ordinary_tools_are_allowed() {
    for (command, args) in [
        ("timeout", json!(["5", "ffmpeg", "-i", "{{path}}"])),
        ("timeout", json!(["-k", "10", "60", "yt-dlp", "{{url}}"])),
        (
            "env",
            json!(["FOO=1", "python3", "tool.py", "{{fields.v}}"]),
        ),
        ("nice", json!(["-n", "10", "ffmpeg"])),
        ("xargs", json!(["-0", "-n", "1", "rm"])),
        (
            "find",
            json!([".", "-name", "*.tmp", "-exec", "rm", "{}", ";"]),
        ),
        ("watch", json!(["-x", "-n", "2", "ls", "-l"])),
        ("caffeinate", json!(["-i", "-t", "60"])),
        ("env", json!(["{{vars.Tool}}", "--version"])),
        ("ffmpeg", json!(["-i", "x"])),
        ("chroot", json!(["/srv/root", "ls"])),
    ] {
        assert!(
            problems(command, args.clone()).is_empty(),
            "{command} {args}"
        );
    }
}
