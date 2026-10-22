// Bridge output files — engine metadata extraction and reading written
// files back for the results screen.

use super::super::screens::results::OutputFile;

/// Per-file metadata extracted from engine results before writing to disk.
#[derive(Debug, Clone)]
pub struct FileResultMeta {
    pub name: String,
    pub original_size: Option<u64>,
}

/// Extract per-file metadata (original size) from the engine's PipelineResult.
/// A size is shown only when a node reported one (`originalSize`), and is
/// then the file's size as it entered the pipeline when known: a middle
/// step's `originalSize` is that step's input, not the user's file.
pub fn extract_file_metadata(result: &bnto_core::pipeline::PipelineResult) -> Vec<FileResultMeta> {
    result
        .files
        .iter()
        .map(|f| {
            let reported = f.metadata.get("originalSize").and_then(|v| v.as_u64());
            let input = f
                .metadata
                .get(bnto_core::INPUT_SIZE_KEY)
                .and_then(|v| v.as_u64());
            let original_size = reported.map(|r| input.unwrap_or(r));
            FileResultMeta {
                name: f.name.clone(),
                original_size,
            }
        })
        .collect()
}

/// Read output files from the bridge's output directory, merging engine
/// metadata. Listed in the order the run produced them (`file_metadata`),
/// since `read_dir` order is arbitrary; files it didn't report go last, by name.
pub fn build_output_files(output_dir: &str, file_metadata: &[FileResultMeta]) -> Vec<OutputFile> {
    let Ok(entries) = std::fs::read_dir(std::path::Path::new(output_dir)) else {
        return Vec::new();
    };
    let mut files: Vec<OutputFile> = entries
        .flatten()
        .map(|entry| output_file(&entry, file_metadata))
        .collect();
    let position = |name: &str| file_metadata.iter().position(|m| m.name == name);
    files.sort_by(|a, b| {
        let key = |f: &OutputFile| (position(&f.name).unwrap_or(usize::MAX), f.name.clone());
        key(a).cmp(&key(b))
    });
    files
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs_are_listed_in_the_order_the_run_produced_them() {
        // read_dir order is arbitrary; the results screen follows input order.
        let dir = tempfile::tempdir().unwrap();
        let names = ["photo3.jpg", "photo1.jpg", "photo2.jpg"];
        for name in names {
            std::fs::write(dir.path().join(name), b"x").unwrap();
        }
        let meta: Vec<FileResultMeta> = names
            .iter()
            .map(|n| FileResultMeta {
                name: n.to_string(),
                original_size: None,
            })
            .collect();
        let listed: Vec<String> = build_output_files(dir.path().to_str().unwrap(), &meta)
            .into_iter()
            .map(|f| f.name)
            .collect();
        assert_eq!(listed, names);
    }
}
