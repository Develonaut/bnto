// Writes a run's result files so no write replaces a file still to be read.
//
// A result that is a `Path` (often the user's own input) is read when its
// write runs. If another write's destination is that same file (ISS-48:
// `a` renamed to `b.txt` written beside `b.txt`), it waits until the file
// has been read; a swap first copies one file into a staging folder
// (`staging`), as `bnto_file::relocate` parks one file to run a swap of
// moves. A plan whose destinations are one file on disk can't keep both, so
// it's refused before any write: links are found by name and identity
// (`aliases`), and any other names a filesystem treats as one by creating
// every new destination exclusively first (`claims`).
// Every write goes through `GATE`, so a quitting process (Ctrl+C, the TUI
// exiting) stops it between steps instead of ending it mid-copy.

mod aliases;
mod claims;
mod gate;
mod landing;
mod order;
mod progressive;
mod readers;
mod staging;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bnto_core::PipelineFileResult;
use bnto_core::processor::FileData;
use bnto_file::relocate;

use claims::Claims;
use order::Step;
use staging::Staging;

pub use gate::{GATE, WriteGate};
#[cfg(test)]
pub use progressive::already_written;
pub use progressive::{forget_progressive, note_progressive};
pub use staging::report_kept_staging;

/// Refuse a plan two of whose destinations are one file on disk through a
/// link, a folder link or `..`, naming them. A dry run calls this too, so it
/// refuses those as a run would; names only the filesystem treats as one
/// (Unicode forms, Windows short names) are caught by the run's exclusive
/// creates (`claims`), which a dry run doesn't make. `held[i]` marks a
/// result whose made-up name's file already holds its bytes (`output_names`):
/// it's not written, so it clashes only with a write that would replace it.
pub fn check(files: &[PipelineFileResult], paths: &[PathBuf], held: &[bool]) -> Result<(), String> {
    let pairs = aliases::shared_kept(paths, &left_as_is(files, paths, held));
    if pairs.is_empty() {
        return Ok(());
    }
    let lines = aliases::describe(paths, &pairs);
    let listing = relocate::listing(&lines, ("conflict", "conflicts"));
    Err(refusal(&listing, &progressive::already_written(paths)))
}

/// Which writes leave their destination as is: a held file, or a file
/// already at its destination (ISS-36). `held` may be shorter (none held).
fn left_as_is(files: &[PipelineFileResult], paths: &[PathBuf], held: &[bool]) -> Vec<bool> {
    let held_at = |i: usize| held.get(i).copied().unwrap_or(false);
    let pairs = files.iter().zip(paths).enumerate();
    pairs
        .map(|(i, (f, p))| held_at(i) || f.data.is_at(p))
        .collect()
}

/// The refusal text. Files a progressive loop already wrote are named: the
/// final write wrote nothing, but the run did.
fn refusal(listing: &str, by_iterations: &[PathBuf]) -> String {
    if by_iterations.is_empty() {
        return format!("Output refused, nothing was written: {listing}");
    }
    let names: Vec<String> = by_iterations
        .iter()
        .map(|p| p.display().to_string())
        .collect();
    let wrote = relocate::listing(&names, ("file", "files"));
    format!(
        "Output refused, the final write wrote nothing: {listing}\n  \
         Loop iterations already wrote (left as they are): {wrote}"
    )
}

/// Write `files[i]` to `paths[i]` for every `i`, ordered so a write never
/// replaces a file a later write still reads. A swap's staged copies go in
/// a `.bnto-staged-*` folder inside `output_dir`, removed once every write
/// is done; a failure to remove it is an error. A `held[i]` result isn't
/// written: its file already holds its bytes (see `check`).
pub fn write_all(
    files: &[PipelineFileResult],
    paths: &[PathBuf],
    held: &[bool],
    output_dir: &Path,
) -> Result<(), String> {
    write_through(&GATE, Writes { files, paths, held }, output_dir)
}

/// One final write's results, destinations and held marks.
#[derive(Clone, Copy)]
struct Writes<'a> {
    files: &'a [PipelineFileResult],
    paths: &'a [PathBuf],
    held: &'a [bool],
}

