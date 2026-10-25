// Names the files a stopped move left under a temporary name.
//
// `bnto_file::relocate` (overwrite-mode output, `file-move`) breaks a swap
// by parking one file (or a folder `file-move` moves) under a
// `.bnto-rename-*` name until its final rename. Ctrl+C in `bnto run`
// (`process::exit` skips cleanup) or quitting the TUI mid-run can land in
// between. The parked file holds the only copy of its content, so it is
// named on stderr, never deleted. `file-collect` skips the prefix, so a
// later run doesn't take it as an input.

use std::path::PathBuf;

/// Print a warning for each file a move parked and hadn't renamed by the
/// time the process ends. Called next to `report_kept_staging`: by the
/// Ctrl+C handler before `process::exit`, and by the TUI once the terminal
/// is restored. Moves aren't stopped first: one may still finish its final
/// rename after this runs (a stale line at worst), but none can park a file
/// after it (`take_parked` refuses every later park).
pub fn report_parked_files() {
    use colored::Colorize;

    for line in parked_lines(&bnto_file::relocate::take_parked()) {
        eprintln!("  {} {line}", "Warning:".yellow().bold());
    }
}

fn parked_lines(files: &[PathBuf]) -> Vec<String> {
    let line = |f: &PathBuf| {
        format!(
            "a move didn't finish; a file or folder is kept under a temporary name: {}",
            f.display()
        )
    };
    files.iter().map(line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_interrupt_warning_names_each_parked_file() {
        // RED (ISS-53): a run stopped mid-swap left a file under a
        // `.bnto-rename-*` name and said nothing about it.
        let parked = PathBuf::from("out/.bnto-rename-0-a.wav");
        let lines = parked_lines(std::slice::from_ref(&parked));
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("out/.bnto-rename-0-a.wav"), "{lines:?}");
        assert!(lines[0].contains("temporary name"), "{lines:?}");
    }
}
