// Render the recipe detail screen — recipe header + input section + form + Run button.
//
// Three-section layout:
// 1. Recipe header (name, trust and OS badges, description) — always shown
// 2. INPUT section — file picker for file-mode recipes (conditional)
// 3. PARAMETERS section — form fields rendered by tonkotsu
// 4. Run button — always present at the bottom

use std::ops::Range;

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::host_support::os_badge;
use crate::trust;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::screens::detail::{DetailFocus, DetailModel};
use super::screens::detail_bridge::BntoFormTheme;
use super::theme::Theme;
use super::widgets::file_list;
use super::wrap_lines::wrap_lines;

/// Render the recipe detail screen.
#[expect(clippy::too_many_lines)]
pub fn draw_detail(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());

    let Some(detail) = &model.detail else {
        let fallback = Paragraph::new("Loading...").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let lines = wrap_lines(detail_lines(detail, theme), usize::from(inner.width));
    let scroll_offset = detail.viewport.offset();
    let content = Paragraph::new(lines).scroll((scroll_offset as u16, 0));
    frame.render_widget(content, inner);

    // Overflow indicators
    let inner_width = inner.width as usize;
    if inner_width > 1 && detail.viewport.overflows() {
        if !detail.viewport.at_top() {
            let indicator = Paragraph::new("↑");
            let area = Rect::new(inner.x + inner.width.saturating_sub(2), inner.y, 1, 1);
            frame.render_widget(indicator, area);
        }
        if !detail.viewport.at_bottom() {
            let indicator = Paragraph::new("↓");
            let area = Rect::new(
                inner.x + inner.width.saturating_sub(2),
                inner.y + inner.height.saturating_sub(1),
                1,
                1,
            );
            frame.render_widget(indicator, area);
        }
    }
}

/// Rows the detail screen's content takes in a terminal `term_width` wide,
/// after wrapping: the viewport's scroll range.
pub fn detail_content_height(detail: &DetailModel, theme: &Theme, term_width: u16) -> usize {
    wrapped_rows(&detail_lines(detail, theme), content_width(term_width))
}

/// Rows the focused part of the screen spans after wrapping, in a terminal
/// `term_width` wide: what to scroll into view when focus moves. Measured
/// from the drawn lines, since a field's height depends on how its
/// description wraps.
pub fn detail_focus_rows(detail: &DetailModel, theme: &Theme, term_width: u16) -> Range<usize> {
    let width = content_width(term_width);
    match detail.focus {
        // The input section sits under the recipe header: show both.
        DetailFocus::Input => 0..1,
        DetailFocus::Params => {
            let form = form_lines(detail, theme);
            let field =
                tonkotsu::focused_field_lines(&detail.form, &BntoFormTheme(theme)).unwrap_or(0..0);
            let start = wrapped_rows(&lead_lines(detail, theme), width)
                + wrapped_rows(&form[..field.start], width);
            start..start + wrapped_rows(&form[field], width)
        }
        DetailFocus::Run => {
            let rows = detail_content_height(detail, theme, term_width);
            rows.saturating_sub(1)..rows
        }
    }
}

fn content_width(term_width: u16) -> usize {
    usize::from(super::render_layout::panel_inner_width(term_width))
}

/// Build the lines for the detail screen content.
fn detail_lines<'a>(detail: &'a DetailModel, theme: &Theme) -> Vec<Line<'a>> {
    let mut lines = lead_lines(detail, theme);
    lines.extend(form_lines(detail, theme));
    lines.extend(run_lines(detail, theme));
    lines
}

/// Everything above the form's fields: the recipe header, the input
/// section, and the PARAMETERS heading.
#[expect(clippy::too_many_lines)]
fn lead_lines<'a>(detail: &'a DetailModel, theme: &Theme) -> Vec<Line<'a>> {
    let mut lines: Vec<Line> = Vec::new();

    // Recipe header — trust badge from catalog source.
    let badge = trust::trust_badge(detail.is_bundled, detail.has_shell);
    let mut header = vec![
        Span::raw("  "),
        Span::styled(detail.name.as_str(), theme.heading()),
        Span::raw("  "),
        Span::styled(format!("[{badge}]"), theme.category()),
    ];
    if let Some(os) = os_badge(&detail.os) {
        header.extend([Span::raw("  "), Span::styled(os, theme.category())]);
    }
    lines.push(Line::from(header));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(detail.description.as_str(), theme.muted()),
    ]));
    lines.push(Line::from(""));

    // INPUT section — file picker (only for file-mode recipes)
    if let Some(picker) = &detail.input_picker {
        let input_active = detail.focus == DetailFocus::Input;
        let section_style = if input_active {
            theme.heading()
        } else {
            theme.category()
        };
        lines.push(Line::from(Span::styled("  INPUT", section_style)));
        lines.push(Line::from(Span::styled(
            format!("  {}", picker.current_dir.display()),
            theme.muted(),
        )));
        lines.push(Line::from(""));

        if input_active {
            // Render file entries with cursor and selection
            if picker.entries.is_empty() {
                lines.push(Line::from(Span::styled(
                    "  No matching files",
                    theme.muted(),
                )));
            } else {
                let offset = picker.viewport_offset;
                let end = (offset + picker.viewport_height).min(picker.entries.len());
                let visible = &picker.entries[offset..end];
                let display_cursor = picker.cursor.saturating_sub(offset);
                let file_lines = file_list::render_file_list(
                    visible,
                    display_cursor,
                    &picker.selected,
                    theme,
                    false,
                );
                lines.extend(file_lines);
            }
        }

        // Selection count
        let count = picker.selected.len();
        let count_text = format!(
            "  {} file{} selected",
            count,
            if count == 1 { "" } else { "s" }
        );
        let count_style = if count > 0 {
            theme.category()
        } else {
            theme.muted()
        };
        lines.push(Line::from(Span::styled(count_text, count_style)));
        lines.push(Line::from(""));
    }

    // PARAMETERS section
    if detail.params.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No configurable parameters.",
            theme.muted(),
        )));
    } else {
        let params_active = detail.focus == DetailFocus::Params;
        let section_style = if params_active {
            theme.heading()
        } else {
            theme.category()
        };
        lines.push(Line::from(Span::styled("  PARAMETERS", section_style)));
        lines.push(Line::from(""));
    }
    lines
}

/// The form's fields, rendered by tonkotsu and indented to the panel padding.
fn form_lines(detail: &DetailModel, theme: &Theme) -> Vec<Line<'static>> {
    tonkotsu::render_form(&detail.form, &BntoFormTheme(theme))
        .into_iter()
        .map(|line| {
            let mut spans = vec![Span::raw("  ")];
            spans.extend(line.spans);
            Line::from(spans)
        })
        .collect()
}

/// The "Run" action — always present, focusable at the bottom.
fn run_lines(detail: &DetailModel, theme: &Theme) -> Vec<Line<'static>> {
    let on_run = detail.is_run_focused();
    let marker = if on_run { "▸ " } else { "  " };
    let style = if on_run {
        theme.selected()
    } else {
        theme.text()
    };
    vec![
        Line::from(""),
        Line::from(Span::styled(format!("  {marker}Run →"), style)),
    ]
}

/// How many rows `lines` take once wrapped to `width`.
fn wrapped_rows(lines: &[Line<'_>], width: usize) -> usize {
    wrap_lines(lines.to_vec(), width).len()
}
