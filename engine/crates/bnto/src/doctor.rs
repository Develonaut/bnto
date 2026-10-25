// `bnto doctor` — check external dependencies and recipe secrets.

use bnto_core::HostPlatform;
use bnto_engine::deps::{DependencyStatus, ScopedDependency};
use colored::Colorize;

use crate::catalog::RecipeCatalog;
use crate::context;

/// Run the doctor command — check dependencies and recipe secrets.
pub fn run_doctor(catalog: &RecipeCatalog) {
    let registry = bnto_engine::create_registry();
    let ctx = crate::unwrap_or_exit(context::NativeContext::current_dir());

    // Parse once; an unparsable recipe is itself an issue (it can't run).
    let recipes = parse_catalog(catalog);
    let mut has_issues = print_parse_failures(&recipes.invalid);
    has_issues |= check_binaries(&registry, &recipes.definitions, &ctx);
    // `|=` (not `||`) so the secrets check always runs and prints.
    has_issues |= check_all_recipe_secrets(&recipes, &ctx);
    report_outcome(has_issues);
}

/// Check binary dependencies across all recipes. Returns true on any issue.
/// A dependency only other OSes need is listed, not checked.
fn check_binaries(
    registry: &bnto_core::NodeRegistry,
    definitions: &[bnto_core::PipelineDefinition],
    ctx: &dyn bnto_core::ProcessContext,
) -> bool {
    let scoped = bnto_engine::deps::scoped_catalog_dependencies(registry, definitions);
    if scoped.is_empty() {
        println!("{} All processors are self-contained.", "ok".green().bold());
        return false;
    }
    let (here, elsewhere) = bnto_engine::deps::split_by_host(scoped, HostPlatform::current());
    let deps: Vec<_> = here.iter().map(|s| s.dependency.clone()).collect();
    let statuses = bnto_engine::deps::check_dependencies(&deps, ctx);
    let has_issues = print_dep_statuses(&statuses, &here);
    print_not_needed(&elsewhere);
    has_issues
}

/// Print the final verdict, exiting non-zero when any check failed.
fn report_outcome(has_issues: bool) {
    if has_issues {
        eprintln!(
            "
{}",
            "Some checks failed. See above for details.".yellow()
        );
        std::process::exit(1);
    }
    println!(
        "
{}",
        "All checks passed.".green()
    );
}

/// Catalog recipes parsed once. `slugs[i]` names `definitions[i]`; the
/// definitions stay a plain slice so dependency collection can borrow them.
struct ParsedRecipes {
    slugs: Vec<String>,
    definitions: Vec<bnto_core::PipelineDefinition>,
    /// `(slug, parse error)` for recipes that failed to parse.
    invalid: Vec<(String, String)>,
}

/// Parse every catalog recipe exactly once, keeping failures for reporting.
fn parse_catalog(catalog: &RecipeCatalog) -> ParsedRecipes {
    let mut parsed = ParsedRecipes {
        slugs: Vec::new(),
        definitions: Vec::new(),
        invalid: Vec::new(),
    };
    for entry in catalog.all() {
        match serde_json::from_str(&entry.definition_json) {
            Ok(def) => {
                parsed.slugs.push(entry.slug.clone());
                parsed.definitions.push(def);
            }
            Err(e) => parsed.invalid.push((entry.slug.clone(), e.to_string())),
        }
    }
    parsed
}

/// Warn about each recipe that failed to parse. Returns true if any did.
fn print_parse_failures(invalid: &[(String, String)]) -> bool {
    for (slug, err) in invalid {
        println!(
            "{} Could not parse recipe {}: {}",
            "Warning:".yellow().bold(),
            slug,
            err
        );
    }
    if !invalid.is_empty() {
        println!();
    }
    !invalid.is_empty()
}

/// Print each dependency status line. Returns true if any have issues.
fn print_dep_statuses(statuses: &[DependencyStatus], scoped: &[ScopedDependency]) -> bool {
    println!(
        "{}
",
        "Checking external dependencies...".bold()
    );
    let mut has_issues = false;
    for (status, scoped) in statuses.iter().zip(scoped) {
        has_issues |= print_dep_status(status, &name_with_os(scoped));
    }
    has_issues
}

