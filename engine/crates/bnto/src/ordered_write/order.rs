// The order a run's writes go in so none replaces a file another still reads.
//
// Pure: works on indexes only. Write `i` waits for every write that reads
// the file at `i`'s destination. Writes with nothing to wait for run in index
// order; when every write left waits on another (a swap, a→b with b→a) one
// destination is staged first, so the writes still to read it read a copy.

use std::collections::BTreeSet;

/// One step of an ordered write.
#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    /// Copy the file at write `at`'s destination aside, and have `readers`
    /// (the writes still to read it) read that copy instead.
    Stage { at: usize, readers: Vec<usize> },
    /// Run write `i`.
    Write(usize),
}

/// Every write once, each after the writes it waits for. `waits[i]` lists
/// the writes that read write `i`'s destination, so must run before it.
pub fn order(waits: &[Vec<usize>]) -> Vec<Step> {
    let mut sorter = Sorter::new(waits);
    let mut unwritten = waits.len();
    while unwritten > 0 {
        match sorter.ready.pop_first() {
            Some(i) => {
                sorter.write(i);
                unwritten -= 1;
            }
            None => sorter.stage(),
        }
    }
    sorter.steps
}

/// Kahn's topological sort, with a stage step wherever a cycle blocks it.
struct Sorter<'a> {
    waits: &'a [Vec<usize>],
    /// For each write, the writes waiting for it.
    unblocks: Vec<Vec<usize>>,
    /// How many writes each write still waits for.
    left: Vec<usize>,
    done: Vec<bool>,
    ready: BTreeSet<usize>,
    steps: Vec<Step>,
}

impl<'a> Sorter<'a> {
    fn new(waits: &'a [Vec<usize>]) -> Self {
        let mut unblocks = vec![Vec::new(); waits.len()];
        for (i, readers) in waits.iter().enumerate() {
            readers.iter().for_each(|&j| unblocks[j].push(i));
        }
        let left: Vec<usize> = waits.iter().map(Vec::len).collect();
        Self {
            ready: (0..waits.len()).filter(|&i| left[i] == 0).collect(),
            waits,
            unblocks,
            left,
            done: vec![false; waits.len()],
            steps: Vec::new(),
        }
    }

    fn write(&mut self, i: usize) {
        self.done[i] = true;
        self.steps.push(Step::Write(i));
        for &k in &self.unblocks[i] {
            // A staged write already has nothing left to wait for.
            if !self.done[k] && self.left[k] > 0 {
                self.left[k] -= 1;
                if self.left[k] == 0 {
                    self.ready.insert(k);
                }
            }
        }
    }

    /// Every write left waits on another: stage one on a cycle, freeing it.
    fn stage(&mut self) {
        let at = self.on_a_cycle();
        let readers = self.waits[at]
            .iter()
            .copied()
            .filter(|&j| !self.done[j])
            .collect();
        self.steps.push(Step::Stage { at, readers });
        self.left[at] = 0;
        self.ready.insert(at);
    }

    /// From the first write left, follow what it waits for until a write
    /// repeats: that one is on a cycle.
    fn on_a_cycle(&self) -> usize {
        let mut at = self.done.iter().position(|d| !d).expect("a write is left");
        let mut seen = vec![false; self.done.len()];
        while !seen[at] {
            seen[at] = true;
            at = self.waits[at]
                .iter()
                .copied()
                .find(|&j| !self.done[j])
                .expect("a blocked write waits for one still to run");
        }
        at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Step::{Stage, Write};

    #[test]
    fn test_writes_with_nothing_to_wait_for_keep_their_order() {
        assert_eq!(
            order(&[vec![], vec![], vec![]]),
            [Write(0), Write(1), Write(2)]
        );
    }

    #[test]
    fn test_a_write_onto_a_later_writes_source_goes_after_it() {
        // ISS-48: `a` → `b.txt` while `b.txt` → `b_1.txt`: `b` is read first.
        assert_eq!(order(&[vec![1], vec![]]), [Write(1), Write(0)]);
    }

    #[test]
    fn test_a_chain_runs_from_its_free_end() {
        assert_eq!(
            order(&[vec![1], vec![2], vec![]]),
            [Write(2), Write(1), Write(0)]
        );
    }

    #[test]
    fn test_a_destination_two_writes_read_waits_for_both() {
        assert_eq!(
            order(&[vec![1, 2], vec![], vec![]]),
            [Write(1), Write(2), Write(0)]
        );
    }

    #[test]
    fn test_a_swap_stages_one_destination_first() {
        // a → b.txt and b → a.txt: `b.txt` is copied aside for `b` to read.
        assert_eq!(
            order(&[vec![1], vec![0]]),
            [
                Stage {
                    at: 0,
                    readers: vec![1]
                },
                Write(0),
                Write(1)
            ]
        );
    }

    #[test]
    fn test_a_three_way_cycle_stages_once() {
        assert_eq!(
            order(&[vec![1], vec![2], vec![0]]),
            [
                Stage {
                    at: 0,
                    readers: vec![1]
                },
                Write(0),
                Write(2),
                Write(1)
            ]
        );
    }

    #[test]
    fn test_a_cycle_after_free_writes_waits_for_them() {
        // 0 is free; 1 and 2 swap. The free write runs before any staging.
        assert_eq!(
            order(&[vec![], vec![2], vec![1]]),
            [
                Write(0),
                Stage {
                    at: 1,
                    readers: vec![2]
                },
                Write(1),
                Write(2)
            ]
        );
    }

    #[test]
    fn test_staging_lists_only_the_readers_still_to_run() {
        // 2 reads 0's destination and runs first (it waits for nothing);
        // 1 and 0 then swap, so only 1 still reads the staged copy.
        assert_eq!(
            order(&[vec![1, 2], vec![0], vec![]]),
            [
                Write(2),
                Stage {
                    at: 0,
                    readers: vec![1]
                },
                Write(0),
                Write(1)
            ]
        );
    }
}
