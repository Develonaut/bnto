// Editor file I/O — load and save `.bnto.json` files.

use std::path::{Path, PathBuf};

use crate::definition::Definition;

use super::convert::slug_from_name;
use super::types::{EditorError, EditorModel, EditorSource};

impl EditorModel {
    /// Load a recipe from a `.bnto.json` file.
    pub fn load(path: &Path) -> Result<Self, EditorError> {
        if !path.exists() {
            return Err(EditorError::NotFound(path.to_path_buf()));
        }
        let content = std::fs::read_to_string(path)?;
        let def: Definition =
            serde_json::from_str(&content).map_err(|e| EditorError::InvalidJson(e.to_string()))?;
        Ok(Self::from_definition(
            &def,
            EditorSource::File(path.to_path_buf()),
        ))
    }

    /// Save the current editor state to a `.bnto.json` file.
    /// Uses atomic write (write to temp, then rename) to prevent corruption.
    pub fn save_to(&self, path: &Path) -> Result<(), EditorError> {
        let def = self.to_definition();
        let json = serde_json::to_string_pretty(&def)
            .map_err(|e| EditorError::InvalidJson(e.to_string()))?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Atomic write: write to a sibling temp file, then rename.
        let temp_path = path.with_extension("bnto.json.tmp");
        std::fs::write(&temp_path, &json)?;
        std::fs::rename(&temp_path, path)?;

        Ok(())
    }

    /// Resolve the save destination path based on the recipe's source.
    ///
    /// - `File(path)` → save back to that path
    /// - `New` or `Predefined` → save to `recipes_dir/{slug}.bnto.json`
    pub fn save_path(&self, recipes_dir: &Path) -> PathBuf {
        match &self.source {
            EditorSource::File(path) => path.clone(),
            EditorSource::New | EditorSource::Predefined(_) => {
                let slug = slug_from_name(&self.recipe_name);
                let filename = if slug.is_empty() {
                    "untitled".to_string()
                } else {
                    slug
                };
                recipes_dir.join(format!("{filename}.bnto.json"))
            }
        }
    }
}
