// File I/O helpers — read input files and write pipeline results.

use std::path::Path;

use bnto_core::{PipelineFile, PipelineResult};

/// Read a file (or directory) from disk into a PipelineFile.
///
/// For regular files, reads bytes into memory. For directories, creates a
/// synthetic entry with the absolute path as the name and empty data —
/// processors like `file-collect` use the filename as the directory to traverse.
pub fn read_pipeline_file(path: &str) -> Result<PipelineFile, String> {
    let p = Path::new(path);

    if p.is_dir() {
        let abs = p
            .canonicalize()
            .map_err(|e| format!("{e}"))?
            .to_string_lossy()
            .to_string();
        return Ok(PipelineFile {
            name: abs,
            data: bnto_core::processor::FileData::Bytes(vec![]),
            mime_type: "inode/directory".to_string(),
            metadata: serde_json::Map::new(),
        });
    }

    let data = std::fs::read(path).map_err(|e| format!("{e}"))?;
    let name = p
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string());
    let mime_type = guess_mime(path);

    Ok(PipelineFile {
        name,
        data: bnto_core::processor::FileData::Bytes(data),
        mime_type,
        metadata: serde_json::Map::new(),
    })
}

/// Write all result files to an output directory.
///
/// File names may contain subdirectory separators (e.g. `"subdir/video.mp4"`).
/// Parent directories are created automatically before writing.
///
/// Uses `FileData::write_to()` which renames (O(1)) for path-referenced files
/// and writes bytes for in-memory files.
pub fn write_results(result: &PipelineResult, output_dir: &str) -> Result<(), String> {
    std::fs::create_dir_all(output_dir).map_err(|e| format!("Cannot create {output_dir}: {e}"))?;

    for file in &result.files {
        let out_path = Path::new(output_dir).join(&file.name);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create {}: {e}", parent.display()))?;
        }
        file.data
            .write_to(&out_path)
            .map_err(|e| format!("Cannot write {}: {e}", out_path.display()))?;
    }

    Ok(())
}

/// Guess MIME type from file extension.
fn guess_mime(path: &str) -> String {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "csv" => "text/csv",
        "json" => "application/json",
        "txt" => "text/plain",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_pipeline_file_directory() {
        let dir = std::env::temp_dir().join("bnto-test-dir-input");
        let _ = std::fs::create_dir_all(&dir);

        let pf = read_pipeline_file(dir.to_str().unwrap()).unwrap();
        assert!(pf.name.contains("bnto-test-dir-input"));
        assert_eq!(pf.mime_type, "inode/directory");
        assert!(
            matches!(&pf.data, bnto_core::processor::FileData::Bytes(b) if b.is_empty()),
            "Directory input should have empty bytes"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_guess_mime_jpeg() {
        assert_eq!(guess_mime("photo.jpg"), "image/jpeg");
        assert_eq!(guess_mime("photo.jpeg"), "image/jpeg");
        assert_eq!(guess_mime("photo.JPG"), "image/jpeg");
    }

    #[test]
    fn test_guess_mime_png() {
        assert_eq!(guess_mime("image.png"), "image/png");
    }

    #[test]
    fn test_guess_mime_csv() {
        assert_eq!(guess_mime("data.csv"), "text/csv");
    }

    #[test]
    fn test_guess_mime_unknown() {
        assert_eq!(guess_mime("file.xyz"), "application/octet-stream");
        assert_eq!(guess_mime("noext"), "application/octet-stream");
    }

    #[test]
    fn test_write_results_creates_subdirectories() {
        use bnto_core::PipelineFileResult;
        use bnto_core::processor::FileData;

        let dir = std::env::temp_dir().join("bnto-test-write-subdirs");
        let _ = std::fs::remove_dir_all(&dir);

        let result = PipelineResult {
            files: vec![
                PipelineFileResult {
                    name: "top.txt".to_string(),
                    data: FileData::Bytes(b"top-level".to_vec()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                },
                PipelineFileResult {
                    name: "group/nested.mp4".to_string(),
                    data: FileData::Bytes(b"video-data".to_vec()),
                    mime_type: "video/mp4".to_string(),
                    metadata: serde_json::Map::new(),
                },
                PipelineFileResult {
                    name: "a/b/deep.txt".to_string(),
                    data: FileData::Bytes(b"deep-data".to_vec()),
                    mime_type: "text/plain".to_string(),
                    metadata: serde_json::Map::new(),
                },
            ],
            duration_ms: 0,
            warnings: vec![],
        };

        write_results(&result, dir.to_str().unwrap()).unwrap();

        assert!(dir.join("top.txt").exists());
        assert!(dir.join("group").join("nested.mp4").exists());
        assert!(dir.join("a").join("b").join("deep.txt").exists());
        assert_eq!(
            std::fs::read(dir.join("group/nested.mp4")).unwrap(),
            b"video-data"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
