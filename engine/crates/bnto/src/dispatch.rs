// Route a parsed subcommand to its implementation.

use std::sync::Arc;

use bnto_core::logging::Logger;

use crate::catalog::RecipeCatalog;
use crate::cli::{Command, TelemetryAction};
use crate::inspect::{list_recipes, show_dry_run, show_info};
use crate::launch::launch_tui;
use crate::run::{RunFlags, run_recipe};
use crate::{doctor, install, migrate, storage, telemetry};

/// Shared state every subcommand may need.
pub struct Session {
    pub logger: Arc<dyn Logger>,
    pub catalog: RecipeCatalog,
    pub paths: Option<storage::BntoPaths>,
}

/// Run `command` (no subcommand launches the TUI).
///
/// Every command except `telemetry` reports its own use first.
pub fn dispatch(command: Option<Command>, session: Session) {
    let Some(command) = command else {
        telemetry::capture(telemetry::events::cli_command("tui"));
        launch_tui("los-angeles", None, false, &session.logger, session.paths);
        return;
    };
    if !matches!(command, Command::Telemetry { .. }) {
        telemetry::capture(telemetry::events::cli_command(command.name()));
    }
    run_command(command, session);
}

/// Execute one subcommand.
fn run_command(command: Command, session: Session) {
    let catalog = &session.catalog;
    match command {
        Command::Run {
            recipe,
            inputs,
            output,
            param,
            yes,
            dry_run,
        } => run_recipe(
            &recipe,
            &inputs,
            output.as_deref(),
            &param,
            RunFlags { yes, dry_run },
            &session.logger,
            catalog,
        ),
        Command::List => list_recipes(catalog),
        Command::Info { recipe } => show_info(&recipe, catalog),
        Command::DryRun {
            recipe,
            inputs,
            param,
            yes,
        } => show_dry_run(&recipe, &inputs, &param, yes, catalog),
        Command::Install { recipe, yes, all } => {
            install::run_install(recipe.as_deref(), yes, all, catalog)
        }
        Command::Doctor => doctor::run_doctor(catalog),
        Command::Migrate { path, dry_run } => migrate::run_migrate(&path, dry_run),
        Command::Tui { recipe, theme, new } => {
            launch_tui(&theme, recipe, new, &session.logger, session.paths)
        }
        Command::Telemetry { action } => run_telemetry(action),
    }
}

/// `bnto telemetry enable|disable|status`.
fn run_telemetry(action: TelemetryAction) {
    match action {
        TelemetryAction::Enable => {
            telemetry::set_enabled(true);
            eprintln!("Telemetry enabled.");
        }
        TelemetryAction::Disable => {
            telemetry::set_enabled(false);
            eprintln!("Telemetry disabled.");
        }
        TelemetryAction::Status => telemetry::print_status(),
    }
}
