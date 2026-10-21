// TUI user journeys: the real `bnto tui` in a pseudo-terminal, driven by
// keystrokes. The TUI's counterpart to the web's Playwright journeys; the
// matrix lives in .claude/journeys/tui.md.

#[path = "tui/harness.rs"]
mod harness;

use harness::{Key, Tui};

/// Move down to the detail screen's Run button with `j`. Not Tab: on Run
/// itself Tab starts the run, and this checks the screen between presses.
fn focus_run(tui: &mut Tui) {
    for _ in 0..20 {
        if tui.screen().contains("▸ Run →") {
            return;
        }
        tui.press(&[Key::Char('j')]);
    }
    panic!("never reached Run. Screen:\n{}", tui.screen());
}

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

#[test]
#[cfg_attr(windows, ignore = "ConPTY journeys not yet stable in CI")]
fn tj5_shell_output_streams_live_and_stays_after_the_run() {
    let recipe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tui/slow-shell.bnto.json");
    let mut tui = Tui::launch_with_library(120, 40, &[&recipe]);
    tui.wait_for("slow-shell"); // the home Library pane lists file names
    tui.press(&[Key::Enter]); // Library pane is focused: open the library
    tui.wait_for("Slow Shell");
    tui.press(&[Key::Enter]); // ...and the recipe
    tui.wait_for("Slow Shell  [local (shell)]");
    focus_run(&mut tui);
    tui.press(&[Key::Enter]);

    // A library shell recipe asks first, showing the whole script.
    tui.wait_until("the consent prompt with the script", |screen| {
        screen.contains("This recipe runs commands") && screen.contains("echo \"line $i\"")
    });
    tui.press(&[Key::Char('y')]);

    // Live: line 2 is on screen while the command is still running.
    tui.wait_until("line 2 while the command is still running", |screen| {
        screen.contains("line 2") && !screen.contains("line 5")
    });
    tui.wait_until("the results screen keeping every line", |screen| {
        ["R E S U L T S", "OUTPUT", "line 1", "line 5"]
            .iter()
            .all(|part| screen.contains(part))
    });
}

#[test]
#[cfg_attr(windows, ignore = "keep-awake's script runs under sh")]
fn tj6_keep_awake_status_shows_its_message() {
    // Status only reads state: with a fresh BNTO_HOME there's no session.
    let mut tui = Tui::launch(120, 40);
    tui.wait_for("R E C I P E S");
    tui.press(&[Key::Tab]);
    select(&mut tui, "Keep Awake");
    tui.press(&[Key::Enter]);
    tui.wait_for("Start a session");
    tui.press(&[Key::Right, Key::Right]);
    tui.wait_for("Check status");
    focus_run(&mut tui);
    tui.press(&[Key::Enter]);
    tui.wait_until("keep-awake's session log on the results screen", |screen| {
        [
            "R E S U L T S",
            "S E S S I O N   L O G",
            "No active session.",
        ]
        .iter()
        .all(|part| screen.contains(part))
    });
}

#[test]
#[cfg_attr(windows, ignore = "ConPTY journeys not yet stable in CI")]
fn tj11_tab_walks_every_field_to_run_on_a_short_terminal() {
    // A shell recipe with no fields shows the shell node's raw params. Their
    // descriptions wrap, so at 80x24 the later fields sit below the fold:
    // each Tab must scroll the newly focused field into view.
    let recipe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tui/slow-shell.bnto.json");
    let mut tui = Tui::launch_with_library(80, 24, &[&recipe]);
    tui.wait_for("slow-shell"); // the home Library pane lists file names
    tui.press(&[Key::Enter]); // Library pane is focused: open the library
    tui.wait_for("Slow Shell");
    tui.press(&[Key::Enter]); // ...and the recipe
    tui.wait_for("> Command");
    for label in ["Arguments", "Output Mode", "Timeout", "Max Output Size"] {
        tui.press(&[Key::Tab]);
        tui.wait_for(&format!("> {label}"));
    }
    tui.press(&[Key::Tab]);
    tui.wait_for("▸ Run →");
}

#[test]
#[cfg_attr(windows, ignore = "ConPTY journeys not yet stable in CI")]
fn tj10_a_newer_release_shows_in_the_status_line() {
    let tui = Tui::launch_with_update(120, 36, "99.0.0");
    tui.wait_for("bnto 99.0.0 available, run bnto upgrade");
}
