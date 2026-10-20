// `{"each": [...]}` arg groups — the group once per incoming file.
//
// shell-run gets every file its incoming edges carry. An `each` group in
// `args` repeats once per file, in that order, so one command can take a
// whole collected list (`-object=<path>` per DLL). Inside a group `{{path}}`
// is the file on disk, `{{file}}` its pipeline name and `{{item.KEY}}` its
// metadata. With no files a group adds nothing. Files never reach a command
// any other way, so a recipe without a group runs exactly as before.

use std::path::{Path, PathBuf};

use bnto_core::processor::{BatchFile, FileData};
use bnto_core::{BntoError, ProcessContext};
use serde_json::Value;

mod unfilled;

pub(crate) use unfilled::unfilled_item;

const EACH: &str = "each";
const PATH_TOKEN: &str = "{{path}}";
const FILE_TOKEN: &str = "{{file}}";
const ITEM_OPEN: &str = "{{item.";
const CLOSE: &str = "}}";

/// Tokens shell-run fills after its each groups (SEC-27): a file's path,
/// name or metadata carrying one would have it filled, so it's refused.
const FILLED_LATER: &[&str] = &["{{output_dir}}", "{{url}}", "{{input}}"];

/// Why a group couldn't be filled for a file.
enum FillError {
    /// The group reads `{{item.KEY}}` the file doesn't have.
    Missing(String),
    /// A value to fill in carries a token filled after the group.
    Reserved(String),
}

/// `args` with every `each` group replaced by its filled copies, one per
/// file, as plain strings; other entries are kept as they are.
pub(crate) fn expand_each(
    args: &[Value],
    files: &[BatchFile],
    ctx: &dyn ProcessContext,
) -> Result<Vec<Value>, BntoError> {
    let paths = disk_paths(args, files, ctx)?;
    let mut out = Vec::with_capacity(args.len());
    for arg in args {
        match read_group(arg).map_err(BntoError::InvalidInput)? {
            Some(group) => out.extend(fill_group(group, files, &paths)?),
            None => out.push(arg.clone()),
        }
    }
    Ok(out)
}

/// Every `each` group in `args` is well formed, or the first reason one isn't.
pub(crate) fn check_groups(args: Option<&Value>) -> Result<(), String> {
    let Some(Value::Array(args)) = args else {
        return Ok(());
    };
    args.iter().try_for_each(|arg| read_group(arg).map(|_| ()))
}

/// An `each` group's templates, `None` for any other entry, or why an
/// object entry isn't a group.
fn read_group(arg: &Value) -> Result<Option<Vec<&str>>, String> {
    let Value::Object(map) = arg else {
        return Ok(None);
    };
    let refused = || "an object in 'args' must be {\"each\": [text, ...]}".to_string();
    let Some(Value::Array(items)) = map.get(EACH).filter(|_| map.len() == 1) else {
        return Err(refused());
    };
    let templates = items.iter().map(Value::as_str).collect::<Option<Vec<_>>>();
    templates.map(Some).ok_or_else(refused)
}

/// The group's templates filled for each file in turn.
fn fill_group(
    group: Vec<&str>,
    files: &[BatchFile],
    paths: &[PathBuf],
) -> Result<Vec<Value>, BntoError> {
    let mut out = Vec::with_capacity(group.len() * files.len());
    for (i, file) in files.iter().enumerate() {
        let path = paths.get(i).map(|p| p.to_string_lossy().into_owned());
        for template in &group {
            let text =
                fill(template, file, path.as_deref().unwrap_or("")).map_err(|e| match e {
                    FillError::Missing(key) => missing(&key, file),
                    FillError::Reserved(token) => reserved(&token, file),
                })?;
            out.push(Value::String(text));
        }
    }
    Ok(out)
}

