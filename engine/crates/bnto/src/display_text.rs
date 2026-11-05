// Recipe text made safe to print (ISS-125). The escape lives in `bnto-core`
// so nodes can escape recipe text where they build an error; the CLI and the
// TUI reach it through this path.

pub use bnto_core::display_text::{display_lines, display_path, display_text};
