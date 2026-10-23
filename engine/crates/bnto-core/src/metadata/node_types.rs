// Node type definitions: one file per category under `node_types/`, each
// returning its `NodeTypeInfo` entries. `all_node_types()` aggregates them
// into the engine's authoritative registry.

use super::{NodeTypeInfo, ParameterDef};

/// Constructs a `NodeTypeInfo` from positional fields. Reduces per-entry
/// boilerplate — keeps the table scannable.
macro_rules! node_type {
    ($name:expr, $label:expr, $desc:expr, $cat:expr, $container:expr, $platform:expr, $icon:expr) => {
        NodeTypeInfo {
            name: $name.to_string(),
            label: $label.to_string(),
            description: $desc.to_string(),
            category: $cat,
            is_container: $container,
            platforms: vec![$platform.to_string()],
            icon: $icon.to_string(),
        }
    };
}

mod control;
mod data;
mod file;
mod image;
mod io;
mod network;
mod spreadsheet;
mod system;
mod vector;

/// Return metadata for all 39 registered node types.
///
/// Single source of truth for the engine's node type registry.
/// Composed from per-category helpers, then sorted alphabetically for stable output.
pub fn all_node_types() -> Vec<NodeTypeInfo> {
    let mut types = Vec::with_capacity(26);
    types.extend(control::node_types());
    types.extend(data::node_types());
    types.extend(file::node_types());
    types.extend(image::node_types());
    types.extend(io::node_types());
    types.extend(network::node_types());
    types.extend(spreadsheet::node_types());
    types.extend(system::node_types());
    types.extend(vector::node_types());
    types.sort_by(|a, b| a.name.cmp(&b.name));
    types
}

/// Parameter definitions for a node type, when the engine knows them
/// independently of a `NodeProcessor` registration.
///
/// Processor node types carry their params on `NodeMetadata::parameters`
/// (collected from the registry). Structural / declarative node types
/// (input, output, loop, group, parallel, transform) declare
/// their params in `metadata::io_container`, which this accessor exposes
/// through a single lookup by type name.
///
/// Returns `None` for unknown types and for processor types (use the
/// registry's `NodeMetadata::parameters` for those).
pub fn node_type_params(type_name: &str) -> Option<Vec<ParameterDef>> {
    super::io_container::io_container_param_defs()
        .get(type_name)
        .cloned()
}
