// edit-fields — Set fields on each item from templates over its other fields.
//
// A generic data node: for every incoming file it adds or overwrites the
// fields `values` names, each value a static one or a template over the
// fields the file arrived with (`{{item.first}} {{item.last}}`). Files pass
// through with their data untouched; only their metadata changes. It runs as
// a batch and fills `{{item.*}}` and `{{file}}` (the file's name) itself, in
// a loop too (the file's fields over the loop's item), so a field an item lacks
// fails the run naming the field and the item. A dry run sets fields too:
// nothing is written.

mod fill;
mod params;

use crate::control::placeholder::{self, Scope};
use crate::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use crate::processor::{BatchFile, BatchInput, NodeInput, NodeOutput, NodeProcessor, OutputFile};
use crate::progress::ProgressReporter;
use crate::{BntoError, ProcessContext};

use params::{EditFieldsParams, build_edit_fields_parameters};

/// Edit Fields processor — sets templated fields on each item.
pub struct EditFields;

impl EditFields {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EditFields {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for EditFields {
    fn name(&self) -> &str {
        "edit-fields"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "edit-fields".to_string(),
            name: "Edit Fields".to_string(),
            description: "Edit fields on each item, setting them from static values or templates \
                          over its other fields and its file name, such as {{item.first}} \
                          {{item.last}} or {{file}}; files pass through unchanged."
                .to_string(),
            category: NodeCategory::Data,
            accepts: vec![],
            platforms: vec![
                "browser".to_string(),
                "cli".to_string(),
                "desktop".to_string(),
            ],
            parameters: build_edit_fields_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        EditFieldsParams::read(params).err().into_iter().collect()
    }

    /// Required by the trait; the executor always runs this node as a batch.
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let file = BatchFile {
            data: input.data,
            filename: input.filename,
            mime_type: input.mime_type,
            metadata: serde_json::Map::new(),
        };
        let batch = BatchInput {
            files: vec![file],
            params: input.params,
        };
        self.process_batch(batch, progress, ctx)
    }

    fn resolves_items(&self) -> bool {
        true
    }

    fn process_batch(
        &self,
        input: BatchInput,
        _progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let loop_item = ctx.loop_item();
        let once = Scope {
            item: loop_item.as_ref(),
            run: Some(ctx),
            ..Scope::default()
        };
        let params = read_filled(&input.params, once).map_err(BntoError::InvalidInput)?;
        let files = input
            .files
            .into_iter()
            .map(|f| edit_file(&params, f, once))
            .collect::<Result<_, _>>()?;
        Ok(NodeOutput {
            files,
            metadata: serde_json::Map::new(),
        })
    }
}

/// The params, all but `values` filled once from `once` (`values` is filled
/// per file).
fn read_filled(
    params: &serde_json::Map<String, serde_json::Value>,
    once: Scope<'_>,
) -> Result<EditFieldsParams, String> {
    let filled = placeholder::fill_params(params, once, &["values"]).map_err(|(param, m)| {
        format!(
            "'{param}': {}",
            placeholder::no_value(&m.key, None, once.item)
        )
    })?;
    EditFieldsParams::read(&filled)
}

/// One file with its fields set, its data and name as they arrived. The
/// values read its metadata over the loop's item; the file keeps
/// only its own metadata and what was set.
fn edit_file(
    params: &EditFieldsParams,
    f: BatchFile,
    once: Scope<'_>,
) -> Result<OutputFile, BntoError> {
    let item = placeholder::file_item(&f.metadata, once.item);
    let set = fill::fill_values(&params.values, &f.filename, &item, once)
        .map_err(|why| BntoError::InvalidInput(format!("{}: {why}", f.filename)))?;
    let mut metadata = if params.keep_only_set {
        serde_json::Map::new()
    } else {
        f.metadata
    };
    metadata.extend(set);
    Ok(OutputFile {
        data: f.data,
        filename: f.filename,
        mime_type: f
            .mime_type
            .unwrap_or_else(|| "application/octet-stream".to_string()),
        metadata,
    })
}

#[cfg(test)]
mod tests;
