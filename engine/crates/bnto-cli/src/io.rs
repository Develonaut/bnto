// Disk I/O adapter — reads definition + input files, writes output files.

use std::fs;
use std::path::Path;

use bnto_core::{PipelineDefinition, PipelineFile};

/// Read a `.bnto.json` file and deserialize it into a PipelineDefinition.
pub fn read_definition(path: &Path) -> Result<PipelineDefinition, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Invalid definition in {}: {}", path.display(), e))
}

/// Read input files from disk into PipelineFile structs.
pub fn read_input_files(paths: &[std::path::PathBuf]) -> Result<Vec<PipelineFile>, String> {
    paths.iter().map(|p| read_single_file(p)).collect()
}

fn read_single_file(path: &Path) -> Result<PipelineFile, String> {
    let data = fs::read(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let mime_type = mime_from_extension(path);

    Ok(PipelineFile {
        name,
        data,
        mime_type,
        metadata: serde_json::Map::new(),
    })
}

/// Write pipeline output files to a directory.
pub fn write_output_files(
    files: &[bnto_core::PipelineFileResult],
    output_dir: &Path,
) -> Result<(), String> {
    fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create {}: {}", output_dir.display(), e))?;

    for file in files {
        let out_path = output_dir.join(&file.name);
        fs::write(&out_path, &file.data)
            .map_err(|e| format!("Failed to write {}: {}", out_path.display(), e))?;
    }

    Ok(())
}

/// Guess MIME type from file extension.
fn mime_from_extension(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        Some("csv") => "text/csv",
        Some("json") => "application/json",
        Some("txt") => "text/plain",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mime_detection_common_types() {
        assert_eq!(mime_from_extension(Path::new("photo.jpg")), "image/jpeg");
        assert_eq!(mime_from_extension(Path::new("photo.jpeg")), "image/jpeg");
        assert_eq!(mime_from_extension(Path::new("image.png")), "image/png");
        assert_eq!(mime_from_extension(Path::new("image.webp")), "image/webp");
        assert_eq!(mime_from_extension(Path::new("data.csv")), "text/csv");
        assert_eq!(
            mime_from_extension(Path::new("unknown.xyz")),
            "application/octet-stream"
        );
    }
}
