// Preview screen — shows file transformation previews before committing to execution.
//
// TEA pattern: PreviewModel (state) + PreviewMessage (events) + update() (pure transitions).
// Reuses `FilePreview` from the dry-run module for the before/after data.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::dry_run::files::FilePreview;

/// Preview screen state — displays file transformations and holds
/// inputs needed to re-run the pipeline on confirm.
#[derive(Debug)]
pub struct PreviewModel {
    pub slug: String,
    pub entries: Vec<FilePreview>,
    pub cursor: usize,
    pub duration_ms: u64,
    pub warnings: Vec<String>,
    /// Carried forward from execution so confirm can re-run with writes.
    pub selected_files: Vec<PathBuf>,
    pub param_overrides: HashMap<String, String>,
    pub definition_json: String,
}

/// Messages the preview screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreviewMessage {
    CursorDown,
    CursorUp,
}

impl PreviewModel {
    /// Create a preview model from pipeline execution results.
    pub fn new(
        slug: &str,
        entries: Vec<FilePreview>,
        duration_ms: u64,
        warnings: Vec<String>,
        selected_files: Vec<PathBuf>,
        param_overrides: HashMap<String, String>,
        definition_json: String,
    ) -> Self {
        Self {
            slug: slug.to_string(),
            entries,
            cursor: 0,
            duration_ms,
            warnings,
            selected_files,
            param_overrides,
            definition_json,
        }
    }
}

/// Pure state transition for the preview screen.
pub fn update(mut model: PreviewModel, msg: PreviewMessage) -> PreviewModel {
    match msg {
        PreviewMessage::CursorDown => {
            if !model.entries.is_empty() {
                model.cursor = (model.cursor + 1) % model.entries.len();
            }
        }
        PreviewMessage::CursorUp => {
            if !model.entries.is_empty() {
                model.cursor = if model.cursor == 0 {
                    model.entries.len() - 1
                } else {
                    model.cursor - 1
                };
            }
        }
    }
    model
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<FilePreview> {
        vec![
            FilePreview {
                original: "VIDEO： Movie.mp4".into(),
                result: "Movie.mp4".into(),
            },
            FilePreview {
                original: "VIDEO： Concert.mp4".into(),
                result: "Concert.mp4".into(),
            },
            FilePreview {
                original: "VIDEO： Clip.mp4".into(),
                result: "Clip.mp4".into(),
            },
        ]
    }

    fn make_model(entries: Vec<FilePreview>) -> PreviewModel {
        PreviewModel::new(
            "strip-video-prefix",
            entries,
            245,
            vec![],
            vec![],
            HashMap::new(),
            String::new(),
        )
    }

    #[test]
    fn new_creates_model_with_entries() {
        let m = make_model(sample_entries());
        assert_eq!(m.entries.len(), 3);
        assert_eq!(m.cursor, 0);
        assert_eq!(m.duration_ms, 245);
        assert_eq!(m.slug, "strip-video-prefix");
    }

    #[test]
    fn cursor_down_advances() {
        let m = make_model(sample_entries());
        let m = update(m, PreviewMessage::CursorDown);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_down_wraps_at_end() {
        let mut m = make_model(sample_entries());
        m.cursor = 2;
        let m = update(m, PreviewMessage::CursorDown);
        assert_eq!(m.cursor, 0);
    }

    #[test]
    fn cursor_up_moves_back() {
        let mut m = make_model(sample_entries());
        m.cursor = 2;
        let m = update(m, PreviewMessage::CursorUp);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_up_wraps_at_start() {
        let m = make_model(sample_entries());
        let m = update(m, PreviewMessage::CursorUp);
        assert_eq!(m.cursor, 2);
    }

    #[test]
    fn empty_entries_cursor_stays_at_zero() {
        let m = make_model(vec![]);
        let m = update(m, PreviewMessage::CursorDown);
        assert_eq!(m.cursor, 0);
        let m = update(m, PreviewMessage::CursorUp);
        assert_eq!(m.cursor, 0);
    }

    #[test]
    fn preserves_execution_inputs_for_confirm() {
        let files = vec![PathBuf::from("/tmp/test.mp4")];
        let mut overrides = HashMap::new();
        overrides.insert("node:key".into(), "value".into());
        let def = r#"{"nodes":[]}"#.to_string();

        let m = PreviewModel::new(
            "test",
            sample_entries(),
            100,
            vec!["warning".into()],
            files.clone(),
            overrides.clone(),
            def.clone(),
        );

        assert_eq!(m.selected_files, files);
        assert_eq!(m.param_overrides, overrides);
        assert_eq!(m.definition_json, def);
        assert_eq!(m.warnings, vec!["warning"]);
    }
}
