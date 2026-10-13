// Command-line definition — the clap parser for every `bnto` subcommand.

use clap::{Parser, Subcommand};

/// bnto — run recipes from the command line.
#[derive(Parser)]
#[command(name = "bnto", version, about = "Run .bnto.json recipes")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Top-level subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// Execute a recipe against input files or a URL.
    Run {
        /// Recipe slug (e.g. compress-images) or path to a .bnto.json file.
        recipe: String,

        /// Input files, URL, or text (depends on recipe input mode).
        inputs: Vec<String>,

        /// Output directory. Overrides recipe-defined output directory.
        /// Defaults to recipe's directory param, or current directory if unset.
        #[arg(short, long)]
        output: Option<String>,

        /// Override a node parameter. Format: key=value or nodeId:key=value
        #[arg(short = 'p', long = "param")]
        param: Vec<String>,

        /// Skip trust consent prompt for local recipes with shell commands.
        #[arg(long)]
        yes: bool,

        /// Run every node but write nothing; the summary says what would have
        /// been written. shell-command nodes still run.
        #[arg(long)]
        dry_run: bool,
    },

    /// List available recipes (built-in and library).
    List,

    /// Show details about a recipe.
    Info {
        /// Recipe slug (e.g. compress-images).
        recipe: String,
    },

    /// Dry-run: preview what a recipe will do without executing or writing.
    DryRun {
        /// Recipe slug (e.g. download-video) or path to a .bnto.json file.
        recipe: String,

        /// Input files or directories (enables file transformation preview).
        inputs: Vec<String>,

        /// Override a field value. Format: key=value or nodeId:key=value
        #[arg(short = 'p', long = "param")]
        param: Vec<String>,

        /// Skip trust consent for local recipes with shell commands (when
        /// inputs make the preview execute the pipeline).
        #[arg(long)]
        yes: bool,
    },

    /// Install external dependencies for a recipe.
    Install {
        /// Recipe slug (e.g. download-video) or path to a .bnto.json file.
        recipe: Option<String>,

        /// Skip confirmation prompt.
        #[arg(long)]
        yes: bool,

        /// Install dependencies for all built-in recipes.
        #[arg(long)]
        all: bool,
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

    /// Migrate .bnto.json files across breaking parameter changes.
    Migrate {
        /// Path to a .bnto.json file or directory containing recipe files.
        path: String,

        /// Preview changes without modifying files.
        #[arg(long)]
        dry_run: bool,
    },

    /// Manage anonymous telemetry settings.
    Telemetry {
        #[command(subcommand)]
        action: TelemetryAction,
    },
}

/// `bnto telemetry` subcommands.
#[derive(Subcommand)]
pub enum TelemetryAction {
    /// Enable anonymous telemetry.
    Enable,
    /// Disable anonymous telemetry.
    Disable,
    /// Show current telemetry status.
    Status,
}

impl Command {
    /// Subcommand name, used for logging and telemetry.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Run { .. } => "run",
            Self::List => "list",
            Self::Info { .. } => "info",
            Self::DryRun { .. } => "dry-run",
            Self::Install { .. } => "install",
            Self::Doctor => "doctor",
            Self::Migrate { .. } => "migrate",
            Self::Tui { .. } => "tui",
            Self::Telemetry { .. } => "telemetry",
        }
    }
}
