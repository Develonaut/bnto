// Fit the detail screen's viewport to the terminal and keep its focus in view.
//
// Focus moves in `update`, which doesn't know the terminal width, and a
// field's height depends on how its description wraps. So the main loop calls
// `fit_detail` before each draw, and it measures the real wrapped layout.

use ratatui::layout::Size;

use super::render_detail::{detail_content_height, detail_focus_rows};
use super::render_layout::viewport_height;
use super::screens::detail::DetailModel;
use super::theme::Theme;

/// Screen chrome above the detail's scrolling content.
const DETAIL_CHROME: u16 = 2;

/// Fit the detail's viewport to a `term`-sized terminal, and scroll the
/// focused field (or Run) into view when focus has moved since last time.
pub fn fit_detail(detail: &mut DetailModel, theme: &Theme, term: Size) {
    let height = viewport_height(term.height, DETAIL_CHROME);
    if height == 0 {
        return;
    }
    detail.viewport.set_height(height);
    let content = detail_content_height(detail, theme, term.width);
    detail.viewport.set_content_height(content);

    let focus = (detail.focus, detail.form.focused);
    if detail.revealed == Some(focus) {
        return;
    }
    let rows = detail_focus_rows(detail, theme, term.width);
    // Bottom first, then top: a field taller than the view shows its label.
    detail.viewport.ensure_visible(rows.end.saturating_sub(1));
    detail.viewport.ensure_visible(rows.start);
    detail.revealed = Some(focus);
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ratatui::layout::Size;
    use ratatui::{Terminal, backend::TestBackend};

    use super::*;
    use crate::storage::paths::BntoPaths;
    use crate::tui::app::{AppMessage, AppModel, DetailOrigin, Screen, update};
    use crate::tui::keys::handle_key;
    use crate::tui::render::draw_frame;
    use crate::tui::screens::detail::{DetailModel, ParamEntry};
    use crate::tui::screens::detail_scroll::DetailScrollMsg;
    use crate::tui::theme::ThemeVariant;
    use bnto_core::metadata::ParameterType;

    /// The classic terminal: short enough that the fields don't all fit.
    const SIZE: Size = Size {
        width: 80,
        height: 24,
    };

    /// Labels of the shell node's raw params, in form order.
    const LABELS: [&str; 5] = [
        "Command",
        "Arguments",
        "Output Mode",
        "Timeout",
        "Max Output Size",
    ];

    /// A param whose description wraps to several rows at 80 columns.
    fn wordy(label: &str) -> ParamEntry {
        ParamEntry {
            node_id: "run".into(),
            name: label.to_lowercase().replace(' ', "_"),
            label: label.into(),
            value: String::new(),
            param_type: ParameterType::String,
            default: String::new(),
            description: Some(format!(
                "{label} help. {}",
                "Explains it at length. ".repeat(8)
            )),
            constraints: None,
            suffix: None,
            control: None,
            visible_when: None,
        }
    }

    fn session(home: &tempfile::TempDir) -> AppModel {
        let paths = BntoPaths {
            home: home.path().to_path_buf(),
        };
        let model = AppModel::with_paths(ThemeVariant::LosAngeles, None, false, paths);
        let params = LABELS.iter().map(|l| wordy(l)).collect();
        let mut model = AppModel {
            screen: Screen::Detail {
                slug: "s".into(),
                from: DetailOrigin::Home,
            },
            detail: Some(DetailModel::from_test_data("s", "Shell", "d", params)),
            ..model
        };
        fit(&mut model);
        model
    }

    fn fit(model: &mut AppModel) {
        let detail = model.detail.as_mut().unwrap();
        fit_detail(detail, &model.theme, SIZE);
    }

    /// Press `code`, then fit before the next draw, as the main loop does.
    fn press(model: AppModel, code: KeyCode) -> AppModel {
        let msg = handle_key(&model, KeyEvent::new(code, KeyModifiers::NONE)).unwrap();
        let mut model = update(model, msg);
        fit(&mut model);
        model
    }

    fn screen(model: &AppModel) -> String {
        let mut terminal = Terminal::new(TestBackend::new(SIZE.width, SIZE.height)).unwrap();
        terminal.draw(|f| draw_frame(f, model)).unwrap();
        let buffer = terminal.backend().buffer();
        (0..SIZE.height)
            .map(|y| (0..SIZE.width).map(|x| buffer[(x, y)].symbol()).collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn assert_shows(model: &AppModel, text: &str) {
        let screen = screen(model);
        assert!(screen.contains(text), "{text:?} not on screen:\n{screen}");
    }

    #[test]
    fn each_tab_keeps_the_focused_field_on_screen() {
        let home = tempfile::tempdir().unwrap();
        let mut model = session(&home);
        assert_shows(&model, "> Command");
        for label in &LABELS[1..] {
            model = press(model, KeyCode::Tab);
            assert_shows(&model, &format!("> {label}"));
        }
    }

    #[test]
    fn tab_past_the_last_field_shows_run() {
        let home = tempfile::tempdir().unwrap();
        let mut model = session(&home);
        for _ in &LABELS {
            model = press(model, KeyCode::Tab);
        }
        assert!(model.detail.as_ref().unwrap().is_run_focused());
        assert_shows(&model, "▸ Run →");
    }

    #[test]
    fn leaving_run_shows_the_last_field_again() {
        let home = tempfile::tempdir().unwrap();
        let mut model = session(&home);
        for _ in &LABELS {
            model = press(model, KeyCode::Tab);
        }
        model = update(model, AppMessage::DetailScroll(DetailScrollMsg::GoToTop));
        model = press(model, KeyCode::Up);
        assert_shows(&model, "> Max Output Size");
    }

    #[test]
    fn scrolling_away_from_the_focus_stays_scrolled() {
        let home = tempfile::tempdir().unwrap();
        let mut model = session(&home);
        model = update(model, AppMessage::DetailScroll(DetailScrollMsg::PageDown));
        let offset = model.detail.as_ref().unwrap().viewport.offset();
        assert!(offset > 0, "the content should overflow 80x24");
        fit(&mut model);
        assert_eq!(model.detail.as_ref().unwrap().viewport.offset(), offset);
    }
}
