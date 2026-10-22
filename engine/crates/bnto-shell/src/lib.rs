// bnto-shell — Shell command execution for native targets.
//
// Provides the `shell-run` processor: a generic NodeProcessor that
// executes external CLI tools via ProcessContext::run_command(). This
// enables connector-as-recipe architecture — recipes can wrap any CLI
// tool (ffmpeg, yt-dlp, imagemagick) without needing a dedicated Rust
// processor for each one.
//
// Native-only: registered behind `#[cfg(feature = "native")]` in
// bnto-engine. Browser (WASM) gets NoopContext which blocks execution.

mod args;
mod command_options;
mod cwd;
mod each;
#[cfg(test)]
mod failing_tool;
mod file_mode;
mod interpreter_code;
mod invocation;
mod output_files;
mod parameters;
mod powershell;
mod program_text;
mod raw_checks;
pub mod run;
mod script;
pub mod search_path;
mod stdout_mode;
pub mod validate;
mod wrappers;

pub use powershell::is_powershell;
pub use run::ShellRun;

/// The node type the shell processor registers under. Trust, consent and
/// dry-run match nodes against this, so a recipe can't run a command
/// without the consent check seeing it.
pub const NODE_TYPE: &str = "shell-run";
