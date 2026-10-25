// Runs an accepted move plan in an order that keeps every file.
//
// A move waits until the move vacating its target has run (a→b, b→c runs
// b→c first). A cycle (a→b, b→a) is broken by parking one file under a
// temporary name, registered (`parked`) until its final rename so a process
// that ends first can name it. Each move re-checks its target first, by file
// identity, so a file that appeared after planning is never replaced. Every
// move is a rename: across drives it fails rather than copying.

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use super::{Move, key, occupied, parked, same_file};

/// Run the moves, creating each target's folder; on failure, say how far the
/// run got and what it left. Check the plan with `conflicts` first.
pub fn run_in_order(moves: Vec<Move>) -> Result<(), String> {
    let total = moves.len();
    let mut runner = Runner::new(moves);
    runner.run().map_err(|e| runner.failure(&e, total))
}

struct Runner {
    /// Folded source key → move, ordered so runs are deterministic.
    pending: BTreeMap<String, Move>,
    done: usize,
    /// Files sitting under a temporary name, waiting for their final rename.
    parked: Vec<PathBuf>,
}

impl Runner {
    fn new(moves: Vec<Move>) -> Self {
        Self {
            pending: moves.into_iter().map(|m| (key(&m.src), m)).collect(),
            done: 0,
            parked: Vec::new(),
        }
    }

    fn run(&mut self) -> Result<(), String> {
        while let Some(start) = self.pending.keys().next().cloned() {
            let (chain, cycle_at) = self.blocking_chain(start);
            match cycle_at {
                Some(at) => self.park(&at)?,
                None => self.run_chain(&chain)?,
            }
        }
        Ok(())
    }

    /// From `start`, follow "my target is your source" until a target is
    /// free; returns the keys in order, and the key a loop re-entered at.
    fn blocking_chain(&self, start: String) -> (Vec<String>, Option<String>) {
        let mut chain = vec![start];
        let mut seen: HashSet<String> = chain.iter().cloned().collect();
        loop {
            let current = &self.pending[chain.last().expect("chain is never empty")];
            let next = key(&current.dest);
            if next == key(&current.src) || !self.pending.contains_key(&next) {
                return (chain, None);
            }
            if !seen.insert(next.clone()) {
                return (chain, Some(next));
            }
            chain.push(next);
        }
    }

    /// Run a chain from its free end, so each target is vacated first.
    fn run_chain(&mut self, chain: &[String]) -> Result<(), String> {
        for src_key in chain.iter().rev() {
            let m = self
                .pending
                .remove(src_key)
                .expect("chain keys are pending");
            if let Err(e) = guarded_rename(&m.src, &m.dest) {
                self.pending.insert(src_key.clone(), m);
                return Err(e);
            }
            if let Some(at) = self.parked.iter().position(|p| p == &m.src) {
                self.parked.remove(at);
                parked::unregister(&m.src);
            }
            self.done += 1;
        }
        Ok(())
    }

    /// Break a cycle: move one file to a free temporary name in its folder.
    /// The rename and its registration happen together (`parked::park`), so
    /// the process's end names every parked file or none was made.
    fn park(&mut self, src_key: &str) -> Result<(), String> {
        let mut m = self
            .pending
            .remove(src_key)
            .expect("cycle keys are pending");
        let temp = temp_path(&m.src);
        let renamed = parked::park(&temp, || guarded_rename(&m.src, &temp));
        if renamed.is_ok() {
            self.parked.push(temp.clone());
            m.src = temp;
        }
        self.pending.insert(key(&m.src), m);
        renamed
    }

    fn failure(&self, error: &str, total: usize) -> String {
        let mut text = format!(
            "{error}\n  {} of {total} renames ran before this one",
            self.done
        );
        if !self.parked.is_empty() {
            let names: Vec<String> = self
                .parked
                .iter()
                .map(|p| p.display().to_string())
                .collect();
            text.push_str(&format!(
                "; left under a temporary name: {}",
                names.join(", ")
            ));
        }
        text
    }
}

