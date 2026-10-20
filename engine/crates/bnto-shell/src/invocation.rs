// What one shell-run node runs, gathered once and handed to its output
// mode (stdout or file).

use bnto_core::CommandOptions;

/// The command a recipe named, the program that actually runs (its full
/// path from `search_path`, or the command itself), its arguments, and the
/// env and time limit it runs with.
pub(crate) struct Invocation<'a> {
    pub command: &'a str,
    pub program: &'a str,
    pub args: &'a [String],
    pub options: CommandOptions,
}
