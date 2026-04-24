// ShellCommand processor — execute external CLI tools.
//
// This is the generic "run any command" processor that enables
// connector-as-recipe architecture. Recipes declare what tools
// they need via recipe-level `requires`, and this processor
// executes them via ProcessContext::run_command().
//
// Two output modes:
//   - "stdout" (default): captures command stdout as a single output file
//   - "file": runs command in a temp dir, collects written files as output.
//     Use `{{output_dir}}` in args to inject the temp directory path.
//     Designed for tools like yt-dlp and ffmpeg that write to disk.
//
// Security boundary: validate.rs checks every command before execution.
// See that module for the full threat model (shell denylist, path
// validation, env var sanitization).

use bnto_core::metadata::{InputCardinality, ParameterDef, ParameterType};
use bnto_core::processor::{NodeInput, NodeOutput, OutputFile};
use bnto_core::{
    BntoError, NodeCategory, NodeMetadata, NodeProcessor, ProcessContext, ProgressReporter,
};

use crate::validate::{self, DEFAULT_MAX_OUTPUT_MB, DEFAULT_TIMEOUT_SECS};

/// Placeholder in args that gets replaced with the temp output directory path.
const OUTPUT_DIR_PLACEHOLDER: &str = "{{output_dir}}";

/// Placeholder in args for URL/text input injected by the CLI's input preparation.
const URL_PLACEHOLDER: &str = "{{url}}";
const INPUT_PLACEHOLDER: &str = "{{input}}";

/// Shell command processor — runs external CLI tools.
pub struct ShellCommand;

impl ShellCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ShellCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for ShellCommand {
    fn name(&self) -> &str {
        "shell-command"
    }

    fn validate(&self, params: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
        let mut errors = Vec::new();

        match params.get("command").and_then(serde_json::Value::as_str) {
            None => errors.push("'command' parameter is required".to_string()),
            Some(cmd) => {
                if let Err(e) = validate::validate_command(cmd) {
                    errors.push(e);
                }
            }
        }

        if let Some(timeout) = params.get("timeout").and_then(serde_json::Value::as_u64)
            && timeout == 0
        {
            errors.push("'timeout' must be greater than 0".to_string());
        }

        errors
    }

    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let command = input
            .params
            .get("command")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                BntoError::InvalidInput("'command' parameter is required".to_string())
            })?;

        let mut args: Vec<String> = input
            .params
            .get("args")
            .and_then(serde_json::Value::as_array)
            .map(|arr: &Vec<serde_json::Value>| {
                arr.iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        let _timeout = input
            .params
            .get("timeout")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(DEFAULT_TIMEOUT_SECS);

        // Sanitize env vars — strip dangerous ones silently
        let _env = input
            .params
            .get("env")
            .and_then(serde_json::Value::as_object)
            .map(validate::sanitize_env)
            .unwrap_or_default();

        // Security check: reject shell interpreters and path-based commands
        validate::validate_command(command).map_err(BntoError::InvalidInput)?;

        // Resolve {{url}} and {{input}} placeholders from injected params.
        // The CLI injects "url" or "text" params for URL/Text mode recipes.
        resolve_input_placeholders(&mut args, &input.params);

        let max_output_mb = input
            .params
            .get("maxOutputSize")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(DEFAULT_MAX_OUTPUT_MB);

        let output_mode = input
            .params
            .get("outputMode")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("stdout");

        match output_mode {
            "file" => process_file_mode(command, &args, &input, progress, ctx, max_output_mb),
            _ => process_stdout_mode(command, &args, &input, progress, ctx, max_output_mb),
        }
    }

    fn metadata(&self) -> NodeMetadata {
        NodeMetadata {
            node_type: "shell-command".to_string(),
            name: "Shell Command".to_string(),
            description: "Execute external CLI tools with security validation.".to_string(),
            category: NodeCategory::System,
            accepts: vec![],
            platforms: vec![
                "cli".to_string(),
                "server".to_string(),
                "desktop".to_string(),
            ],
            parameters: build_parameters(),
            input_cardinality: InputCardinality::Source,
            requires: vec![],
        }
    }
}

