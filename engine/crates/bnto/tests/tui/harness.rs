// Drive the real `bnto tui` in a pseudo-terminal, like Playwright drives a
// browser: press keys, wait for text, read the screen a user would see.
// Each session gets its own HOME and BNTO_HOME, so it never sees or touches
// a real ~/.bnto (.claude/strategy/tui-testing.md).

use std::io::{Read, Write};
use std::num::NonZeroU16;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
use tempfile::TempDir;

/// How long `wait_for` polls before failing with the screen it last saw.
const WAIT: Duration = Duration::from_secs(10);
const POLL: Duration = Duration::from_millis(25);

/// A key, as the bytes a terminal sends for it.
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)] // each journey file uses a subset
pub enum Key {
    Enter,
    Tab,
    Esc,
    Up,
    Down,
    Left,
    Right,
    Char(char),
}

impl Key {
    fn bytes(self) -> Vec<u8> {
        match self {
            Key::Enter => b"\r".to_vec(),
            Key::Tab => b"\t".to_vec(),
            Key::Esc => b"\x1b".to_vec(),
            Key::Up => b"\x1b[A".to_vec(),
            Key::Down => b"\x1b[B".to_vec(),
            Key::Right => b"\x1b[C".to_vec(),
            Key::Left => b"\x1b[D".to_vec(),
            Key::Char(c) => c.to_string().into_bytes(),
        }
    }
}

/// A running `bnto tui` and the screen it has drawn so far.
pub struct Tui {
    child: Box<dyn Child + Send + Sync>,
    writer: Box<dyn Write + Send>,
    screen: Arc<Mutex<vt100::Parser>>,
    // Kept alive for the session; the pty closes when it drops.
    _master: Box<dyn MasterPty + Send>,
    _homes: (TempDir, TempDir),
}

impl Tui {
    /// Start `bnto tui` in a `cols`×`rows` terminal with sandboxed homes.
    pub fn launch(cols: u16, rows: u16) -> Self {
        Self::launch_with(cols, rows, &[])
    }

    /// Like [`Tui::launch`], with `files` copied into the folder the TUI
    /// starts in (a file recipe's default input).
    pub fn launch_with(cols: u16, rows: u16, files: &[&std::path::Path]) -> Self {
        Self::launch_full(cols, rows, files, &[], None)
    }

    /// Like [`Tui::launch`], with the update check on and its cache saying
    /// `latest` is out (checked just now, so nothing reaches the network).
    #[allow(dead_code)] // each journey file uses a subset
    pub fn launch_with_update(cols: u16, rows: u16, latest: &str) -> Self {
        Self::launch_full(cols, rows, &[], &[], Some(latest))
    }

    /// Like [`Tui::launch`], with `recipes` in the sandbox's library
    /// (`BNTO_HOME/recipes`), as if the user had saved them.
    #[allow(dead_code)] // each journey file uses a subset
    pub fn launch_with_library(cols: u16, rows: u16, recipes: &[&std::path::Path]) -> Self {
        Self::launch_full(cols, rows, &[], recipes, None)
    }

    /// Like [`Tui::launch_with_library`], with `files` in the start folder too.
    #[allow(dead_code)] // each journey file uses a subset
    pub fn launch_with_library_and_files(
        cols: u16,
        rows: u16,
        recipes: &[&std::path::Path],
        files: &[&std::path::Path],
    ) -> Self {
        Self::launch_full(cols, rows, files, recipes, None)
    }

    fn launch_full(
        cols: u16,
        rows: u16,
        files: &[&std::path::Path],
        recipes: &[&std::path::Path],
        update: Option<&str>,
    ) -> Self {
        let size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };
        let pair = native_pty_system().openpty(size).expect("open a pty");
        let homes = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let work = work_dir(&homes);
        std::fs::create_dir_all(&work).unwrap();
        for file in files {
            std::fs::copy(file, work.join(file.file_name().unwrap())).unwrap();
        }
        let library = homes.1.path().join("recipes");
        std::fs::create_dir_all(&library).unwrap();
        for recipe in recipes {
            std::fs::copy(recipe, library.join(recipe.file_name().unwrap())).unwrap();
        }
        let mut cmd = command(&homes);
        if let Some(latest) = update {
            seed_update_cache(homes.1.path(), latest);
            allow_update_check(&mut cmd);
        }
        let child = pair.slave.spawn_command(cmd).expect("spawn bnto tui");
        drop(pair.slave);

