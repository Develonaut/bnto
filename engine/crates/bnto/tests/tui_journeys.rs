// TUI user journeys: the real `bnto tui` in a pseudo-terminal, driven by
// keystrokes. The TUI's counterpart to the web's Playwright journeys; the
// matrix lives in .claude/journeys/tui.md.

#[path = "tui/harness.rs"]
mod harness;

use harness::{Key, Tui, selected_row};

/// Move down to the detail screen's Run button with `j`. Not Tab: on Run
/// itself Tab starts the run, and this checks the screen between presses
/// (each one's answer, never an older frame: one `j` too many skips Run).
fn focus_run(tui: &mut Tui) {
    let mut screen = tui.screen();
    for _ in 0..20 {
        if screen.contains("▸ Run →") {
            return;
        }
        screen = tui.step(Key::Char('j'));
    }
    panic!("never reached Run. Screen:\n{screen}");
}

/// Press Down until the selection marker sits on `name`. Each press waits
/// for the marker to move, so a slow redraw can't make it press past `name`.
fn select(tui: &mut Tui, name: &str) {
    let mut screen = tui.wait_until("a selection marker", |s| selected_row(s).is_some());
    for _ in 0..40 {
        let row = selected_row(&screen).unwrap_or_default().to_string();
        if row.contains(name) {
            return;
        }
        tui.press(&[Key::Down]);
        screen = tui.wait_until(&format!("the selection to move off {row:?}"), |s| {
            selected_row(s).is_some_and(|moved| moved != row)
        });
    }
    panic!("never selected {name}. Screen:\n{screen}");
}

/// The journeys allowed to skip Windows, each with its reason. Windows CI
/// runs every other one (ISS-112); a new skip must be added here, in review.
const WINDOWS_SKIPS: &[(&str, &str)] = &[
    (
        "tj5_",
        "a PowerShell step streaming to stderr never exits (ISS-118)",
    ),
    (
        "tj6_",
        "a PowerShell step takes most of the wait budget to exit (ISS-118)",
    ),
];

#[test]
fn only_the_listed_journeys_skip_windows() {
    let skips = skipped_tests(include_str!("tui_journeys.rs"));
    let listed: Vec<(String, String)> = WINDOWS_SKIPS
        .iter()
        .map(|(n, r)| (n.to_string(), r.to_string()))
        .collect();
    assert_eq!(
        skips, listed,
        "a test is skipped or compiled out without being listed"
    );
}

/// Every test in `source` that an attribute may skip or leave out, with
/// the name's prefix up to its first `_` and the reason. Any attribute
/// mentioning `ignore` (plain or under any `cfg_attr`) or starting `#[cfg(`
/// counts, however it's spelled or wrapped. An early `return` in a test's
/// body can't be seen here; review catches those.
fn skipped_tests(source: &str) -> Vec<(String, String)> {
    let (mut skips, mut attrs, mut is_test) = (Vec::new(), Vec::<String>::new(), false);
    let mut open = String::new();
    for line in source.lines().map(str::trim) {
        let code = line.split("//").next().unwrap_or_default().trim();
        if !open.is_empty() || code.starts_with("#[") {
            open.push_str(code);
            if code.ends_with(']') && open.matches('[').count() == open.matches(']').count() {
                is_test |= open == "#[test]";
                attrs.push(std::mem::take(&mut open));
            }
        } else if let Some(name) = code.split("fn ").nth(1).filter(|_| is_test) {
            let prefix = format!("{}_", name.split('_').next().unwrap_or_default());
            skips.extend(
                attrs
                    .iter()
                    .filter_map(|a| skip_reason(a))
                    .map(|r| (prefix.clone(), r)),
            );
            (attrs, is_test) = (Vec::new(), false);
        } else if !code.is_empty() {
            (attrs, is_test) = (Vec::new(), false);
        }
    }
    skips
}

/// Why `attr` may skip a test: its `ignore = "…"` reason, or the attribute
/// itself when it ignores without one or compiles the test out.
fn skip_reason(attr: &str) -> Option<String> {
    if !attr.contains("ignore") && !attr.starts_with("#[cfg(") {
        return None;
    }
    let reason = attr
        .split_once("ignore = \"")
        .and_then(|(_, r)| r.split_once('"'));
    Some(reason.map_or(attr.to_string(), |(r, _)| r.to_string()))
}

