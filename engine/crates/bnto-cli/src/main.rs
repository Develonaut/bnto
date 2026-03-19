// bnto CLI — run .bnto.json recipes against local files.
//
// Sibling adapter to bnto-wasm. Both call execute_pipeline() from bnto-core.
// WASM adapts JS types; CLI adapts disk I/O.
//
// Usage:
//   bnto run recipe.bnto.json input1.jpg input2.jpg --output-dir ./out

mod io;
mod progress;
mod registry;

use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::{Parser, Subcommand};

use bnto_core::execute_pipeline;

#[derive(Parser)]
#[command(
    name = "bnto",
    version,
    about = "Run .bnto.json recipes against local files"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a recipe against input files
    Run {
        /// Path to the .bnto.json recipe definition
        recipe: PathBuf,

        /// Input files to process
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output directory (default: ./output)
        #[arg(long, default_value = "output")]
        output_dir: PathBuf,

        /// Suppress progress output
        #[arg(long)]
        quiet: bool,

        /// Emit progress events as JSON lines to stderr
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            recipe,
            files,
            output_dir,
            quiet,
            json,
        } => {
            if let Err(e) = run_recipe(&recipe, &files, &output_dir, quiet, json) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_recipe(
    recipe_path: &Path,
    input_paths: &[PathBuf],
    output_dir: &Path,
    quiet: bool,
    json: bool,
) -> Result<(), String> {
    let definition = io::read_definition(recipe_path)?;
    let files = io::read_input_files(input_paths)?;
    let registry = registry::create_default_registry();

    let reporter = if json {
        progress::json_reporter()
    } else {
        progress::terminal_reporter(quiet)
    };

    // Native Rust — use std::time::Instant directly.
    let epoch = Instant::now();
    let now_ms = move || epoch.elapsed().as_millis() as u64;

    let result = execute_pipeline(&definition, files, &registry, &reporter, now_ms)
        .map_err(|e| e.to_string())?;

    io::write_output_files(&result.files, output_dir)?;

    if !quiet && !json {
        eprintln!(
            "Wrote {} file(s) to {}",
            result.files.len(),
            output_dir.display()
        );
    }

    Ok(())
}
