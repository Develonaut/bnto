// Read-only recipe commands: `bnto list`, `bnto info`, `bnto dry-run`.

use std::process;

use colored::Colorize;

use crate::catalog::RecipeCatalog;
use crate::consent::check_trust;
use crate::{dry_run, host_support, info, list};

/// `bnto list` — print the catalog grouped by category.
pub fn list_recipes(catalog: &RecipeCatalog) {
    let groups = list::group_recipes(catalog.all());
    list::print_recipe_list(&groups);
}

/// `bnto info <recipe>` — print recipe details and dependencies.
pub fn show_info(slug: &str, catalog: &RecipeCatalog) {
    let Some(recipe_info) = info::get_recipe_info(slug, catalog) else {
        exit_unknown_recipe(slug);
    };
    info::print_recipe_info(slug, &recipe_info);
}

/// `bnto dry-run <recipe>` — preview what a recipe will do.
pub fn show_dry_run(
    slug: &str,
    inputs: &[String],
    param_overrides: &[String],
    yes: bool,
    catalog: &RecipeCatalog,
) {
    refuse_unsupported_host(slug, catalog);
    // With inputs the preview executes the pipeline (shell commands
    // included), so it needs the same consent as `bnto run`.
    if let Some(entry) = catalog.resolve(slug)
        && !inputs.is_empty()
    {
        check_trust(slug, &entry.definition_json, yes, catalog);
        dry_run::print_shell_note(&entry.definition_json);
    }
    let Some(result) = dry_run::dry_run_recipe(slug, inputs, param_overrides, catalog) else {
        exit_unknown_recipe(slug);
    };
    dry_run::print_dry_run(slug, &result);
}

/// Stop a recipe with no branch for this OS, as `bnto run` does: a preview
/// of a run that would be refused would mislead.
fn refuse_unsupported_host(slug: &str, catalog: &RecipeCatalog) {
    let Some(entry) = catalog.resolve(slug) else {
        return;
    };
    let host = bnto_core::HostPlatform::current();
    if let Some(why) = host_support::unsupported_host(slug, &entry.definition_json, host) {
        eprintln!("{} {why}", "Error:".red());
        process::exit(1);
    }
}

/// Report an unknown recipe slug, then exit.
fn exit_unknown_recipe(slug: &str) -> ! {
    eprintln!("{} Unknown recipe: {slug}", "Error:".red());
    eprintln!("Run {} to see available recipes.", "bnto list".cyan());
    process::exit(1);
}
