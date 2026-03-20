// bnto CLI — run .bnto.json recipes from the command line.
//
// Usage: bnto run <recipe.bnto.json> <file1> [file2 ...]
//        bnto run --recipe compress-images <file1> [file2 ...]

mod io;
mod progress;

use std::process;

use clap::{Parser, Subcommand};

use bnto_core::PipelineFile;

/// bnto — run recipes from the command line.
#[derive(Parser)]
#[command(name = "bnto", version, about = "Run .bnto.json recipes")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Execute a recipe against input files.
    Run {
        /// Path to a .bnto.json recipe file.
        recipe: String,

        /// Input files to process.
        #[arg(required = true)]
        files: Vec<String>,

        /// Output directory (default: current directory).
        #[arg(short, long, default_value = ".")]
        output: String,
    },

    /// List available built-in recipes.
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Run {
            recipe,
            files,
            output,
        } => run_recipe(&recipe, &files, &output),
        Command::List => list_recipes(),
    }
}

/// Read recipe JSON from disk, exit on failure.
fn read_recipe(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error reading recipe: {e}");
            process::exit(1);
        }
    }
}

/// Load input files, skipping any that fail. Exits if none are valid.
fn load_input_files(paths: &[String]) -> Vec<PipelineFile> {
    let files: Vec<PipelineFile> = paths
        .iter()
        .filter_map(|path| match io::read_pipeline_file(path) {
            Ok(f) => Some(f),
            Err(e) => {
                eprintln!("Warning: skipping {path}: {e}");
                None
            }
        })
        .collect();

    if files.is_empty() {
        eprintln!("Error: no valid input files");
        process::exit(1);
    }
    files
}

/// Write pipeline results to disk, exit on failure.
fn write_output(result: &bnto_core::PipelineResult, output_dir: &str) {
    if let Err(e) = io::write_results(result, output_dir) {
        eprintln!("Error writing output: {e}");
        process::exit(1);
    }
    let n = result.files.len();
    eprintln!(
        "Done. {n} file{} written to {output_dir}/ in {}ms",
        if n == 1 { "" } else { "s" },
        result.duration_ms
    );
}

fn run_recipe(recipe_path: &str, input_paths: &[String], output_dir: &str) {
    let definition_json = read_recipe(recipe_path);
    let files = load_input_files(input_paths);

    let count = files.len();
    eprintln!(
        "Running {recipe_path} with {count} file{}...",
        if count == 1 { "" } else { "s" }
    );

    let reporter = progress::stderr_reporter();
    match bnto_engine::run_pipeline(&definition_json, files, &reporter) {
        Ok(result) => write_output(&result, output_dir),
        Err(e) => {
            eprintln!("Pipeline failed: {e}");
            process::exit(1);
        }
    }
}

fn list_recipes() {
    let registry = bnto_engine::create_default_registry();
    let mut catalog = registry.catalog();
    catalog.sort_by(|a, b| a.node_type.cmp(&b.node_type));

    println!("Available processors:");
    for entry in &catalog {
        println!("  {:<25} {}", entry.node_type, entry.description);
    }
    println!("\nUse a .bnto.json recipe file to compose processors into pipelines.");
}
