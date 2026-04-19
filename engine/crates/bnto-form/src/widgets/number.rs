//! Number widget — renders a numeric field with optional slider bar as `Vec<Line>`.
//!
//! Idle + focused: `> Label   < 80% >  ████████░░░░`
//! Idle + unfocused: `  Label   80%`
//! Editing: cursor-tracked text input (delegates to text_input rendering pattern)

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use unicode_segmentation::UnicodeSegmentation;

use crate::field::{Field, FieldKind, FieldState};
use crate::theme::FormTheme;

/// Default slider bar width in terminal columns.
const SLIDER_WIDTH: u16 = 16;

/// Render a number field as lines of styled spans.
pub fn render(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    match &field.state {
        FieldState::NumberEditing { buffer, cursor } => {
            render_editing(field, buffer, *cursor, theme)
        }
        _ => render_idle(field, focused, theme),
    }
}

fn render_idle(field: &Field, focused: bool, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    let prefix = if focused { "> " } else { "  " };
    let label_style = if focused {
        theme.heading()
    } else {
        theme.text()
    };

    let (min, max, suffix) = match &field.kind {
        FieldKind::Number {
            min, max, suffix, ..
        } => (*min, *max, suffix.clone()),
        _ => (None, None, None),
    };

    let suffix_str = suffix.as_deref().unwrap_or("");
    let display = format!("{}{suffix_str}", field.value);

    let mut spans = vec![
        Span::styled(prefix.to_string(), label_style),
        Span::styled(format!("{}   ", field.label), label_style),
    ];

    if focused {
        spans.push(Span::styled("< ", theme.muted()));
        spans.push(Span::raw(display));
        spans.push(Span::styled(" >", theme.muted()));
    } else {
        spans.push(Span::raw(display));
    }

    // Slider bar when bounds exist
    if let (Some(lo), Some(hi)) = (min, max)
        && hi > lo
    {
        let val: f64 = field.value.parse().unwrap_or(lo);
        spans.push(Span::raw("  "));
        spans.push(render_slider_span(val, lo, hi, SLIDER_WIDTH));
    }

    let mut lines = vec![Line::from(spans)];

    if let Some(ref err) = field.error {
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(err.clone(), theme.error()),
        ]));
    }

    lines
}

fn render_editing(
    field: &Field,
    buffer: &str,
    cursor: usize,
    theme: &dyn FormTheme,
) -> Vec<Line<'static>> {
    let label_style = theme.heading();
    let cursor_style = Style::default().bg(Color::White).fg(Color::Black);

    let graphemes: Vec<&str> = buffer.graphemes(true).collect();

    let mut spans = vec![
        Span::styled("> ".to_string(), label_style),
        Span::styled(format!("{}   ", field.label), label_style),
    ];

    // Text before cursor
    let before: String = graphemes[..cursor].concat();
    if !before.is_empty() {
        spans.push(Span::raw(before));
    }

    // Cursor character
    if cursor < graphemes.len() {
        spans.push(Span::styled(graphemes[cursor].to_string(), cursor_style));
        let after: String = graphemes[cursor + 1..].concat();
        if !after.is_empty() {
            spans.push(Span::raw(after));
        }
    } else {
        spans.push(Span::styled(" ".to_string(), cursor_style));
    }

    let mut lines = vec![Line::from(spans)];

    if let Some(ref err) = field.error {
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(err.clone(), theme.error()),
        ]));
    }

    lines
}

// --- Slider rendering ---
//
// Inspired by tui-slider. Uses Unicode block characters for sub-cell
// precision: full block (█), partial blocks, and light shade (░).

/// Render a slider bar as a single `Span`.
fn render_slider_span(value: f64, min: f64, max: f64, width: u16) -> Span<'static> {
    let ratio = ((value - min) / (max - min)).clamp(0.0, 1.0);
    let bar = render_slider_bar(ratio, width);
    Span::styled(bar, Style::default().fg(Color::Cyan))
}

