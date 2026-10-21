// TUI user journeys: the real `bnto tui` in a pseudo-terminal, driven by
// keystrokes. The TUI's counterpart to the web's Playwright journeys; the
// matrix lives in .claude/journeys/tui.md.

#[path = "tui/harness.rs"]
mod harness;

use harness::{Key, Tui};

/// Press Down until the selection marker sits on `name`.
fn select(tui: &mut Tui, name: &str) {
    for _ in 0..40 {
        if tui.selected_row().is_some_and(|row| row.contains(name)) {
            return;
        }
        tui.press(&[Key::Down]);
    }
    panic!("never selected {name}. Screen:\n{}", tui.screen());
}

#[test]
#[cfg_attr(windows, ignore = "ConPTY journeys not yet stable in CI")]
fn tj1_home_shows_the_catalog_and_version() {
    // Wait for the whole frame, not its first row: a read can land mid-draw.
    let tui = Tui::launch(120, 36);
    let status = format!("recipes  ·  v{}", env!("CARGO_PKG_VERSION"));
    tui.wait_until("the home screen with its status bar", |screen| {
        ["L I B R A R Y", "R E C I P E S", status.as_str()]
            .iter()
            .all(|part| screen.contains(part))
    });
}

#[test]
#[cfg_attr(windows, ignore = "ConPTY journeys not yet stable in CI")]
fn tj2_scroll_to_keep_awake_and_open_it() {
    // Keep Awake sits below the fold at 120x36; the pane must scroll to it.
    let mut tui = Tui::launch(120, 36);
    tui.wait_for("R E C I P E S");
    tui.press(&[Key::Tab]);
    select(&mut tui, "Keep Awake");
    tui.press(&[Key::Enter]);
    tui.wait_until("keep-awake's detail with its Action field", |screen| {
        ["Keep Awake  [built-in]", "Action", "Start a session"]
            .iter()
            .all(|part| screen.contains(part))
    });
}

#[test]
#[cfg_attr(windows, ignore = "ConPTY journeys not yet stable in CI")]
fn tj3_compress_an_image_and_see_the_result() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../test-fixtures/images/gradient.jpg");
    let mut tui = Tui::launch_with(120, 36, &[&fixture]);
    tui.wait_for("R E C I P E S");
    tui.press(&[Key::Tab]);
    select(&mut tui, "Compress Images");
    tui.press(&[Key::Enter]);

    // The detail opens on its file list, rooted where the TUI started.
    tui.wait_for("gradient.jpg");
    tui.press(&[Key::Char(' ')]);
    tui.wait_for("1 file selected");
    tui.press(&[Key::Tab, Key::Tab]);
    tui.wait_for("▸ Run →");
    tui.press(&[Key::Enter]);

    tui.wait_until("the results screen", |screen| {
        ["R E S U L T S", "Completed in", "gradient-compressed.jpg"]
            .iter()
            .all(|part| screen.contains(part))
    });
    let written = std::fs::read_dir(tui.bnto_home().join("output"))
        .expect("the run wrote an output folder")
        .count();
    assert!(written > 0, "no output under BNTO_HOME");
}
