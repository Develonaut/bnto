// Render the Wizard screen — step-by-step guided recipe creation.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::app::AppModel;
use super::render_layout::content_panel;
use super::screens::detail_bridge::BntoFormTheme;
use super::screens::wizard::WizardStep;
use super::theme::Theme;

/// Draw the Wizard screen content area.
pub fn draw_wizard(frame: &mut ratatui::Frame, model: &AppModel, theme: &Theme, area: Rect) {
    let inner = content_panel(frame, theme, area, model.screen.title());
    let wizard = match &model.wizard {
        Some(w) => w,
        None => return,
    };

    let mut lines: Vec<Line> = Vec::new();

    match wizard.step {
        WizardStep::Category => render_category_step(&mut lines, wizard, theme),
        WizardStep::Operation => render_operation_step(&mut lines, wizard, theme),
        WizardStep::Configure => render_configure_step(&mut lines, wizard, theme),
        WizardStep::Complete => render_complete_step(&mut lines, wizard, theme),
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

/// Step 1: "What kind of files?"
fn render_category_step(
    lines: &mut Vec<Line>,
    wizard: &super::screens::wizard::WizardModel,
    theme: &Theme,
) {
    lines.push(Line::from(Span::styled(
        "  What kind of files?",
        theme.heading(),
    )));
    lines.push(Line::from(""));

    for (i, cat) in wizard.categories.iter().enumerate() {
        let is_selected = wizard.category_cursor == i;
        let marker = if is_selected { "▸ " } else { "  " };
        let style = if is_selected {
            theme.selected()
        } else {
            theme.text()
        };
        lines.push(Line::from(vec![
            Span::styled(format!("  {marker}{}", cat.label), style),
            Span::styled(format!("  ({} types)", cat.count), theme.muted()),
        ]));
    }
}

/// Step 2: "What do you want to do?"
fn render_operation_step(
    lines: &mut Vec<Line>,
    wizard: &super::screens::wizard::WizardModel,
    theme: &Theme,
) {
    let category = wizard
        .categories
        .iter()
        .find(|c| Some(&c.category) == wizard.selected_category.as_ref())
        .map(|c| c.label.as_str())
        .unwrap_or("files");
    lines.push(Line::from(Span::styled(
        format!("  What do you want to do with {category}?"),
        theme.heading(),
    )));
    lines.push(Line::from(""));

    for (i, op) in wizard.operations.iter().enumerate() {
        let is_selected = wizard.operation_cursor == i;
        let marker = if is_selected { "▸ " } else { "  " };
        let style = if is_selected {
            theme.selected()
        } else {
            theme.text()
        };
        lines.push(Line::from(vec![
            Span::styled(format!("  {marker}{}", op.label), style),
            Span::styled(format!("  {}", op.description), theme.muted()),
        ]));
    }
}

/// Step 3: Configure — schema-driven form.
fn render_configure_step(
    lines: &mut Vec<Line>,
    wizard: &super::screens::wizard::WizardModel,
    theme: &Theme,
) {
    let op_label = wizard
        .operations
        .iter()
        .find(|o| Some(&o.name) == wizard.selected_operation.as_ref())
        .map(|o| o.label.as_str())
        .unwrap_or("operation");
    lines.push(Line::from(Span::styled(
        format!("  Configure: {op_label}"),
        theme.heading(),
    )));
    lines.push(Line::from(""));

    if let Some(form) = &wizard.form {
        let form_theme = BntoFormTheme(theme);
        let form_lines = bnto_form::render_form(form, &form_theme);
        for line in form_lines {
            let mut spans = vec![Span::raw("  ")];
            spans.extend(line.spans);
            lines.push(Line::from(spans));
        }
    }
}

/// Step 4: Summary — ready to open in editor.
fn render_complete_step(
    lines: &mut Vec<Line>,
    wizard: &super::screens::wizard::WizardModel,
    theme: &Theme,
) {
    lines.push(Line::from(Span::styled(
        "  Ready to edit!",
        theme.heading(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  Recipe: ", theme.muted()),
        Span::styled(wizard.recipe_name.clone(), theme.text()),
    ]));

    if let Some(ref op) = wizard.selected_operation {
        lines.push(Line::from(vec![
            Span::styled("  Operation: ", theme.muted()),
            Span::styled(op.clone(), theme.text()),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Press Enter to open in the editor.",
        theme.muted(),
    )));
}
