// The TUI's consent screen: the same body `bnto run` prints
// (`consent_body`, escaped there), styled, with how to answer below. The
// heading and the risk summary stay pinned at the top; the rest scrolls one
// row at a time (j/k), so reaching the last row means every row was drawn,
// and `y` allows only then. A panel too short to show the prompt asks for a
// bigger terminal instead.

use ratatui::layout::{Rect, Size};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::render_layout::{panel_inner_width, viewport_height};
use super::screens::execution::ExecutionModel;
use super::theme::Theme;
use super::wrap_lines::wrap_lines;
use crate::consent_body::{ConsentLine, consent_body};
use crate::trust_gate::ConsentRequest;

/// Scrolling rows a panel needs under the pinned ones: with a "more above"
/// and a "more below" line, three leave one row of the prompt in view.
const MIN_BODY_ROWS: usize = 3;

/// Columns a panel needs: narrower, a command line breaks into pieces too
/// short to read.
const MIN_WIDTH: usize = 20;

/// The prompt laid out for a panel `width` x `height`.
struct Laid {
    /// The heading and risk summary, always in view.
    pinned: Vec<Line<'static>>,
    /// Everything else, scrolled.
    body: Vec<Line<'static>>,
    /// Rows the body has under the pinned rows.
    body_height: usize,
    width: usize,
}

impl Laid {
    fn new(request: &ConsentRequest, theme: &Theme, width: usize, height: usize) -> Self {
        let (pinned, body) = consent_lines(request, theme);
        let pinned = wrap_lines(pinned, width);
        let body_height = height.saturating_sub(pinned.len());
        let body = wrap_lines(body, width);
        Self {
            pinned,
            body,
            body_height,
            width,
        }
    }

    /// Whether the panel can show the prompt at all.
    fn viable(&self) -> bool {
        self.body_height >= MIN_BODY_ROWS && self.width >= MIN_WIDTH
    }

    fn max_scroll(&self) -> usize {
        self.body.len().saturating_sub(self.body_height)
    }
}

/// Draw `request` in `area`, its body scrolled down `scroll` rows. A row
/// hidden above or below gives its place to a line saying there's more.
pub fn draw_consent(
    frame: &mut ratatui::Frame,
    request: &ConsentRequest,
    scroll: usize,
    theme: &Theme,
    area: Rect,
) {
    let laid = Laid::new(
        request,
        theme,
        usize::from(area.width),
        usize::from(area.height),
    );
    if !laid.viable() {
        let ask = Span::styled(
            "  Enlarge the terminal to review this recipe",
            theme.error(),
        );
        frame.render_widget(Paragraph::new(Line::from(ask)), area);
        return;
    }
    let scroll = scroll.min(laid.max_scroll());
    let mut shown = laid.pinned.clone();
    shown.extend(scrolled(laid, scroll, theme));
    frame.render_widget(Paragraph::new(shown), area);
}

/// The body rows in view at `scroll`, with the "more" lines.
fn scrolled(laid: Laid, scroll: usize, theme: &Theme) -> Vec<Line<'static>> {
    let height = laid.body_height;
    let mut shown: Vec<Line> = laid.body.into_iter().skip(scroll).collect();
    let below = shown.len() > height;
    shown.truncate(height);
    let more = |text| Line::from(Span::styled(text, theme.muted()));
    if scroll > 0
        && let Some(first) = shown.first_mut()
    {
        *first = more("  ↑ more above");
    }
    if below && let Some(last) = shown.last_mut() {
        *last = more("  ↓ more below · j/k scroll");
    }
    shown
}

/// Keep `exec`'s consent scroll within its prompt, for a `term`-sized
/// terminal, so a key past the end isn't owed back before scrolling up, and
/// note whether the prompt's last row shows in a panel that can show it
/// (which `y` needs).
pub fn fit_consent(exec: &mut ExecutionModel, theme: &Theme, term: Size) {
    let Some(request) = &exec.consent else {
        return;
    };
    let width = usize::from(panel_inner_width(term.width));
    let laid = Laid::new(request, theme, width, viewport_height(term.height, 0));
    // Rows re-wrap at a new width: start over, so no row is skipped unseen.
    if exec.consent_width != Some(width) {
        exec.consent_scroll = 0;
        exec.consent_width = Some(width);
    }
    exec.consent_scroll = exec.consent_scroll.min(laid.max_scroll());
    exec.consent_at_end = laid.viable() && exec.consent_scroll == laid.max_scroll();
}

/// The consent prompt, split into the rows pinned at the top (heading and
/// risk summary) and the rest: what the recipe is, where it reaches, what
/// it runs, and how to answer. Mirrors the CLI prompt line for line.
fn consent_lines(
    request: &ConsentRequest,
    theme: &Theme,
) -> (Vec<Line<'static>>, Vec<Line<'static>>) {
    let (pinned, rest): (Vec<_>, Vec<_>) = consent_body(request)
        .into_iter()
        .partition(|line| matches!(line, ConsentLine::Heading(_) | ConsentLine::Summary(_)));
    let mut body: Vec<Line<'static>> = rest.into_iter().map(|l| styled(l, theme)).collect();
    body.push(Line::from(""));
    body.push(Line::from(Span::styled(
        "  y allow and run  ·  n / Esc cancel",
        theme.heading(),
    )));
    let pinned = pinned.into_iter().map(|l| styled(l, theme)).collect();
    (pinned, body)
}

fn styled(line: ConsentLine, theme: &Theme) -> Line<'static> {
    let (text, style) = match line {
        ConsentLine::Heading(text) => (format!("  {text}"), theme.heading()),
        ConsentLine::Summary(text) => (format!("  {text}"), theme.error()),
        ConsentLine::Detail(text) => (format!("  {text}"), theme.muted()),
        ConsentLine::Section(title) => (format!("  {}", title.to_uppercase()), theme.category()),
        ConsentLine::Item(text) => (format!("    - {text}"), theme.text()),
        ConsentLine::Flagged(text) => (format!("    ! {text}"), theme.error()),
        // A script's lines carry their own indent (`consent_lines`).
        ConsentLine::More(text) => (format!("      {text}"), theme.text()),
        ConsentLine::Note(text) => (format!("    {text}"), theme.muted()),
        ConsentLine::Blank => (String::new(), theme.text()),
    };
    Line::from(Span::styled(text, style))
}

#[cfg(test)]
mod tests;