/// Resolve `{{url}}` and `{{input}}` placeholders in args from injected params.
///
/// The CLI's input preparation injects `"url"` or `"text"` into the node's
/// params for URL/Text mode recipes. If no placeholder exists in the args
/// but a `url` param is present, append the URL as the last argument.
fn resolve_input_placeholders(
    args: &mut Vec<String>,
    params: &serde_json::Map<String, serde_json::Value>,
) {
    let url = params
        .get("url")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    let text = params
        .get("text")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");

    // The input value is whichever was injected (url or text).
    let input_val = if !url.is_empty() { url } else { text };

    let has_url_placeholder = args.iter().any(|a| a.contains(URL_PLACEHOLDER));
    let has_input_placeholder = args.iter().any(|a| a.contains(INPUT_PLACEHOLDER));

    // Substitute placeholders in existing args.
    for arg in args.iter_mut() {
        if arg.contains(URL_PLACEHOLDER) && !url.is_empty() {
            *arg = arg.replace(URL_PLACEHOLDER, url);
        }
        if arg.contains(INPUT_PLACEHOLDER) && !input_val.is_empty() {
            *arg = arg.replace(INPUT_PLACEHOLDER, input_val);
        }
    }

    // If no placeholder was found but a URL was injected, append it.
    // This is the common case: `yt-dlp <args...> <url>`
    if !has_url_placeholder && !has_input_placeholder && !url.is_empty() {
        args.push(url.to_string());
    }
}

/// Stdout mode: capture command stdout as a single output file.
fn process_stdout_mode(
    command: &str,
    args: &[String],
    input: &NodeInput,
    progress: &ProgressReporter,
    ctx: &dyn ProcessContext,
    max_output_mb: u64,
) -> Result<NodeOutput, BntoError> {
    progress.report(10, &format!("Running {command}..."));

    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let output_bytes = ctx.run_command_streaming(command, &arg_refs, &|line| {
        progress.report_output(line);
    })?;

    let limit = validate::max_output_bytes(max_output_mb);
    if output_bytes.len() > limit {
        return Err(BntoError::ProcessingFailed(format!(
            "Command output exceeded {max_output_mb} MB limit"
        )));
    }

    progress.report(100, "Done");

    let output_filename = make_output_filename(command, &input.filename);

    let mut metadata = serde_json::Map::new();
    metadata.insert("command".into(), command.into());
    metadata.insert(
        "outputBytes".into(),
        serde_json::Number::from(output_bytes.len()).into(),
    );

    Ok(NodeOutput {
        files: vec![OutputFile {
            data: output_bytes,
            filename: output_filename,
            mime_type: "application/octet-stream".to_string(),
        }],
        metadata,
    })
}

/// File mode: run command in a temp dir, collect written files as output.
/// Replaces `{{output_dir}}` in args with the temp directory path.
fn process_file_mode(
    command: &str,
    args: &[String],
    _input: &NodeInput,
    progress: &ProgressReporter,
    ctx: &dyn ProcessContext,
    max_output_mb: u64,
) -> Result<NodeOutput, BntoError> {
    // Create a temp directory for the command's output files.
    let temp_dir = ctx.temp_file("-output")?;
    let output_dir = temp_dir.with_extension("d");
    std::fs::create_dir_all(&output_dir).map_err(|e| {
        BntoError::ProcessingFailed(format!("Failed to create output directory: {e}"))
    })?;
    let dir_str = output_dir.to_string_lossy();

    // Substitute {{output_dir}} placeholder in args.
    let resolved_args: Vec<String> = args
        .iter()
        .map(|a| a.replace(OUTPUT_DIR_PLACEHOLDER, &dir_str))
        .collect();

    progress.report(10, &format!("Running {command}..."));

    let arg_refs: Vec<&str> = resolved_args.iter().map(String::as_str).collect();
    // Run the command with stderr streaming — stdout is captured but unused.
    let _stdout = ctx.run_command_streaming(command, &arg_refs, &|line| {
        progress.report_output(line);
    })?;

    progress.report(80, "Collecting output files...");

    let files = collect_output_files(&output_dir, max_output_mb)?;

    if files.is_empty() {
        return Err(BntoError::ProcessingFailed(format!(
            "Command '{command}' produced no output files in {dir_str}"
        )));
    }

    // Clean up temp dir after reading files.
    let _ = std::fs::remove_dir_all(&output_dir);

    progress.report(100, "Done");

    let total_bytes: usize = files.iter().map(|f| f.data.len()).sum();
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

    Ok(NodeOutput { files, metadata })
}

