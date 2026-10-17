// Stdout output mode — the command's stdout becomes one output file.

use bnto_core::processor::{FileData, NodeInput, NodeOutput, OutputFile};
use bnto_core::{BntoError, ProcessContext, ProgressReporter};

use crate::validate;

/// Stdout mode: capture command stdout as a single output file. `program` is
/// what runs (the full path from `search_path`, or `command` itself); the
/// output is named after `command`.
pub(crate) fn process_stdout_mode(
    command: &str,
    program: &str,
    args: &[String],
    input: &NodeInput,
    progress: &ProgressReporter,
    ctx: &dyn ProcessContext,
    max_output_mb: u64,
) -> Result<NodeOutput, BntoError> {
    progress.report(10, &format!("Running {command}..."));
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let output_bytes = ctx.run_command_streaming(program, &arg_refs, &|line| {
        progress.report_output(line);
    })?;
    if output_bytes.len() > validate::max_output_bytes(max_output_mb) {
        return Err(BntoError::ProcessingFailed(format!(
            "Command output exceeded {max_output_mb} MB limit"
        )));
    }
    progress.report(100, "Done");
    let filename = make_output_filename(command, &input.filename);
    Ok(stdout_output(command, output_bytes, filename))
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
        let stem = input_filename
            .rsplit_once('.')
            .map(|(s, _)| s)
            .unwrap_or(input_filename);
        format!("{stem}-{command}-output")
    }
}
