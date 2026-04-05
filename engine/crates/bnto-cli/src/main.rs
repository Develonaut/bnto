// bnto CLI — run .bnto.json recipes from the command line.
//
// Usage: bnto run <recipe.bnto.json> <file1> [file2 ...]
//        bnto run <recipe.bnto.json> <url>  (for url-mode recipes)
//        bnto run <recipe.bnto.json> <file1> --param quality=50

mod context;
mod input;
mod io;
mod progress;

use std::process;

use clap::{Parser, Subcommand};

/// bnto — run recipes from the command line.
#[derive(Parser)]
#[command(name = "bnto", version, about = "Run .bnto.json recipes")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Execute a recipe against input files or a URL.
    Run {
        /// Path to a .bnto.json recipe file.
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

    /// Check that all external dependencies are installed.
    Doctor,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Run {
            recipe,
            inputs,
            output,
            param,
        } => run_recipe(&recipe, &inputs, &output, &param),
        Command::List => list_recipes(),
        Command::Doctor => run_doctor(),
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

fn run_recipe(recipe_path: &str, inputs: &[String], output_dir: &str, param_overrides: &[String]) {
    let raw_json = read_recipe(recipe_path);

    let prepared = match input::prepare_inputs(&raw_json, inputs, param_overrides) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let label = input::mode_label(&raw_json);
    if prepared.files.is_empty() {
        eprintln!("Running {recipe_path} with {label}...");
    } else {
        let count = prepared.files.len();
        eprintln!(
            "Running {recipe_path} with {count} file{}...",
            if count == 1 { "" } else { "s" }
        );
    }

    let ctx = match context::NativeContext::current_dir() {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let reporter = progress::stderr_reporter();
    match bnto_engine::run_pipeline(&prepared.definition_json, prepared.files, &reporter, &ctx) {
        Ok(result) => write_output(&result, output_dir),
        Err(e) => {
            eprintln!("Pipeline failed: {e}");
            process::exit(1);
        }
    }
}

fn list_recipes() {
    let registry = bnto_engine::create_registry();
    let mut catalog = registry.catalog();
    catalog.sort_by(|a, b| a.node_type.cmp(&b.node_type));

    println!("Available processors:");
    for entry in &catalog {
        println!("  {:<25} {}", entry.node_type, entry.description);
    }
    println!("\nUse a .bnto.json recipe file to compose processors into pipelines.");
}

fn run_doctor() {
    let registry = bnto_engine::create_registry();
    let deps = bnto_engine::deps::collect_all_dependencies(&registry);

    if deps.is_empty() {
        println!("All processors are self-contained. No external dependencies required.");
        return;
    }

    let ctx = match context::NativeContext::current_dir() {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let statuses = bnto_engine::deps::check_dependencies(&deps, &ctx);
    let mut has_missing = false;

    println!("Checking external dependencies...\n");
    for status in &statuses {
        let icon = if status.found { "ok" } else { "MISSING" };
        println!("  [{icon}] {}", status.dependency.binary);
        if !status.found {
            has_missing = true;
            println!("         Install: {}", status.dependency.install_hint);
            if !status.dependency.homepage.is_empty() {
                println!("         Homepage: {}", status.dependency.homepage);
            }
        }
    }

    if has_missing {
        println!("\nSome dependencies are missing. Install them to use all processors.");
        process::exit(1);
    } else {
        println!("\nAll dependencies satisfied.");
    }
}
