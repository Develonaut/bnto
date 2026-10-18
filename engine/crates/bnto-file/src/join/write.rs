// The joined bytes: in memory when every input already is (the browser's
// case), otherwise streamed file by file into a temp file from the context,
// so joining large disk files never holds them in memory.

use std::fs::File;
use std::io::{BufWriter, Write};

use bnto_core::processor::{BatchFile, FileData};
use bnto_core::{BntoError, ProcessContext};

/// `files` joined in order, `separator` between them.
pub(super) fn joined(
    files: &[BatchFile],
    separator: &[u8],
    ctx: &dyn ProcessContext,
) -> Result<FileData, BntoError> {
    refuse_missing(files)?;
    if files.iter().all(|f| matches!(f.data, FileData::Bytes(_))) {
        let mut out = Vec::new();
        write_all(files, separator, &mut out)?;
        return Ok(FileData::Bytes(out));
    }
    let path = ctx.temp_file("-join")?;
    let streamed = File::create(&path)
        .map_err(|e| cannot_write(&e))
        .and_then(|file| {
            let mut out = BufWriter::new(file);
            write_all(files, separator, &mut out)?;
            out.flush().map_err(|e| cannot_write(&e))
        });
    if let Err(error) = streamed {
        let _ = std::fs::remove_file(&path);
        return Err(error);
    }
    Ok(FileData::Path(path))
}

/// Fail before writing anything when a disk file is gone, naming every one.
fn refuse_missing(files: &[BatchFile]) -> Result<(), BntoError> {
    let missing: Vec<String> = files
        .iter()
        .filter_map(|f| match &f.data {
            FileData::Path(path) if !path.exists() => {
                Some(format!("{} ({})", f.filename, path.display()))
            }
            _ => None,
        })
        .collect();
    if missing.is_empty() {
        return Ok(());
    }
    let verb = if missing.len() == 1 {
        "exists"
    } else {
        "exist"
    };
    Err(BntoError::InvalidInput(format!(
        "Cannot join: {} no longer {verb}",
        missing.join(", ")
    )))
}

fn write_all<W: Write>(
    files: &[BatchFile],
    separator: &[u8],
    out: &mut W,
) -> Result<(), BntoError> {
    for (i, file) in files.iter().enumerate() {
        if i > 0 {
            out.write_all(separator).map_err(|e| cannot_write(&e))?;
        }
        append(file, out)?;
    }
    Ok(())
}

/// One file's bytes onto `out`; a disk file is copied through a buffer.
fn append<W: Write>(file: &BatchFile, out: &mut W) -> Result<(), BntoError> {
    let failed = |e: std::io::Error| {
        BntoError::ProcessingFailed(format!("Cannot join {}: {e}", file.filename))
    };
    match &file.data {
        FileData::Bytes(bytes) => out.write_all(bytes).map_err(failed),
        FileData::Path(path) => {
            let mut source = File::open(path).map_err(failed)?;
            std::io::copy(&mut source, out).map(drop).map_err(failed)
        }
    }
}

fn cannot_write(e: &std::io::Error) -> BntoError {
    BntoError::ProcessingFailed(format!("Cannot write the joined file: {e}"))
}
