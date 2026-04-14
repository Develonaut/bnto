// Results screen — shows execution output: files, sizes, timing, savings.
//
// TEA pattern: ResultsModel (state) + ResultsMessage (events) + update() (pure transitions).

/// An output file with size and optional original size for savings calculation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputFile {
    pub name: String,
    pub size_bytes: u64,
    pub original_size: Option<u64>,
}

/// Aggregate size savings across all files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SizeSavings {
    pub before: u64,
    pub after: u64,
}

impl SizeSavings {
    /// Savings as a percentage (0-100). Returns 0 if before is zero or output grew.
    pub fn percent(&self) -> u64 {
        if self.before == 0 || self.after >= self.before {
            return 0;
        }
        ((self.before - self.after) * 100) / self.before
    }
}

/// Results screen state.
#[derive(Debug)]
pub struct ResultsModel {
    pub slug: String,
    pub outputs: Vec<OutputFile>,
    pub total_time_ms: u64,
    pub savings: Option<SizeSavings>,
    pub cursor: usize,
    pub output_dir: Option<String>,
}

/// Messages the results screen can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultsMessage {
    CursorDown,
    CursorUp,
}

impl ResultsModel {
    /// Create a results model from pipeline output.
    pub fn new(
        slug: &str,
        outputs: Vec<OutputFile>,
        total_time_ms: u64,
        output_dir: Option<String>,
    ) -> Self {
        let savings = compute_savings(&outputs);
        Self {
            slug: slug.to_string(),
            outputs,
            total_time_ms,
            savings,
            cursor: 0,
            output_dir,
        }
    }
}

/// Compute aggregate savings if any output has original_size data.
fn compute_savings(outputs: &[OutputFile]) -> Option<SizeSavings> {
    let mut before: u64 = 0;
    let mut after: u64 = 0;
    let mut has_any = false;

    for file in outputs {
        if let Some(orig) = file.original_size {
            has_any = true;
            before += orig;
            after += file.size_bytes;
        }
    }

    if has_any && before > 0 {
        Some(SizeSavings { before, after })
    } else {
        None
    }
}

/// Pure state transition for the results screen.
pub fn update(mut model: ResultsModel, msg: ResultsMessage) -> ResultsModel {
    match msg {
        ResultsMessage::CursorDown => {
            if !model.outputs.is_empty() {
                model.cursor = (model.cursor + 1) % model.outputs.len();
            }
        }
        ResultsMessage::CursorUp => {
            if !model.outputs.is_empty() {
                model.cursor = if model.cursor == 0 {
                    model.outputs.len() - 1
                } else {
                    model.cursor - 1
                };
            }
        }
    }
    model
}

/// Per-file savings percentage. Returns None if no original or output grew.
pub fn file_savings_percent(original: u64, output: u64) -> Option<u64> {
    if original == 0 || output >= original {
        return None;
    }
    Some(((original - output) * 100) / original)
}

// Re-export format helpers from the shared module for backwards compatibility.
pub use super::super::format::{format_duration, format_size};

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_outputs() -> Vec<OutputFile> {
        vec![
            OutputFile {
                name: "photo-1.jpg".into(),
                size_bytes: 290_000,
                original_size: Some(780_000),
            },
            OutputFile {
                name: "photo-2.jpg".into(),
                size_bytes: 340_000,
                original_size: Some(920_000),
            },
            OutputFile {
                name: "photo-3.jpg".into(),
                size_bytes: 260_000,
                original_size: Some(640_000),
            },
        ]
    }

    #[test]
    fn new_creates_model_with_outputs() {
        let m = ResultsModel::new("s", sample_outputs(), 4100, None);
        assert_eq!(m.outputs.len(), 3);
        assert_eq!(m.cursor, 0);
        assert_eq!(m.total_time_ms, 4100);
        assert!(m.savings.is_some());
    }

    #[test]
    fn cursor_down_advances() {
        let m = ResultsModel::new("s", sample_outputs(), 0, None);
        let m = update(m, ResultsMessage::CursorDown);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_down_wraps_at_end() {
        let mut m = ResultsModel::new("s", sample_outputs(), 0, None);
        m.cursor = 2;
        let m = update(m, ResultsMessage::CursorDown);
        assert_eq!(m.cursor, 0);
    }

    #[test]
    fn cursor_up_moves_back() {
        let mut m = ResultsModel::new("s", sample_outputs(), 0, None);
        m.cursor = 2;
        let m = update(m, ResultsMessage::CursorUp);
        assert_eq!(m.cursor, 1);
    }

    #[test]
    fn cursor_up_wraps_at_start() {
        let m = ResultsModel::new("s", sample_outputs(), 0, None);
        let m = update(m, ResultsMessage::CursorUp);
        assert_eq!(m.cursor, 2);
    }

    #[test]
    fn savings_percentage() {
        let m = ResultsModel::new("s", sample_outputs(), 0, None);
        let savings = m.savings.unwrap();
        // before: 780000 + 920000 + 640000 = 2340000
        // after:  290000 + 340000 + 260000 = 890000
        // savings: (2340000 - 890000) * 100 / 2340000 = 61%
        assert_eq!(savings.percent(), 61);
    }

    #[test]
    fn savings_percent_zero_when_output_grew() {
        let savings = SizeSavings {
            before: 100,
            after: 150,
        };
        assert_eq!(savings.percent(), 0);
    }

    #[test]
    fn savings_percent_zero_when_sizes_equal() {
        let savings = SizeSavings {
            before: 500,
            after: 500,
        };
        assert_eq!(savings.percent(), 0);
    }

    #[test]
    fn file_savings_percent_normal_reduction() {
        // 780KB -> 290KB = 62% savings
        assert_eq!(file_savings_percent(780_000, 290_000), Some(62));
    }

    #[test]
    fn file_savings_percent_none_when_output_grew() {
        assert_eq!(file_savings_percent(100, 150), None);
    }

    #[test]
    fn file_savings_percent_none_when_equal() {
        assert_eq!(file_savings_percent(100, 100), None);
    }

    #[test]
    fn file_savings_percent_none_when_original_zero() {
        assert_eq!(file_savings_percent(0, 50), None);
    }

    #[test]
    fn savings_still_computed_when_some_files_grew() {
        // Mix: one file shrunk, one grew. Aggregate: before=200, after=250 -> grew overall.
        let outputs = vec![
            OutputFile {
                name: "shrunk.jpg".into(),
                size_bytes: 50,
                original_size: Some(100),
            },
            OutputFile {
                name: "grew.jpg".into(),
                size_bytes: 200,
                original_size: Some(100),
            },
        ];
        let m = ResultsModel::new("s", outputs, 0, None);
        let savings = m.savings.unwrap();
        // before=200, after=250 -> output grew -> 0%
        assert_eq!(savings.percent(), 0);
    }

    #[test]
    fn savings_none_when_no_original() {
        let outputs = vec![
            OutputFile {
                name: "a.txt".into(),
                size_bytes: 100,
                original_size: None,
            },
            OutputFile {
                name: "b.txt".into(),
                size_bytes: 200,
                original_size: None,
            },
        ];
        let m = ResultsModel::new("s", outputs, 0, None);
        assert!(m.savings.is_none());
    }
}
