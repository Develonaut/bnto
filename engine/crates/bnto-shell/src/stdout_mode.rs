// Stdout output mode — the command's stdout becomes one output file.

use bnto_core::processor::{FileData, NodeInput, NodeOutput, OutputFile};
use bnto_core::{BntoError, ProcessContext, ProgressReporter};

use crate::invocation::Invocation;
use crate::validate;

/// Stdout mode: capture command stdout as a single output file. `program` is
/// what runs (the full path from `searchPath`, or `command` itself); the
/// output is named after `command`.
#[expect(clippy::too_many_lines)]
pub(crate) fn process_stdout_mode(
    invocation: &Invocation,
    input: &NodeInput,
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
    progress.report(10, &format!("Running {command}..."));
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let filename = make_output_filename(command, &input.filename);
    let limit = validate::max_output_bytes(max_output_mb);
    let output_bytes = ctx
        .run_command_with(program, &arg_refs, options, &|line| {
            invocation.relay(line, progress);
        })
        .map_err(|e| hand_over_stdout(invocation.readable(e), command, &filename, limit))?;
    if output_bytes.len() > limit {
        return Err(BntoError::ProcessingFailed(format!(
            "Command output exceeded {max_output_mb} MB limit"
        )));
    }
    progress.report(100, "Done");
    Ok(stdout_output(command, output_bytes, filename))
}

/// A command that exited non-zero still fails the step, but hands over the
/// stdout it printed (within the size limit) as the step's output file.
fn hand_over_stdout(error: BntoError, command: &str, filename: &str, limit: usize) -> BntoError {
    match error {
        BntoError::CommandFailed { message, stdout } => {
            let error = BntoError::ProcessingFailed(message);
            if stdout.len() > limit {
                return error;
            }
            let output = stdout_output(command, stdout, filename.to_string());
            BntoError::with_output(error, output)
        }
        other => other,
    }
}

/// One octet-stream output file plus `command` / `outputBytes` metadata.
fn stdout_output(command: &str, output_bytes: Vec<u8>, filename: String) -> NodeOutput {
    let mut metadata = serde_json::Map::new();
    metadata.insert("command".into(), command.into());
    metadata.insert(
        "outputBytes".into(),
        serde_json::Number::from(output_bytes.len()).into(),
    );
    NodeOutput {
        files: vec![OutputFile {
            data: FileData::Bytes(output_bytes),
            filename,
            mime_type: "application/octet-stream".to_string(),
            metadata: serde_json::Map::new(),
        }],
        metadata,
    }
}

/// Generate a filename for stdout mode output.
pub(crate) fn make_output_filename(command: &str, input_filename: &str) -> String {
    if input_filename.is_empty() {
        format!("{command}-output")
    } else {
        let stem = bnto_core::file_stem(input_filename);
        format!("{stem}-{command}-output")
    }
}

#[cfg(test)]
mod tests {
    use crate::failing_tool::{FailingTool, MESSAGE, STDOUT};
    use bnto_core::BntoError;

    #[test]
    fn test_failed_command_hands_over_its_stdout_with_the_failure() {
        let err = FailingTool::new().run(serde_json::json!({})).err().unwrap();
        assert_eq!(err.to_string(), format!("Processing failed: {MESSAGE}"));
        let (error, output) = err.into_parts();
        assert!(matches!(error, BntoError::ProcessingFailed(_)), "{error:?}");
        let output = output.expect("stdout handed over");
        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files[0].filename, "tool-output");
        assert_eq!(output.files[0].data.to_bytes().unwrap().as_ref(), STDOUT);
        assert_eq!(output.metadata["outputBytes"], STDOUT.len());
    }

    #[test]
    fn test_failed_command_with_oversized_stdout_hands_over_nothing() {
        let params = serde_json::json!({ "maxOutputSize": 0 });
        let err = FailingTool::new().run(params).err().unwrap();
        let (error, output) = err.into_parts();
        assert_eq!(error.to_string(), format!("Processing failed: {MESSAGE}"));
        assert!(output.is_none());
    }

    #[test]
    fn test_command_missing_from_search_path_fails_before_running_with_no_output() {
        let empty = tempfile::tempdir().unwrap();
        let params = serde_json::json!({ "searchPath": [empty.path().to_str().unwrap()] });
        let err = FailingTool::new().run(params).err().unwrap();
        let (error, output) = err.into_parts();
        assert!(
            error
                .to_string()
                .contains("'tool' was not found in searchPath"),
            "{error}"
        );
        assert!(output.is_none(), "nothing ran, so nothing to hand over");
    }

    #[test]
    fn test_output_filename_keeps_a_dotted_folder() {
        // ISS-33: the dot in the folder `v1.2` used to split the name.
        let name = super::make_output_filename("ls", "v1.2/data");
        assert_eq!(name, "v1.2/data-ls-output");
    }
}
