// bnto CLI — run .bnto.json recipes from the command line.
//
// Usage: bnto run <recipe> <file1> [file2 ...]
//        bnto run <recipe> <url>  (for url-mode recipes)
//        bnto run <recipe> <file1> --param quality=50
//        bnto list
//        bnto info <recipe>
//        bnto doctor
//        bnto tui [--theme <variant>]  (interactive TUI, beta)

mod context;
mod doctor;
mod info;
mod input;
mod io;
mod list;
pub mod logging;
mod progress;
pub mod telemetry;
mod tui;

use std::process;
use std::sync::Arc;

use bnto_core::logging::{LogEntry, LogLevel, Logger, NoopLogger};
use clap::{Parser, Subcommand};
use colored::Colorize;

use logging::FileLogger;

/// bnto — run recipes from the command line.
#[derive(Parser)]
#[command(name = "bnto", version, about = "Run .bnto.json recipes")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Execute a recipe against input files or a URL.
    Run {
        /// Recipe slug (e.g. compress-images) or path to a .bnto.json file.
        recipe: String,

        /// Input files, URL, or text (depends on recipe input mode).
        inputs: Vec<String>,

        /// Output directory (default: current directory).
        #[arg(short, long, default_value = ".")]
        output: String,

        /// Override a node parameter. Format: key=value or nodeId:key=value
        #[arg(short = 'p', long = "param")]
        param: Vec<String>,
    },

    /// List available built-in recipes.
    List,

    /// Show details about a built-in recipe.
    Info {
        /// Recipe slug (e.g. compress-images).
        recipe: String,
    },

    /// Check that all external dependencies are installed.
    Doctor,

    /// Launch the interactive terminal UI.
    Tui {
        /// Path to a .bnto.json recipe file to open directly.
        recipe: Option<String>,

        /// Color theme: los-angeles (default), tokyo (dark), monaco (sunset).
        #[arg(long, default_value = "los-angeles")]
        theme: String,

        /// Start with a blank recipe in the editor.
        #[arg(long)]
        new: bool,
    },

    /// Manage anonymous telemetry settings.
    Telemetry {
        #[command(subcommand)]
        action: TelemetryAction,
    },
}

#[derive(Subcommand)]
enum TelemetryAction {
    /// Enable anonymous telemetry.
    Enable,
    /// Disable anonymous telemetry.
    Disable,
    /// Show current telemetry status.
    Status,
}

fn main() {
    let cli = Cli::parse();
    telemetry::init();

    // Session logger — shared across all commands.
    let logger = create_logger();

    let cmd_name = match &cli.command {
        Some(Command::Run { .. }) => "run",
        Some(Command::List) => "list",
        Some(Command::Info { .. }) => "info",
        Some(Command::Doctor) => "doctor",
        Some(Command::Tui { .. }) => "tui",
        Some(Command::Telemetry { .. }) => "telemetry",
        None => "tui",
    };
    logger.log(LogEntry {
        level: LogLevel::Info,
        target: "cli",
        message: format!("command: {cmd_name}"),
        elapsed_us: None,
    });

    match cli.command {
        Some(Command::Run {
            recipe,
            inputs,
            output,
            param,
        }) => {
            telemetry::capture(telemetry::events::cli_command("run"));
            run_recipe(&recipe, &inputs, &output, &param, &logger);
        }
        Some(Command::List) => {
            telemetry::capture(telemetry::events::cli_command("list"));
            list_recipes();
        }
        Some(Command::Info { recipe }) => {
            telemetry::capture(telemetry::events::cli_command("info"));
            show_info(&recipe);
        }
        Some(Command::Doctor) => {
            telemetry::capture(telemetry::events::cli_command("doctor"));
            doctor::run_doctor();
        }
        Some(Command::Tui { recipe, theme, new }) => {
            telemetry::capture(telemetry::events::cli_command("tui"));
            launch_tui(&theme, recipe, new, &logger);
        }
        Some(Command::Telemetry { action }) => match action {
            TelemetryAction::Enable => {
                telemetry::set_enabled(true);
                eprintln!("Telemetry enabled.");
            }
            TelemetryAction::Disable => {
                telemetry::set_enabled(false);
                eprintln!("Telemetry disabled.");
            }
            TelemetryAction::Status => telemetry::print_status(),
        },
        None => {
            telemetry::capture(telemetry::events::cli_command("tui"));
            launch_tui("los-angeles", None, false, &logger);
        }
    }

    logger.flush();
}

/// Create the session logger. Falls back to NoopLogger if paths can't be resolved.
fn create_logger() -> Arc<dyn Logger> {
    tui::paths::BntoPaths::resolve()
        .and_then(|p| FileLogger::new(&p.logs_dir(), LogLevel::Debug))
        .map(|fl| Arc::new(fl) as Arc<dyn Logger>)
        .unwrap_or_else(|| Arc::new(NoopLogger))
}

