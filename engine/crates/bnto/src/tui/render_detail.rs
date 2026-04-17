// Render the recipe detail screen — recipe info + editable parameter list.
//
// Type-aware controls: booleans show [x]/[ ], enums show ◂ label ▸,
// numbers show value + suffix, focused params show description.

use bnto_core::metadata::ParameterType;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use super::app::AppModel;
use super::screens::controls;
use super::screens::detail::DetailModel;
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
fn detail_lines<'a>(detail: &'a DetailModel, theme: &Theme) -> Vec<Line<'a>> {
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
    } else {
        detail_param_lines(detail, theme, &mut lines);
    }

    // Error line (if present)
    if let Some(error) = &detail.error {
        lines.push(Line::from(Span::styled(
            format!("  ⚠ {error}"),
            theme.heading(), // use heading (bold) for visibility
        )));
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

/// Append parameter list lines to the output.
fn detail_param_lines<'a>(detail: &'a DetailModel, theme: &Theme, lines: &mut Vec<Line<'a>>) {
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

        let value_display = render_param_value(param, is_editing, &detail.edit_buffer);
        let value_style = if is_editing {
            theme.selected()
        } else {
            theme.muted()
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {marker}{}", param.label), label_style),
            Span::styled(format!("  {value_display}"), value_style),
        ]));

        // Description shown only for focused param.
        if is_focused && let Some(desc) = &param.description {
            lines.push(Line::from(Span::styled(
                format!("    {desc}"),
                theme.muted(),
            )));
        }
    }
}

/// Render the value portion of a parameter based on its type.
fn render_param_value(
    param: &super::screens::detail::ParamEntry,
    is_editing: bool,
    edit_buffer: &str,
) -> String {
    if is_editing {
        return format!("{edit_buffer}_");
    }

    match &param.param_type {
        ParameterType::Boolean => controls::boolean::display_label(&param.value).to_string(),
        ParameterType::Enum { options } => {
            let label = controls::enum_select::display_label(&param.value, options);
            format!("◂ {label} ▸")
        }
        ParameterType::Number if param.suffix.is_some() => {
            let suffix = param.suffix.as_deref().unwrap_or("");
            format!("{}{}", param.value, suffix)
        }
        _ => param.value.clone(),
    }
}
