// file-copy `newer` mode — copy only when the source is newer than the target.
//
// Keeps mtimes comparable across FAT and Google Drive, which round them, by
// treating a gap within `tolerance_ms` as equal. Never deletes anything.

use bnto_core::BntoError;
use bnto_core::processor::{FileData, NodeInput, NodeOutput, OutputFile};
use filetime::FileTime;
use std::fs::{self, OpenOptions};
use std::path::Path;

/// Default tolerance when `tolerance_ms` is not set.
pub(crate) const DEFAULT_TOLERANCE_MS: u64 = 2000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Copied,
    UpToDate,
    SkippedNewer,
}

/// Copy `input` to `dest_path` if the target is missing or older, and report
/// the outcome per file. Path data only: bytes carry no source time.
pub(crate) fn process_newer(input: NodeInput, dest_path: &Path) -> Result<NodeOutput, BntoError> {
    let FileData::Path(src) = &input.data else {
        return Err(BntoError::InvalidInput(
            "conflict \"newer\" needs a source file on disk; bytes have no modification time"
                .to_string(),
        ));
    };
    if dest_path.is_dir() {
        return Err(BntoError::InvalidInput(format!(
            "destination {} is a directory, not a file",
            dest_path.display()
        )));
    }
    let outcome = decide(src, dest_path, tolerance_ms(&input.params)?)?;
    if outcome == Outcome::Copied {
        overwrite_in_place(src, dest_path)?;
    }
    Ok(passthrough_output(input, outcome))
}

/// The input file handed on unchanged, flagged with what happened to it.
fn passthrough_output(input: NodeInput, outcome: Outcome) -> NodeOutput {
    let file = OutputFile {
        filename: input.filename,
        mime_type: input
            .mime_type
            .unwrap_or_else(|| "application/octet-stream".to_string()),
        data: input.data,
        metadata: outcome_metadata(outcome),
    };
    NodeOutput {
        files: vec![file],
        metadata: serde_json::Map::new(),
    }
}

/// `tolerance_ms` as a whole number of milliseconds, 0 or more; the default when unset.
fn tolerance_ms(params: &serde_json::Map<String, serde_json::Value>) -> Result<u64, BntoError> {
    match params.get("tolerance_ms") {
        None => Ok(DEFAULT_TOLERANCE_MS),
        // as_f64 so `2000.0` (what the TUI and `{{fields.x}}` defaults emit) counts.
        Some(v) => v
            .as_f64()
            .filter(|n| n.is_finite() && *n >= 0.0 && n.fract() == 0.0)
            .map(|n| n as u64)
            .ok_or_else(|| {
                BntoError::InvalidInput(format!(
                    "tolerance_ms must be a whole number of milliseconds, 0 or more; got {v}"
                ))
            }),
    }
}

/// Per-file 0/1 flags: exactly one of the three is 1.
fn outcome_metadata(outcome: Outcome) -> serde_json::Map<String, serde_json::Value> {
    [
        ("copied", Outcome::Copied),
        ("up_to_date", Outcome::UpToDate),
        ("skipped_newer", Outcome::SkippedNewer),
    ]
    .into_iter()
    .map(|(key, o)| (key.to_string(), serde_json::json!(u8::from(o == outcome))))
    .collect()
}

fn mtime_ms(path: &Path) -> Result<i128, BntoError> {
    let meta = fs::metadata(path).map_err(|e| {
        BntoError::ProcessingFailed(format!("Failed to stat {}: {e}", path.display()))
    })?;
    let t = FileTime::from_last_modification_time(&meta);
    Ok(i128::from(t.unix_seconds()) * 1000 + i128::from(t.nanoseconds() / 1_000_000))
}

fn decide(src: &Path, dest: &Path, tolerance_ms: u64) -> Result<Outcome, BntoError> {
    if !dest.exists() {
        return Ok(Outcome::Copied);
    }
    let gap = mtime_ms(src)? - mtime_ms(dest)?;
    let tolerance = i128::from(tolerance_ms);
    Ok(if gap > tolerance {
        Outcome::Copied
    } else if gap < -tolerance {
        Outcome::SkippedNewer
    } else {
        Outcome::UpToDate
    })
}

fn io_fail(what: &str, path: &Path, e: std::io::Error) -> BntoError {
    BntoError::ProcessingFailed(format!("Failed to {what} {}: {e}", path.display()))
}

/// Write the source bytes into the existing target file (not remove+recreate,
/// so hard links to it see the new bytes), then stamp the source's mtime
/// (read before the copy, so a source saved mid-copy never stamps its newer
/// time on the older bytes).
fn overwrite_in_place(src: &Path, dest: &Path) -> Result<(), BntoError> {
    let src_meta = fs::metadata(src).map_err(|e| io_fail("stat source", src, e))?;
    let src_mtime = FileTime::from_last_modification_time(&src_meta);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| io_fail("create directory for", dest, e))?;
    }
    clear_read_only(dest)?;
    let mut from = fs::File::open(src).map_err(|e| io_fail("read source", src, e))?;
    let mut to = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dest)
        .map_err(|e| io_fail("open", dest, e))?;
    std::io::copy(&mut from, &mut to).map_err(|e| io_fail("write", dest, e))?;
    drop(to);
    filetime::set_file_mtime(dest, src_mtime)
        .map_err(|e| io_fail("set modification time of", dest, e))
}

fn clear_read_only(dest: &Path) -> Result<(), BntoError> {
    let Ok(meta) = fs::metadata(dest) else {
        return Ok(());
    };
    let mut perms = meta.permissions();
    if owner_can_write(&perms) {
        return Ok(());
    }
    add_owner_write(&mut perms);
    fs::set_permissions(dest, perms).map_err(|e| io_fail("clear read-only on", dest, e))
}

/// Unix: the owner's write bit, since `readonly()` is false while any write bit is set.
#[cfg(unix)]
fn owner_can_write(perms: &fs::Permissions) -> bool {
    use std::os::unix::fs::PermissionsExt;
    perms.mode() & 0o200 != 0
}

#[cfg(not(unix))]
fn owner_can_write(perms: &fs::Permissions) -> bool {
    !perms.readonly()
}

/// Unix: add the owner's write bit only, never group or world write.
#[cfg(unix)]
fn add_owner_write(perms: &mut fs::Permissions) {
    use std::os::unix::fs::PermissionsExt;
    perms.set_mode(perms.mode() | 0o200);
}

/// Windows: clears the read-only attribute, the only write control there.
#[cfg(not(unix))]
fn add_owner_write(perms: &mut fs::Permissions) {
    #[allow(clippy::permissions_set_readonly_false)]
    perms.set_readonly(false);
}
