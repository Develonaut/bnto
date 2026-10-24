// `bnto run` results — write output files and print the run summary.

use bnto_core::RunSummary;
use colored::Colorize;

use crate::io;
use crate::run_message::{RunMessage, run_message};
use crate::run_verdict::RunVerdict;

/// Write pipeline results using mode-aware output, print any warnings, then
/// the summary: the `Done.` line last on stderr and, with `--json`, on stdout.
/// `label` is the output node's, shown over a message-mode run's text.
pub fn write_output(
    result: &bnto_core::PipelineResult,
    mode: &str,
    output_dir: &str,
    dry_run: bool,
    label: Option<String>,
    verdict: &RunVerdict,
) {
    let outcome = io::write_results_with_mode(result, mode, output_dir, dry_run);
    let outcome = verdict.or_fail(outcome, "Error writing output:");
    print_warnings(result, &outcome, dry_run);
    let message = reported_message(result, &outcome, label);
    let summary = RunSummary {
        files_written: written_count(&outcome),
        ..verdict.passed(result)
    }
    .with_message(message.as_ref().map(|m| m.text.clone()));
    print_done(&outcome, message.as_ref(), &summary, dry_run);
    verdict.print_json(&summary);
}

/// The text a message-mode run reported, when its output reads as text.
fn reported_message(
    result: &bnto_core::PipelineResult,
    outcome: &io::WriteOutcome,
    label: Option<String>,
) -> Option<RunMessage> {
    match outcome {
        io::WriteOutcome::Message { .. } => run_message(result, label),
        _ => None,
    }
}

/// Warnings and totals, printed before the Done line so it stays last.
fn print_warnings(result: &bnto_core::PipelineResult, outcome: &io::WriteOutcome, dry_run: bool) {
    for warning in &result.warnings {
        eprintln!("  {} {warning}", "Warning:".yellow().bold());
    }
    io::warn_renamed(outcome, dry_run);
    io::warn_files_left_as_newer(result);
    io::warn_files_without_a_row(result);
    io::print_copy_totals(result, dry_run);
}

/// Files the output step wrote or renamed in place (on a dry run, would have).
fn written_count(outcome: &io::WriteOutcome) -> usize {
    match outcome {
        io::WriteOutcome::Written { count, .. } | io::WriteOutcome::Overwritten { count } => *count,
        io::WriteOutcome::Message { .. } | io::WriteOutcome::None => 0,
    }
}

/// Print the final `Done.` line for a write outcome, from the run's summary.
/// A reported message prints above it, so `Done.` stays the last line.
fn print_done(
    outcome: &io::WriteOutcome,
    message: Option<&RunMessage>,
    summary: &RunSummary,
    dry_run: bool,
) {
    let duration = format_duration(summary.duration_ms);
    let done = "Done.".green().bold();
    if let Some(message) = message {
        let tag = done_wording(dry_run).2;
        eprintln!("\n{}\n\n{done} in {duration}{tag}", message_block(message));
        return;
    }
    let text = done_text(outcome, summary.files_written, &duration, dry_run);
    eprintln!("\n{done} {text}");
}

/// The `Done.` line's text after `Done.` for a write outcome; `count` is the
/// files written or renamed.
fn done_text(outcome: &io::WriteOutcome, count: usize, duration: &str, dry_run: bool) -> String {
    let (written, renamed, tag) = done_wording(dry_run);
    match outcome {
        io::WriteOutcome::Written { dir, unchanged, .. } => format!(
            "{count} file{} {written} to {dir}/{} in {duration}{tag}",
            plural_s(count),
            left_as_is_note(*unchanged, dry_run),
        ),
        io::WriteOutcome::Overwritten { .. } => {
            format!(
                "{count} file{} {renamed} in {duration}{tag}",
                plural_s(count)
            )
        }
        io::WriteOutcome::Message { summary: text } => format!("{text}{tag}"),
        io::WriteOutcome::None => format!("in {duration}{tag}"),
    }
}

/// A message under its label, every line indented:
/// `Session Log:\n  No active session.`
fn message_block(message: &RunMessage) -> String {
    let header = message.label.as_ref().map(|label| format!("{label}:"));
    let lines = message.text.lines().map(|line| format!("  {line}"));
    header
        .into_iter()
        .chain(lines)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Verbs and suffix for the summary — conditional for dry runs.
fn done_wording(dry_run: bool) -> (&'static str, &'static str, &'static str) {
    if dry_run {
        (
            "would be written",
            "would be renamed in place",
            " (dry run)",
        )
    } else {
        ("written", "renamed in place", "")
    }
}

/// ` (N already there, left as is)` for files a write found at their
/// destination already (the source itself); empty when there were none.
fn left_as_is_note(unchanged: usize, dry_run: bool) -> String {
    if unchanged == 0 {
        return String::new();
    }
    let verb = if dry_run { "would be left" } else { "left" };
    format!(" ({unchanged} already there, {verb} as is)")
}

/// "" for one file, "s" otherwise.
fn plural_s(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

/// Format milliseconds into a human-readable duration string.
fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        format!("{ms}ms")
    } else {
        format!("{:.1}s", ms as f64 / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A message as `run_message` builds it.
    fn message(label: Option<&str>, text: &str) -> RunMessage {
        RunMessage {
            label: label.map(str::to_string),
            text: text.into(),
        }
    }

    #[test]
    fn test_message_block_puts_the_text_under_its_label() {
        let block = message_block(&message(Some("Session Log"), "No active session."));
        assert_eq!(block, "Session Log:\n  No active session.");
    }

    #[test]
    fn test_message_block_indents_every_line() {
        let text = "Session active.\nActivity simulation: off";
        let block = message_block(&message(Some("Session Log"), text));
        assert_eq!(
            block,
            "Session Log:\n  Session active.\n  Activity simulation: off"
        );
    }

    #[test]
    fn test_message_block_without_a_label_is_the_indented_text() {
        assert_eq!(message_block(&message(None, "a\nb")), "  a\n  b");
    }

    #[test]
    fn test_left_as_is_note_only_when_a_file_was_already_there() {
        assert_eq!(left_as_is_note(0, false), "");
        assert_eq!(left_as_is_note(1, false), " (1 already there, left as is)");
        assert_eq!(
            left_as_is_note(2, true),
            " (2 already there, would be left as is)"
        );
    }

    fn written(unchanged: usize) -> io::WriteOutcome {
        io::WriteOutcome::Written {
            dir: "F".into(),
            count: 0,
            renamed: vec![],
            unchanged,
        }
    }

    #[test]
    fn test_done_text_says_a_file_already_there_was_left_as_is() {
        assert_eq!(
            done_text(&written(1), 0, "1ms", false),
            "0 files written to F/ (1 already there, left as is) in 1ms"
        );
        assert_eq!(
            done_text(&written(1), 0, "1ms", true),
            "0 files would be written to F/ (1 already there, would be left as is) in 1ms (dry run)"
        );
        assert_eq!(
            done_text(&written(0), 1, "1ms", false),
            "1 file written to F/ in 1ms"
        );
    }

    #[test]
    fn test_format_duration_millis() {
        assert_eq!(format_duration(42), "42ms");
        assert_eq!(format_duration(999), "999ms");
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(1000), "1.0s");
        assert_eq!(format_duration(1500), "1.5s");
        assert_eq!(format_duration(12345), "12.3s");
    }
}
