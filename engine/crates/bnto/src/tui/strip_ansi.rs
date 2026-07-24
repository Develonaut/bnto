// Strips ANSI escape sequences and carriage-return redraws from command
// output lines so the TUI output pane shows clean text instead of literal
// escape codes (e.g. `[32minfo:[39m`) from tools like patreon-dl.

use std::iter::Peekable;
use std::str::Chars;

/// Remove ANSI escape sequences (CSI color/cursor codes, OSC titles) from a
/// line, keeping only the final redraw of `\r`-overwritten progress lines.
pub fn strip_ansi(line: &str) -> String {
    // Progress-style output (yt-dlp, ffmpeg) redraws via `\r`; only the last
    // non-empty segment is what a real terminal would leave visible.
    let visible = line.rsplit('\r').find(|seg| !seg.is_empty()).unwrap_or("");
    let mut out = String::with_capacity(visible.len());
    let mut chars = visible.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\u{1b}' {
            out.push(c);
            continue;
        }
        match chars.peek() {
            Some('[') => skip_csi(&mut chars),
            Some(']') => skip_osc(&mut chars),
            _ => {
                chars.next(); // two-character escape (ESC c, ESC =, …)
            }
        }
    }
    out
}

/// Consume a CSI sequence: `[`, parameter/intermediate bytes, one final byte.
fn skip_csi(chars: &mut Peekable<Chars>) {
    chars.next(); // consume '['
    for c in chars.by_ref() {
        if ('\u{40}'..='\u{7e}').contains(&c) {
            break;
        }
    }
}

/// Consume an OSC sequence: `]` until BEL or the two-character `ESC \` (ST).
fn skip_osc(chars: &mut Peekable<Chars>) {
    chars.next(); // consume ']'
    while let Some(c) = chars.next() {
        if c == '\u{07}' {
            break;
        }
        if c == '\u{1b}' && chars.peek() == Some(&'\\') {
            chars.next();
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_passes_through() {
        assert_eq!(
            strip_ansi("Download complete: file.mp4"),
            "Download complete: file.mp4"
        );
    }

    #[test]
    fn strips_color_codes() {
        assert_eq!(
            strip_ansi("\u{1b}[32minfo:\u{1b}[39m PostDownloader: done"),
            "info: PostDownloader: done"
        );
    }

    #[test]
    fn strips_multi_parameter_codes() {
        assert_eq!(strip_ansi("\u{1b}[1;31mbold red\u{1b}[0m"), "bold red");
    }

    #[test]
    fn strips_osc_title_sequence() {
        assert_eq!(strip_ansi("\u{1b}]0;window title\u{07}rest"), "rest");
        assert_eq!(strip_ansi("\u{1b}]8;;https://x\u{1b}\\link"), "link");
    }

    #[test]
    fn keeps_last_carriage_return_redraw() {
        assert_eq!(
            strip_ansi("[download]  10.0%\r[download]  99.5%"),
            "[download]  99.5%"
        );
        assert_eq!(strip_ansi("done\r"), "done");
    }

    #[test]
    fn lone_escape_is_dropped() {
        assert_eq!(strip_ansi("before\u{1b}cafter"), "beforeafter");
    }

    #[test]
    fn empty_and_control_only_lines_become_empty() {
        assert_eq!(strip_ansi(""), "");
        assert_eq!(strip_ansi("\r"), "");
        assert_eq!(strip_ansi("\u{1b}[2K\r"), "");
    }
}