/// `template` filled for one file, or the first `{{item.KEY}}` key it lacks.
/// Only the template is scanned, so a value that reads like a token is
/// written as it is.
fn fill(template: &str, file: &BatchFile, path: &str) -> Result<String, FillError> {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let (piece, after) = token(&rest[start..], file, path)?;
        out.push_str(&piece);
        rest = after;
    }
    out.push_str(rest);
    Ok(out)
}

/// The text for the token `at` starts with, and what follows it. A `{{`
/// that isn't one of the group's tokens is kept as written.
fn token<'a>(at: &'a str, file: &BatchFile, path: &str) -> Result<(String, &'a str), FillError> {
    if let Some(after) = at.strip_prefix(PATH_TOKEN) {
        return Ok((checked(path.to_string())?, after));
    }
    if let Some(after) = at.strip_prefix(FILE_TOKEN) {
        return Ok((checked(file.filename.clone())?, after));
    }
    let item = at
        .strip_prefix(ITEM_OPEN)
        .and_then(|rest| rest.split_once(CLOSE));
    let Some((key, after)) = item else {
        return Ok((at[..2].to_string(), &at[2..]));
    };
    let value = file
        .metadata
        .get(key)
        .ok_or_else(|| FillError::Missing(key.to_string()))?;
    Ok((checked(text(value))?, after))
}

/// `value`, unless it carries a token shell-run fills after the group.
fn checked(value: String) -> Result<String, FillError> {
    match FILLED_LATER.iter().find(|t| value.contains(**t)) {
        Some(token) => Err(FillError::Reserved(token.to_string())),
        None => Ok(value),
    }
}

/// A metadata value as text: strings as written, other JSON as JSON.
fn text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// A file's path, name or metadata carries `token`, which shell-run fills
/// after the group, so the value can't be told apart from the recipe's own.
fn reserved(token: &str, file: &BatchFile) -> BntoError {
    BntoError::InvalidInput(format!(
        "{} carries '{token}' in its path, name or metadata, which shell-run fills \
         itself; rename it or drop it from the group",
        file.filename
    ))
}

/// A group reads `{{item.KEY}}` a file lacks: name the key, the file and
/// the keys it has.
fn missing(key: &str, file: &BatchFile) -> BntoError {
    let has = if file.metadata.is_empty() {
        "it has no metadata".to_string()
    } else {
        let keys: Vec<&str> = file.metadata.keys().map(String::as_str).collect();
        format!("it has: {}", keys.join(", "))
    };
    BntoError::InvalidInput(format!(
        "'args' reads {{{{item.{key}}}}}, which {} doesn't have ({has})",
        file.filename
    ))
}

/// Each file's path on disk when a group reads `{{path}}`, else none.
fn disk_paths(
    args: &[Value],
    files: &[BatchFile],
    ctx: &dyn ProcessContext,
) -> Result<Vec<PathBuf>, BntoError> {
    let reads_path = args.iter().any(|arg| {
        matches!(read_group(arg), Ok(Some(group)) if group.iter().any(|t| t.contains(PATH_TOKEN)))
    });
    if !reads_path {
        return Ok(Vec::new());
    }
    files.iter().map(|file| disk_path(file, ctx)).collect()
}

/// Where a file is on disk: a reference's path (under the work dir when
/// it's relative), or a temp copy of in-memory bytes, keeping the file's
/// extension for tools that go by it.
fn disk_path(file: &BatchFile, ctx: &dyn ProcessContext) -> Result<PathBuf, BntoError> {
    match &file.data {
        FileData::Path(path) if path.is_absolute() => Ok(path.clone()),
        FileData::Path(path) => Ok(ctx.work_dir()?.join(path)),
        FileData::Bytes(_) => {
            let ext = Path::new(&file.filename)
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();
            let temp = ctx.temp_file(&ext)?;
            file.data.copy_to(&temp).map_err(|e| {
                BntoError::ProcessingFailed(format!("Failed to write {}: {e}", file.filename))
            })?;
            Ok(temp)
        }
    }
}

#[cfg(test)]
mod tests;
