// `bnto tui` — validate the theme, load an optional recipe, run the TUI.

use std::process;
use std::sync::Arc;

use bnto_core::logging::Logger;
use colored::Colorize;

use crate::{storage, telemetry, tui};

/// Launch the interactive TUI and report the session length.
pub fn launch_tui(
    theme_str: &str,
    recipe_path: Option<String>,
    new: bool,
    logger: &Arc<dyn Logger>,
    paths: Option<storage::BntoPaths>,
) {
    let variant = match tui::theme::ThemeVariant::from_str_lossy(theme_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} {e}", "Error:".red());
            process::exit(1);
        }
    };
    let recipe_json = recipe_path.map(|path| read_recipe_file(&path));
    let start = std::time::Instant::now();
    if let Err(e) = tui::launch_tui(variant, recipe_json, new, logger, paths) {
        eprintln!("{} {e}", "TUI error:".red());
        process::exit(1);
    }
    let duration_ms = start.elapsed().as_millis() as u64;
    telemetry::capture(telemetry::events::cli_tui_session(duration_ms, theme_str));
}

/// Read recipe JSON from disk, exiting on failure.
fn read_recipe_file(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("{} Cannot read recipe file '{}': {e}", "Error:".red(), path);
            process::exit(1);
        }
    }
}
