// bnto-shell — Shell command execution for native targets.
//
// Provides the `shell-command` processor: a generic NodeProcessor that
// executes external CLI tools via ProcessContext::run_command(). This
// enables connector-as-recipe architecture — recipes can wrap any CLI
// tool (ffmpeg, yt-dlp, imagemagick) without needing a dedicated Rust
// processor for each one.
//
// Native-only: registered behind `#[cfg(feature = "native")]` in
// bnto-engine. Browser (WASM) gets NoopContext which blocks execution.

pub mod execute;
pub mod validate;

pub use execute::ShellCommand;
