// SEC-28: data placeholders in program text that a tool takes as a plain
// argument, not behind a code flag: `awk '{{x}}'`, `sed 's/{{x}}//'`,
// `jq '{{x}}'`. awk's system() and GNU sed's `e` command run shell commands,
// so data written into the program is code. The safe channels stay allowed:
// `awk -v name={{x}}`, `jq --arg name {{x}}`.

/// How a tool takes its program: flags whose value is program text, flags
/// that name a program file instead, and options taking one or two values.
struct Spec {
    code_flags: &'static [&'static str],
    file_flags: &'static [&'static str],
    one_value: &'static [&'static str],
    two_values: &'static [&'static str],
    safe_form: &'static str,
}

#[expect(clippy::too_many_lines)]
fn spec(stem: &str) -> Option<Spec> {
    Some(match stem {
        "awk" | "gawk" | "mawk" | "nawk" => Spec {
            code_flags: &["-e", "--source"],
            file_flags: &["-f", "--file"],
            one_value: &[
                "-v",
                "--assign",
                "-F",
                "--field-separator",
                "-i",
                "--include",
            ],
            two_values: &[],
            safe_form: "pass it with -v name=value and read `name` in the program",
        },
        "sed" | "gsed" => Spec {
            code_flags: &["-e", "--expression"],
            file_flags: &["-f", "--file"],
            one_value: &["-l", "--line-length"],
            two_values: &[],
            safe_form: "sed can't take a value safely; use awk -v, or a script reading \"$1\"",
        },
        "jq" | "gojq" | "jaq" => Spec {
            code_flags: &[],
            file_flags: &["-f", "--from-file"],
            one_value: &["--indent", "--tab"],
            two_values: &["--arg", "--argjson", "--slurpfile", "--rawfile"],
            safe_form: "pass it with --arg name value and read $name in the filter",
        },
        _ => return None,
    })
}

/// Problems where `args` put data placeholders into `stem`'s program text.
pub(crate) fn program_text_problems(command: &str, stem: &str, args: &[&str]) -> Vec<String> {
    let Some(spec) = spec(stem) else {
        return Vec::new();
    };
    program_texts(&spec, args)
        .into_iter()
        .filter_map(|text| {
            let found = bnto_core::placeholders::data_placeholders(text);
            let first = found.first()?;
            Some(format!(
                "templates can't go inside {command} program text: '{first}' would run data \
                 as code. {}",
                spec.safe_form
            ))
        })
        .collect()
}

/// Every piece of program text in `args`: code-flag values, then the first
/// positional when no code or file flag already gave the program.
#[expect(clippy::too_many_lines)]
fn program_texts<'a>(spec: &Spec, args: &[&'a str]) -> Vec<&'a str> {
    let mut texts = Vec::new();
    let (mut i, mut given, mut options) = (0, false, true);
    while let Some(arg) = args.get(i).copied() {
        let next = args.get(i + 1).copied();
        if options && arg == "--" {
            options = false;
        } else if options && arg.starts_with('-') && arg.len() > 1 {
            let codes: Vec<_> = spec
                .code_flags
                .iter()
                .filter_map(|flag| crate::interpreter_code::code_after(arg, flag, next))
                .collect();
            given |= !codes.is_empty() || flag_matches(spec.file_flags, arg);
            texts.extend(codes);
            i += skipped_values(spec, arg);
        } else if !given {
            texts.push(arg);
            break;
        } else {
            break;
        }
        i += 1;
    }
    texts
}

/// How many following args `arg` consumes as its values.
fn skipped_values(spec: &Spec, arg: &str) -> usize {
    if spec.two_values.contains(&arg) {
        2
    } else if spec.code_flags.contains(&arg)
        || spec.file_flags.contains(&arg)
        || spec.one_value.contains(&arg)
    {
        1
    } else {
        0
    }
}

/// `-f prog.awk`, `-fprog.awk` or `--file=prog.awk`.
fn flag_matches(flags: &[&str], arg: &str) -> bool {
    flags.iter().any(|flag| {
        arg == *flag
            || arg.strip_prefix(flag).is_some_and(|rest| {
                if flag.starts_with("--") {
                    rest.starts_with('=')
                } else {
                    !rest.is_empty()
                }
            })
    })
}

#[cfg(test)]
mod tests;
