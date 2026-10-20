// bnto CLI — run .bnto.json recipes from the command line.
//
// Usage: bnto run <recipe> <file1> [file2 ...]
//        bnto run <recipe> <url>  (for url-mode recipes)
//        bnto run <recipe> <file1> --param quality=50
//        bnto run <recipe> <file1> --dry-run  (run every node, write nothing)
//        bnto run <recipe> <file1> --json  (summary as one JSON line on stdout)
//        bnto list
//        bnto info <recipe>
//        bnto dry-run <recipe> [files...] [--param key=value]
//        bnto check <recipe file>...  (validate recipe files without running)
//        bnto install <recipe> [--yes]
//        bnto install --all [--yes]
//        bnto doctor
//        bnto migrate <path> [--dry-run]
//        bnto tui [--theme <variant>]  (interactive TUI, beta)

pub mod catalog;
mod check;
mod cli;
mod command_stream;
mod consent;
mod context;
mod dispatch;
mod doctor;
pub mod dry_run;
mod info;
mod input;
mod inspect;
mod install;
mod install_plan;
mod install_trust;
mod io;
#[cfg(windows)]
mod job_object;
mod launch;
mod list;
mod live_bar;
pub mod logging;
mod migrate;
mod overwrite;
mod package_manager;
mod process_registry;
mod progress;
mod progress_indent;
mod progress_line;
mod progressive_output;
mod run;
mod run_output;
mod run_verdict;
mod scratch;
pub mod storage;
pub mod telemetry;
mod trust;
mod tui;
mod vars;

use std::process;
use std::sync::Arc;

use bnto_core::logging::{LogEntry, LogLevel, Logger, NoopLogger};
use clap::Parser;
use colored::Colorize;

use catalog::RecipeCatalog;
use cli::Cli;
use dispatch::Session;
use logging::FileLogger;

fn main() {
    let cli = Cli::parse();
    telemetry::init();

    install_signal_handler();

    let paths = init_storage();
    // Session logger — shared across all commands.
    let logger = create_logger(&paths);
    // Build unified recipe catalog — bundled + user library.
    let library_dir = paths.as_ref().map(|p| p.recipes_dir()).unwrap_or_default();
    let catalog = RecipeCatalog::load(&library_dir);

    let cmd_name = cli.command.as_ref().map_or("tui", cli::Command::name);
    log_command(&logger, cmd_name);
    let session = Session {
        logger: Arc::clone(&logger),
        catalog,
        paths,
    };
    dispatch::dispatch(cli.command, session);
    logger.flush();
}

/// Kill spawned command trees (shell-command children + grandchildren) on
/// SIGINT/SIGTERM — they run in their own process groups, so the terminal's
/// Ctrl+C no longer reaches them on its own. Then delete any run scratch
/// dirs, which `process::exit` would otherwise leave behind.
fn install_signal_handler() {
    let _ = ctrlc::set_handler(|| {
        process_registry::global().terminate_all();
        scratch::remove_live_dirs();
        process::exit(130);
    });
}

/// Resolve paths, create dirs, run pending migrations — before anything else.
fn init_storage() -> Option<storage::BntoPaths> {
    match storage::ensure_ready() {
        Ok(p) => {
            // Apply config home override so all derived paths follow it.
            let config = storage::config::TomlConfig::load(&p);
            Some(p.with_config_override(&config))
        }
        Err(e) => {
            eprintln!("{} storage init: {e}", "Warning:".yellow());
            None
        }
    }
}

/// Create the session logger. Falls back to NoopLogger if paths unavailable.
fn create_logger(paths: &Option<storage::BntoPaths>) -> Arc<dyn Logger> {
    paths
        .as_ref()
        .and_then(|p| FileLogger::new(&p.logs_dir(), LogLevel::Debug))
        .map(|fl| Arc::new(fl) as Arc<dyn Logger>)
        .unwrap_or_else(|| Arc::new(NoopLogger))
}

/// Record which subcommand this session runs.
fn log_command(logger: &Arc<dyn Logger>, cmd_name: &str) {
    logger.log(LogEntry {
        level: LogLevel::Info,
        target: "cli",
        message: format!("command: {cmd_name}"),
        elapsed_us: None,
    });
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
