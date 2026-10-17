// File output mode — run the command in a temp dir and collect what it writes.

use std::path::PathBuf;

use bnto_core::processor::{NodeInput, NodeOutput, OutputFile};
use bnto_core::{BntoError, ProcessContext, ProgressReporter};

use crate::output_files::collect_output_files;

/// Placeholder in args that gets replaced with the temp output directory path.
pub(crate) const OUTPUT_DIR_PLACEHOLDER: &str = "{{output_dir}}";

/// File mode: run command in a temp dir, collect written files as output.
/// Replaces `{{output_dir}}` in args with the temp directory path.
pub(crate) fn process_file_mode(
    command: &str,
    program: &str,
    args: &[String],
    _input: &NodeInput,
    progress: &ProgressReporter,
    ctx: &dyn ProcessContext,
    max_output_mb: u64,
) -> Result<NodeOutput, BntoError> {
    let output_dir = create_output_dir(ctx)?;
    let dir_str = output_dir.to_string_lossy();
    let resolved_args = substitute_output_dir(args, &dir_str);
    progress.report(10, &format!("Running {command}..."));
    let arg_refs: Vec<&str> = resolved_args.iter().map(String::as_str).collect();
    // Run the command with stderr streaming — stdout is captured but unused.
    let _stdout = ctx.run_command_streaming(program, &arg_refs, &|line| {
        progress.report_output(line);
    })?;
    progress.report(80, "Collecting output files...");
    let files = collect_output_files(&output_dir, max_output_mb)?;
    if files.is_empty() {
        return Err(BntoError::ProcessingFailed(format!(
            "Command '{command}' produced no output files in {dir_str}"
        )));
    }
    // Don't delete temp dir — files are referenced by path (FileData::Path).
    // Consumers move files out via write_to() / rename(). Empty temp dirs
    // are cleaned by the OS. This avoids reading multi-GB files into memory.
    progress.report(100, "Done");
    Ok(file_output(command, files))
}

/// Create a temp directory for the command's output files.
fn create_output_dir(ctx: &dyn ProcessContext) -> Result<PathBuf, BntoError> {
    let output_dir = ctx.temp_file("-output")?.with_extension("d");
    std::fs::create_dir_all(&output_dir).map_err(|e| {
        BntoError::ProcessingFailed(format!("Failed to create output directory: {e}"))
    })?;
    Ok(output_dir)
}

/// Substitute the `{{output_dir}}` placeholder in args.
fn substitute_output_dir(args: &[String], dir: &str) -> Vec<String> {
    args.iter()
        .map(|a| a.replace(OUTPUT_DIR_PLACEHOLDER, dir))
        .collect()
}

/// The collected files plus command / mode / byte / count metadata.
fn file_output(command: &str, files: Vec<OutputFile>) -> NodeOutput {
    let total_bytes: u64 = files.iter().map(|f| f.data.len().unwrap_or(0)).sum();
    let mut metadata = serde_json::Map::new();
    metadata.insert("command".into(), command.into());
    metadata.insert("outputMode".into(), "file".into());
    metadata.insert(
        "outputBytes".into(),
        serde_json::Number::from(total_bytes).into(),
    );
    metadata.insert(
        "fileCount".into(),
        serde_json::Number::from(files.len()).into(),
    );
    NodeOutput { files, metadata }
}
