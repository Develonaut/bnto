// Render the Editor screen — node list with expand/collapse, picker overlay,
// and delete confirmation dialog.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::screens::detail_bridge::BntoFormTheme;
use super::theme::Theme;
use super::widgets::search_input;

/// Draw the Editor screen content area.
pub fn draw_editor(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());
    let editor_screen = match &model.editor {
        Some(e) => e,
        None => return,
    };

    let editor = &editor_screen.editor;
    let mut lines: Vec<Line> = Vec::new();

    // Recipe header.
    let name = if editor.recipe_name.is_empty() {
        "Untitled Recipe"
    } else {
        &editor.recipe_name
    };
    lines.push(Line::from(Span::styled(
        format!("  {name}"),
        theme.heading(),
    )));
    if !editor.recipe_description.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  {}", editor.recipe_description),
            theme.muted(),
        )));
    }

    // Dirty indicator.
    if editor.dirty {
        lines.push(Line::from(Span::styled("  [modified]", theme.muted())));
    }
    lines.push(Line::from(""));

    // Delete confirmation overlay.
    if editor_screen.confirming_delete {
        let node_label = editor
            .selected_index
            .and_then(|i| editor.nodes.get(i))
            .map(|n| n.label.as_str())
            .unwrap_or("node");
        lines.push(Line::from(Span::styled(
            format!("  Delete '{node_label}'? (y/n)"),
            theme.heading(),
        )));
        lines.push(Line::from(""));
    }

    // Node list.
    if editor.nodes.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No nodes. Press 'a' to add one.",
            theme.muted(),
        )));
    } else {
        for (i, node) in editor.nodes.iter().enumerate() {
            let is_selected = editor.selected_index == Some(i);
            let marker = if is_selected { "▸ " } else { "  " };
            let name_style = if is_selected {
                theme.selected()
            } else {
                theme.text()
            };
            let expand_icon = if node.expanded { "▾ " } else { "▸ " };

            lines.push(Line::from(vec![
                Span::styled(format!("  {marker}{expand_icon}"), name_style),
                Span::styled(&node.label, name_style),
                Span::styled(format!("  ({})", node.node_type), theme.muted()),
            ]));

            // Expanded: show parameters (form controls or static fallback).
            if node.expanded {
                let has_form = is_selected
                    && editor_screen
                        .active_form
                        .as_ref()
                        .is_some_and(|f| f.node_index == i);

                if has_form {
                    let active = editor_screen.active_form.as_ref().unwrap();
                    let form_theme = BntoFormTheme(theme);
                    let form_lines = bnto_form::render_form(&active.form, &form_theme);
                    for line in form_lines {
                        // Indent form lines to align with node content.
                        let mut spans = vec![Span::raw("      ")];
                        spans.extend(line.spans);
                        lines.push(Line::from(spans));
                    }
                } else if node.params.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "      No parameters",
                        theme.muted(),
                    )));
                } else {
                    let mut keys: Vec<_> = node.params.keys().collect();
                    keys.sort();
                    for key in keys {
                        let value = &node.params[key];
                        let value_str = match value {
                            serde_json::Value::String(s) => s.clone(),
                            other => other.to_string(),
                        };
                        lines.push(Line::from(vec![
                            Span::styled(format!("      {key}: "), theme.muted()),
                            Span::styled(value_str, theme.text()),
                        ]));
                    }
                }
            }
        }
    }

    // Picker overlay.
    if let Some(picker) = &editor_screen.picker {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  ── Add Node ──",
            theme.heading(),
        )));

        // Search input.
        if let Some(search_line) = search_input::render_search_input(&picker.query, true, theme) {
            lines.push(search_line);
        }

        // Filtered entries.
        if picker.filtered.is_empty() {
            lines.push(Line::from(Span::styled("  No matches", theme.muted())));
        } else {
            for (list_idx, &entry_idx) in picker.filtered.iter().enumerate() {
                let entry = &picker.entries[entry_idx];
                let is_selected = list_idx == picker.cursor;
                let marker = if is_selected { "▸ " } else { "  " };
                let style = if is_selected {
                    theme.selected()
                } else {
                    theme.text()
                };
                lines.push(Line::from(vec![
                    Span::styled(format!("  {marker}"), style),
                    Span::styled(&entry.label, style),
                    Span::styled(format!("  [{}]", entry.category), theme.muted()),
                ]));
            }
        }
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}
