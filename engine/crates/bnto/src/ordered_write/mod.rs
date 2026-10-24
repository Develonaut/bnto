// Writes a run's result files so no write replaces a file still to be read.
//
// A result that is a `Path` (often the user's own input) is read when its
// write runs. If another write's destination is that same file (ISS-48:
// `a` renamed to `b.txt` written beside `b.txt`), it waits until the file
// has been read; a swap stages one file aside first, as `bnto_file::relocate`
// parks one file to run a swap of moves. A plan whose destinations are one
// file through a link can't keep both, so it's refused before any write.

mod aliases;
mod order;
mod readers;
mod staging;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bnto_core::PipelineFileResult;
use bnto_core::processor::FileData;
use bnto_file::relocate;

use order::Step;
use staging::Staging;

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
    Err(format!("Output refused, nothing was written: {listing}"))
}

/// Write `files[i]` to `paths[i]` for every `i`, ordered so a write never
/// replaces a file a later write still reads. Staged copies go in a hidden
/// folder inside `output_dir`; a failure to remove it is an error.
pub fn write_all(
    files: &[PipelineFileResult],
    paths: &[PathBuf],
    output_dir: &Path,
) -> Result<(), String> {
    check(files, paths)?;
    let mut run = Run {
        files,
        paths,
        staging: Staging::new(output_dir),
        staged: HashMap::new(),
    };
    for step in order::order(&readers::waits(files, paths)) {
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
}

impl Run<'_> {
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
                write_one(data, &self.paths[i])
            }
        }
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
