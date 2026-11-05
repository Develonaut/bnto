// Word-wrap pre-built lines to a width, before they're scrolled.
//
// Scrollable screens count their content in lines, so wrapping happens here
// rather than in ratatui's `Paragraph::wrap`, which would hide the extra rows
// from the scroll math. A wrapped line's continuation rows keep its indent.

use ratatui::style::Style;
use ratatui::text::{Line, Span};

/// Wrap every line wider than `width` at word boundaries. Continuation
/// rows repeat the line's leading indent; words keep their span's style.
/// A width of 0 (nothing laid out yet) leaves the lines as they are.
pub fn wrap_lines(lines: Vec<Line<'_>>, width: usize) -> Vec<Line<'static>> {
    lines
        .into_iter()
        .flat_map(|line| {
            if width == 0 || line.width() <= width {
                vec![owned(line)]
            } else {
                wrap_line(&line, width)
            }
        })
        .collect()
}

fn owned(line: Line<'_>) -> Line<'static> {
    let spans: Vec<Span<'static>> = line
        .spans
        .into_iter()
        .map(|s| Span::styled(s.content.into_owned(), s.style))
        .collect();
    Line::from(spans).style(line.style)
}

/// One line split into rows of at most `width` columns.
fn wrap_line(line: &Line<'_>, width: usize) -> Vec<Line<'static>> {
    let indent = line.to_string().chars().take_while(|c| *c == ' ').count();
    let indent = indent.min(width.saturating_sub(1));
    let mut rows = Vec::new();
    let mut row = Row::new(indent);
    for (word, style) in words(line) {
        for piece in split_to_fit(&word, width - indent) {
            if !row.fits(&piece, width) {
                rows.push(row.finish());
                row = Row::new(indent);
            }
            row.push(piece, style);
        }
    }
    rows.push(row.finish());
    rows
}

/// The line's words with their styles; runs of spaces between words drop.
fn words(line: &Line<'_>) -> Vec<(String, Style)> {
    line.spans
        .iter()
        .flat_map(|span| {
            let style = span.style;
            span.content
                .split(' ')
                .filter(|w| !w.is_empty())
                .map(move |w| (w.to_string(), style))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// `word` cut into pieces no wider than `max` columns (most words are one
/// piece). Measured in display width, so a double-width character counts
/// two; a character wider than `max` gets a piece of its own.
fn split_to_fit(word: &str, max: usize) -> Vec<String> {
    let mut pieces = vec![String::new()];
    let mut used = 0;
    for c in word.chars() {
        let width = Span::raw(c.to_string()).width();
        let last = pieces.last_mut().expect("never empty");
        if used + width > max && !last.is_empty() {
            pieces.push(String::new());
            used = 0;
        }
        pieces.last_mut().expect("never empty").push(c);
        used += width;
    }
    pieces
}

/// A row being filled: its indent, then words separated by single spaces.
struct Row {
    spans: Vec<Span<'static>>,
    used: usize,
    empty: bool,
}

impl Row {
    fn new(indent: usize) -> Self {
        Self {
            spans: vec![Span::raw(" ".repeat(indent))],
            used: indent,
            empty: true,
        }
    }

    fn fits(&self, word: &str, width: usize) -> bool {
        self.empty || self.used + 1 + Span::raw(word).width() <= width
    }

    fn push(&mut self, word: String, style: Style) {
        if !self.empty {
            self.spans.push(Span::raw(" "));
            self.used += 1;
        }
        self.used += Span::raw(word.as_str()).width();
        self.spans.push(Span::styled(word, style));
        self.empty = false;
    }

    fn finish(self) -> Line<'static> {
        Line::from(self.spans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(lines: &[Line]) -> Vec<String> {
        lines.iter().map(|l| l.to_string()).collect()
    }

    #[test]
    fn a_long_word_of_wide_characters_splits_by_display_width() {
        // Each CJK character takes two columns: split by count, a row would
        // run past the panel and its end would never show.
        let word = "日本語のコマンド名前長い".to_string() + "-tail";
        let rows = wrap_lines(vec![Line::from(format!("  {word}"))], 10);
        for row in &rows {
            assert!(
                row.width() <= 10,
                "{:?} is {} wide",
                row.to_string(),
                row.width()
            );
        }
        let shown: String = text(&rows).concat().replace(' ', "");
        assert_eq!(shown, word);
    }

    #[test]
    fn a_line_that_fits_is_unchanged() {
        let lines = vec![Line::from("  short")];
        assert_eq!(text(&wrap_lines(lines, 20)), ["  short"]);
    }

    #[test]
    fn a_long_line_wraps_at_words_keeping_its_indent() {
        let lines = vec![Line::from("    one two three four five")];
        assert_eq!(
            text(&wrap_lines(lines, 16)),
            ["    one two", "    three four", "    five"]
        );
    }

    #[test]
    fn styles_carry_over_to_continuation_rows() {
        let muted = Style::new().dim();
        let lines = vec![Line::from(vec![
            Span::raw("  "),
            Span::styled("alpha beta gamma", muted),
        ])];
        let wrapped = wrap_lines(lines, 12);
        assert_eq!(text(&wrapped), ["  alpha beta", "  gamma"]);
        let gamma = wrapped[1]
            .spans
            .iter()
            .find(|s| s.content.contains("gamma"));
        assert_eq!(gamma.unwrap().style, muted);
    }

    #[test]
    fn a_word_longer_than_the_row_is_split() {
        let lines = vec![Line::from("  abcdefghij")];
        assert_eq!(text(&wrap_lines(lines, 8)), ["  abcdef", "  ghij"]);
    }

    #[test]
    fn zero_width_leaves_lines_alone() {
        let lines = vec![Line::from("anything at all")];
        assert_eq!(text(&wrap_lines(lines, 0)), ["anything at all"]);
    }
}
