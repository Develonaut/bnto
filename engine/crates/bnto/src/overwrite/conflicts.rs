// Refuses a rename plan that would replace a file.
//
// Names are compared case-folded to find candidates (two names that differ
// only in case are one file on Windows and macOS); whether a target is really
// the file moving away is decided by file identity, so a case-sensitive
// filesystem holding both `Kick.wav` and `kick.wav` is handled too.

use std::collections::HashMap;

use super::plan::{Move, refused};
use super::{key, occupied, same_file};

/// How many conflicts the refusal lists before folding the rest into a count.
const MAX_CONFLICTS_LISTED: usize = 10;

/// Fail, listing every conflict, when the plan would replace any file.
pub(super) fn refuse_conflicts(moves: &[Move]) -> Result<(), String> {
    let mut conflicts = case_twins(moves);
    conflicts.extend(shared_targets(moves));
    conflicts.extend(occupied_targets(moves));
    if conflicts.is_empty() {
        return Ok(());
    }
    Err(refusal_text(&conflicts))
}

/// Moving sources whose names differ only in case. They're distinct files
/// (a case-sensitive filesystem), and renames keyed by folded name can't
/// tell them apart, so the plan is refused rather than guessed at.
fn case_twins(moves: &[Move]) -> Vec<String> {
    grouped(moves, |m| key(&m.src))
        .into_iter()
        .map(|group| format!("{} differ only in case; rename one first", sources(&group)))
        .collect()
}

fn shared_targets(moves: &[Move]) -> Vec<String> {
    grouped(moves, |m| key(&m.dest))
        .into_iter()
        .map(|group| {
            format!(
                "{} would all be renamed to {}",
                sources(&group),
                group[0].dest.display()
            )
        })
        .collect()
}

/// Moves whose target holds a file that isn't one of the files moving away.
fn occupied_targets(moves: &[Move]) -> Vec<String> {
    let moving: HashMap<String, &Move> = moves.iter().map(|m| (key(&m.src), m)).collect();
    moves
        .iter()
        .filter(|m| occupied(&m.dest))
        .filter(|m| {
            !moving
                .get(&key(&m.dest))
                .is_some_and(|v| same_file(&v.src, &m.dest))
        })
        .map(|m| {
            format!(
                "{} would replace {}, which isn't being renamed away",
                m.src.display(),
                m.dest.display()
            )
        })
        .collect()
}

/// Groups of two or more moves sharing a key, in a stable order.
fn grouped(moves: &[Move], key_of: impl Fn(&Move) -> String) -> Vec<Vec<&Move>> {
    let mut by_key: HashMap<String, Vec<&Move>> = HashMap::new();
    for m in moves {
        by_key.entry(key_of(m)).or_default().push(m);
    }
    let mut groups: Vec<Vec<&Move>> = by_key.into_values().filter(|g| g.len() > 1).collect();
    groups.sort_by(|a, b| a[0].src.cmp(&b[0].src));
    groups
}

fn sources(group: &[&Move]) -> String {
    let names: Vec<String> = group.iter().map(|m| m.src.display().to_string()).collect();
    names.join(", ")
}

pub(super) fn refusal_text(conflicts: &[String]) -> String {
    let mut lines: Vec<String> = conflicts
        .iter()
        .take(MAX_CONFLICTS_LISTED)
        .map(|c| format!("  {c}"))
        .collect();
    if conflicts.len() > MAX_CONFLICTS_LISTED {
        lines.push(format!(
            "  …and {} more",
            conflicts.len() - MAX_CONFLICTS_LISTED
        ));
    }
    let noun = if conflicts.len() == 1 {
        "conflict"
    } else {
        "conflicts"
    };
    refused(&format!("{} {noun}\n{}", conflicts.len(), lines.join("\n")))
}
