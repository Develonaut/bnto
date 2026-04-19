// Render the recipe detail screen — recipe header + bnto-form controls.
//
// The form fields are rendered by bnto_form::render_form(). This module
// adds the recipe header (name, description) and the "Continue" button.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::screens::detail_bridge::BntoFormTheme;
use super::theme::Theme;

/// Render the recipe detail screen.
pub fn draw_detail(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());

    let Some(detail) = &model.detail else {
        let fallback = Paragraph::new("Loading...").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let lines = detail_lines(detail, theme);
    let total_lines = lines.len();
    let scroll_offset = detail.form.scroll_offset;
    let content = Paragraph::new(lines).scroll((scroll_offset as u16, 0));
    frame.render_widget(content, inner);

    // Overflow indicators
    let inner_width = inner.width as usize;
    if inner_width > 1 {
        if scroll_offset > 0 {
            let indicator = Paragraph::new("↑");
            let area = Rect::new(inner.x + inner.width.saturating_sub(2), inner.y, 1, 1);
            frame.render_widget(indicator, area);
        }
        if total_lines > scroll_offset + (inner.height as usize) {
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

    // Recipe header
    lines.push(Line::from(Span::styled(
        detail.name.as_str(),
        theme.heading(),
    )));
    lines.push(Line::from(Span::styled(
        detail.description.as_str(),
        theme.muted(),
    )));
    lines.push(Line::from(""));

    if detail.params.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No configurable parameters.",
            theme.muted(),
        )));
    } else {
        lines.push(Line::from(Span::styled("  PARAMETERS", theme.category())));
        lines.push(Line::from(""));

        // Delegate field rendering to bnto-form.
        let form_theme = BntoFormTheme(theme);
        let form_lines = bnto_form::render_form(&detail.form, &form_theme);
        lines.extend(form_lines);
    }

    // "Continue" action — always present, focusable at the bottom
    lines.push(Line::from(""));
    let on_continue = detail.is_continue_focused();
    let marker = if on_continue { "▸ " } else { "  " };
    let style = if on_continue {
        theme.selected()
    } else {
        theme.text()
    };
    lines.push(Line::from(Span::styled(
        format!("  {marker}Continue →"),
        style,
    )));

    lines
}
