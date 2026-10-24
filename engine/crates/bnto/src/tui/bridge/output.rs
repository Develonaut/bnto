// Bridge output files — engine metadata extraction and reading written
// files back for the results screen.

use std::path::{Path, PathBuf};

use bnto_core::pipeline::{PipelineFileResult, PipelineResult};

use super::super::screens::results::OutputFile;

/// Per-file metadata extracted from engine results before writing to disk:
/// where the output mode puts the file and the name it's listed under.
#[derive(Debug, Clone)]
pub struct FileResultMeta {
    pub name: String,
    pub path: PathBuf,
    pub original_size: Option<u64>,
}

/// Each result file the run writes, with where it lands and its original
/// size. `mode` is the output node's: `write` copies into `output_dir`
/// under the names `output_names` keeps apart, `overwrite` renames each file
/// in its own folder, and `message`/`none` write nothing, so list nothing.
pub fn extract_file_metadata(
    result: &PipelineResult,
    mode: &str,
    output_dir: &str,
) -> Vec<FileResultMeta> {
    let landed = landing_places(result, mode, Path::new(output_dir));
    result
        .files
        .iter()
        .zip(landed)
        .filter_map(|(file, landed)| {
            let (name, path) = landed?;
            Some(FileResultMeta {
                name,
                path,
                original_size: original_size(file),
            })
        })
        .collect()
}

/// Per result file, its listed name and path, or `None` if nothing is written.
fn landing_places(
    result: &PipelineResult,
    mode: &str,
    output_dir: &Path,
) -> Vec<Option<(String, PathBuf)>> {
    match mode {
        "write" => {
            let files = result.files.iter().map(|f| (f.name.as_str(), &f.data));
            let (names, _) = crate::output_names::plan(output_dir, files);
            names
                .into_iter()
                .map(|name| Some((name.clone(), output_dir.join(name))))
                .collect()
        }
        "overwrite" => result.files.iter().map(renamed_in_place).collect(),
        _ => vec![None; result.files.len()],
    }
}

/// Where overwrite mode renames `file` to, listed by its file name.
fn renamed_in_place(file: &PipelineFileResult) -> Option<(String, PathBuf)> {
    let dest = crate::overwrite::destination(&file.data, &file.name)?;
    let name = dest.file_name()?.to_string_lossy().into_owned();
    Some((name, dest))
}

/// A size is shown only when a node reported one (`originalSize`), and is
/// then the file's size as it entered the pipeline when known: a middle
/// step's `originalSize` is that step's input, not the user's file.
fn original_size(file: &PipelineFileResult) -> Option<u64> {
    let reported = file.metadata.get("originalSize").and_then(|v| v.as_u64());
    let input = file
        .metadata
        .get(bnto_core::INPUT_SIZE_KEY)
        .and_then(|v| v.as_u64());
    reported.map(|r| input.unwrap_or(r))
}

/// The files this run wrote, in the order it produced them, with their
/// sizes on disk. Only the run's own: a recipe's folder can also hold the
/// user's files and earlier runs' results, which aren't this run's.
pub fn build_output_files(file_metadata: &[FileResultMeta]) -> Vec<OutputFile> {
    file_metadata
        .iter()
        .filter_map(|meta| {
            let on_disk = std::fs::metadata(&meta.path).ok()?;
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
                path: dir.path().join(n),
                original_size: None,
            })
            .collect();
        let listed: Vec<String> = build_output_files(&meta)
            .into_iter()
            .map(|f| f.name)
            .collect();
        assert_eq!(listed, names);
    }

    #[test]
    fn outputs_are_listed_under_the_names_they_were_written_as() {
        // Two results named `logo.svg` are written as `logo.svg` and
        // `logo_1.svg` (output_names); both are this run's results, each
        // with its own original size.
        let sizes = [
            serde_json::json!({}),
            serde_json::json!({ "originalSize": 900 }),
        ];
        let result = PipelineResult {
            files: sizes
                .map(|metadata| PipelineFileResult {
                    name: "logo.svg".into(),
                    data: bnto_core::FileData::Bytes(b"<svg/>".to_vec()),
                    mime_type: "image/svg+xml".into(),
                    metadata: metadata.as_object().unwrap().clone(),
                })
                .into(),
            duration_ms: 1,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        };
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().to_str().unwrap();
        crate::io::write_results(&result, out).unwrap();
        let listed: Vec<(String, Option<u64>)> =
            build_output_files(&extract_file_metadata(&result, "write", out))
                .into_iter()
                .map(|f| (f.name, f.original_size))
                .collect();
        let expected = [
            ("logo.svg".to_string(), None),
            ("logo_1.svg".to_string(), Some(900)),
        ];
        assert_eq!(listed, expected);
    }

    #[test]
    fn a_users_file_at_a_made_up_name_is_kept_and_not_listed() {
        // RED (data loss): the second `logo.svg` went to `logo_1.svg`, the
        // user's own file. The bridge lists before it writes, as here; both
        // must pick `logo_2.svg`, and the user's file stays out of the list.
        let dir = tempfile::tempdir().unwrap();
        let mine = dir.path().join("logo_1.svg");
        std::fs::write(&mine, b"mine").unwrap();
        let result = PipelineResult {
            files: ["<svg/>", "<svg></svg>"]
                .map(|body| PipelineFileResult {
                    name: "logo.svg".into(),
                    data: bnto_core::FileData::Bytes(body.into()),
                    mime_type: "image/svg+xml".into(),
                    metadata: serde_json::Map::new(),
                })
                .into(),
            duration_ms: 1,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        };
        let out = dir.path().to_str().unwrap();
        let meta = extract_file_metadata(&result, "write", out);
        crate::io::write_results(&result, out).unwrap();

        let listed: Vec<(String, u64)> = build_output_files(&meta)
            .into_iter()
            .map(|f| (f.name, f.size_bytes))
            .collect();
        assert_eq!(listed, [("logo.svg".into(), 6), ("logo_2.svg".into(), 11)]);
        assert_eq!(std::fs::read(&mine).unwrap(), b"mine");
    }
}
