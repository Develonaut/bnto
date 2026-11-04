// The `$` line a dry run prints, escaped (ISS-125): a script keeps its line
// breaks, so it reads as the script it is, but every other word shows a
// newline as `\n`, so an arg can't start a line of its own.

use super::shell::{ShellCommandInfo, format_words};
use crate::display_text::{display_lines, display_text};

/// Shells a script step runs its script with (`bnto_shell`'s list).
const SCRIPT_SHELLS: &[&str] = &["sh", "bash", "zsh", "dash", "ksh"];

/// `cmd`'s command line, each word escaped.
pub fn shown_command_line(cmd: &ShellCommandInfo) -> String {
    let script = script_index(cmd);
    let args: Vec<String> = cmd
        .args
        .iter()
        .enumerate()
        .map(|(i, arg)| match Some(i) == script {
            true => display_lines(arg).into_owned(),
            false => display_text(arg).into_owned(),
        })
        .collect();
    format_words(&display_text(&cmd.command), &args)
}

/// Which arg is the script: a command can't be a shell (`validate.rs`), so
/// a shell as the command means a script step, `<shell> -c <script>` or a
/// PowerShell `<shell> <script>`.
fn script_index(cmd: &ShellCommandInfo) -> Option<usize> {
    if bnto_shell::is_powershell(&cmd.command) {
        Some(0)
    } else if SCRIPT_SHELLS.contains(&cmd.command.as_str())
        && cmd.args.first().is_some_and(|a| a == "-c")
    {
        Some(1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(command: &str, args: &[&str]) -> ShellCommandInfo {
        ShellCommandInfo {
            env: Vec::new(),
            node_id: "s".into(),
            node_name: "s".into(),
            command: command.into(),
            args: args.iter().map(|a| a.to_string()).collect(),
            output_mode: "stdout".into(),
            unresolved: Vec::new(),
            skipped: None,
        }
    }

    #[test]
    fn a_newline_in_an_arg_shows_escaped() {
        let line = shown_command_line(&info("echo", &["ok\n$ rm -rf ~"]));
        assert!(!line.contains('\n'), "{line}");
        assert!(line.contains("ok\\n$ rm -rf ~"), "{line}");
    }

    #[test]
    fn a_script_keeps_its_line_breaks() {
        let line = shown_command_line(&info("sh", &["-c", "set -eu\necho hi", "bnto", "a\nb"]));
        assert!(line.contains("set -eu\necho hi"), "{line}");
        assert!(line.contains("a\\nb"), "{line}");
    }
}