#[test]
fn every_way_of_skipping_a_test_is_seen() {
    for case in [
        "#[cfg_attr(windows, ignore)]",
        "#[ignore = \"x\"]",
        "#[ignore]",
        "#[cfg_attr(target_os = \"windows\", ignore = \"x\")]",
        "#[cfg_attr(not(unix), ignore)] // a comment",
        "#[cfg(not(windows))]",
        "#[cfg_attr(\n    macos,\n    ignore = \"x\"\n)]",
    ] {
        let source = format!("{case}\n#[test]\npub fn tjx_case() {{}}\n");
        let skips = skipped_tests(&source);
        assert_eq!(skips.len(), 1, "missed {case:?}: {skips:?}");
        let source = format!("#[test]\n{case}\nfn tjx_case() {{}}\n");
        assert_eq!(
            skipped_tests(&source).len(),
            1,
            "missed {case:?} after #[test]"
        );
    }
    assert!(skipped_tests("#[test]\n#[allow(dead_code)]\nfn tjx_case() {}\n").is_empty());
}

#[test]
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
fn tj2_scroll_to_keep_awake_and_open_it() {
    // Keep Awake sits below the fold at 120x36; the pane must scroll to it.
    let mut tui = Tui::launch(120, 36);
    tui.wait_for("R E C I P E S");
    tui.step(Key::Tab); // focus Recipes
    select(&mut tui, "Keep Awake");
    tui.press(&[Key::Enter]);
    tui.wait_until("keep-awake's detail with its Action field", |screen| {
        ["Keep Awake  [built-in]", "Action", "Start a session"]
            .iter()
            .all(|part| screen.contains(part))
    });
}

#[test]
fn tj12_a_library_copy_never_hides_the_built_in() {
    // A library recipe named like a built-in used to drop the built-in from
    // the Recipes pane (and its count) and open the copy instead.
    let shipped = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../bnto-engine/recipes/keep-awake.bnto.json");
    let copy = std::fs::read_to_string(shipped).unwrap().replacen(
        "\"Keep Awake\"",
        "\"My Keep Awake\"",
        1,
    );
    let dir = tempfile::tempdir().unwrap();
    let recipe = dir.path().join("keep-awake.bnto.json");
    std::fs::write(&recipe, copy).unwrap();

    let mut tui = Tui::launch_with_library(120, 36, &[&recipe]);
    tui.wait_until("the library copy and the full catalog count", |screen| {
        screen.contains("keep-awake") && screen.contains("22 recipes")
    });
    tui.step(Key::Tab); // focus Recipes
    select(&mut tui, "Keep Awake");
    tui.press(&[Key::Enter]);
    tui.wait_for("Keep Awake  [built-in]");
}

#[test]
fn tj13_a_shell_recipe_without_fields_offers_just_run() {
    // Its command, args and limits are the program, not options (R4).
    let recipe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tui/slow-shell.bnto.json");
    let mut tui = Tui::launch_with_library(120, 36, &[&recipe]);
    tui.wait_for("slow-shell");
    tui.press(&[Key::Enter]);
    tui.wait_for("Slow Shell");
    tui.press(&[Key::Enter]);
    let screen = tui.wait_for("Run →");
    for raw in ["Command", "Arguments", "Output Mode", "Timeout"] {
        assert!(!screen.contains(raw), "{raw} shown:\n{screen}");
    }
}

#[test]
fn tj3_compress_an_image_and_see_the_result() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../test-fixtures/images/gradient.jpg");
    let mut tui = Tui::launch_with(120, 36, &[&fixture]);
    tui.wait_for("R E C I P E S");
    tui.step(Key::Tab); // focus Recipes
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
fn tj14_a_run_keeps_what_is_already_in_the_recipes_folder() {
    // ISS-21: the TUI emptied a recipe's output folder before each run.
    let recipe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tui/keep-folder.bnto.json");
    let inputs = tempfile::tempdir().unwrap();
    let report = inputs.path().join("report.txt");
    std::fs::write(&report, b"new").unwrap();
    let mut tui = Tui::launch_with_library_and_files(120, 36, &[&recipe], &[&report]);

    let folder = tui.bnto_home().join("output/kept");
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("notes.txt"), b"mine").unwrap();
    std::fs::write(folder.join("renamed-report.txt"), b"old").unwrap();

    tui.wait_for("keep-folder"); // the home Library pane lists file names
    tui.press(&[Key::Enter]); // Library pane is focused: open the library
    tui.wait_for("Keep Folder");
    tui.press(&[Key::Enter]); // ...and the recipe
    tui.wait_for("report.txt");
    tui.press(&[Key::Char(' ')]);
    tui.wait_for("1 file selected");
    tui.press(&[Key::Tab]); // to the rename's fields
    tui.wait_for("> Sanitize");
    focus_run(&mut tui);
    tui.press(&[Key::Enter]);

    let screen = tui.wait_until("the results screen", |screen| {
        ["R E S U L T S", "Completed in", "renamed-report.txt"]
            .iter()
            .all(|part| screen.contains(part))
    });
    assert!(
        !screen.contains("notes.txt"),
        "listed as a result:\n{screen}"
    );
    assert_eq!(std::fs::read(folder.join("notes.txt")).unwrap(), b"mine");
    assert_eq!(
        std::fs::read(folder.join("renamed-report.txt")).unwrap(),
        b"new"
    );
}

