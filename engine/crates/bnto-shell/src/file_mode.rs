// File output mode — run the command in a temp dir and collect what it writes.

use std::path::{Path, PathBuf};

use bnto_core::processor::{NodeInput, NodeOutput, OutputFile};
use bnto_core::{BntoError, CommandOptions, ProcessContext, ProgressReporter};

use crate::invocation::Invocation;
use crate::output_files::collect_output_files;
use crate::powershell::ARG_PREFIX;

/// Placeholder in args that gets replaced with the temp output directory path.
pub(crate) const OUTPUT_DIR_PLACEHOLDER: &str = "{{output_dir}}";

/// File mode: run command in a temp dir, collect written files as output.
/// Replaces `{{output_dir}}` in args with the temp directory path.
pub(crate) fn process_file_mode(
    invocation: &Invocation,
    _input: &NodeInput,
    progress: &ProgressReporter,
    ctx: &dyn ProcessContext,
    max_output_mb: u64,
) -> Result<NodeOutput, BntoError> {
    let Invocation {
        command,
        program,
        args,
        options,
        ..
    } = invocation;
    let output_dir = create_output_dir(ctx)?;
    let dir_str = output_dir.to_string_lossy();
    let resolved_args = substitute_output_dir(args, &dir_str);
    let options = substitute_output_dir_in_values(options, &dir_str);
    progress.report(10, &format!("Running {command}..."));
    let arg_refs: Vec<&str> = resolved_args.iter().map(String::as_str).collect();
    // Run the command with stderr streaming — stdout is captured but unused.
    let _stdout = ctx
        .run_command_with(program, &arg_refs, &options, &|line| {
            invocation.relay(line, progress);
        })
        .map_err(|e| {
            let error = invocation.readable(e);
            hand_over_written(error, command, &output_dir, max_output_mb)
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

/// A command that exited non-zero still fails the step, but hands over the
/// files it wrote before failing (a build log, say) as the step's output.
fn hand_over_written(
    error: BntoError,
    command: &str,
    output_dir: &Path,
    max_output_mb: u64,
) -> BntoError {
    let BntoError::CommandFailed { message, .. } = error else {
        return error;
    };
    let error = BntoError::ProcessingFailed(message);
    match collect_output_files(output_dir, max_output_mb) {
        Ok(files) if !files.is_empty() => {
            BntoError::with_output(error, file_output(command, files))
        }
        _ => error,
    }
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

/// Substitute `{{output_dir}}` in the values a PowerShell script reads from
/// `$env:BNTO_ARG_n`, as in args. The step's own `env` is left as written.
fn substitute_output_dir_in_values(options: &CommandOptions, dir: &str) -> CommandOptions {
    let mut options = options.clone();
    for (name, value) in &mut options.env {
        if name.starts_with(ARG_PREFIX) {
            *value = value.replace(OUTPUT_DIR_PLACEHOLDER, dir);
        }
    }
    options
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::failing_tool::{FailingTool, MESSAGE};

    #[test]
    fn output_dir_is_filled_in_a_powershell_scripts_values_only() {
        let options = CommandOptions {
            env: vec![
                ("BNTO_ARG_1".into(), "{{output_dir}}/out.txt".into()),
                ("MY_VAR".into(), "{{output_dir}}".into()),
            ],
            ..CommandOptions::default()
        };
        let filled = substitute_output_dir_in_values(&options, "/tmp/d");
        assert_eq!(filled.env[0].1, "/tmp/d/out.txt");
        assert_eq!(filled.env[1].1, "{{output_dir}}");
    }

    #[test]
    fn test_failed_command_hands_over_the_files_it_wrote() {
        let params = serde_json::json!({ "outputMode": "file", "args": ["{{output_dir}}"] });
        let err = FailingTool::new().run(params).err().unwrap();
        assert_eq!(err.to_string(), format!("Processing failed: {MESSAGE}"));
        let output = err.into_parts().1.expect("written files handed over");
        let names: Vec<&str> = output.files.iter().map(|f| f.filename.as_str()).collect();
        assert_eq!(names, ["build.log"]);
    }

    #[test]
    fn test_failed_command_that_wrote_nothing_hands_over_nothing() {
        let params = serde_json::json!({ "outputMode": "file" });
        let err = FailingTool::new().run(params).err().unwrap();
        assert_eq!(err.to_string(), format!("Processing failed: {MESSAGE}"));
        assert!(err.into_parts().1.is_none());
    }
}