/// Read all files from a directory as OutputFile entries.
fn collect_output_files(
    dir: &std::path::Path,
    max_output_mb: u64,
) -> Result<Vec<OutputFile>, BntoError> {
    let entries = std::fs::read_dir(dir).map_err(|e| {
        BntoError::ProcessingFailed(format!("Failed to read output directory: {e}"))
    })?;

    let mut files = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "output".to_string());

        let data = std::fs::read(&path).map_err(|e| {
            BntoError::ProcessingFailed(format!("Failed to read output file {filename}: {e}"))
        })?;

        let limit = validate::max_output_bytes(max_output_mb);
        if data.len() > limit {
            return Err(BntoError::ProcessingFailed(format!(
                "Output file '{filename}' exceeded {max_output_mb} MB limit"
            )));
        }

        let mime = mime_from_extension(&filename);
        files.push(OutputFile {
            data,
            filename,
            mime_type: mime,
        });
    }
    Ok(files)
}

/// Derive MIME type from file extension for common media formats.
fn mime_from_extension(filename: &str) -> String {
    let ext = filename
        .rsplit_once('.')
        .map(|(_, e)| e.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mkv" => "video/x-matroska",
        "mp3" => "audio/mpeg",
        "m4a" => "audio/mp4",
        "ogg" | "opus" => "audio/ogg",
        "wav" => "audio/wav",
        "flac" => "audio/flac",
        "json" => "application/json",
        "txt" | "log" => "text/plain",
        _ => "application/octet-stream",
    }
    .to_string()
}

