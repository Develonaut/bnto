// Dry-run output formatting — print results to stdout with colored output.

use colored::Colorize;

use super::DryRunResult;
use super::shell::format_command_line;
use crate::dry_run_hint::DryRunHint;

/// Print dry-run results to stdout with colored formatting. `given` is what
/// the dry run was given, which the `Run with:` footer repeats.
pub fn print_dry_run(given: &DryRunHint, result: &DryRunResult) {
    println!("\n{}", result.name.bold());
    println!("{}\n", result.description);

    print_dependencies(result);
    print_shell_commands(result);
    print_file_previews(result);
    print_footer(given, result);
}

fn print_dependencies(result: &DryRunResult) {
    if result.dependencies.is_empty() {
        return;
    }
    println!("  {}", "Dependencies:".dimmed());
    for dep in &result.dependencies {
        let version = if dep.version.is_empty() {
            String::new()
        } else {
            format!(" ({})", dep.version)
        };
        println!("    {}{version}", dep.binary.yellow());
    }
    println!();
}

#[expect(clippy::too_many_lines)]
fn print_shell_commands(result: &DryRunResult) {
    if result.shell_commands.is_empty() {
        if result.file_previews.is_empty() {
            println!(
                "  {}",
                "No shell commands — this recipe runs entirely in-process.".dimmed()
            );
            if result.other_node_count > 0 {
                println!(
                    "  {} processing node{}",
                    result.other_node_count,
                    if result.other_node_count == 1 {
                        ""
                    } else {
                        "s"
                    }
                );
            }
        }
        return;
    }

    println!("  {}", "Commands:".dimmed());
    for cmd in &result.shell_commands {
        println!("\n    {} ({})", cmd.node_name.cyan(), cmd.node_id.dimmed());
        println!("    {} {}", "$".dimmed(), format_command_line(cmd).bold());
        println!("    {}  {}", "Output:".dimmed(), cmd.output_mode);

        if !cmd.unresolved.is_empty() {
            println!(
                "    {}  {} (resolved at runtime)",
                "Placeholders:".dimmed(),
                cmd.unresolved.join(", ").yellow()
            );
        }
        if let Some(reason) = &cmd.skipped {
            println!(
                "    {}  {}",
                "Skipped on this host:".dimmed(),
                reason.yellow()
            );
        }
    }
}

#[expect(clippy::too_many_lines)]
fn print_file_previews(result: &DryRunResult) {
    if result.file_previews.is_empty() {
        return;
    }

    let count = result.file_previews.len();
    println!(
        "\n  {} ({count} file{}):",
        "File transformations".dimmed(),
        if count == 1 { "" } else { "s" }
    );

    for preview in &result.file_previews {
        if preview.original == preview.result {
            // No change — just show the filename.
            println!("    {}", preview.result.dimmed());
        } else {
            println!(
                "    {}  {}  {}",
                preview.original,
                "→".dimmed(),
                preview.result.green()
            );
        }
    }
}

fn print_footer(given: &DryRunHint, result: &DryRunResult) {
    if let Some(output_dir) = &result.output_dir {
        println!("\n  {}  {output_dir}", "Output:".dimmed());
    }
    if let Some(ms) = result.duration_ms {
        println!("  {}  {ms}ms", "Preview time:".dimmed());
    }
    if let Some(secs) = result.time_limit {
        println!(
            "
  {}  {}",
            "Time limit:".dimmed(),
            time_limit_line(secs)
        );
    }

    println!(
        "\n{}",
        format!("Run with: {}", given.run_command()).dimmed()
    );
}

/// What the run's time limit does, after the `Time limit:` label.
fn time_limit_line(secs: u64) -> String {
    format!("{secs}s (the run stops and fails past it; bnto run --timeout overrides)")
}

