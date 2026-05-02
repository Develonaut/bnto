// v2 → v3: Drop per-directory path overrides.
//
// Removes `paths.recipes` and `paths.output` from config.
// Recipes now own their output directory via the `directory` parameter.
// The only remaining path setting is `paths.home` (the bnto root).
//
// No data transformation needed — serde ignores unknown fields on load.
// This migration just bumps the version so the runner stays consistent.

use super::super::paths::BntoPaths;
use super::super::runner::MigrationError;

pub fn apply(_paths: &BntoPaths) -> Result<(), MigrationError> {
    // Version bump only — old path fields are silently dropped on next load/save.
    Ok(())
}