/// Build the slider bar string using Unicode block characters.
///
/// `ratio` is 0.0–1.0. `width` is the total character width.
/// Uses full block (█) for filled portion and light shade (░) for empty.
/// Partial blocks (▏▎▍▌▋▊▉) provide sub-cell precision at the boundary.
fn render_slider_bar(ratio: f64, width: u16) -> String {
    let total_eighths = (ratio * (width as f64) * 8.0).round() as usize;
    let full_blocks = total_eighths / 8;
    let remainder = total_eighths % 8;

    let mut bar = String::with_capacity(width as usize * 3);

    for _ in 0..full_blocks.min(width as usize) {
        bar.push('█');
    }

    if full_blocks < width as usize && remainder > 0 {
        let partial = match remainder {
            1 => '▏',
            2 => '▎',
            3 => '▍',
            4 => '▌',
            5 => '▋',
            6 => '▊',
            7 => '▉',
            _ => ' ',
        };
        bar.push(partial);
    }

    let filled_count = full_blocks + if remainder > 0 { 1 } else { 0 };
    for _ in filled_count..width as usize {
        bar.push('░');
    }

    bar
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::number;
    use crate::theme::DefaultTheme;

    fn theme() -> DefaultTheme {
        DefaultTheme
    }

    fn line_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn test_number_idle_with_suffix() {
        let field = number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .suffix("%")
            .value("80")
            .build();
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("< 80% >"), "got: {text}");
    }

    #[test]
    fn test_number_idle_no_suffix() {
        let field = number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .value("80")
            .build();
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("< 80 >"), "got: {text}");
    }

    #[test]
    fn test_number_idle_no_bounds_no_arrows_unfocused() {
        let field = number("q").label("Quality").value("80").build();
        let lines = render(&field, false, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("80"));
        assert!(!text.contains("<"), "got: {text}");
    }

    #[test]
    fn test_number_idle_focused_has_arrows() {
        let field = number("q").label("Quality").value("80").build();
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(text.contains("< 80 >"), "got: {text}");
    }

    #[test]
    fn test_number_idle_focused_prefix() {
        let field = number("q").label("Quality").value("80").build();
        let lines = render(&field, true, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "> ");
    }

    #[test]
    fn test_number_idle_unfocused_prefix() {
        let field = number("q").label("Quality").value("80").build();
        let lines = render(&field, false, &theme());
        assert_eq!(lines[0].spans[0].content.as_ref(), "  ");
    }

    #[test]
    fn test_number_slider_present_with_bounds() {
        let field = number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .value("50")
            .build();
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(
            text.contains('█') || text.contains('░'),
            "slider missing: {text}"
        );
    }

    #[test]
    fn test_number_slider_absent_without_bounds() {
        let field = number("q").label("Quality").value("50").build();
        let lines = render(&field, true, &theme());
        let text = line_text(&lines[0]);
        assert!(!text.contains('█'), "slider should be absent: {text}");
        assert!(!text.contains('░'), "slider should be absent: {text}");
    }

    #[test]
    fn test_number_editing_shows_cursor() {
        let mut field = number("q")
            .label("Quality")
            .range(0.0, 100.0)
            .value("80")
            .build();
        field.state = FieldState::NumberEditing {
            buffer: "80".to_string(),
            cursor: 1,
        };
        let lines = render(&field, true, &theme());
        let spans = &lines[0].spans;
        let cursor_span = spans.iter().find(|s| s.content.as_ref() == "0");
        assert!(cursor_span.is_some(), "cursor span missing");
        assert_eq!(cursor_span.unwrap().style.bg, Some(Color::White));
    }

    #[test]
    fn test_number_editing_cursor_at_end() {
        let mut field = number("q").label("Quality").value("80").build();
        field.state = FieldState::NumberEditing {
            buffer: "80".to_string(),
            cursor: 2,
        };
        let lines = render(&field, true, &theme());
        let spans = &lines[0].spans;
        let last = spans.last().unwrap();
        assert_eq!(last.content.as_ref(), " ");
        assert_eq!(last.style.bg, Some(Color::White));
    }

    // --- Slider bar unit tests ---

    #[test]
    fn test_slider_bar_full() {
        let bar = render_slider_bar(1.0, 10);
        assert_eq!(bar, "██████████");
    }

    #[test]
    fn test_slider_bar_empty() {
        let bar = render_slider_bar(0.0, 10);
        assert_eq!(bar, "░░░░░░░░░░");
    }

    #[test]
    fn test_slider_bar_half() {
        let bar = render_slider_bar(0.5, 10);
        assert_eq!(bar.chars().filter(|c| *c == '█').count(), 5);
        assert_eq!(bar.chars().filter(|c| *c == '░').count(), 5);
    }

    #[test]
    fn test_slider_bar_partial_block() {
        let bar = render_slider_bar(0.3125, 8);
        assert_eq!(bar.chars().count(), 8);
        let full_count = bar.chars().filter(|c| *c == '█').count();
        assert_eq!(full_count, 2);
        let has_partial = bar.chars().any(|c| "▏▎▍▌▋▊▉".contains(c));
        assert!(has_partial, "should have partial block: {bar}");
    }

    #[test]
    fn test_slider_bar_correct_length() {
        for width in [8, 12, 16, 20] {
            for ratio_pct in [0, 25, 33, 50, 67, 75, 100] {
                let bar = render_slider_bar(ratio_pct as f64 / 100.0, width);
                let char_count = bar.chars().count();
                assert_eq!(
                    char_count, width as usize,
                    "width={width}, ratio={ratio_pct}%: got {char_count} chars in: {bar}"
                );
            }
        }
    }

    #[test]
    fn test_slider_span_is_cyan() {
        let span = render_slider_span(50.0, 0.0, 100.0, 10);
        assert_eq!(span.style.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_number_error_renders() {
        let mut field = number("q").label("Quality").value("80").build();
        field.error = Some("Out of range".to_string());
        let lines = render(&field, false, &theme());
        assert_eq!(lines.len(), 2);
        let err_text = line_text(&lines[1]);
        assert!(err_text.contains("Out of range"));
    }
}
