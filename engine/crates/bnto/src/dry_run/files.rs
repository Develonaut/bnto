// Pipeline execution preview — run without writing, extract file transformations.
//
// For file-processing recipes (rename, collect, convert), the only way to
// preview what will happen is to actually run the pipeline. This module
// runs the pipeline but skips `write_results()`, then extracts before/after
// filenames from the engine's metadata.
//
// Memory optimization: after `prepare_inputs` resolves files, we replace
// file data with empty bytes. Rename-only pipelines don't inspect content,
// so this avoids loading large files into memory. Recipes with file-collect
// (directory input) already pass empty bytes for the directory entry.
//
// The preview runs under the recipe's time limit, watched like `bnto run`.

use bnto_core::BntoError;
use bnto_core::events::PipelineReporter;
use bnto_core::pipeline::PipelineResult;
use bnto_core::processor::FileData;

use crate::context::NativeContext;
use crate::run_watchdog::{self, Watched};
use crate::{input, run_limit};

/// A single file transformation preview: original name → result name.
#[derive(Debug, Clone, PartialEq)]
pub struct FilePreview {
    pub original: String,
    pub result: String,
}

/// Run a pipeline in preview mode — stub file data, skip writes.
///
/// Uses `prepare_inputs` to resolve the definition and file list, then
/// replaces file data with empty bytes before execution. The pipeline
/// runs without writing output, so no disk space is consumed.
///
/// For rename-only pipelines, this is safe because rename processors
/// only inspect filenames, not content. For recipes with file-collect,
/// the directory input already has empty bytes; file-collect walks the
/// directory itself.
pub fn run_preview(
    definition_json: &str,
    inputs: &[String],
    param_overrides: &[String],
) -> Result<PipelineResult, String> {
    let prepared = input::prepare_inputs(definition_json, inputs, param_overrides)?;

    // Replace file data with empty bytes to minimize memory usage.
    // Safe for rename pipelines — they only modify filenames, not content.
    let stub_files = prepared
        .files
        .into_iter()
        .map(|f| bnto_core::pipeline::PipelineFile {
            data: FileData::Bytes(vec![]),
            ..f
        })
        .collect();

    // A preview never writes: file-copy decides but copies nothing.
    let ctx = NativeContext::current_dir()
        .map_err(|e| format!("{e}"))?
        .with_dry_run(true);
    let reporter = PipelineReporter::new_noop();
    let (json, time_limit) = run_limit::apply(&prepared.definition_json, None)?;
    let run = || bnto_engine::run_pipeline(&json, stub_files, &reporter, &ctx);
    match run_watchdog::watch(ctx.registry(), time_limit, end_stuck_preview, run) {
        Watched::Finished(result) => result.map_err(|e| format!("{e}")),
        Watched::Ended => run_watchdog::wait_for_exit(),
    }
}

/// A preview stuck past its time limit can't finish printing, so report the
/// timeout as its failure, delete its temp files and exit 1.
fn end_stuck_preview(error: BntoError) {
    eprintln!("Warning: pipeline preview failed: {error}");
    crate::scratch::remove_live_dirs();
    std::process::exit(1);
}

/// Extract file previews from a completed pipeline result.
///
/// The original name comes from `originalFilename` metadata set by the rename
/// processor; the result is always the file's final name. Metadata accumulates
/// across nodes, so a rename's `newFilename` can be stale after a later node
/// renames the file again (e.g. image-convert changing the extension).
pub fn extract_previews(result: &PipelineResult) -> Vec<FilePreview> {
    result
        .files
        .iter()
        .map(|f| {
            let original = f
                .metadata
                .get("originalFilename")
                .and_then(|v| v.as_str())
                .unwrap_or(&f.name)
                .to_string();

            FilePreview {
                original,
                result: f.name.clone(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_result(
        files: Vec<bnto_core::pipeline::PipelineFileResult>,
    ) -> bnto_core::pipeline::PipelineResult {
        bnto_core::pipeline::PipelineResult {
            files,
            duration_ms: 100,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        }
    }

    #[test]
    fn extract_previews_uses_the_final_name_over_a_stale_new_filename() {
        // file-rename → image-convert: rename's newFilename survives, but the
        // file's final name has the converted extension.
        let mut metadata = serde_json::Map::new();
        metadata.insert("originalFilename".into(), "IMG_1.png".into());
        metadata.insert("newFilename".into(), "vacation.png".into());
        let result = make_result(vec![bnto_core::pipeline::PipelineFileResult {
            name: "vacation.webp".into(),
            data: bnto_core::processor::FileData::Bytes(vec![]),
            mime_type: "image/webp".into(),
            metadata,
        }]);
        let previews = extract_previews(&result);
        assert_eq!(previews[0].original, "IMG_1.png");
        assert_eq!(previews[0].result, "vacation.webp");
    }

    #[test]
    fn extract_previews_reads_rename_metadata() {
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "originalFilename".into(),
            serde_json::Value::String("VIDEO： Movie.mp4".into()),
        );
        metadata.insert(
            "newFilename".into(),
            serde_json::Value::String("Movie.mp4".into()),
        );

        let result = make_result(vec![bnto_core::pipeline::PipelineFileResult {
            name: "Movie.mp4".into(),
            data: FileData::Bytes(vec![]),
            mime_type: "video/mp4".into(),
            metadata,
        }]);

        let previews = extract_previews(&result);
        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].original, "VIDEO： Movie.mp4");
        assert_eq!(previews[0].result, "Movie.mp4");
    }

    #[test]
    fn extract_previews_handles_missing_metadata() {
        let result = make_result(vec![bnto_core::pipeline::PipelineFileResult {
            name: "photo.jpg".into(),
            data: FileData::Bytes(vec![]),
            mime_type: "image/jpeg".into(),
            metadata: serde_json::Map::new(),
        }]);

        let previews = extract_previews(&result);
        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].original, "photo.jpg");
        assert_eq!(previews[0].result, "photo.jpg");
    }

    #[test]
    fn extract_previews_multiple_files() {
        let mut meta1 = serde_json::Map::new();
        meta1.insert(
            "originalFilename".into(),
            serde_json::Value::String("VIDEO： A.mp4".into()),
        );
        meta1.insert(
            "newFilename".into(),
            serde_json::Value::String("A.mp4".into()),
        );

        let mut meta2 = serde_json::Map::new();
        meta2.insert(
            "originalFilename".into(),
            serde_json::Value::String("VIDEO： B.mp4".into()),
        );
        meta2.insert(
            "newFilename".into(),
            serde_json::Value::String("B.mp4".into()),
        );

        let result = make_result(vec![
            bnto_core::pipeline::PipelineFileResult {
                name: "A.mp4".into(),
                data: FileData::Bytes(vec![]),
                mime_type: "video/mp4".into(),
                metadata: meta1,
            },
            bnto_core::pipeline::PipelineFileResult {
                name: "B.mp4".into(),
                data: FileData::Bytes(vec![]),
                mime_type: "video/mp4".into(),
                metadata: meta2,
            },
        ]);

        let previews = extract_previews(&result);
        assert_eq!(previews.len(), 2);
        assert_eq!(previews[0].original, "VIDEO： A.mp4");
        assert_eq!(previews[1].original, "VIDEO： B.mp4");
    }
}
