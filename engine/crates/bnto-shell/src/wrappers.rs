// SEC-08: exec wrappers (`env`, `xargs`, `timeout`, ...) run a program named
// in their own args, so `env sh -c ...` reached a shell around the denylist.
// Find what each wrapper runs and hold it to the rules `command` follows: no
// shell, no program chosen by data. `timeout 5 ffmpeg` stays allowed. The
// consent hash is the real boundary for authors; this keeps the denylist
// from being routed around.

use serde_json::Value;

use crate::interpreter_code::flat_strings;
use crate::validate::{is_shell, program_stem};

/// How a wrapper's args reach its program: options that take a value, and
/// positionals before the program (`timeout DURATION prog`).
struct Spec {
    value_opts: &'static [&'static str],
    positionals: usize,
}

#[expect(clippy::too_many_lines)]
fn spec(stem: &str) -> Option<Spec> {
    let (value_opts, positionals): (&'static [&'static str], usize) = match stem {
        "env" => (&["-u", "--unset", "-C", "--chdir"], 0),
        "xargs" => (
            &["-I", "-n", "-L", "-P", "-s", "-d", "-E", "-a", "--arg-file"],
            0,
        ),
        "nohup" | "setsid" | "unbuffer" | "busybox" => (&[], 0),
        "nice" => (&["-n", "--adjustment"], 0),
        "ionice" => (&["-c", "-n", "--class", "--classdata"], 0),
        "timeout" => (&["-s", "--signal", "-k", "--kill-after"], 1),
        "stdbuf" => (&["-i", "-o", "-e", "--input", "--output", "--error"], 0),
        "sudo" | "doas" => (&["-u", "-g", "-C", "-p", "-h", "-U", "-r", "-t", "-D"], 0),
        "chroot" => (&["--userspec", "--groups"], 1),
        "time" => (&["-o", "-f", "--output", "--format"], 0),
        "caffeinate" => (&["-t", "-w"], 0),
        "taskset" | "chrt" => (&[], 1),
        "watch" => (&["-n", "--interval"], 0),
        _ => return None,
    };
    Some(Spec {
        value_opts,
        positionals,
    })
}

/// What a wrapper does with its args.
enum Runs<'a> {
    /// The program at `args[index]`, with its own args after it.
    Program(usize, &'a str),
    /// The wrapper starts a shell itself; the reason why.
    Shell(&'static str),
}

/// Problems where `command` wraps a shell, or a program data picks.
pub(crate) fn wrapper_problems(command: &str, args: Option<&Value>) -> Vec<String> {
    problems_for(command, &flat_strings(args), 0)
}

#[expect(clippy::too_many_lines)]
fn problems_for(command: &str, args: &[&str], depth: usize) -> Vec<String> {
    let mut problems = Vec::new();
    for runs in what_runs(&program_stem(command), args) {
        match runs {
            Runs::Shell(why) => problems.push(format!(
                "'{command}' {why}, which shell-run doesn't allow. Run the tool directly"
            )),
            Runs::Program(_, program) if bnto_core::placeholders::mentions_data(program) => {
                problems.push(format!(
                    "'{command}' would run '{program}', which lets data choose the \
                     program. Name the tool literally"
                ));
            }
            Runs::Program(_, program) if is_shell(program) => problems.push(format!(
                "'{command}' would run the shell '{program}', which shell-run \
                 doesn't allow. Run the tool directly"
            )),
            // The wrapped program gets the same checks: interpreter code
            // (`env python3 -c ...`) and wrappers inside wrappers.
            Runs::Program(at, program) => {
                let rest = &args[at + 1..];
                problems.extend(crate::interpreter_code::code_problems_in(program, rest));
                if depth < 4 {
                    problems.extend(problems_for(program, rest, depth + 1));
                }
            }
        }
    }
    problems
}

fn what_runs<'a>(stem: &str, args: &[&'a str]) -> Vec<Runs<'a>> {
    match stem {
        "script" => vec![Runs::Shell("runs its command through a shell")],
        "watch" if !args.iter().any(|a| matches!(*a, "-x" | "--exec")) => {
            vec![Runs::Shell(
                "runs its command through sh -c unless given -x",
            )]
        }
        "find" => find_exec_programs(args),
        _ => spec(stem)
            .and_then(|s| program_after_options(stem, &s, args))
            .into_iter()
            .collect(),
    }
}

/// `find ... -exec prog {} ;` runs each prog after `-exec`-style actions.
fn find_exec_programs<'a>(args: &[&'a str]) -> Vec<Runs<'a>> {
    args.iter()
        .enumerate()
        .filter(|(_, a)| matches!(**a, "-exec" | "-execdir" | "-ok" | "-okdir"))
        .filter_map(|(i, _)| args.get(i + 1).map(|p| Runs::Program(i + 1, p)))
        .collect()
}

/// The program a wrapper runs: past its options (and their values), any
/// leading positionals, and `env`'s `NAME=value` assignments.
fn program_after_options<'a>(stem: &str, spec: &Spec, args: &[&'a str]) -> Option<Runs<'a>> {
    let (mut i, mut skipped, mut options) = (0, 0, true);
    while let Some(arg) = args.get(i).copied() {
        if options && arg == "--" {
            options = false;
        } else if options && arg.starts_with('-') && arg.len() > 1 {
            if let Some(runs) = shell_option(stem, arg, args.get(i + 1).copied()) {
                return Some(runs);
            }
            i += usize::from(spec.value_opts.contains(&arg));
        } else if stem == "env" && arg.contains('=') {
            // `env NAME=value prog`
        } else if skipped < spec.positionals {
            skipped += 1;
        } else {
            return Some(Runs::Program(i, arg));
        }
        i += 1;
    }
    (stem == "chroot").then_some(Runs::Shell("runs a shell when given no program"))
}

/// Options that make the wrapper start a shell (or a command line) itself.
fn shell_option<'a>(stem: &str, arg: &str, next: Option<&'a str>) -> Option<Runs<'a>> {
    match (stem, arg) {
        ("sudo" | "doas", "-s" | "-i" | "--shell" | "--login") => {
            Some(Runs::Shell("starts a shell with -s / -i"))
        }
        // `env -S "sh -c ..."` splits the string into a command line.
        ("env", "-S" | "--split-string") => {
            let first = next?.split_whitespace().next()?;
            is_shell(first).then_some(Runs::Shell(
                "splits -S into a command line that runs a shell",
            ))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests;