fn launch_tui(theme_str: &str, recipe_path: Option<String>, new: bool, logger: &Arc<dyn Logger>) {
    let variant = match tui::theme::ThemeVariant::from_str_lossy(theme_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} {e}", "Error:".red());
            process::exit(1);
        }
    };
    // Read recipe JSON from disk if a path was provided.
    let recipe_json = recipe_path.map(|path| match std::fs::read_to_string(&path) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("{} Cannot read recipe file '{}': {e}", "Error:".red(), path);
            process::exit(1);
        }
    });
    let start = std::time::Instant::now();
    if let Err(e) = tui::launch_tui(variant, recipe_json, new, logger) {
        eprintln!("{} {e}", "TUI error:".red());
        process::exit(1);
    }
    let duration_ms = start.elapsed().as_millis() as u64;
    telemetry::capture(telemetry::events::cli_tui_session(duration_ms, theme_str));
}

/// Read recipe JSON — try built-in slug first, then disk path.
fn read_recipe(path: &str) -> String {
    if let Some(recipe) = bnto_engine::recipes::builtin_recipe_by_slug(path) {
        return recipe.definition_json.to_string();
    }
    match std::fs::read_to_string(path) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("{} {e}", "Error reading recipe:".red());
            process::exit(1);
        }
    }
}

/// Write pipeline results to disk, print summary.
fn write_output(result: &bnto_core::PipelineResult, output_dir: &str) {
    if let Err(e) = io::write_results(result, output_dir) {
        eprintln!("{} {e}", "Error writing output:".red());
        process::exit(1);
    }
    let n = result.files.len();
    let duration = format_duration(result.duration_ms);
    eprintln!(
        "\n{} {n} file{} written to {output_dir}/ in {duration}",
        "Done.".green().bold(),
        if n == 1 { "" } else { "s" },
    );
}

fn run_recipe(
    recipe_path: &str,
    inputs: &[String],
    output_dir: &str,
    param_overrides: &[String],
    logger: &Arc<dyn Logger>,
) {
    let raw_json = read_recipe(recipe_path);
    let prepared = unwrap_or_exit(input::prepare_inputs(&raw_json, inputs, param_overrides));
    print_run_banner(recipe_path, &raw_json, &prepared);

    let file_count = prepared.files.len();
    let total_bytes: u64 = prepared.files.iter().map(|f| f.data.len() as u64).sum();
    let param_names: Vec<String> = param_overrides
        .iter()
        .filter_map(|p| p.split('=').next().map(|k| k.to_string()))
        .collect();

    logger.log(LogEntry {
        level: LogLevel::Info,
        target: "cli",
        message: format!("run: recipe={recipe_path} files={file_count} bytes={total_bytes}"),
        elapsed_us: None,
    });

    let start = std::time::Instant::now();
    let ctx = unwrap_or_exit(context::NativeContext::current_dir());
    let reporter = progress::stderr_reporter(Arc::clone(logger));
    match bnto_engine::run_pipeline(&prepared.definition_json, prepared.files, &reporter, &ctx) {
        Ok(result) => {
            let elapsed_us = start.elapsed().as_micros() as u64;
            logger.log(LogEntry {
                level: LogLevel::Info,
                target: "cli",
                message: format!("run complete: {} output files", result.files.len()),
                elapsed_us: Some(elapsed_us),
            });
            telemetry::capture(telemetry::events::cli_recipe_run_with_params(
                recipe_path,
                result.duration_ms,
                file_count,
                total_bytes,
                result.files.len(),
                true,
                &param_names,
            ));
            write_output(&result, output_dir);
        }
        Err(e) => {
            logger.log(LogEntry {
                level: LogLevel::Error,
                target: "cli",
                message: format!("run failed: {e}"),
                elapsed_us: None,
            });
            telemetry::capture(telemetry::events::cli_error("run", &e.to_string()));
            eprintln!("{} {e}", "Pipeline failed:".red());
            process::exit(1);
        }
    }
}

fn print_run_banner(recipe_path: &str, raw_json: &str, prepared: &input::PreparedInput) {
    let label = input::mode_label(raw_json);
    if prepared.files.is_empty() {
        eprintln!("{} with {label}...", recipe_path.bold());
    } else {
        let count = prepared.files.len();
        eprintln!(
            "{} with {count} file{}...",
            recipe_path.bold(),
            if count == 1 { "" } else { "s" }
        );
    }
}

/// Unwrap a Result or print the error to stderr and exit.
pub(crate) fn unwrap_or_exit<T, E: std::fmt::Display>(result: Result<T, E>) -> T {
    match result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} {e}", "Error:".red());
            process::exit(1);
        }
    }
}

fn list_recipes() {
    let recipes = bnto_engine::recipes::builtin_recipes();
    let groups = list::group_recipes(recipes);
    list::print_recipe_list(&groups);
}

fn show_info(slug: &str) {
    let Some(recipe_info) = info::get_recipe_info(slug) else {
        eprintln!("{} Unknown recipe: {slug}", "Error:".red());
        eprintln!("Run {} to see available recipes.", "bnto list".cyan());
        process::exit(1);
    };
    info::print_recipe_info(slug, &recipe_info);
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