/// Plain text formatter for test assertions (no ANSI colors).
#[cfg(test)]
pub fn format_dry_run_plain(result: &DryRunResult) -> String {
    use std::fmt::Write;

    let mut out = String::new();
    writeln!(out, "  {}", result.name).ok();
    writeln!(out, "  {}", result.description).ok();

    if !result.dependencies.is_empty() {
        writeln!(out).ok();
        writeln!(out, "  Dependencies:").ok();
        for dep in &result.dependencies {
            write!(out, "    {}", dep.binary).ok();
            if !dep.version.is_empty() {
                write!(out, " ({})", dep.version).ok();
            }
            writeln!(out).ok();
        }
    }

    if result.shell_commands.is_empty() && result.file_previews.is_empty() {
        writeln!(out).ok();
        writeln!(out, "  No shell commands").ok();
        if result.other_node_count > 0 {
            writeln!(out, "  {} processing node(s)", result.other_node_count).ok();
        }
    } else {
        if !result.shell_commands.is_empty() {
            writeln!(out).ok();
            writeln!(out, "  Commands:").ok();
            for cmd in &result.shell_commands {
                writeln!(out, "    {} ({})", cmd.node_name, cmd.node_id).ok();
                writeln!(out, "    $ {}", format_command_line(cmd)).ok();
                writeln!(out, "    Output: {}", cmd.output_mode).ok();
                if !cmd.unresolved.is_empty() {
                    writeln!(out, "    Placeholders: {}", cmd.unresolved.join(", ")).ok();
                }
                if let Some(reason) = &cmd.skipped {
                    writeln!(out, "    Skipped on this host: {reason}").ok();
                }
            }
        }
        if !result.file_previews.is_empty() {
            writeln!(out).ok();
            writeln!(
                out,
                "  File transformations ({} file(s)):",
                result.file_previews.len()
            )
            .ok();
            for p in &result.file_previews {
                if p.original == p.result {
                    writeln!(out, "    {}", p.result).ok();
                } else {
                    writeln!(out, "    {} -> {}", p.original, p.result).ok();
                }
            }
        }
    }

    if let Some(secs) = result.time_limit {
        writeln!(out, "  Time limit: {}", time_limit_line(secs)).ok();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::super::files::FilePreview;
    use super::super::shell::ShellCommandInfo;
    use super::*;

    fn make_shell_result() -> DryRunResult {
        DryRunResult {
            name: "Download Video".into(),
            description: "Download a video".into(),
            dependencies: vec![],
            shell_commands: vec![ShellCommandInfo {
                node_id: "dl".into(),
                node_name: "download".into(),
                command: "yt-dlp".into(),
                args: vec!["--format".into(), "mp4".into()],
                output_mode: "file".into(),
                unresolved: vec![],
                skipped: None,
            }],
            file_previews: vec![],
            other_node_count: 0,
            output_dir: None,
            duration_ms: None,
            warnings: vec![],
            time_limit: None,
        }
    }

    fn make_file_result() -> DryRunResult {
        DryRunResult {
            name: "Rename Files".into(),
            description: "Rename files with patterns".into(),
            dependencies: vec![],
            shell_commands: vec![],
            file_previews: vec![
                FilePreview {
                    original: "VIDEO： Movie.mp4".into(),
                    result: "Movie.mp4".into(),
                },
                FilePreview {
                    original: "VIDEO： Concert.mp4".into(),
                    result: "Concert.mp4".into(),
                },
            ],
            other_node_count: 1,
            output_dir: Some("/tmp/output".into()),
            duration_ms: Some(245),
            warnings: vec![],
            time_limit: None,
        }
    }

    #[test]
    fn format_shows_shell_commands() {
        let result = make_shell_result();
        let output = format_dry_run_plain(&result);
        assert!(output.contains("yt-dlp"));
        assert!(output.contains("download"));
        assert!(output.contains("Commands:"));
    }

    #[test]
    fn format_marks_a_command_skipped_on_this_host() {
        let mut result = make_shell_result();
        result.shell_commands[0].skipped =
            Some("when {{ctx.platform}} equals linux: got \"macos\", expected \"linux\"".into());
        let output = format_dry_run_plain(&result);
        assert!(
            output.contains("    Skipped on this host: when {{ctx.platform}} equals linux"),
            "{output}"
        );
    }

    #[test]
    fn format_shows_file_previews() {
        let result = make_file_result();
        let output = format_dry_run_plain(&result);
        assert!(output.contains("VIDEO： Movie.mp4"));
        assert!(output.contains("Movie.mp4"));
        assert!(output.contains("File transformations (2 file(s)):"));
    }

    #[test]
    fn format_shows_no_shell_message_when_empty() {
        let result = DryRunResult {
            name: "Compress Images".into(),
            description: "Compress images".into(),
            dependencies: vec![],
            shell_commands: vec![],
            file_previews: vec![],
            other_node_count: 2,
            output_dir: None,
            duration_ms: None,
            warnings: vec![],
            time_limit: None,
        };
        let output = format_dry_run_plain(&result);
        assert!(output.contains("No shell commands"));
        assert!(output.contains("2 processing node(s)"));
    }

    #[test]
    fn format_shows_the_runs_time_limit() {
        let result = DryRunResult {
            time_limit: Some(90),
            ..make_shell_result()
        };
        let output = format_dry_run_plain(&result);
        assert!(output.contains("Time limit: 90s"), "{output}");
        assert!(!format_dry_run_plain(&make_shell_result()).contains("Time limit"));
    }

    #[test]
    fn format_shows_unchanged_files() {
        let result = DryRunResult {
            name: "Test".into(),
            description: "Test".into(),
            dependencies: vec![],
            shell_commands: vec![],
            file_previews: vec![FilePreview {
                original: "same.txt".into(),
                result: "same.txt".into(),
            }],
            other_node_count: 0,
            output_dir: None,
            duration_ms: None,
            warnings: vec![],
            time_limit: None,
        };
        let output = format_dry_run_plain(&result);
        assert!(output.contains("same.txt"));
        // Unchanged files don't show an arrow.
        assert!(!output.contains("->"));
    }
}
