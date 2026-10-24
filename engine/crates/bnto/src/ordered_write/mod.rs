// Writes a run's result files so no write replaces a file still to be read.
//
// A result that is a `Path` (often the user's own input) is read when its
// write runs. If another write's destination is that same file (ISS-48:
// `a` renamed to `b.txt` written beside `b.txt`), it waits until the file
// has been read; a swap stages one file aside first, as `bnto_file::relocate`
// parks one file to run a swap of moves. A plan whose destinations are one
// file on disk can't keep both, so it's refused before any write: links are
// found by name and identity (`aliases`), and any other names a filesystem
// treats as one by creating every new destination exclusively (`claims`).
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
pub use progressive::{forget_progressive, note_progressive};
pub use staging::report_kept_staging;

/// Refuse a plan two of whose destinations are one file on disk (a link),
/// naming them. A dry run calls this too, so it refuses what a run would.
pub fn check(files: &[PipelineFileResult], paths: &[PathBuf]) -> Result<(), String> {
    let pairs = aliases::shared_destinations(files, paths);
    if pairs.is_empty() {
        return Ok(());
    }
    let lines = aliases::describe(paths, &pairs);
    let listing = relocate::listing(&lines, ("conflict", "conflicts"));
    Err(refusal(&listing, &progressive::already_written(paths)))
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
/// replaces a file a later write still reads. Staged copies go in a hidden
/// folder inside `output_dir`; a failure to remove it is an error.
pub fn write_all(
    files: &[PipelineFileResult],
    paths: &[PathBuf],
    output_dir: &Path,
) -> Result<(), String> {
    write_through(&GATE, files, paths, output_dir)
}

/// What a write stopped by a quitting process reports. Nobody may read it:
/// the quitter reports kept staging folders itself, then exits.
const STOPPED: &str = "Stopped before every file was written: bnto is quitting";

/// `write_all` through `gate`, which stops it between steps when bnto quits.
fn write_through(
    gate: &WriteGate,
    files: &[PipelineFileResult],
    paths: &[PathBuf],
    output_dir: &Path,
) -> Result<(), String> {
    check(files, paths)?;
    // First, so it drops last: a quitter waiting on the gate goes on only
    // once the staging folder below is removed, or kept and registered.
    let Some(_writing) = gate.enter() else {
        return Err(STOPPED.to_string());
    };
    // Planned before the claims create the new destinations empty.
    let steps = order::order(&readers::waits(files, paths));
    let mut run = Run::new(files, paths, output_dir)?;
    for step in steps {
        if gate.stopping() {
            // Dropping `run` keeps a folder holding copies, for the report.
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
    staging: Staging,
    /// Writes that read a staged copy instead of their own source.
    staged: HashMap<usize, FileData>,
    claims: Claims,
}

impl<'a> Run<'a> {
    /// A run whose new destinations are claimed; refused if two clash.
    fn new(
        files: &'a [PipelineFileResult],
        paths: &'a [PathBuf],
        output_dir: &Path,
    ) -> Result<Self, String> {
        Ok(Self {
            files,
            paths,
            staging: Staging::new(output_dir),
            staged: HashMap::new(),
            claims: Claims::claim(paths, &claims::missing(paths))?,
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
            Step::Write(i) => {
                let data = self.staged.get(&i).unwrap_or(&self.files[i].data);
                write_one(data, &self.paths[i])?;
                self.claims.written(i);
                Ok(())
            }
        }
    }

    /// `error`, plus where the staged copies were kept, if there are any.
    fn failed(mut self, error: String) -> String {
        match self.staging.keep() {
            Some(dir) => format!(
                "{error}\n  Files set aside for a swap are kept in {}",
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
