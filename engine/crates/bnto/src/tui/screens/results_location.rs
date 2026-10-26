// Where a run's results are: the label over the file list, each row's name,
// and what Open and Open folder open. A run that writes into a folder has
// `output_dir`; one that wrote in place (overwrite mode) leaves each file in
// its own folder, so those come from the files' paths (ISS-43, ISS-46).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::results::{OutputFile, ResultsModel};

impl ResultsModel {
    /// The line above the files: the output folder, or for files written in
    /// place, their one folder or how many folders they're in.
    pub fn location_line(&self) -> Option<String> {
        if let Some(dir) = &self.output_dir {
            return Some(format!("Output: {dir}"));
        }
        let folders = distinct_folders(&self.outputs);
        match folders.as_slice() {
            [] => None,
            [folder] => Some(format!("Written in place: {}", folder.display())),
            many => Some(format!("Written in place, in {} folders", many.len())),
        }
    }

    /// Each row's name: the file name, or with its folder's name when another
    /// row has the same name.
    pub fn row_names(&self) -> Vec<String> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for file in &self.outputs {
            *counts.entry(file.name.as_str()).or_default() += 1;
        }
        let row_name = |file: &OutputFile| match counts[file.name.as_str()] {
            1 => file.name.clone(),
            _ => with_folder(file),
        };
        self.outputs.iter().map(row_name).collect()
    }

    /// The selected row's file.
    pub fn selected_path(&self) -> Option<&Path> {
        self.outputs
            .get(self.cursor)
            .map(|file| file.path.as_path())
    }

    /// The folder Open folder opens: the output folder, else the selected
    /// file's own folder.
    pub fn folder_to_open(&self) -> Option<PathBuf> {
        match &self.output_dir {
            Some(dir) => Some(PathBuf::from(dir)),
            None => self.selected_path()?.parent().map(Path::to_path_buf),
        }
    }
}

/// The folders the files are in, in first-seen order.
fn distinct_folders(outputs: &[OutputFile]) -> Vec<&Path> {
    let mut folders: Vec<&Path> = Vec::new();
    for folder in outputs.iter().filter_map(|file| file.path.parent()) {
        if !folders.contains(&folder) {
            folders.push(folder);
        }
    }
    folders
}

/// `<folder name>/<file name>`, or the full path when the folder has no name.
fn with_folder(file: &OutputFile) -> String {
    match file.path.parent().and_then(Path::file_name) {
        Some(folder) => format!("{}/{}", folder.to_string_lossy(), file.name),
        None => file.path.display().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(path: &str) -> OutputFile {
        let path = PathBuf::from(path);
        OutputFile {
            name: path.file_name().unwrap().to_string_lossy().into_owned(),
            path,
            size_bytes: 1,
            original_size: None,
        }
    }

    fn results(paths: &[&str], output_dir: Option<&str>) -> ResultsModel {
        let outputs = paths.iter().map(|p| file(p)).collect();
        ResultsModel::new("s", outputs, 0, output_dir.map(str::to_string))
    }

    #[test]
    fn a_run_into_a_folder_names_it() {
        let r = results(&["/out/a.txt"], Some("/out"));
        assert_eq!(r.location_line().as_deref(), Some("Output: /out"));
    }

    #[test]
    fn files_written_in_place_in_one_folder_name_it() {
        let r = results(&["/photos/a.jpg", "/photos/b.jpg"], None);
        let line = r.location_line().unwrap();
        assert_eq!(
            line,
            format!("Written in place: {}", Path::new("/photos").display())
        );
    }

    #[test]
    fn files_written_in_place_in_several_folders_count_them() {
        let r = results(&["/a/x.txt", "/b/y.txt", "/a/z.txt"], None);
        assert_eq!(
            r.location_line().as_deref(),
            Some("Written in place, in 2 folders")
        );
    }

    #[test]
    fn no_files_and_no_folder_is_no_line() {
        assert_eq!(results(&[], None).location_line(), None);
    }

    #[test]
    fn a_repeated_name_shows_its_folder() {
        let r = results(&["/a/renamed.txt", "/b/renamed.txt", "/b/other.txt"], None);
        assert_eq!(
            r.row_names(),
            ["a/renamed.txt", "b/renamed.txt", "other.txt"]
        );
    }

    #[test]
    fn open_opens_the_selected_rows_own_file() {
        let mut r = results(&["/a/renamed.txt", "/b/renamed.txt"], None);
        r.cursor = 1;
        assert_eq!(r.selected_path(), Some(Path::new("/b/renamed.txt")));
        assert_eq!(r.folder_to_open(), Some(PathBuf::from("/b")));
    }

    #[test]
    fn open_folder_opens_the_output_folder_when_there_is_one() {
        let r = results(&["/out/sub/a.txt"], Some("/out"));
        assert_eq!(r.folder_to_open(), Some(PathBuf::from("/out")));
    }
}
