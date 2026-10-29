// Collect files a command wrote into its output directory as path
// references (never read into memory), with size limits and MIME types.

use std::path::{Path, PathBuf};

use bnto_core::BntoError;
use bnto_core::processor::{FileData, OutputFile};

use crate::validate;

/// Collect all files from a directory (recursively) as OutputFile entries.
///
/// Commands like yt-dlp create subdirectories in the output dir, so we
/// walk the entire tree to find all produced files. Filenames preserve
/// relative paths from the root dir (e.g. `"subdir/video.mp4"`).
///
/// Returns `FileData::Path` references — files are NOT read into memory.
/// Consumers move them to the final destination via `write_to()` (rename).
pub(crate) fn collect_output_files(
    dir: &Path,
    max_output_mb: u64,
) -> Result<Vec<OutputFile>, BntoError> {
    let mut files = Vec::new();
    collect_output_files_recursive(dir, dir, max_output_mb, &mut files)?;
    Ok(files)
}

/// Walk `dir`, appending every file found (depth-first, read_dir order).
fn collect_output_files_recursive(
    root_dir: &Path,
    dir: &Path,
    max_output_mb: u64,
    files: &mut Vec<OutputFile>,
) -> Result<(), BntoError> {
    let entries = std::fs::read_dir(dir).map_err(|e| {
        BntoError::ProcessingFailed(format!("Failed to read output directory: {e}"))
    })?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_output_files_recursive(root_dir, &path, max_output_mb, files)?;
        } else if path.is_file() {
            files.push(output_file(root_dir, path, max_output_mb)?);
        }
    }
    Ok(())
}

/// A path-referenced output file, rejected when over the size limit.
fn output_file(
    root_dir: &Path,
    path: PathBuf,
    max_output_mb: u64,
) -> Result<OutputFile, BntoError> {
    let filename = relative_name(root_dir, &path);
    check_file_size(&path, &filename, max_output_mb)?;
    // Return a path reference — file stays on disk, not read into memory.
    let mime = mime_from_extension(&filename);
    Ok(OutputFile {
        data: FileData::Path(path),
        filename,
        mime_type: mime,
        metadata: serde_json::Map::new(),
    })
}

/// Path relative to `root_dir` so subdirectory structure is retained.
fn relative_name(root_dir: &Path, path: &Path) -> String {
    path.strip_prefix(root_dir)
        .map(bnto_core::slash_path)
        .unwrap_or_else(|_| {
            path.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "output".to_string())
        })
}

/// Check file size via metadata — validates without loading into memory.
fn check_file_size(path: &Path, filename: &str, max_output_mb: u64) -> Result<(), BntoError> {
    let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let limit = validate::max_output_bytes(max_output_mb) as u64;
    if file_size > limit {
        return Err(BntoError::ProcessingFailed(format!(
            "Output file '{}' is {} MB, exceeding the {} MB limit",
            filename,
            file_size / (1024 * 1024),
            max_output_mb
        )));
    }
    Ok(())
}

/// Derive MIME type from file extension for common media formats.
pub(crate) fn mime_from_extension(filename: &str) -> String {
    let ext = bnto_core::file_extension(filename).to_lowercase();

    match ext.as_str() {
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mkv" => "video/x-matroska",
        "mp3" => "audio/mpeg",
        "m4a" => "audio/mp4",
        "ogg" | "opus" => "audio/ogg",
        "wav" => "audio/wav",
        "flac" => "audio/flac",
        "json" => "application/json",
        "txt" | "log" => "text/plain",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_dotfile_has_no_extension_to_guess_from() {
        // ISS-33: `clips/.mp4` used to read as an MP4.
        assert_eq!(
            mime_from_extension("clips/.mp4"),
            mime_from_extension("clips/data")
        );
        assert_eq!(mime_from_extension("v1.2/clip.mp4"), "video/mp4");
    }
}