        let [rows_nz, cols_nz] =
            [rows, cols].map(|n| NonZeroU16::new(n).expect("a non-empty terminal"));
        let screen = Arc::new(Mutex::new(vt100::Parser::new(rows_nz, cols_nz, 0)));
        pump(pair.master.try_clone_reader().unwrap(), Arc::clone(&screen));
        let writer = pair.master.take_writer().unwrap();
        Self {
            child,
            writer,
            screen,
            _master: pair.master,
            _homes: homes,
        }
    }

    /// Press `keys` in order. A short pause between keys lets the TUI read
    /// each one separately (a lone Esc is told apart from an escape sequence
    /// by timing).
    pub fn press(&mut self, keys: &[Key]) -> &mut Self {
        for key in keys {
            self.writer.write_all(&key.bytes()).unwrap();
            self.writer.flush().unwrap();
            thread::sleep(Duration::from_millis(30));
        }
        self
    }

    /// The screen as text, one line per row, trailing spaces trimmed.
    pub fn screen(&self) -> String {
        let parser = self.screen.lock().unwrap();
        let screen = parser.screen();
        let (_, cols) = screen.size();
        screen
            .rows(0, cols.get())
            .map(|row| row.trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Wait until the screen shows `text`, then return the screen.
    pub fn wait_for(&self, text: &str) -> String {
        self.wait_until(&format!("{text:?} to show"), |screen| screen.contains(text))
    }

    /// Wait until `done` holds for the screen, then return it. Fails with
    /// the last screen seen, so a failing journey shows what the user saw.
    pub fn wait_until(&self, what: &str, done: impl Fn(&str) -> bool) -> String {
        let start = Instant::now();
        loop {
            let screen = self.screen();
            if done(&screen) {
                return screen;
            }
            assert!(
                start.elapsed() < WAIT,
                "timed out waiting for {what}. Screen:\n{screen}"
            );
            thread::sleep(POLL);
        }
    }

    /// The session's BNTO_HOME, where runs write their output.
    #[allow(dead_code)] // each journey file uses a subset
    pub fn bnto_home(&self) -> &std::path::Path {
        self._homes.1.path()
    }

    /// The row holding the `▸` selection marker, if one is on screen.
    pub fn selected_row(&self) -> Option<String> {
        self.screen()
            .lines()
            .find(|line| line.contains('▸'))
            .map(str::to_string)
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// `bnto tui` with every home it could read or write pointed into `homes`.
fn command(homes: &(TempDir, TempDir)) -> CommandBuilder {
    let (user, bnto) = (homes.0.path(), homes.1.path());
    let mut cmd = CommandBuilder::new(env!("CARGO_BIN_EXE_bnto"));
    cmd.arg("tui");
    cmd.cwd(work_dir(homes));
    cmd.env("HOME", user);
    cmd.env("USERPROFILE", user);
    cmd.env("APPDATA", user.join("AppData"));
    cmd.env("XDG_CONFIG_HOME", user.join(".config"));
    cmd.env("BNTO_HOME", bnto);
    cmd.env("BNTO_TELEMETRY_DISABLED", "1");
    // Off unless a journey asks for it: no journey may reach the network.
    cmd.env("BNTO_NO_UPDATE_CHECK", "1");
    cmd.env("TERM", "xterm-256color");
    cmd
}

/// A fresh cached update check saying `latest` is out.
fn seed_update_cache(bnto_home: &std::path::Path, latest: &str) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let cache = bnto_home.join("cache");
    std::fs::create_dir_all(&cache).unwrap();
    let json = format!(r#"{{"checkedAt":{now},"latest":"{latest}"}}"#);
    std::fs::write(cache.join("update-check.json"), json).unwrap();
}

/// Let the update check run: no opt-out, and not CI (CI turns it off).
fn allow_update_check(cmd: &mut CommandBuilder) {
    cmd.env_remove("BNTO_NO_UPDATE_CHECK");
    for var in [
        "CI",
        "GITHUB_ACTIONS",
        "GITLAB_CI",
        "CIRCLECI",
        "TRAVIS",
        "BUILDKITE",
        "JENKINS_URL",
    ] {
        cmd.env_remove(var);
    }
}

/// The empty folder the TUI starts in. Not HOME itself, where bnto keeps
/// config (e.g. `~/Library` on macOS) that a file list would show first.
fn work_dir(homes: &(TempDir, TempDir)) -> std::path::PathBuf {
    homes.0.path().join("work")
}

/// Feed everything the TUI writes into the VT100 screen, on a thread.
fn pump(mut reader: Box<dyn Read + Send>, screen: Arc<Mutex<vt100::Parser>>) {
    thread::spawn(move || {
        let mut buf = [0u8; 8192];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 {
                break;
            }
            screen.lock().unwrap().process(&buf[..n]);
        }
    });
}
