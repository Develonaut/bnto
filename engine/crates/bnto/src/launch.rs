// `bnto tui` — validate the theme, load an optional recipe, run the TUI.

use std::process;
use std::sync::Arc;

use bnto_core::logging::Logger;
use colored::Colorize;

use crate::{storage, telemetry, tui};

/// Launch the interactive TUI and report the session length. When the user
/// pressed U to upgrade, run `bnto upgrade` once the TUI has closed, so
/// cargo's output lands in the terminal.
pub fn launch_tui(
    theme_str: &str,
    recipe_path: Option<String>,
    new: bool,
    logger: &Arc<dyn Logger>,
    paths: Option<storage::BntoPaths>,
) {
    let variant = parse_theme(theme_str);
    let recipe_json = recipe_path.map(|path| read_recipe_file(&path));
    let start = std::time::Instant::now();
    let end = tui::launch_tui(variant, recipe_json, new, logger, paths.clone());
    let end = end.unwrap_or_else(|e| {
        eprintln!("{} {e}", "TUI error:".red());
        process::exit(1);
    });
    let duration_ms = start.elapsed().as_millis() as u64;
    telemetry::capture(telemetry::events::cli_tui_session(duration_ms, theme_str));
    if end == tui::SessionEnd::Upgrade {
        crate::upgrade::run_upgrade(false, paths.as_ref());
    }
}

/// The theme `theme_str` names, exiting on an unknown one.
fn parse_theme(theme_str: &str) -> tui::theme::ThemeVariant {
    tui::theme::ThemeVariant::from_str_lossy(theme_str).unwrap_or_else(|e| {
        eprintln!("{} {e}", "Error:".red());
        process::exit(1);
    })
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
