// File I/O helpers — read input files and write pipeline results.

use std::path::Path;

use bnto_core::{PipelineFile, PipelineResult};

/// Read a file from disk into a PipelineFile.
pub fn read_pipeline_file(path: &str) -> Result<PipelineFile, String> {
    let data = std::fs::read(path).map_err(|e| format!("{e}"))?;
    let name = Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string());
    let mime_type = guess_mime(path);

    Ok(PipelineFile {
        name,
        data,
        mime_type,
        metadata: serde_json::Map::new(),
    })
}

/// Write all result files to an output directory.
pub fn write_results(result: &PipelineResult, output_dir: &str) -> Result<(), String> {
    std::fs::create_dir_all(output_dir).map_err(|e| format!("Cannot create {output_dir}: {e}"))?;

    for file in &result.files {
        let out_path = Path::new(output_dir).join(&file.name);
        std::fs::write(&out_path, &file.data)
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
}