/// The binary, plus " (macOS only)" when some OS doesn't need it.
fn name_with_os(scoped: &ScopedDependency) -> String {
    match scoped.os_note() {
        Some(note) => format!(
            "{} {}",
            scoped.dependency.binary,
            format!("({note})").dimmed()
        ),
        None => scoped.dependency.binary.clone(),
    }
}

/// Print one dependency's status line(s). Returns true if it has an issue.
fn print_dep_status(status: &DependencyStatus, name: &str) -> bool {
    if !status.found {
        print_missing(status, name);
        true
    } else if status.version_satisfied == Some(false) {
        print_outdated(status, name);
        true
    } else {
        print_found(status, name);
        false
    }
}

fn print_missing(status: &DependencyStatus, name: &str) {
    println!("  {} {name}", "MISSING".red().bold());
    println!(
        "         Install: {}",
        status.dependency.host_install_hint().cyan()
    );
    if !status.dependency.homepage.is_empty() {
        println!("         Homepage: {}", status.dependency.homepage);
    }
}

fn print_outdated(status: &DependencyStatus, name: &str) {
    let installed = status.installed_version.as_deref().unwrap_or("unknown");
    println!(
        "  {} {name} (installed: {}, requires: {})",
        "OUTDATED".yellow().bold(),
        installed,
        status.dependency.version
    );
    println!(
        "         Update: {}",
        status.dependency.host_install_hint().cyan()
    );
}

fn print_found(status: &DependencyStatus, name: &str) {
    let version_info = match &status.installed_version {
        Some(v) => format!(" ({})", v.dimmed()),
        None => String::new(),
    };
    println!("  {} {name}{version_info}", "ok".green());
}

/// List the dependencies only other OSes need, unchecked.
fn print_not_needed(elsewhere: &[ScopedDependency]) {
    for scoped in elsewhere {
        println!("  {} {}", "skip".dimmed(), name_with_os(scoped));
    }
}

/// Check secrets across all catalog recipes that declare them.
/// Returns true if any required secrets are missing.
fn check_all_recipe_secrets(recipes: &ParsedRecipes, ctx: &dyn bnto_core::ProcessContext) -> bool {
    let mut has_missing = false;
    let mut checked_any = false;

    for (slug, def) in recipes.slugs.iter().zip(&recipes.definitions) {
        if def.secrets.is_empty() {
            continue;
        }

        if !checked_any {
            println!("\n{}\n", "Checking recipe secrets...".bold());
            checked_any = true;
        }

        has_missing |= print_recipe_secret_status(slug, def, ctx);
    }

    has_missing
}

/// Check and print secret status for a single recipe. Returns true if missing.
fn print_recipe_secret_status(
    slug: &str,
    def: &bnto_core::PipelineDefinition,
    ctx: &dyn bnto_core::ProcessContext,
) -> bool {
    let statuses = bnto_core::secrets::check_secrets(&def.secrets, ctx);
    let missing = bnto_core::secrets::missing_required(&statuses);

    if missing.is_empty() {
        println!("  {} {}: all secrets present", "ok".green(), slug);
        return false;
    }

    println!("  {} {}", "MISSING".red().bold(), slug);
    for s in &missing {
        if s.description.is_empty() {
            println!("         - {}", s.key);
        } else {
            println!("         - {} ({})", s.key, s.description);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_catalog_parses_every_bundled_recipe() {
        let catalog = RecipeCatalog::bundled_only();
        let recipes = parse_catalog(&catalog);
        assert_eq!(recipes.definitions.len(), 22);
        assert_eq!(recipes.slugs.len(), 22);
        assert!(recipes.invalid.is_empty());
    }

    #[test]
    fn parse_catalog_reports_unparsable_recipe() {
        let tmp = tempfile::tempdir().unwrap();
        // Valid JSON (so the catalog loads it) but not a valid definition.
        std::fs::write(tmp.path().join("broken.bnto.json"), r#"{"nodes":"x"}"#).unwrap();
        let catalog = RecipeCatalog::load(tmp.path());
        let recipes = parse_catalog(&catalog);
        assert_eq!(recipes.definitions.len(), 22);
        assert_eq!(recipes.invalid.len(), 1);
        assert_eq!(recipes.invalid[0].0, "broken");
        assert!(!recipes.invalid[0].1.is_empty());
    }

    #[test]
    fn parse_failures_count_as_issues() {
        assert!(!print_parse_failures(&[]));
        assert!(print_parse_failures(&[("x".into(), "bad".into())]));
    }
}
