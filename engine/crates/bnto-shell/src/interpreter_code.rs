// SEC-11: data placeholders inside interpreter code. `python3 -c "...{{x}}..."`
// makes a field value, file name or CSV cell part of the program — the
// Budibase CVE-2026-25044 / Airflow CVE-2020-11978 bug class. The safe form
// stays allowed: literal code reading data from trailing argv.

use serde_json::Value;

use crate::validate::program_stem;

/// The flags whose argument is program text, for interpreter `stem`.
fn code_flags(stem: &str) -> &'static [&'static str] {
    match stem {
        s if is_python(s) => &["-c"],
        "node" | "nodejs" | "bun" => &["-e", "--eval", "-p", "--print"],
        "deno" => &["eval"],
        "perl" => &["-e", "-E"],
        "ruby" | "lua" | "osascript" | "rscript" => &["-e"],
        "php" => &["-r"],
        _ => &[],
    }
}

/// Interpreter options that take the next argument as their value, so it
/// isn't mistaken for the script name that ends option parsing.
fn value_options(stem: &str) -> &'static [&'static str] {
    match stem {
        s if is_python(s) => &["-W", "-X"],
        "node" | "nodejs" | "bun" => &[
            "-r",
            "--require",
            "--import",
            "--loader",
            "--experimental-loader",
            "-C",
            "--conditions",
            "--input-type",
            "--title",
        ],
        "ruby" => &["-I", "-r"],
        "php" => &["-c", "-d", "-z"],
        "osascript" => &["-l", "-s"],
        "lua" => &["-l"],
        _ => &[],
    }
}

/// `python`, `python3`, `python3.12`, `pypy3`, ...
fn is_python(stem: &str) -> bool {
    ["python", "pypy"].iter().any(|p| {
        stem.strip_prefix(p)
            .is_some_and(|v| v.chars().all(|c| c.is_ascii_digit() || c == '.'))
    })
}

/// Problems where `args` put data placeholders into `command`'s code.
pub(crate) fn code_problems(command: &str, args: Option<&Value>) -> Vec<String> {
    code_problems_in(command, &flat_strings(args))
}

/// `code_problems` over args already flattened, e.g. a wrapped program's:
/// interpreter code flags, plus program text tools take as a plain
/// argument (awk, sed, jq).
#[expect(clippy::too_many_lines)]
pub(crate) fn code_problems_in(command: &str, args: &[&str]) -> Vec<String> {
    let stem = program_stem(command);
    let mut problems = crate::program_text::program_text_problems(command, &stem, args);
    let flags = code_flags(&stem);
    if flags.is_empty() {
        return problems;
    }
    problems.extend(
        code_args(args, flags, value_options(&stem))
            .into_iter()
            .filter_map(|(flag, code)| {
                let found = bnto_core::placeholders::data_placeholders(code);
                let first = found.first()?;
                Some(format!(
                    "templates can't go inside {command} code ({flag}): '{first}' would run \
                 data as code. Pass the value as a trailing argument and read it from \
                 sys.argv / process.argv / ARGV"
                ))
            })
            .collect::<Vec<_>>(),
    );
    problems
}

/// Each code argument in `args` with the flag that introduced it, in any
/// spelling: `-c CODE`, clustered `-ne CODE`, attached `-cCODE`, `--eval=CODE`.
/// Only the interpreter's own options are read: the first positional (a
/// script name) or `--` ends them, as it does for the interpreter, so a
/// script's own `-c config.toml` isn't mistaken for code.
fn code_args<'a>(
    args: &'a [&'a str],
    flags: &[&str],
    value_opts: &[&str],
) -> Vec<(String, &'a str)> {
    let mut found = Vec::new();
    let mut i = 0;
    while let Some(arg) = args.get(i).copied() {
        let next = args.get(i + 1).copied();
        let codes: Vec<_> = flags
            .iter()
            .filter_map(|flag| code_after(arg, flag, next).map(|code| (flag.to_string(), code)))
            .collect();
        let takes_next = flags.contains(&arg) || value_opts.contains(&arg);
        if codes.is_empty() && !takes_next && (!arg.starts_with('-') || arg == "--") {
            break;
        }
        found.extend(codes);
        i += if takes_next { 2 } else { 1 };
    }
    found
}

/// The code `arg` hands to `flag`, if it is that flag.
pub(crate) fn code_after<'a>(arg: &'a str, flag: &str, next: Option<&'a str>) -> Option<&'a str> {
    if arg == flag {
        return next;
    }
    if let Some(code) = flag
        .starts_with("--")
        .then(|| arg.strip_prefix(flag)?.strip_prefix('='))
        .flatten()
    {
        return Some(code);
    }
    let letter = flag.strip_prefix('-').filter(|l| l.len() == 1)?;
    let cluster = arg.strip_prefix('-').filter(|c| !c.starts_with('-'))?;
    let at = cluster.find(letter)?;
    let (before, rest) = (&cluster[..at], &cluster[at + 1..]);
    if !before.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    if rest.is_empty() { next } else { Some(rest) }
}

/// Every string in `args`, in order, through conditional and `each` groups.
pub(crate) fn flat_strings(args: Option<&Value>) -> Vec<&str> {
    fn walk<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
        match value {
            Value::String(s) => out.push(s),
            Value::Array(items) => items.iter().for_each(|v| walk(v, out)),
            Value::Object(map) => map.values().for_each(|v| walk(v, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    if let Some(args) = args {
        walk(args, &mut out);
    }
    out
}

#[cfg(test)]
mod tests;
