// Bridge output files — engine metadata extraction and reading written
// files back for the results screen.

use super::super::screens::results::OutputFile;

/// Per-file metadata extracted from engine results before writing to disk.
/// `name` is the name the file is written under (see `output_names`).
#[derive(Debug, Clone)]
pub struct FileResultMeta {
    pub name: String,
    pub original_size: Option<u64>,
}

/// Extract each result file's written name and original size from the
/// engine's PipelineResult.
pub fn extract_file_metadata(result: &bnto_core::pipeline::PipelineResult) -> Vec<FileResultMeta> {
    let (names, _) = crate::output_names::plan(result.files.iter().map(|f| f.name.as_str()));
    result
        .files
        .iter()
        .zip(names)
        .map(|(file, name)| FileResultMeta {
            name,
            original_size: original_size(file),
        })
        .collect()
}

/// A size is shown only when a node reported one (`originalSize`), and is
/// then the file's size as it entered the pipeline when known: a middle
/// step's `originalSize` is that step's input, not the user's file.
fn original_size(file: &bnto_core::pipeline::PipelineFileResult) -> Option<u64> {
    let reported = file.metadata.get("originalSize").and_then(|v| v.as_u64());
    let input = file
        .metadata
        .get(bnto_core::INPUT_SIZE_KEY)
        .and_then(|v| v.as_u64());
    reported.map(|r| input.unwrap_or(r))
}

/// The files this run wrote to `output_dir`, in the order it produced them,
/// with their sizes on disk. Only the run's own: a recipe's folder can also
/// hold the user's files and earlier runs' results, which aren't this run's.
pub fn build_output_files(output_dir: &str, file_metadata: &[FileResultMeta]) -> Vec<OutputFile> {
    let dir = std::path::Path::new(output_dir);
    file_metadata
        .iter()
        .filter_map(|meta| {
            let on_disk = std::fs::metadata(dir.join(&meta.name)).ok()?;
            on_disk.is_file().then(|| OutputFile {
                name: meta.name.clone(),
                size_bytes: on_disk.len(),
                original_size: meta.original_size,
            })
        })
        .collect()
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

    #[test]
    fn outputs_are_listed_under_the_names_they_were_written_as() {
        // Two results named `logo.svg` are written as `logo.svg` and
        // `logo_1.svg` (output_names); both are this run's results.
        let result = bnto_core::pipeline::PipelineResult {
            files: ["logo.svg", "logo.svg"]
                .map(|name| bnto_core::pipeline::PipelineFileResult {
                    name: name.into(),
                    data: bnto_core::FileData::Bytes(b"<svg/>".to_vec()),
                    mime_type: "image/svg+xml".into(),
                    metadata: serde_json::Map::new(),
                })
                .into(),
            duration_ms: 1,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        };
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().to_str().unwrap();
        crate::io::write_results(&result, out).unwrap();
        let listed: Vec<String> = build_output_files(out, &extract_file_metadata(&result))
            .into_iter()
            .map(|f| f.name)
            .collect();
        assert_eq!(listed, ["logo.svg", "logo_1.svg"]);
    }
}
