// Render the recipe detail screen — recipe info + editable parameter list.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use super::app::AppModel;
use super::theme::{ROUNDED_BORDERS, Theme};

/// Render the recipe detail screen.
pub fn draw_detail(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let block = Block::bordered()
        .title(model.screen.title())
        .title_style(theme.heading())
        .border_set(ROUNDED_BORDERS)
        .border_style(theme.border());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Some(detail) = &model.detail else {
        let fallback = Paragraph::new("Loading...").style(theme.muted());
        frame.render_widget(fallback, inner);
        return;
    };

    let lines = detail_lines(detail, theme);
    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Build the lines for the detail screen content.
fn detail_lines<'a>(
    detail: &'a super::screens::detail::DetailModel,
    theme: &Theme,
) -> Vec<Line<'a>> {
    let mut lines: Vec<Line> = Vec::new();

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
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Press Enter to continue.",
            theme.text(),
        )));
    } else {
        detail_param_lines(detail, theme, &mut lines);
    }

    lines
}

/// Append parameter list lines to the output.
fn detail_param_lines<'a>(
    detail: &'a super::screens::detail::DetailModel,
    theme: &Theme,
    lines: &mut Vec<Line<'a>>,
) {
    lines.push(Line::from(Span::styled("  PARAMETERS", theme.category())));
    lines.push(Line::from(""));

    for (i, param) in detail.params.iter().enumerate() {
        let is_focused = i == detail.focused;
        let is_editing = is_focused && detail.editing;
        let marker = if is_focused { "▸ " } else { "  " };
        let label_style = if is_focused {
            theme.selected()
        } else {
            theme.text()
        };

        let display_value = if is_editing {
            format!("{}_", detail.edit_buffer)
        } else {
            param.value.clone()
        };
        let value_style = if is_editing {
            theme.selected()
        } else {
            theme.muted()
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {marker}{}", param.label), label_style),
            Span::styled(format!("  {display_value}"), value_style),
        ]));
    }
}
