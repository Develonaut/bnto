// Closing Steps — what a chain does once one of its nodes has failed.
//
// The first failure is the chain's verdict. Every later node is skipped
// (no events) except those marked `always`, which run in order like
// `finally`: the first gets the files the failed step handed over, each
// next one gets the previous one's output. A closing step that fails joins
// the verdict after the first failure and hands over its own files.

use crate::errors::BntoError;
use crate::pipeline::PipelineFile;

use super::leaf_output::{node_output, pipeline_files};
use super::{ChainPosition, PipelineContext, PipelineNodeRef, execute_node};

/// A chain after one of its nodes failed.
pub(super) struct ChainFailure {
    /// The node whose failure is the verdict (a container's id when the
    /// failure was inside it).
    pub(super) node_id: String,
    /// The first failure, plus any closing step's that failed after it.
    error: BntoError,
    /// What the next closing step receives.
    files: Vec<PipelineFile>,
}

impl ChainFailure {
    /// The chain's first failure, at `node_id`, keeping the files it handed over.
    pub(super) fn new(node_id: &str, error: BntoError) -> Self {
        let (error, files) = split(error);
        Self {
            node_id: node_id.to_string(),
            error,
            files,
        }
    }

    /// Run each `always` node of `rest` (placed by `position`) in order,
    /// skipping the others. Closing steps record their output metadata in
    /// `ctx`, so later ones can read it.
    pub(super) fn close<F: Fn() -> u64 + Copy>(
        mut self,
        ctx: &mut PipelineContext<F>,
        rest: &[PipelineNodeRef],
        file_offset: usize,
        position: ChainPosition,
    ) -> Self {
        for (i, node) in rest.iter().enumerate().filter(|(_, n)| n.always) {
            let index = position.first_index + i;
            self = self.run_step(ctx, node, index, position.total_nodes, file_offset);
        }
        self
    }

    /// Run one closing step on the files handed over so far.
    fn run_step<F: Fn() -> u64 + Copy>(
        mut self,
        ctx: &mut PipelineContext<F>,
        node: PipelineNodeRef,
        index: usize,
        total: usize,
        file_offset: usize,
    ) -> Self {
        let files = std::mem::take(&mut self.files);
        match execute_node(ctx, node, files, index, total, file_offset) {
            Ok(result) => {
                let count_keys = ctx.count_keys;
                ctx.node_outputs.add(result.node_outputs, count_keys);
                self.files = result.output_files;
                self
            }
            Err(error) => self.closing_failed(&node.id, error),
        }
    }

    /// Join a closing step's failure to the verdict, after the first one.
    fn closing_failed(self, node_id: &str, error: BntoError) -> Self {
        let (error, files) = split(error);
        let error = BntoError::ClosingStepFailed {
            first: Box::new(self.error),
            node_id: node_id.to_string(),
            error: Box::new(error),
        };
        Self {
            node_id: self.node_id,
            error,
            files,
        }
    }

    /// The verdict, still handing over the chain's last files, for the
    /// closing steps of an enclosing chain.
    pub(super) fn into_error(self) -> BntoError {
        if self.files.is_empty() {
            return self.error;
        }
        BntoError::with_output(self.error, node_output(self.files))
    }

    /// The verdict alone, once no closing step is left anywhere.
    pub(super) fn into_verdict(self) -> BntoError {
        self.error
    }
}

/// An error and the pipeline files it handed over.
fn split(error: BntoError) -> (BntoError, Vec<PipelineFile>) {
    let (error, output) = error.into_parts();
    (error, output.map(pipeline_files).unwrap_or_default())
}
