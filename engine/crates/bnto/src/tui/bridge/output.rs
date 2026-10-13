// Bridge output files — engine metadata extraction, progressive writes, and
// reading written files back for the results screen.

use super::super::screens::results::OutputFile;

/// Per-file metadata extracted from engine results before writing to disk.
#[derive(Debug, Clone)]
pub struct FileResultMeta {
    pub name: String,
    pub original_size: Option<u64>,
}

/// Extract per-file metadata (original size) from the engine's PipelineResult.
pub fn extract_file_metadata(result: &bnto_core::pipeline::PipelineResult) -> Vec<FileResultMeta> {
    result
        .files
        .iter()
        .map(|f| {
            let original_size = f.metadata.get("originalSize").and_then(|v| v.as_u64());
            FileResultMeta {
                name: f.name.clone(),
                original_size,
            }
        })
        .collect()
}

/// Write progressive output files from an iteration to the output directory.
/// Called from the TUI event loop when an `IterationCompleted` event
/// carries file data (progressive output mode).
pub fn write_progressive_files(output_dir: &str, files: &[bnto_core::events::ProgressOutputFile]) {
    let dir = std::path::Path::new(output_dir);
    for file in files {
        let path = dir.join(&file.name);
        let _ = file.data.write_to(&path);
    }
}

/// Read output files from the bridge's output directory, merging engine metadata.
pub fn build_output_files(output_dir: &str, file_metadata: &[FileResultMeta]) -> Vec<OutputFile> {
    let Ok(entries) = std::fs::read_dir(std::path::Path::new(output_dir)) else {
        return Vec::new();
    };
    entries
        .flatten()
        .map(|entry| output_file(&entry, file_metadata))
        .collect()
}

/// One results-screen entry, with `original_size` matched by filename.
fn output_file(entry: &std::fs::DirEntry, file_metadata: &[FileResultMeta]) -> OutputFile {
    let name = entry.file_name().to_string_lossy().into_owned();
    let size_bytes = entry.metadata().map(|m| m.len()).unwrap_or(0);
    let original_size = file_metadata
        .iter()
        .find(|m| m.name == name)
        .and_then(|m| m.original_size);
    OutputFile {
        name,
        size_bytes,
        original_size,
    }
}
