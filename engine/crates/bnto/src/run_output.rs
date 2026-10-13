// `bnto run` results — write output files and print the run summary.

use std::process;

use colored::Colorize;

use crate::io;

/// Write pipeline results using mode-aware output, print summary and any warnings.
pub fn write_output(
    result: &bnto_core::PipelineResult,
    mode: &str,
    output_dir: &str,
    dry_run: bool,
) {
    let outcome = match io::write_results_with_mode(result, mode, output_dir, dry_run) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{} {e}", "Error writing output:".red());
            process::exit(1);
        }
    };

    // Warnings and totals print first so the Done line is always last.
    for warning in &result.warnings {
        eprintln!("  {} {warning}", "Warning:".yellow().bold());
    }
    io::warn_files_left_as_newer(result);
    io::print_copy_totals(result, dry_run);
    print_done(outcome, result.duration_ms, dry_run);
}

/// Print the final `Done.` line for a write outcome.
fn print_done(outcome: io::WriteOutcome, duration_ms: u64, dry_run: bool) {
    let duration = format_duration(duration_ms);
    let (written, renamed, tag) = done_wording(dry_run);
    let done = "Done.".green().bold();
    match outcome {
        io::WriteOutcome::Written { dir, count } => eprintln!(
            "\n{done} {count} file{} {written} to {dir}/ in {duration}{tag}",
            plural_s(count),
        ),
        io::WriteOutcome::Overwritten { count } => eprintln!(
            "\n{done} {count} file{} {renamed} in {duration}{tag}",
            plural_s(count),
        ),
        io::WriteOutcome::Message { summary } => eprintln!("\n{done} {summary}{tag}"),
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
