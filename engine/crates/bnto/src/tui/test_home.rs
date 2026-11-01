// A throwaway bnto home for TUI model tests.
//
// `AppModel` holds only the home's path, so the `TempDir` guard can't ride
// along with it. Each test runs on its own thread, so the guard lives in a
// thread-local and the folder is removed when the test ends, pass or panic.

use std::cell::RefCell;

use tempfile::TempDir;

use crate::storage::{config::TomlConfig, paths::BntoPaths};

thread_local! {
    static HOMES: RefCell<Vec<TempDir>> = const { RefCell::new(Vec::new()) };
}

/// A fresh home with its folders and a default config, removed with the test thread.
pub(crate) fn test_paths() -> BntoPaths {
    let dir = tempfile::Builder::new()
        .prefix("bnto-tui-home-")
        .tempdir()
        .unwrap();
    let paths = BntoPaths {
        home: dir.path().to_path_buf(),
    };
    HOMES.with(|homes| homes.borrow_mut().push(dir));
    let _ = paths.ensure_dirs();
    let _ = TomlConfig::default().save(&paths);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_is_removed_when_the_test_thread_ends() {
        let home = std::thread::spawn(|| {
            let paths = test_paths();
            assert!(paths.config_file().exists());
            paths.home
        })
        .join()
        .unwrap();
        assert!(!home.exists(), "{} was left behind", home.display());
    }

    #[test]
    fn test_home_is_removed_when_the_test_panics() {
        let (tx, rx) = std::sync::mpsc::channel();
        let joined = std::thread::spawn(move || {
            tx.send(test_paths().home).unwrap();
            panic!("a failing test");
        })
        .join();
        assert!(joined.is_err());
        let home = rx.recv().unwrap();
        assert!(!home.exists(), "{} was left behind", home.display());
    }
}
