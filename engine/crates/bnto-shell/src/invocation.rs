// What one shell-run node runs, gathered once and handed to its output
// mode (stdout or file).

use bnto_core::{BntoError, CommandOptions, ProgressReporter};

use crate::clixml;

/// The command a recipe named, the program that actually runs (its full
/// path from `searchPath`, or the command itself), its arguments, and the
/// env and time limit it runs with.
pub(crate) struct Invocation<'a> {
    pub command: &'a str,
    pub program: &'a str,
    pub args: &'a [String],
    pub options: CommandOptions,
    /// A PowerShell script, whose stderr may arrive as CLIXML.
    pub clixml: bool,
}

impl Invocation<'_> {
    /// Relay one line the command printed, as text.
    pub(crate) fn relay(&self, line: &str, progress: &ProgressReporter) {
        if !self.clixml {
            return progress.report_output(line);
        }
        for text in clixml::output_lines(line) {
            progress.report_output(&text);
        }
    }

    /// A failure whose message quotes stderr, with any CLIXML as text.
    pub(crate) fn readable(&self, error: BntoError) -> BntoError {
        match error {
            BntoError::CommandFailed { message, stdout } if self.clixml => {
                let message = clixml::decode(&message);
                BntoError::CommandFailed { message, stdout }
            }
            other => other,
        }
    }
}
