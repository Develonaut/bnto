// `bnto doctor` — check external dependencies are installed.

use colored::Colorize;

use crate::context;

/// Run the doctor command — check all external dependencies.
pub fn run_doctor() {
    let registry = bnto_engine::create_registry();
    let deps = bnto_engine::deps::collect_all_dependencies(&registry);
    if deps.is_empty() {
        println!("{} All processors are self-contained.", "ok".green().bold());
        return;
    }
    let ctx = crate::unwrap_or_exit(context::NativeContext::current_dir());
    let statuses = bnto_engine::deps::check_dependencies(&deps, &ctx);
    if print_statuses(&statuses) {
        eprintln!(
            "\n{}",
            "Some dependencies are missing. Install them to use all processors.".yellow()
        );
        std::process::exit(1);
    }
    println!("\n{}", "All dependencies satisfied.".green());
}

/// Print each dependency status line. Returns true if any are missing.
fn print_statuses(statuses: &[bnto_engine::deps::DependencyStatus]) -> bool {
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
