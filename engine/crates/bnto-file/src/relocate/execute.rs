// Runs an accepted move plan in an order that keeps every file.
//
// A move waits until the move vacating its target has run (a→b, b→c runs
// b→c first). A cycle (a→b, b→a) is broken by parking one file under a
// temporary name. Each move re-checks its target first, by file identity,
// so a file that appeared after planning is never replaced. Every move is a
// rename: across drives it fails rather than copying.

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use super::{Move, key, occupied, same_file};

/// Run the moves, creating each target's folder; on failure, say how far the
/// run got and what it left. Check the plan with `conflicts` first.
pub fn run_in_order(moves: Vec<Move>) -> Result<(), String> {
    let total = moves.len();
    let mut runner = Runner {
        pending: moves.into_iter().map(|m| (key(&m.src), m)).collect(),
        done: 0,
        parked: Vec::new(),
    };
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
            self.parked.retain(|p| p != &m.src);
            self.done += 1;
        }
        Ok(())
    }

    /// Break a cycle: move one file to a free temporary name in its folder.
    fn park(&mut self, src_key: &str) -> Result<(), String> {
        let mut m = self
            .pending
            .remove(src_key)
            .expect("cycle keys are pending");
        let temp = temp_path(&m.src);
        let parked = guarded_rename(&m.src, &temp);
        if parked.is_ok() {
            self.parked.push(temp.clone());
            m.src = temp;
        }
        self.pending.insert(key(&m.src), m);
        parked
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
        .map(|n| dir.join(format!(".bnto-rename-{n}-{leaf}")))
        .find(|p| !occupied(p))
        .expect("some temp name is free")
}
