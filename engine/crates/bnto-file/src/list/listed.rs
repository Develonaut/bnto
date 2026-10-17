// One listed path: its name under the base, its full path, and whether it exists.

use std::path::{MAIN_SEPARATOR, Path, PathBuf};

use bnto_core::processor::{FileData, OutputFile};
use serde_json::{Map, Value, json};

/// A path from the list, checked against the base folder.
pub(super) struct Listed {
    /// The relative name with `/` separators, as listed.
    pub name: String,
    full: PathBuf,
    pub found: bool,
}

impl Listed {
    /// `name` under `base`, and whether anything is there.
    pub(super) fn check(base: &Path, name: String) -> Self {
        let full = base.join(&name);
        Self {
            found: full.exists(),
            full,
            name,
        }
    }

    /// The node metadata entry: `path`, `full_path` and `status`.
    pub(super) fn entry(&self) -> Value {
        let status = if self.found { "found" } else { "missing" };
        json!({ "path": self.name, "full_path": self.full_path(), "status": status })
    }

    /// A path reference to the listed file, its full path in its metadata.
    pub(super) fn into_output(self) -> OutputFile {
        let mut metadata = Map::new();
        metadata.insert("full_path".to_string(), self.full_path().into());
        OutputFile {
            data: FileData::Path(self.full),
            mime_type: "application/octet-stream".to_string(),
            filename: self.name,
            metadata,
        }
    }

    /// The full path as text with `/` separators, the same on every OS. Unlike
    /// `bnto_core::slash_path` it keeps the root and drive, and only Windows
    /// separators change, since `\` is an ordinary name character elsewhere.
    fn full_path(&self) -> String {
        let text = self.full.to_string_lossy();
        if MAIN_SEPARATOR == '\\' {
            text.replace('\\', "/")
        } else {
            text.into_owned()
        }
    }
}
