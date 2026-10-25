// The folder a recipe's output node names in its `directory` param, with
// every placeholder filled. Shared by `bnto run` and the TUI so both write a
// recipe to the same place, and both refuse one nothing fills.

use bnto_core::{PipelineDefinition, PipelineFile, ProcessContext};

/// The recipe's own output folder, or `None` when its output node names none
/// (or the JSON doesn't parse, which the run refuses on its own). An `Err`
/// names each placeholder nothing fills, so the caller stops before it makes
/// the folder or runs a step.
pub fn recipe_output_dir(
    definition_json: &str,
    ctx: &dyn ProcessContext,
    files: &[PipelineFile],
) -> Result<Option<String>, String> {
    let Ok(def) = serde_json::from_str::<PipelineDefinition>(definition_json) else {
        return Ok(None);
    };
    bnto_core::fill_output_directory(&def, ctx, files)
}
