// A tonkotsu form as the TUI draws it: every span escaped (`display_text`)
// at draw time. The form's values stay raw, so an untouched field still
// equals its default and a value still matches a `visible_when`; only what
// the screen shows changes (ISS-125).

use ratatui::text::{Line, Span};
use tonkotsu::{FormModel, FormTheme};

use crate::display_text::display_text;

/// `form` rendered by tonkotsu, its text escaped.
pub fn render_shown_form(form: &FormModel, theme: &dyn FormTheme) -> Vec<Line<'static>> {
    tonkotsu::render_form(form, theme)
        .into_iter()
        .map(|line| {
            let spans: Vec<Span<'static>> = line
                .spans
                .into_iter()
                .map(|span| Span::styled(display_text(&span.content).into_owned(), span.style))
                .collect();
            Line::from(spans).style(line.style)
        })
        .collect()
}
