// bnto-video — Video operation nodes for the Bnto engine.
//
// CLI/desktop only. Wraps external tools (yt-dlp) via the adapter pattern.
// Browser (WASM) has no shell access — these processors are never registered
// in the default (browser-safe) registry.

pub mod adapters;
pub mod download;

pub use download::VideoDownload;