#[test]
#[cfg_attr(
    windows,
    ignore = "a PowerShell step streaming to stderr never exits (ISS-118)"
)]
fn tj5_shell_output_streams_live_and_stays_after_the_run() {
    // Windows has no `sh`, so its twin runs the same loop in PowerShell.
    let fixture = if cfg!(windows) {
        "slow-shell-windows"
    } else {
        "slow-shell"
    };
    let recipe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/tui/{fixture}.bnto.json"));
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
        screen.contains("This recipe runs commands") && screen.contains("line $i")
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
#[cfg_attr(
    windows,
    ignore = "a PowerShell step takes most of the wait budget to exit (ISS-118)"
)]
fn tj6_a_message_recipe_shows_its_message() {
    // A recipe whose output is a message (keep-awake's session log) shows
    // it on the results screen under the output's label. Windows has no
    // `sh`, so its twin prints the same report in PowerShell.
    let fixture = if cfg!(windows) {
        "report-windows"
    } else {
        "report"
    };
    let recipe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/tui/{fixture}.bnto.json"));
    let mut tui = Tui::launch_with_library(120, 40, &[&recipe]);
    tui.wait_for("report"); // the home Library pane lists file names
    tui.press(&[Key::Enter]);
    tui.wait_for("Report");
    tui.press(&[Key::Enter]);
    tui.wait_for("D E T A I L");
    focus_run(&mut tui);
    tui.press(&[Key::Enter]);
    tui.wait_for("runs commands on your computer"); // a library recipe asks first
    tui.press(&[Key::Char('y')]);
    tui.wait_until("the report on the results screen", |screen| {
        [
            "R E S U L T S",
            "S T A T U S   R E P O R T",
            "Nothing to report.",
        ]
        .iter()
        .all(|part| screen.contains(part))
    });
}

#[test]
fn tj11_tab_walks_every_field_to_run_on_a_short_terminal() {
    // Five declared fields whose descriptions wrap: at 80x24 the later
    // fields sit below the fold, so each Tab must scroll the newly focused
    // field into view.
    let recipe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tui/long-fields.bnto.json");
    let mut tui = Tui::launch_with_library(80, 24, &[&recipe]);
    tui.wait_for("long-fields"); // the home Library pane lists file names
    tui.press(&[Key::Enter]); // Library pane is focused: open the library
    tui.wait_for("Long Fields");
    tui.press(&[Key::Enter]); // ...and the recipe
    tui.wait_for("> Source");
    for label in ["Target", "Format", "Retries", "Verbose"] {
        tui.press(&[Key::Tab]);
        tui.wait_for(&format!("> {label}"));
    }
    tui.press(&[Key::Tab]);
    tui.wait_for("▸ Run →");
}

#[test]
fn tj10_a_newer_release_shows_in_the_status_line() {
    let tui = Tui::launch_with_update(120, 36, "99.0.0");
    tui.wait_for("bnto 99.0.0 available, U to upgrade");
}

#[test]
fn tj15_u_closes_the_tui_and_upgrades() {
    let mut tui = Tui::launch_with_update(120, 36, "99.0.0");
    tui.wait_for("U to upgrade");
    tui.press(&[Key::Char('U')]);
    // The test binary isn't a cargo install, so `bnto upgrade` explains how
    // to upgrade instead of running cargo: proof it ran, with no install.
    let screen = tui.wait_for("wasn't installed with cargo");
    assert!(screen.contains("bnto 99.0.0 is available"), "{screen}");
    assert!(
        !screen.contains("R E C I P E S"),
        "the TUI is still open: {screen}"
    );
}

#[test]
fn tj16_down_moves_the_selection_one_row() {
    // On Windows a key release used to act as a second press (#827), so one
    // Down moved two rows; ConPTY sends releases, so this catches it there.
    let mut tui = Tui::launch(120, 36);
    tui.wait_for("R E C I P E S");
    tui.step(Key::Tab); // focus Recipes, which marks its selection
    let marker_line = |screen: &str| screen.lines().position(|l| l.contains('▸'));
    let screen = tui.wait_until("a selection marker", |s| marker_line(s).is_some());
    let before = marker_line(&screen).unwrap();
    let screen = tui.step(Key::Down);
    assert_eq!(marker_line(&screen), Some(before + 1), "Screen:\n{screen}");
}
