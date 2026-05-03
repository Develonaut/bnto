// Render the recipe detail screen — recipe header + input section + form + Run button.
//
// Three-section layout:
// 1. Recipe header (name, description) — always shown
// 2. INPUT section — file picker for file-mode recipes (conditional)
// 3. PARAMETERS section — form fields rendered by tonkotsu
// 4. Run button — always present at the bottom

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::trust;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::screens::detail::DetailFocus;
use super::screens::detail_bridge::BntoFormTheme;
use super::theme::Theme;
use super::widgets::file_list;

/// Render the recipe detail screen.
pub fn draw_detail(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());

    let Some(detail) = &model.detail else {
        let fallback = Paragraph::new("Loading...").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let lines = detail_lines(detail, theme);
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

/// Build the lines for the detail screen content.
fn detail_lines<'a>(
    detail: &'a super::screens::detail::DetailModel,
    theme: &Theme,
) -> Vec<Line<'a>> {
    let mut lines: Vec<Line> = Vec::new();

    // Recipe header — trust badge from catalog source.
    let badge = trust::trust_badge(detail.is_bundled, false);
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(detail.name.as_str(), theme.heading()),
        Span::raw("  "),
        Span::styled(format!("[{badge}]"), theme.category()),
    ]));
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

        // Delegate field rendering to tonkotsu, then indent to match container padding.
        let form_theme = BntoFormTheme(theme);
        let form_lines = tonkotsu::render_form(&detail.form, &form_theme);
        for line in form_lines {
            let mut spans = vec![Span::raw("  ")];
            spans.extend(line.spans);
            lines.push(Line::from(spans));
        }
    }

    // "Run" action — always present, focusable at the bottom
    lines.push(Line::from(""));
    let on_run = detail.is_run_focused();
    let marker = if on_run { "▸ " } else { "  " };
    let style = if on_run {
        theme.selected()
    } else {
        theme.text()
    };
    lines.push(Line::from(Span::styled(format!("  {marker}Run →"), style)));

    lines
}
