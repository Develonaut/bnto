// `bnto doctor` — check external dependencies are installed.

use colored::Colorize;

use crate::context;

/// Run the doctor command �� check all external dependencies.
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
            "Some dependencies are not satisfied. Install or update them to use all processors."
                .yellow()
        );
        std::process::exit(1);
    }
    println!("\n{}", "All dependencies satisfied.".green());
}

/// Print each dependency status line. Returns true if any have issues.
fn print_statuses(statuses: &[bnto_engine::deps::DependencyStatus]) -> bool {
    let mut has_issues = false;
    println!("{}\n", "Checking external dependencies...".bold());
    for status in statuses {
        if !status.found {
            has_issues = true;
            println!("  {} {}", "MISSING".red().bold(), status.dependency.binary);
            println!(
                "         Install: {}",
                status.dependency.install_hint.cyan()
            );
            if !status.dependency.homepage.is_empty() {
                println!("         Homepage: {}", status.dependency.homepage);
            }
        } else if status.version_satisfied == Some(false) {
            has_issues = true;
            let installed = status.installed_version.as_deref().unwrap_or("unknown");
            println!(
                "  {} {} (installed: {}, requires: {})",
                "OUTDATED".yellow().bold(),
                status.dependency.binary,
                installed,
                status.dependency.version
            );
            println!("         Update: {}", status.dependency.install_hint.cyan());
        } else {
            let version_info = match &status.installed_version {
                Some(v) => format!(" ({})", v.dimmed()),
                None => String::new(),
            };
            println!(
                "  {} {}{}",
                "ok".green(),
                status.dependency.binary,
                version_info
            );
        }
    }
    has_issues
}
