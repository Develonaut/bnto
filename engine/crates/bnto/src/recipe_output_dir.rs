// The folder a recipe's output node names in its `directory` param, with its
// templates resolved. Shared by `bnto run` and the TUI so both write a recipe
// to the same place.

use bnto_core::{PipelineDefinition, PipelineFile, ProcessContext};

/// The recipe's own output folder, or `None` when its output node names none.
/// `{{ctx.*}}` resolves first, then `{{node.*}}` (input file metadata).
pub fn recipe_output_dir(
    definition_json: &str,
    ctx: &dyn ProcessContext,
    files: &[PipelineFile],
) -> Option<String> {
    let def = serde_json::from_str::<PipelineDefinition>(definition_json).ok()?;
    let dir = bnto_core::resolve_output_directory(&def)?;
    let resolved = bnto_core::resolve_ctx_templates(&dir, ctx);
    let node_outputs = bnto_core::build_node_outputs_for_input(&def.nodes, files);
    Some(bnto_core::resolve_node_templates(&resolved, &node_outputs))
}
