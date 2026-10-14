// Finds the moves in a plan that would replace a file.
//
// Names are compared case-folded to find candidates (two names that differ
// only in case are one file on Windows and macOS); whether a target is really
// the file moving away is decided by file identity, so a case-sensitive
// filesystem holding both `Kick.wav` and `kick.wav` is handled too.

use std::collections::HashMap;

use super::{Move, key, occupied, same_file};

/// How many items a listing shows before folding the rest into a count.
const MAX_LISTED: usize = 10;

/// Every way the plan would replace a file; empty when it's safe to run.
/// Pass only real moves: a move onto its own path reads as a conflict.
pub fn conflicts(moves: &[Move]) -> Vec<String> {
    let mut conflicts = case_twins(moves);
    conflicts.extend(shared_targets(moves));
    conflicts.extend(occupied_targets(moves));
    conflicts
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

/// "3 conflicts" and one indented line per item, at most ten of them.
/// `nouns` is the (singular, plural) word for an item.
pub fn listing(items: &[String], nouns: (&str, &str)) -> String {
    let mut lines: Vec<String> = items
        .iter()
        .take(MAX_LISTED)
        .map(|c| format!("  {c}"))
        .collect();
    if items.len() > MAX_LISTED {
        lines.push(format!("  …and {} more", items.len() - MAX_LISTED));
    }
    let noun = if items.len() == 1 { nouns.0 } else { nouns.1 };
    format!("{} {noun}\n{}", items.len(), lines.join("\n"))
}
