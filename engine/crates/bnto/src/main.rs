// bnto CLI — run .bnto.json recipes from the command line.
//
// Usage: bnto                      (launches interactive TUI if terminal detected)
//        bnto run <recipe> <file1> [file2 ...]
//        bnto run <recipe> <url>  (for url-mode recipes)
//        bnto run <recipe> <file1> --param quality=50
//        bnto list
//        bnto info <recipe>
//        bnto doctor

mod context;
mod info;
mod input;
mod io;
mod list;
mod progress;
mod tui;

use std::io::IsTerminal;
use std::process;

use clap::{Parser, Subcommand};
use colored::Colorize;

/// bnto — run recipes from the command line.
#[derive(Parser)]
#[command(name = "bnto", version, about = "Run .bnto.json recipes")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Disable interactive TUI even when running in a terminal.
    #[arg(long)]
    no_interactive: bool,
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
    Tui,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Run {
            recipe,
            inputs,
            output,
            param,
        }) => run_recipe(&recipe, &inputs, &output, &param),
        Some(Command::List) => list_recipes(),
        Some(Command::Info { recipe }) => show_info(&recipe),
        Some(Command::Doctor) => run_doctor(),
        Some(Command::Tui) => launch_tui(),
        None => {
            if !cli.no_interactive && std::io::stdout().is_terminal() {
                launch_tui();
            } else {
                Cli::parse_from(["bnto", "--help"]);
            }
        }
    }
}

fn launch_tui() {
    if let Err(e) = tui::launch_tui() {
        eprintln!("{} {e}", "TUI error:".red());
        process::exit(1);
    }
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

fn run_recipe(recipe_path: &str, inputs: &[String], output_dir: &str, param_overrides: &[String]) {
    let raw_json = read_recipe(recipe_path);
    let prepared = unwrap_or_exit(input::prepare_inputs(&raw_json, inputs, param_overrides));
    print_run_banner(recipe_path, &raw_json, &prepared);

    let ctx = unwrap_or_exit(context::NativeContext::current_dir());
    let reporter = progress::stderr_reporter();
    match bnto_engine::run_pipeline(&prepared.definition_json, prepared.files, &reporter, &ctx) {
        Ok(result) => write_output(&result, output_dir),
        Err(e) => {
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
fn unwrap_or_exit<T, E: std::fmt::Display>(result: Result<T, E>) -> T {
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

fn run_doctor() {
    let registry = bnto_engine::create_registry();
    let deps = bnto_engine::deps::collect_all_dependencies(&registry);
    if deps.is_empty() {
        println!("{} All processors are self-contained.", "ok".green().bold());
        return;
    }
    let ctx = unwrap_or_exit(context::NativeContext::current_dir());
    let statuses = bnto_engine::deps::check_dependencies(&deps, &ctx);
    if print_dependency_statuses(&statuses) {
        eprintln!(
            "\n{}",
            "Some dependencies are missing. Install them to use all processors.".yellow()
        );
        process::exit(1);
    }
    println!("\n{}", "All dependencies satisfied.".green());
}

/// Print each dependency status line. Returns true if any are missing.
fn print_dependency_statuses(statuses: &[bnto_engine::deps::DependencyStatus]) -> bool {
    let mut has_missing = false;
    println!("{}\n", "Checking external dependencies...".bold());
    for status in statuses {
        if status.found {
            println!("  {} {}", "ok".green(), status.dependency.binary);
        } else {
            has_missing = true;
            println!("  {} {}", "MISSING".red().bold(), status.dependency.binary);
            println!(
                "         Install: {}",
                status.dependency.install_hint.cyan()
            );
            if !status.dependency.homepage.is_empty() {
                println!("         Homepage: {}", status.dependency.homepage);
            }
        }
    }
    has_missing
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