/// Rename, refusing to replace anything at `dest` that isn't `src` itself
/// (a case-only rename on a case-insensitive filesystem).
fn guarded_rename(src: &Path, dest: &Path) -> Result<(), String> {
    if occupied(dest) && !same_file(src, dest) {
        return Err(format!(
            "Stopped: {} now exists, so {} was not moved onto it",
            dest.display(),
            src.display()
        ));
    }
    if let Some(folder) = dest.parent() {
        std::fs::create_dir_all(folder)
            .map_err(|e| format!("Cannot create {}: {e}", folder.display()))?;
    }
    std::fs::rename(src, dest)
        .map_err(|e| format!("Cannot rename {} → {}: {e}", src.display(), dest.display()))
}

fn temp_path(src: &Path) -> PathBuf {
    let leaf = src.file_name().and_then(|n| n.to_str()).unwrap_or("file");
    let dir = src.parent().unwrap_or(Path::new("."));
    (0..)
        .map(|n| dir.join(format!("{}{n}-{leaf}", crate::PARKED_NAME_PREFIX)))
        .find(|p| !occupied(p))
        .expect("some temp name is free")
}

#[cfg(test)]
mod tests {
    use super::super::parked::is_parked;
    use super::*;

    /// A folder holding `a.wav` = `A` and `b.wav` = `B`, and a runner set to
    /// swap them, stopped just after it parked one (where a Ctrl+C can land).
    fn parked_swap() -> (tempfile::TempDir, Runner, PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let at = |name: &str| tmp.path().join(name);
        std::fs::write(at("a.wav"), "A").unwrap();
        std::fs::write(at("b.wav"), "B").unwrap();
        let swap = |src: &str, dest: &str| Move {
            src: at(src),
            dest: at(dest),
        };
        let mut runner = Runner::new(vec![swap("a.wav", "b.wav"), swap("b.wav", "a.wav")]);
        let start = runner.pending.keys().next().cloned().unwrap();
        let (_, cycle_at) = runner.blocking_chain(start);
        runner.park(&cycle_at.expect("a swap is a cycle")).unwrap();
        let parked = runner.parked[0].clone();
        (tmp, runner, parked)
    }

    #[test]
    fn test_a_parked_file_is_named_as_file_collect_skips() {
        let (_tmp, mut runner, parked) = parked_swap();
        let name = parked.file_name().unwrap().to_string_lossy().into_owned();
        assert!(name.starts_with(crate::PARKED_NAME_PREFIX), "{name}");
        runner.run().unwrap();
    }

    #[test]
    fn test_a_parked_file_is_registered_until_its_final_rename() {
        // RED (ISS-53): Ctrl+C runs `process::exit`, so a run stopped between
        // parking a file and its final rename reports nothing; the handler
        // can only name what is registered.
        let (tmp, mut runner, parked) = parked_swap();
        assert!(parked.exists());
        assert!(is_parked(&parked), "the quitter must be able to name it");
        runner.run().unwrap();
        assert!(!is_parked(&parked));
        assert_eq!(
            std::fs::read_to_string(tmp.path().join("a.wav")).unwrap(),
            "B"
        );
        assert_eq!(
            std::fs::read_to_string(tmp.path().join("b.wav")).unwrap(),
            "A"
        );
    }

    #[test]
    fn test_a_failed_run_keeps_its_parked_file_registered() {
        // RED (ISS-53): the error names the parked file, but Ctrl+C can exit
        // before that error is printed, so the report names it too.
        let (tmp, mut runner, parked) = parked_swap();
        std::fs::write(tmp.path().join("a.wav"), "LATE").unwrap();
        let error = runner.run().map_err(|e| runner.failure(&e, 2)).unwrap_err();
        assert!(error.contains(&parked.display().to_string()), "{error}");
        assert!(is_parked(&parked));
        parked::unregister(&parked);
    }
}
