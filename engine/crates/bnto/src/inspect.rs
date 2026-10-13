// Read-only recipe commands: `bnto list`, `bnto info`, `bnto dry-run`.

use std::process;

use colored::Colorize;

use crate::catalog::RecipeCatalog;
use crate::consent::check_trust;
use crate::{dry_run, info, list};

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
    // With inputs the preview executes the pipeline (shell commands
    // included), so it needs the same consent as `bnto run`.
    if let Some(entry) = catalog.resolve(slug)
        && !inputs.is_empty()
    {
        check_trust(slug, &entry.definition_json, yes, catalog);
    }
    let Some(result) = dry_run::dry_run_recipe(slug, inputs, param_overrides, catalog) else {
        exit_unknown_recipe(slug);
    };
    dry_run::print_dry_run(slug, &result);
}

/// Report an unknown recipe slug, then exit.
fn exit_unknown_recipe(slug: &str) -> ! {
    eprintln!("{} Unknown recipe: {slug}", "Error:".red());
    eprintln!("Run {} to see available recipes.", "bnto list".cyan());
    process::exit(1);
}
