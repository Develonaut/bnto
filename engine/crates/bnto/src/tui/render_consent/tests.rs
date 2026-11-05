use super::*;
use crate::tui::theme::ThemeVariant;

/// A request with more lines than a short panel holds.
fn tall_request() -> ConsentRequest {
    let commands = (1..=12).map(|i| format!("tool-{i} --flag")).collect();
    ConsentRequest::for_commands("Tall", commands)
}

/// Lay `exec`'s prompt out at `size`, then scroll as far as it goes, as
/// pressing j until the end does.
fn scroll_to_end(exec: &mut ExecutionModel, theme: &Theme, size: Size) {
    fit_consent(exec, theme, size);
    exec.consent_scroll = 100_000;
    fit_consent(exec, theme, size);
}

/// The text of `request` drawn in a `width` x `height` panel, scrolled.
fn drawn(request: &ConsentRequest, scroll: usize, width: u16, height: u16) -> String {
    let theme = Theme::from_variant(ThemeVariant::LosAngeles);
    let backend = ratatui::backend::TestBackend::new(width, height);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();
    terminal
        .draw(|f| draw_consent(f, request, scroll, &theme, f.area()))
        .unwrap();
    let buffer = terminal.backend().buffer().clone();
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| buffer[(x, y)].symbol().to_string())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn a_prompt_taller_than_the_panel_says_theres_more_below() {
    let text = drawn(&tall_request(), 0, 60, 10);
    assert!(text.contains("↓ more below"), "{text}");
    assert!(!text.contains("y allow and run"), "{text}");
}

#[test]
fn scrolled_to_the_end_it_shows_the_answer_keys() {
    let text = drawn(&tall_request(), 100, 60, 10);
    assert!(text.contains("↑ more above"), "{text}");
    assert!(text.contains("tool-12 --flag"), "{text}");
    assert!(text.contains("y allow and run"), "{text}");
    assert!(!text.contains("↓ more below"), "{text}");
}

#[test]
fn a_prompt_that_fits_shows_no_more_lines() {
    let request = ConsentRequest::for_commands("Short", vec!["git --version".into()]);
    let text = drawn(&request, 0, 60, 20);
    assert!(
        !text.contains("more below") && !text.contains("more above"),
        "{text}"
    );
    assert!(text.contains("y allow and run"), "{text}");
}

#[test]
fn fitting_keeps_the_scroll_within_the_prompt_and_its_end_in_view() {
    let theme = Theme::from_variant(ThemeVariant::LosAngeles);
    let mut exec = ExecutionModel::new("run").awaiting_consent(tall_request());
    scroll_to_end(&mut exec, &theme, Size::new(80, 24));
    assert!(exec.consent_at_end, "the fitted maximum is the end");
    // 80x24's panel is 66 columns by 12 rows (`panel_inner_width`, `viewport_height`).
    let text = drawn(exec.consent.as_ref().unwrap(), exec.consent_scroll, 66, 12);
    assert!(text.contains("y allow and run"), "{text}");
    assert!(text.contains("↑ more above"), "{text}");
}

#[test]
fn a_prompt_not_read_to_the_end_isnt_at_its_end() {
    let theme = Theme::from_variant(ThemeVariant::LosAngeles);
    let mut exec = ExecutionModel::new("run").awaiting_consent(tall_request());
    fit_consent(&mut exec, &theme, Size::new(80, 24));
    assert!(!exec.consent_at_end);
    fit_consent(&mut exec, &theme, Size::new(200, 80));
    assert!(exec.consent_at_end, "it all fits a big terminal");
}

#[test]
fn a_short_panel_asks_for_a_bigger_terminal_and_never_reaches_its_end() {
    // 100x16 leaves a 4-row panel: the 2 pinned rows and 2 that scroll,
    // too few to show a row of the prompt between the "more" lines.
    let theme = Theme::from_variant(ThemeVariant::LosAngeles);
    let mut exec = ExecutionModel::new("run").awaiting_consent(tall_request());
    scroll_to_end(&mut exec, &theme, Size::new(100, 16));
    assert!(!exec.consent_at_end);
    for height in [0, 1, 4] {
        let text = drawn(exec.consent.as_ref().unwrap(), 1000, 86, height);
        assert!(!text.contains("y allow and run"), "{text}");
    }
    let text = drawn(exec.consent.as_ref().unwrap(), 1000, 86, 4);
    assert!(text.contains("Enlarge the terminal"), "{text}");
}

#[test]
fn the_shortest_panel_that_shows_the_prompt_can_reach_its_end() {
    let theme = Theme::from_variant(ThemeVariant::LosAngeles);
    let mut exec = ExecutionModel::new("run").awaiting_consent(tall_request());
    scroll_to_end(&mut exec, &theme, Size::new(100, 17));
    assert!(exec.consent_at_end);
    let text = drawn(exec.consent.as_ref().unwrap(), exec.consent_scroll, 86, 5);
    assert!(text.contains("y allow and run"), "{text}");
}

#[test]
fn the_risk_summary_stays_in_view_while_the_prompt_scrolls() {
    for scroll in [0, 3, 100] {
        let text = drawn(&tall_request(), scroll, 60, 10);
        assert!(
            text.contains("runs 12 commands"),
            "scroll {scroll}:\n{text}"
        );
    }
}

#[test]
fn a_fields_default_shows_on_the_consent_screen_escaped() {
    let json = r#"{ "name": "Fdef", "nodes": [{ "id": "s", "type": "shell-run",
        "fields": { "fmt": { "type": "string", "label": "F", "default": "--exec=x\u001b[2K" } },
        "parameters": { "command": "echo", "args": ["{{fields.fmt}}"] } }] }"#;
    let kind = crate::catalog::SourceKind::Library(None);
    let store = crate::trust::TrustStore::default();
    let request = crate::trust_gate::consent_needed(json, "fdef", &kind, &store).unwrap();
    let text = drawn(&request, 0, 80, 30);
    assert!(text.contains("runs: echo --exec=x\\e[2K"), "{text}");
}

#[test]
fn a_new_width_starts_the_prompt_over() {
    // Rows re-wrap at a new width, so a row index kept across it could skip
    // rows nobody saw.
    let theme = Theme::from_variant(ThemeVariant::LosAngeles);
    let mut exec = ExecutionModel::new("run").awaiting_consent(tall_request());
    scroll_to_end(&mut exec, &theme, Size::new(80, 24));
    assert!(exec.consent_at_end);
    fit_consent(&mut exec, &theme, Size::new(120, 24));
    assert_eq!(exec.consent_scroll, 0);
    assert!(!exec.consent_at_end, "the end must be reached again");
}

#[test]
fn a_panel_too_narrow_to_read_never_reaches_its_end() {
    let theme = Theme::from_variant(ThemeVariant::LosAngeles);
    let mut exec = ExecutionModel::new("run").awaiting_consent(tall_request());
    exec.consent_scroll = 100_000;
    fit_consent(&mut exec, &theme, Size::new(6, 100));
    assert!(!exec.consent_at_end);
    exec.consent_scroll = 100_000;
    fit_consent(&mut exec, &theme, Size::new(24, 100));
    assert!(
        !exec.consent_at_end,
        "an 18-column panel is too narrow to read"
    );
}