/// Generate a filename for stdout mode output.
fn make_output_filename(command: &str, input_filename: &str) -> String {
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

fn build_parameters() -> Vec<ParameterDef> {
    vec![
        ParameterDef {
            name: "command".to_string(),
            label: "Command".to_string(),
            description: "Binary to execute (e.g., 'ffmpeg', 'yt-dlp'). Must be on PATH."
                .to_string(),
            param_type: ParameterType::String,
            default: None,
            constraints: None,
            placeholder: Some("ffmpeg".to_string()),
            visible_when: None,
            required_when: None,
            surfaceable: true,
            group: None,
            suffix: None,
            control: None,
            accept: None,
            presets: None,
            inverted: None,
        },
        ParameterDef {
            name: "args".to_string(),
            label: "Arguments".to_string(),
            description: "Command arguments as an array of strings.".to_string(),
            param_type: ParameterType::String,
            default: None,
            constraints: None,
            placeholder: None,
            visible_when: None,
            required_when: None,
            surfaceable: true,
            group: None,
            suffix: None,
            control: Some("tagPicker".to_string()),
            accept: None,
            presets: None,
            inverted: None,
        },
        ParameterDef {
            name: "outputMode".to_string(),
            label: "Output Mode".to_string(),
            description: "How to collect output. 'stdout' captures command output. \
                'file' reads files written by the command to a temp directory \
                (use {{output_dir}} in args to inject the path)."
                .to_string(),
            param_type: ParameterType::String,
            default: Some(serde_json::Value::String("stdout".to_string())),
            constraints: None,
            placeholder: None,
            visible_when: None,
            required_when: None,
            surfaceable: true,
            group: None,
            suffix: None,
            control: None,
            accept: None,
            presets: None,
            inverted: None,
        },
        ParameterDef {
            name: "timeout".to_string(),
            label: "Timeout".to_string(),
            description: "Maximum execution time in seconds. Default: 300.".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::Value::Number(serde_json::Number::from(
                DEFAULT_TIMEOUT_SECS,
            ))),
            constraints: None,
            placeholder: None,
            visible_when: None,
            required_when: None,
            surfaceable: true,
            group: None,
            suffix: Some("seconds".to_string()),
            control: None,
            accept: None,
            presets: None,
            inverted: None,
        },
        ParameterDef {
            name: "maxOutputSize".to_string(),
            label: "Max Output Size".to_string(),
            description: "Maximum size per output file in megabytes. Default: 100 MB.".to_string(),
            param_type: ParameterType::Number,
            default: Some(serde_json::Value::Number(serde_json::Number::from(
                DEFAULT_MAX_OUTPUT_MB,
            ))),
            constraints: None,
            placeholder: None,
            visible_when: None,
            required_when: None,
            surfaceable: true,
            group: None,
            suffix: Some("MB".to_string()),
            control: None,
            accept: None,
            presets: None,
            inverted: None,
        },
        ParameterDef {
            name: "env".to_string(),
            label: "Environment".to_string(),
            description: "Additional environment variables for the command.".to_string(),
            param_type: ParameterType::Object,
            default: None,
            constraints: None,
            placeholder: None,
            visible_when: None,
            required_when: None,
            surfaceable: false,
            group: None,
            suffix: None,
            control: None,
            accept: None,
            presets: None,
            inverted: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NoopContext;

    // --- Trait basics ---

    #[test]
    fn test_processor_name() {
        let processor = ShellCommand::new();
        assert_eq!(processor.name(), "shell-command");
    }

    #[test]
    fn test_metadata_category_system() {
        let processor = ShellCommand::new();
        let meta = processor.metadata();
        assert_eq!(meta.category, NodeCategory::System);
    }

    #[test]
    fn test_metadata_platforms_native_only() {
        let processor = ShellCommand::new();
        let meta = processor.metadata();
        assert_eq!(meta.platforms, vec!["cli", "server", "desktop"]);
        assert!(!meta.platforms.contains(&"browser".to_string()));
    }

    #[test]
    fn test_metadata_parameters_complete() {
        let processor = ShellCommand::new();
        let meta = processor.metadata();
        let param_names: Vec<&str> = meta.parameters.iter().map(|p| p.name.as_str()).collect();
        assert!(param_names.contains(&"command"));
        assert!(param_names.contains(&"args"));
        assert!(param_names.contains(&"outputMode"));
        assert!(param_names.contains(&"timeout"));
        assert!(param_names.contains(&"maxOutputSize"));
        assert!(param_names.contains(&"env"));
    }

    #[test]
    fn test_metadata_no_requires() {
        let processor = ShellCommand::new();
        let meta = processor.metadata();
        assert!(
            meta.requires.is_empty(),
            "shell-command has no inherent deps"
        );
    }

    // --- Validation ---

    #[test]
    fn test_validate_empty_command_fails() {
        let processor = ShellCommand::new();
        let mut params = serde_json::Map::new();
        params.insert("command".into(), serde_json::Value::String("".into()));
        let errors = processor.validate(&params);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_validate_missing_command_fails() {
        let processor = ShellCommand::new();
        let params = serde_json::Map::new();
        let errors = processor.validate(&params);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("required"));
    }

    #[test]
    fn test_validate_present_command_passes() {
        let processor = ShellCommand::new();
        let mut params = serde_json::Map::new();
        params.insert("command".into(), serde_json::Value::String("echo".into()));
        let errors = processor.validate(&params);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_shell_command_rejected() {
        let processor = ShellCommand::new();
        let mut params = serde_json::Map::new();
        params.insert("command".into(), serde_json::Value::String("bash".into()));
        let errors = processor.validate(&params);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("shell interpreter"));
    }

    #[test]
    fn test_validate_zero_timeout_rejected() {
        let processor = ShellCommand::new();
        let mut params = serde_json::Map::new();
        params.insert("command".into(), serde_json::Value::String("echo".into()));
        params.insert(
            "timeout".into(),
            serde_json::Value::Number(serde_json::Number::from(0)),
        );
        let errors = processor.validate(&params);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("greater than 0"));
    }

    // --- Process ---

    #[test]
    fn test_noop_context_returns_error() {
        let processor = ShellCommand::new();
        let progress = ProgressReporter::new_noop();
        let input = NodeInput {
            data: vec![],
            filename: "test.txt".to_string(),
            mime_type: None,
            params: {
                let mut m = serde_json::Map::new();
                m.insert("command".into(), serde_json::Value::String("echo".into()));
                m.insert(
                    "args".into(),
                    serde_json::Value::Array(vec![serde_json::Value::String("hello".into())]),
                );
                m
            },
        };
        let result = processor.process(input, &progress, &NoopContext);
        assert!(result.is_err());
        let err = result.err().expect("should be error");
        assert!(err.to_string().contains("not available in browser"));
    }

    #[test]
    fn test_process_rejects_missing_command() {
        let processor = ShellCommand::new();
        let progress = ProgressReporter::new_noop();
        let input = NodeInput {
            data: vec![],
            filename: "test.txt".to_string(),
            mime_type: None,
            params: serde_json::Map::new(),
        };
        let result = processor.process(input, &progress, &NoopContext);
        assert!(result.is_err());
    }

    #[test]
    fn test_process_rejects_shell_command() {
        let processor = ShellCommand::new();
        let progress = ProgressReporter::new_noop();
        let input = NodeInput {
            data: vec![],
            filename: "test.txt".to_string(),
            mime_type: None,
            params: {
                let mut m = serde_json::Map::new();
                m.insert("command".into(), serde_json::Value::String("bash".into()));
                m.insert(
                    "args".into(),
                    serde_json::Value::Array(vec![
                        serde_json::Value::String("-c".into()),
                        serde_json::Value::String("echo pwned".into()),
                    ]),
                );
                m
            },
        };
        let result = processor.process(input, &progress, &NoopContext);
        assert!(result.is_err());
        let err = result.err().expect("should be error");
        assert!(err.to_string().contains("shell interpreter"));
    }

    #[test]
    fn test_default_timeout_in_metadata() {
        let processor = ShellCommand::new();
        let meta = processor.metadata();
        let timeout_param = meta
            .parameters
            .iter()
            .find(|p| p.name == "timeout")
            .expect("timeout param should exist");
        assert_eq!(
            timeout_param.default,
            Some(serde_json::Value::Number(serde_json::Number::from(300u64)))
        );
    }

    #[test]
    fn test_default_max_output_size_in_metadata() {
        let processor = ShellCommand::new();
        let meta = processor.metadata();
        let param = meta
            .parameters
            .iter()
            .find(|p| p.name == "maxOutputSize")
            .expect("maxOutputSize param should exist");
        assert_eq!(
            param.default,
            Some(serde_json::Value::Number(serde_json::Number::from(
                DEFAULT_MAX_OUTPUT_MB
            )))
        );
        assert_eq!(param.suffix, Some("MB".to_string()));
    }

    #[test]
    fn test_env_param_not_surfaceable() {
        let processor = ShellCommand::new();
        let meta = processor.metadata();
        let env_param = meta
            .parameters
            .iter()
            .find(|p| p.name == "env")
            .expect("env param should exist");
        assert!(!env_param.surfaceable, "env should be internal-only");
    }

    #[test]
    fn test_output_mode_default_is_stdout() {
        let processor = ShellCommand::new();
        let meta = processor.metadata();
        let param = meta
            .parameters
            .iter()
            .find(|p| p.name == "outputMode")
            .expect("outputMode param should exist");
        assert_eq!(
            param.default,
            Some(serde_json::Value::String("stdout".to_string()))
        );
    }

    // --- File mode helpers ---

    #[test]
    fn test_mime_from_extension_video() {
        assert_eq!(mime_from_extension("video.mp4"), "video/mp4");
        assert_eq!(mime_from_extension("video.webm"), "video/webm");
        assert_eq!(mime_from_extension("video.mkv"), "video/x-matroska");
    }

    #[test]
    fn test_mime_from_extension_audio() {
        assert_eq!(mime_from_extension("audio.mp3"), "audio/mpeg");
        assert_eq!(mime_from_extension("audio.m4a"), "audio/mp4");
        assert_eq!(mime_from_extension("audio.wav"), "audio/wav");
        assert_eq!(mime_from_extension("audio.flac"), "audio/flac");
    }

    #[test]
    fn test_mime_from_extension_unknown() {
        assert_eq!(mime_from_extension("file.xyz"), "application/octet-stream");
        assert_eq!(mime_from_extension("noext"), "application/octet-stream");
    }

    #[test]
    fn test_make_output_filename_empty_input() {
        assert_eq!(make_output_filename("echo", ""), "echo-output");
    }

    #[test]
    fn test_make_output_filename_with_input() {
        assert_eq!(
            make_output_filename("ffmpeg", "video.mp4"),
            "video-ffmpeg-output"
        );
    }

    #[test]
    fn test_collect_output_files_reads_dir() {
        let dir = std::env::temp_dir().join("bnto-test-collect-output");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("video.mp4"), vec![0u8; 100]).unwrap();
        std::fs::write(dir.join("info.json"), b"{}").unwrap();

        let files = collect_output_files(&dir, DEFAULT_MAX_OUTPUT_MB).unwrap();
        assert_eq!(files.len(), 2);

        let names: Vec<&str> = files.iter().map(|f| f.filename.as_str()).collect();
        assert!(names.contains(&"video.mp4"));
        assert!(names.contains(&"info.json"));

        // Verify MIME types
        let mp4 = files.iter().find(|f| f.filename == "video.mp4").unwrap();
        assert_eq!(mp4.mime_type, "video/mp4");
        let json = files.iter().find(|f| f.filename == "info.json").unwrap();
        assert_eq!(json.mime_type, "application/json");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_collect_output_files_skips_directories() {
        let dir = std::env::temp_dir().join("bnto-test-collect-skip-dirs");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::create_dir_all(dir.join("subdir")).unwrap();
        std::fs::write(dir.join("file.txt"), b"data").unwrap();

        let files = collect_output_files(&dir, DEFAULT_MAX_OUTPUT_MB).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].filename, "file.txt");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_collect_output_files_empty_dir() {
        let dir = std::env::temp_dir().join("bnto-test-collect-empty");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let files = collect_output_files(&dir, DEFAULT_MAX_OUTPUT_MB).unwrap();
        assert!(files.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_output_dir_placeholder_substitution() {
        let args = [
            "-o".to_string(),
            "{{output_dir}}/%(title)s.%(ext)s".to_string(),
            "--verbose".to_string(),
        ];
        let dir_str = "/tmp/bnto-output";
        let resolved: Vec<String> = args
            .iter()
            .map(|a| a.replace(OUTPUT_DIR_PLACEHOLDER, dir_str))
            .collect();
        assert_eq!(resolved[0], "-o");
        assert_eq!(resolved[1], "/tmp/bnto-output/%(title)s.%(ext)s");
        assert_eq!(resolved[2], "--verbose");
    }

    // --- Input placeholder resolution ---

    #[test]
    fn test_resolve_url_appended_when_no_placeholder() {
        let mut args = vec!["--no-playlist".to_string(), "-o".to_string()];
        let mut params = serde_json::Map::new();
        params.insert("url".into(), "https://example.com/video".into());
        resolve_input_placeholders(&mut args, &params);
        assert_eq!(args.len(), 3);
        assert_eq!(args[2], "https://example.com/video");
    }

    #[test]
    fn test_resolve_url_placeholder_substituted() {
        let mut args = vec!["--download".to_string(), "{{url}}".to_string()];
        let mut params = serde_json::Map::new();
        params.insert("url".into(), "https://example.com/video".into());
        resolve_input_placeholders(&mut args, &params);
        assert_eq!(args.len(), 2);
        assert_eq!(args[1], "https://example.com/video");
    }

    #[test]
    fn test_resolve_input_placeholder_substituted() {
        let mut args = vec!["process".to_string(), "{{input}}".to_string()];
        let mut params = serde_json::Map::new();
        params.insert("url".into(), "https://example.com/data".into());
        resolve_input_placeholders(&mut args, &params);
        assert_eq!(args.len(), 2);
        assert_eq!(args[1], "https://example.com/data");
    }

    #[test]
    fn test_resolve_no_url_no_change() {
        let mut args = vec!["--help".to_string()];
        let params = serde_json::Map::new();
        resolve_input_placeholders(&mut args, &params);
        assert_eq!(args.len(), 1);
        assert_eq!(args[0], "--help");
    }

    #[test]
    fn test_resolve_url_not_appended_when_placeholder_exists() {
        let mut args = vec!["{{url}}".to_string(), "--verbose".to_string()];
        let mut params = serde_json::Map::new();
        params.insert("url".into(), "https://example.com".into());
        resolve_input_placeholders(&mut args, &params);
        // URL substituted in placeholder, NOT also appended
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "https://example.com");
    }
}