/// What a write stopped by a quitting process reports. `bnto run` can print
/// it (as `Error writing output: …`) before the Ctrl+C handler exits, so it
/// reads as a full sentence; kept staging folders are named after it. (A
/// quitting TUI has already left the screen that would show it.)
const STOPPED: &str = "Stopped: bnto is quitting. The files written before that are kept.";

/// `write_all` through `gate`, which stops it between steps when bnto quits.
fn write_through(gate: &WriteGate, writes: Writes, output_dir: &Path) -> Result<(), String> {
    let Writes { files, paths, held } = writes;
    check(files, paths, held)?;
    // First, so it drops last: a quitter waiting on the gate goes on only
    // once `run` below has dropped (staging kept or removed, claims undone).
    let Some(_writing) = gate.enter() else {
        return Err(STOPPED.to_string());
    };
    // Planned before the claims create the new destinations empty.
    let kept = left_as_is(files, paths, held);
    let steps = order::order(&readers::waits_kept(files, paths, &kept));
    let mut run = Run::new(writes, output_dir)?;
    for step in steps {
        if gate.stopping() {
            // Dropping `run` keeps a staging folder holding copies (still
            // registered for the quitter's report) and undoes unreleased claims.
            return Err(STOPPED.to_string());
        }
        if let Err(e) = run.step(step) {
            return Err(run.failed(e));
        }
    }
    run.staging.finish()
}

struct Run<'a> {
    files: &'a [PipelineFileResult],
    paths: &'a [PathBuf],
    held: &'a [bool],
    staging: Staging,
    /// Writes that read a staged copy instead of their own source.
    staged: HashMap<usize, FileData>,
    claims: Claims,
    /// How one file is written: `write_one`, or a failing stand-in in tests.
    write: fn(&FileData, &Path) -> Result<(), String>,
}

impl<'a> Run<'a> {
    /// A run whose new destinations are claimed; refused if two clash. A
    /// held file exists, so it's never claimed.
    fn new(writes: Writes<'a>, output_dir: &Path) -> Result<Self, String> {
        Ok(Self {
            files: writes.files,
            paths: writes.paths,
            held: writes.held,
            staging: Staging::new(output_dir),
            staged: HashMap::new(),
            claims: Claims::claim(writes.paths, &claims::missing(writes.paths))?,
            write: write_one,
        })
    }

    fn step(&mut self, step: Step) -> Result<(), String> {
        match step {
            Step::Stage { at, readers } => {
                let copy = self.staging.stage(&self.paths[at])?;
                for reader in readers {
                    self.staged.insert(reader, FileData::Path(copy.clone()));
                }
                Ok(())
            }
            // A held file already holds the result's bytes: left as is.
            Step::Write(i) if self.held.get(i) == Some(&true) => Ok(()),
            Step::Write(i) => self.write(i),
        }
    }

    /// Write `i`. The claim already proved a new name free, so the copy
    /// creates the file itself, which lets it clone (APFS). If it then fails
    /// (disk full, file too large), the empty file it created is removed: the
    /// name was missing at planning, so it's nobody's file.
    fn write(&mut self, i: usize) -> Result<(), String> {
        let new = self.claims.release(i);
        let data = self.staged.get(&i).unwrap_or(&self.files[i].data);
        let written = (self.write)(data, &self.paths[i]);
        if written.is_err() && new {
            claims::remove_if_empty(&self.paths[i]);
        }
        written
    }

    /// `error`, plus where the staged copies were kept, if there are any.
    fn failed(mut self, error: String) -> String {
        match self.staging.keep() {
            Some(dir) => format!(
                "{error}\n  Copies set aside for a swap are kept in {}",
                dir.display()
            ),
            None => error,
        }
    }
}

/// Copy one file to `out_path`, creating its folder. `copy_to` leaves a file
/// already at its destination as is.
fn write_one(data: &FileData, out_path: &Path) -> Result<(), String> {
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Cannot create {}: {e}", parent.display()))?;
    }
    data.copy_to(out_path)
        .map_err(|e| format!("Cannot write {}: {e}", out_path.display()))
}

#[cfg(test)]
mod tests;
