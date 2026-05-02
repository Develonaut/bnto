// Migration registry — append new migrations here.
//
// Each migration has a version number and an apply function.
// The runner executes them in order, skipping already-applied versions.

pub mod v1_unified_home;
pub mod v2_simplify_paths;

use super::runner::Migration;

/// All registered migrations, in version order.
pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 2,
        name: "v1_unified_home",
        apply: v1_unified_home::apply,
    },
    Migration {
        version: 3,
        name: "v2_simplify_paths",
        apply: v2_simplify_paths::apply,
    },
];
