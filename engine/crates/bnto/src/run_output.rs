// `bnto run` results — write output files and print the run summary.

use bnto_core::RunSummary;
use colored::Colorize;

use crate::io;
use crate::run_verdict::RunVerdict;

/// Write pipeline results using mode-aware output, print any warnings, then
/// the summary: the `Done.` line last on stderr and, with `--json`, on stdout.
pub fn write_output(
    result: &bnto_core::PipelineResult,
    mode: &str,
    output_dir: &str,
    dry_run: bool,
    verdict: &RunVerdict,
) {
    let outcome = io::write_results_with_mode(result, mode, output_dir, dry_run);
    let outcome = verdict.or_fail(outcome, "Error writing output:");
    print_warnings(result, dry_run);
    let summary = RunSummary {
        files_written: written_count(&outcome),
        ..verdict.passed(result)
    };
    print_done(&outcome, &summary, dry_run);
    verdict.print_json(&summary);
}

/// Warnings and totals, printed before the Done line so it stays last.
fn print_warnings(result: &bnto_core::PipelineResult, dry_run: bool) {
    for warning in &result.warnings {
        eprintln!("  {} {warning}", "Warning:".yellow().bold());
    }
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
fn print_done(outcome: &io::WriteOutcome, summary: &RunSummary, dry_run: bool) {
    let duration = format_duration(summary.duration_ms);
    let count = summary.files_written;
    let (written, renamed, tag) = done_wording(dry_run);
    let done = "Done.".green().bold();
    match outcome {
        io::WriteOutcome::Written { dir, .. } => eprintln!(
            "\n{done} {count} file{} {written} to {dir}/ in {duration}{tag}",
            plural_s(count),
        ),
        io::WriteOutcome::Overwritten { .. } => eprintln!(
            "\n{done} {count} file{} {renamed} in {duration}{tag}",
            plural_s(count),
        ),
        io::WriteOutcome::Message { summary: text } => eprintln!("\n{done} {text}{tag}"),
        io::WriteOutcome::None => eprintln!("\n{done} in {duration}{tag}"),
    }
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
