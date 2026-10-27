// file-move — Move each file to its name's path under a destination folder.
//
// CLI-only batch node. A file named `Pistol/Slide/shot.wav` lands at
// `<destination>/Pistol/Slide/shot.wav`, with missing folders created. Every
// move is planned and checked before any runs (shared with `output`
// overwrite mode, see `crate::relocate`): one bad name, or one move that
// would replace a file, refuses the whole run, on a dry run too. Moves are
// renames only, so a file on another drive is refused, never copied. Once the
// run's time limit passes, no more moves start (ISS-60).

mod params;
mod plan;
mod root;
mod stopped;

use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
use bnto_core::processor::{
    BatchFile, BatchInput, FileData, NodeInput, NodeOutput, NodeProcessor, OutputFile,
};
use bnto_core::progress::ProgressReporter;
use bnto_core::{BntoError, ProcessContext};

use crate::held_sources::{HINT, held_sources};
use crate::relocate::{self, Move};
use params::{MoveParams, build_move_parameters};
use plan::{Planned, refused};
use root::Root;

/// File-move processor — rename files into a folder tree, never copying.
pub struct FileMove;

impl FileMove {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileMove {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for FileMove {
    fn name(&self) -> &str {
        "file-move"
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "file-move".to_string(),
            name: "Move Files".to_string(),
            description: "Move files into folders under a destination, by the path in each \
                          file's name; refuses the whole run if any move would replace a file."
                .to_string(),
            category: NodeCategory::File,
            accepts: vec![],
            platforms: vec!["cli".to_string(), "desktop".to_string()],
            parameters: build_move_parameters(),
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        MoveParams::read(params).err().into_iter().collect()
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

    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let params = MoveParams::read(&input.params).map_err(BntoError::InvalidInput)?;
        let mut root = Root::resolve(&params.destination, params.create_dirs)
            .map_err(|e| BntoError::InvalidInput(refused(&e)))?;
        let planned = plan::plan(&input.files, &mut root).map_err(BntoError::InvalidInput)?;
        let moves = real_moves(&planned.moves);
        refuse_conflicts(&moves)?;
        refuse_held(&moves, ctx)?;
        let moved = moves.len();
        if !ctx.dry_run()
            && let Err(stopped) = relocate::run_checked(moves, &|| ctx.check_time_limit())
        {
            return Err(stopped::handed_over(stopped, input.files, planned));
        }
        progress.report_output(&summary(moved, input.files.len() - moved, &root, ctx));
        Ok(outputs(input.files, planned, ctx.dry_run()))
    }
}

/// Refuse the whole batch when a move would replace a file.
fn refuse_conflicts(moves: &[Move]) -> Result<(), BntoError> {
    let conflicts = relocate::conflicts(moves);
    if conflicts.is_empty() {
        return Ok(());
    }
    let text = relocate::listing(&conflicts, ("conflict", "conflicts"));
    Err(BntoError::InvalidInput(refused(&text)))
}

/// Refuse the batch when a later step still reads a file it would move.
fn refuse_held(moves: &[Move], ctx: &dyn ProcessContext) -> Result<(), BntoError> {
    let held = held_sources(moves.iter().map(|m| m.src.as_path()), "moved", ctx);
    if held.is_empty() {
        return Ok(());
    }
    let text = relocate::listing(&held, ("file", "files"));
    Err(BntoError::InvalidInput(format!(
        "{}\n  {HINT}",
        refused(&text)
    )))
}

/// The moves that change a path (a file already in place stays put).
fn real_moves(moves: &[Move]) -> Vec<Move> {
    moves.iter().filter(|m| m.src != m.dest).cloned().collect()
}

fn summary(moved: usize, in_place: usize, root: &Root, ctx: &dyn ProcessContext) -> String {
    let verb = if ctx.dry_run() { "Would move" } else { "Moved" };
    let files = if moved == 1 { "file" } else { "files" };
    let mut text = format!("{verb} {moved} {files} under {}", root.path.display());
    if in_place > 0 {
        text.push_str(&format!(", {in_place} already in place"));
    }
    text
}

/// Each file at its new path, named by its path under the root. A dry run
/// moved nothing, so its files still point at their sources.
#[expect(clippy::too_many_lines)]
fn outputs(files: Vec<BatchFile>, planned: Planned, dry_run: bool) -> NodeOutput {
    let files = files
        .into_iter()
        .zip(planned.moves)
        .zip(planned.names)
        .map(|((f, m), name)| OutputFile {
            data: if dry_run {
                f.data
            } else {
                FileData::Path(m.dest)
            },
            filename: name,
            mime_type: f
                .mime_type
                .unwrap_or_else(|| "application/octet-stream".to_string()),
            metadata: f.metadata,
        })
        .collect();
    NodeOutput {
        files,
        metadata: serde_json::Map::new(),
    }
}

#[cfg(test)]
mod failed_tests;
#[cfg(test)]
mod held_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod time_tests;
