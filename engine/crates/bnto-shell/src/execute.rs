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

use bnto_core::metadata::InputCardinality;
use bnto_core::processor::{BatchFile, BatchInput, FileData, NodeInput, NodeOutput};
use bnto_core::{
    BntoError, NodeCategory, NodeMetadata, NodeProcessor, ProcessContext, ProgressReporter,
};

pub use crate::args::flatten_conditional_args;
use crate::args::resolve_input_placeholders;
use crate::command_options::command_options;
use crate::cwd;
use crate::each;
use crate::file_mode::process_file_mode;
use crate::invocation::Invocation;
use crate::parameters::build_parameters;
use crate::search_path;
use crate::stdout_mode::process_stdout_mode;
use crate::validate::{self, DEFAULT_MAX_OUTPUT_MB};

// Items the tests below exercise directly.
#[cfg(test)]
use crate::file_mode::OUTPUT_DIR_PLACEHOLDER;
#[cfg(test)]
use crate::output_files::{collect_output_files, mime_from_extension};
#[cfg(test)]
use crate::stdout_mode::make_output_filename;

/// JSON params of a node, as the processor receives them.
type Params = serde_json::Map<String, serde_json::Value>;

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

        if let Err(e) = search_path::read_search_path(params.get("search_path")) {
            errors.push(e);
        }
        if let Err(e) = each::check_groups(params.get("args")) {
            errors.push(e);
        }
        if let Err(e) = cwd::read_cwd(params.get("cwd")) {
            errors.push(e);
        }
        if let Some(timeout) = params.get("timeout").and_then(serde_json::Value::as_u64)
            && timeout == 0
        {
            errors.push("'timeout' must be greater than 0".to_string());
        }

        errors
    }

    /// Run with no incoming files (a direct call, not through the executor).
    fn process(
        &self,
        input: NodeInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        run_node(&input, &[], progress, ctx)
    }

    /// Run once with every incoming file, for `each` groups in `args`. The
    /// files never reach the command otherwise, and the output is named
    /// after the command alone, as when the node had no input.
    fn process_batch(
        &self,
        input: BatchInput,
        progress: &ProgressReporter,
        ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let node_input = NodeInput {
            data: FileData::Bytes(Vec::new()),
            filename: String::new(),
            mime_type: None,
            params: input.params,
        };
        run_node(&node_input, &input.files, progress, ctx)
    }

    /// Fills `{{item.*}}` inside `each` groups itself, once per file.
    fn resolves_items(&self) -> bool {
        true
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
            input_cardinality: InputCardinality::Batch,
            requires: vec![],
        }
    }
}

/// Validate, build and run the command `input.params` describe, with
/// `files` for its `each` groups.
fn run_node(
    input: &NodeInput,
    files: &[BatchFile],
    progress: &ProgressReporter,
    ctx: &dyn ProcessContext,
) -> Result<NodeOutput, BntoError> {
    let params = &input.params;
    let command = required_command(params)?;
    // Security check: reject shell interpreters and path-based commands
    validate::validate_command(command).map_err(BntoError::InvalidInput)?;
    let program = resolved_program(command, params, ctx)?;
    let dir = working_dir(params, ctx)?;
    let args = command_args(params, files, ctx)?;
    let max_output_mb = u64_param(params, "maxOutputSize", DEFAULT_MAX_OUTPUT_MB);
    let invocation = Invocation {
        command,
        program: &program,
        args: &args,
        options: command_options(params, dir),
    };
    match params.get("outputMode").and_then(serde_json::Value::as_str) {
        Some("file") => process_file_mode(&invocation, input, progress, ctx, max_output_mb),
        _ => process_stdout_mode(&invocation, input, progress, ctx, max_output_mb),
    }
}

/// The command's arguments: `each` groups filled per file, conditional
/// groups flattened, then `{{url}}` / `{{input}}` resolved from the params
/// the CLI injects for URL/Text mode recipes.
fn command_args(
    params: &Params,
    files: &[BatchFile],
    ctx: &dyn ProcessContext,
) -> Result<Vec<String>, BntoError> {
    if let Some(error) = each::unfilled_item(params) {
        return Err(BntoError::InvalidInput(error));
    }
    let raw = params.get("args").and_then(serde_json::Value::as_array);
    let expanded = each::expand_each(raw.map_or(&[], Vec::as_slice), files, ctx)?;
    let mut args = flatten_conditional_args(&expanded);
    resolve_input_placeholders(&mut args, params);
    Ok(args)
}

/// The required `command` param.
fn required_command(params: &Params) -> Result<&str, BntoError> {
    params
        .get("command")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| BntoError::InvalidInput("'command' parameter is required".to_string()))
}

/// What to spawn: the command's full path from `search_path` when set,
/// else the bare name for PATH resolution.
fn resolved_program(
    command: &str,
    params: &Params,
    ctx: &dyn ProcessContext,
) -> Result<String, BntoError> {
    let folders = search_path::read_search_path(params.get("search_path"))
        .map_err(BntoError::InvalidInput)?;
    let found = search_path::resolve_command(command, &folders, ctx.work_dir().ok())
        .map_err(BntoError::InvalidInput)?;
    Ok(found.map_or_else(|| command.to_string(), |p| p.to_string_lossy().into_owned()))
}

/// The folder `cwd` names under the work dir, or `None` to run in the
/// work dir itself.
fn working_dir(
    params: &Params,
    ctx: &dyn ProcessContext,
) -> Result<Option<std::path::PathBuf>, BntoError> {
    let Some(folder) = cwd::read_cwd(params.get("cwd")).map_err(BntoError::InvalidInput)? else {
        return Ok(None);
    };
    cwd::resolve_cwd(folder, ctx.work_dir()?)
        .map(Some)
        .map_err(BntoError::InvalidInput)
}

/// A numeric param, or `default` when unset.
fn u64_param(params: &Params, key: &str, default: u64) -> u64 {
    params
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(default)
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

    // --- search_path ---

    #[test]
    fn test_validate_refuses_a_search_path_that_is_not_folders() {
        let mut params = serde_json::Map::new();
        params.insert("command".into(), serde_json::json!("tool"));
        params.insert("search_path".into(), serde_json::json!([1]));
        let errors = ShellCommand::new().validate(&params);
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(errors[0].contains("search_path"), "{errors:?}");
    }

    #[test]
    fn test_validate_keeps_refusing_shells_with_a_search_path() {
        let mut params = serde_json::Map::new();
        params.insert("command".into(), serde_json::json!("cmd"));
        params.insert(
            "search_path".into(),
            serde_json::json!(["C:/Windows/System32"]),
        );
        let errors = ShellCommand::new().validate(&params);
        assert!(
            errors.iter().any(|e| e.contains("shell interpreter")),
            "{errors:?}"
        );
    }

    /// Records the program it was asked to run and returns "ok" as stdout.
    struct Recording(std::sync::Mutex<Vec<String>>);

    impl ProcessContext for Recording {
        fn run_command(&self, cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            self.0.lock().unwrap().push(cmd.to_string());
            Ok(b"ok".to_vec())
        }
        fn temp_file(&self, s: &str) -> Result<std::path::PathBuf, BntoError> {
            NoopContext.temp_file(s)
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&std::path::Path, BntoError> {
            NoopContext.work_dir()
        }
    }

    fn search_path_input(dir: &std::path::Path) -> NodeInput {
        let mut params = serde_json::Map::new();
        params.insert("command".into(), serde_json::json!("tool.bat"));
        params.insert(
            "search_path".into(),
            serde_json::json!([dir.to_str().unwrap()]),
        );
        NodeInput {
            data: FileData::Bytes(vec![]),
            filename: String::new(),
            mime_type: None,
            params,
        }
    }

    #[test]
    fn test_process_runs_the_command_found_in_search_path_by_full_path() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("tool.bat"), b"").unwrap();
        let ctx = Recording(Default::default());
        let progress = ProgressReporter::new_noop();
        let output = ShellCommand::new()
            .process(search_path_input(dir.path()), &progress, &ctx)
            .unwrap();
        // Named after the bare command, never the full path it ran from.
        assert_eq!(output.files[0].filename, "tool.bat-output");
        assert_eq!(output.metadata["command"], "tool.bat");
        let ran = ctx.0.lock().unwrap().clone();
        let expected = dir.path().join("tool.bat").to_string_lossy().into_owned();
        assert_eq!(ran, [expected]);
    }

    #[test]
    fn test_process_fails_when_search_path_lacks_the_command() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = Recording(Default::default());
        let progress = ProgressReporter::new_noop();
        let err = ShellCommand::new()
            .process(search_path_input(dir.path()), &progress, &ctx)
            .err()
            .expect("should fail");
        assert!(
            err.to_string().contains("not found in search_path"),
            "{err}"
        );
        assert!(ctx.0.lock().unwrap().is_empty(), "nothing should run");
    }

    #[test]
    fn test_search_path_param_is_internal_and_listed() {
        let meta = ShellCommand::new().metadata();
        let param = meta.parameters.iter().find(|p| p.name == "search_path");
        assert!(!param.expect("search_path should exist").surfaceable);
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
            data: FileData::Bytes(vec![]),
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
            data: FileData::Bytes(vec![]),
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
            data: FileData::Bytes(vec![]),
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
    fn test_collect_output_files_returns_path_variant() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
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

        // Verify FileData::Path variant — files stay on disk, not read into memory.
        for file in &files {
            assert!(
                matches!(&file.data, FileData::Path(_)),
                "collect_output_files should return FileData::Path, got Bytes for {}",
                file.filename,
            );
        }
    }

    #[test]
    fn test_collect_output_files_preserves_subdirectory_in_filename() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        std::fs::create_dir_all(dir.join("subdir")).unwrap();
        std::fs::write(dir.join("file.txt"), b"data").unwrap();
        std::fs::write(dir.join("subdir").join("video.mp4"), vec![0u8; 50]).unwrap();

        let files = collect_output_files(&dir, DEFAULT_MAX_OUTPUT_MB).unwrap();
        assert_eq!(files.len(), 2);

        let names: Vec<&str> = files.iter().map(|f| f.filename.as_str()).collect();
        assert!(names.contains(&"file.txt"));
        assert!(names.contains(&"subdir/video.mp4"));
    }

    #[test]
    fn test_collect_output_files_preserves_deeply_nested_path() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        std::fs::create_dir_all(dir.join("a").join("b")).unwrap();
        std::fs::write(dir.join("a").join("b").join("deep.mp4"), vec![0u8; 10]).unwrap();

        let files = collect_output_files(&dir, DEFAULT_MAX_OUTPUT_MB).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].filename, "a/b/deep.mp4");
    }

    #[test]
    fn test_collect_output_files_multiple_subdirs_preserve_structure() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        std::fs::create_dir_all(dir.join("Alpha Legion")).unwrap();
        std::fs::create_dir_all(dir.join("Suboden Khan")).unwrap();
        std::fs::write(dir.join("Alpha Legion").join("part1.mp4"), b"vid1").unwrap();
        std::fs::write(dir.join("Suboden Khan").join("part2.mp4"), b"vid2").unwrap();

        let files = collect_output_files(&dir, DEFAULT_MAX_OUTPUT_MB).unwrap();
        assert_eq!(files.len(), 2);

        let mut names: Vec<&str> = files.iter().map(|f| f.filename.as_str()).collect();
        names.sort();
        assert_eq!(
            names,
            vec!["Alpha Legion/part1.mp4", "Suboden Khan/part2.mp4"]
        );
    }

    #[test]
    fn test_collect_output_files_empty_dir() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();

        let files = collect_output_files(&dir, DEFAULT_MAX_OUTPUT_MB).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_collect_output_files_rejects_oversized_file() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        // 2 MB file with a 1 MB limit — should fail before reading into memory.
        std::fs::write(dir.join("big.mp4"), vec![0u8; 2 * 1024 * 1024]).unwrap();

        let result = collect_output_files(&dir, 1);
        match result {
            Err(e) => {
                let msg = e.to_string();
                assert!(msg.contains("big.mp4"), "Error should name the file");
                assert!(msg.contains("1 MB limit"), "Error should state the limit");
            }
            Ok(_) => panic!("Expected error for oversized file"),
        }
    }

    #[test]
    fn test_collect_output_files_accepts_file_under_limit() {
        let temp = tempfile::tempdir().unwrap();
        let dir = temp.path().to_path_buf();
        // 512 KB file with a 1 MB limit — should succeed.
        std::fs::write(dir.join("small.mp4"), vec![0u8; 512 * 1024]).unwrap();

        let files = collect_output_files(&dir, 1).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].filename, "small.mp4");
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

    // --- flatten_conditional_args ---

    #[test]
    fn test_flatten_flat_strings_included() {
        let args = vec![
            serde_json::Value::String("--no-playlist".into()),
            serde_json::Value::String("--newline".into()),
        ];
        assert_eq!(
            flatten_conditional_args(&args),
            vec!["--no-playlist", "--newline"]
        );
    }

    #[test]
    fn test_flatten_empty_strings_dropped() {
        let args = vec![
            serde_json::Value::String("--always".into()),
            serde_json::Value::String("".into()),
            serde_json::Value::String("--also-always".into()),
        ];
        assert_eq!(
            flatten_conditional_args(&args),
            vec!["--always", "--also-always"]
        );
    }

    #[test]
    fn test_flatten_nested_array_all_non_empty_flattened() {
        let args = vec![
            serde_json::Value::String("--always".into()),
            serde_json::Value::Array(vec![
                serde_json::Value::String("--cookies-from-browser".into()),
                serde_json::Value::String("chrome".into()),
            ]),
        ];
        assert_eq!(
            flatten_conditional_args(&args),
            vec!["--always", "--cookies-from-browser", "chrome"]
        );
    }

    #[test]
    fn test_flatten_nested_array_any_empty_drops_group() {
        let args = vec![
            serde_json::Value::String("--always".into()),
            serde_json::Value::Array(vec![
                serde_json::Value::String("--cookies-from-browser".into()),
                serde_json::Value::String("".into()),
            ]),
        ];
        assert_eq!(flatten_conditional_args(&args), vec!["--always"]);
    }

    #[test]
    fn test_flatten_nested_array_with_non_string_drops_group() {
        let args = vec![serde_json::Value::Array(vec![
            serde_json::Value::String("--flag".into()),
            serde_json::Value::Number(serde_json::Number::from(42)),
        ])];
        assert!(
            flatten_conditional_args(&args).is_empty(),
            "Non-string element means group.len() != strings.len()"
        );
    }

    #[test]
    fn test_flatten_non_string_types_dropped() {
        let args = vec![
            serde_json::Value::String("--keep".into()),
            serde_json::Value::Number(serde_json::Number::from(42)),
            serde_json::Value::Bool(true),
            serde_json::Value::Null,
        ];
        assert_eq!(flatten_conditional_args(&args), vec!["--keep"]);
    }

    #[test]
    fn test_flatten_empty_array_produces_nothing() {
        let args = vec![serde_json::Value::Array(vec![])];
        // Empty group has all_non_empty = true (vacuously), but len == 0
        // so nothing is actually added.
        assert!(flatten_conditional_args(&args).is_empty());
    }

    #[test]
    fn test_flatten_mixed_conditional_scenario() {
        // Simulates the download-video recipe args after template resolution.
        let args = vec![
            serde_json::Value::String("--no-playlist".into()),
            serde_json::Value::String("--newline".into()),
            serde_json::Value::Array(vec![
                serde_json::Value::String("--cookies-from-browser".into()),
                serde_json::Value::String("chrome".into()),
            ]),
            serde_json::Value::Array(vec![
                serde_json::Value::String("-S".into()),
                serde_json::Value::String("res:720".into()),
            ]),
            serde_json::Value::String("".into()), // empty thumbnail → dropped
            serde_json::Value::String("".into()), // empty sponsorblock → dropped
        ];
        assert_eq!(
            flatten_conditional_args(&args),
            vec![
                "--no-playlist",
                "--newline",
                "--cookies-from-browser",
                "chrome",
                "-S",
                "res:720",
            ]
        );
    }

    #[test]
    fn test_flatten_all_groups_disabled() {
        // All optional groups have empty values — only unconditional args survive.
        let args = vec![
            serde_json::Value::String("--no-playlist".into()),
            serde_json::Value::Array(vec![
                serde_json::Value::String("--cookies-from-browser".into()),
                serde_json::Value::String("".into()),
            ]),
            serde_json::Value::Array(vec![
                serde_json::Value::String("-S".into()),
                serde_json::Value::String("".into()),
            ]),
            serde_json::Value::String("".into()),
            serde_json::Value::String("".into()),
        ];
        assert_eq!(flatten_conditional_args(&args), vec!["--no-playlist"]);
    }

    /// Records the options it was asked to run a command with, in `.1`.
    struct RecordingOptions(
        std::sync::Mutex<Vec<bnto_core::CommandOptions>>,
        std::path::PathBuf,
    );

    impl ProcessContext for RecordingOptions {
        fn run_command(&self, _cmd: &str, _args: &[&str]) -> Result<Vec<u8>, BntoError> {
            Ok(b"ok".to_vec())
        }
        fn run_command_with(
            &self,
            _cmd: &str,
            _args: &[&str],
            options: &bnto_core::CommandOptions,
            _on_output: &dyn Fn(&str),
        ) -> Result<Vec<u8>, BntoError> {
            self.0.lock().unwrap().push(options.clone());
            Ok(b"ok".to_vec())
        }
        fn temp_file(&self, s: &str) -> Result<std::path::PathBuf, BntoError> {
            NoopContext.temp_file(s)
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&std::path::Path, BntoError> {
            Ok(&self.1)
        }
    }

    fn options_input(params: serde_json::Value) -> NodeInput {
        NodeInput {
            data: FileData::Bytes(vec![]),
            filename: String::new(),
            mime_type: None,
            params: params.as_object().unwrap().clone(),
        }
    }

    #[test]
    fn test_the_command_runs_with_the_recipes_env_and_timeout() {
        let ctx = RecordingOptions(Default::default(), Default::default());
        let input = options_input(serde_json::json!({
            "command": "tool", "timeout": 9, "env": { "GREETING": "hi" }
        }));
        ShellCommand::new()
            .process(input, &ProgressReporter::new_noop(), &ctx)
            .unwrap();
        let seen = ctx.0.lock().unwrap();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].timeout, Some(std::time::Duration::from_secs(9)));
        assert_eq!(
            seen[0].env,
            vec![("GREETING".to_string(), "hi".to_string())]
        );
    }

    // --- each groups (batch input) ---

    /// Records the args of every command it was asked to run.
    struct RecordingArgs(std::sync::Mutex<Vec<Vec<String>>>);

    impl ProcessContext for RecordingArgs {
        fn run_command(&self, _cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
            let args = args.iter().map(|a| a.to_string()).collect();
            self.0.lock().unwrap().push(args);
            Ok(b"ok".to_vec())
        }
        fn temp_file(&self, s: &str) -> Result<std::path::PathBuf, BntoError> {
            NoopContext.temp_file(s)
        }
        fn env_var(&self, _key: &str) -> Option<String> {
            None
        }
        fn work_dir(&self) -> Result<&std::path::Path, BntoError> {
            NoopContext.work_dir()
        }
    }

    fn batch(params: serde_json::Value, names: &[&str]) -> BatchInput {
        let files = names
            .iter()
            .map(|name| BatchFile {
                data: FileData::Path(std::env::temp_dir().join(name)),
                filename: name.to_string(),
                mime_type: None,
                metadata: serde_json::Map::new(),
            })
            .collect();
        BatchInput {
            files,
            params: params.as_object().unwrap().clone(),
        }
    }

    #[test]
    fn test_shell_command_takes_its_incoming_files_and_fills_items_itself() {
        let meta = ShellCommand::new().metadata();
        assert_eq!(meta.input_cardinality, InputCardinality::Batch);
        assert!(ShellCommand::new().resolves_items());
    }

    #[test]
    fn test_one_command_gets_every_file_through_an_each_group() {
        let ctx = RecordingArgs(Default::default());
        let input = batch(
            serde_json::json!({
                "command": "llvm-cov",
                "args": ["report", { "each": ["-object={{path}}"] }, "-format=text"]
            }),
            &["a.dll", "b.dll"],
        );
        let output = ShellCommand::new()
            .process_batch(input, &ProgressReporter::new_noop(), &ctx)
            .unwrap();
        let object = |n: &str| format!("-object={}", std::env::temp_dir().join(n).display());
        let ran = ctx.0.lock().unwrap().clone();
        assert_eq!(
            ran,
            [vec![
                "report".to_string(),
                object("a.dll"),
                object("b.dll"),
                "-format=text".to_string()
            ]]
        );
        // Named after the command, never after an incoming file.
        assert_eq!(output.files[0].filename, "llvm-cov-output");
    }

    #[test]
    fn test_incoming_files_without_a_group_change_nothing() {
        let ctx = RecordingArgs(Default::default());
        let input = batch(
            serde_json::json!({ "command": "git", "args": ["--version"] }),
            &["a.txt"],
        );
        let output = ShellCommand::new()
            .process_batch(input, &ProgressReporter::new_noop(), &ctx)
            .unwrap();
        assert_eq!(
            ctx.0.lock().unwrap().clone(),
            [vec!["--version".to_string()]]
        );
        assert_eq!(output.files.len(), 1, "only the command's stdout");
        assert_eq!(output.files[0].filename, "git-output");
    }

    #[test]
    fn test_an_item_outside_a_group_fails_before_running() {
        let ctx = RecordingArgs(Default::default());
        let input = batch(
            serde_json::json!({ "command": "tool", "args": ["{{item.size}}"] }),
            &["a.txt"],
        );
        let err = ShellCommand::new()
            .process_batch(input, &ProgressReporter::new_noop(), &ctx)
            .err()
            .expect("should fail");
        assert!(
            err.to_string().contains("{{item.size}} has no value"),
            "{err}"
        );
        assert!(ctx.0.lock().unwrap().is_empty(), "nothing should run");
    }

    // --- cwd ---

    #[test]
    fn test_the_command_runs_in_the_cwd_folder_under_the_work_dir() {
        let work = tempfile::tempdir().unwrap();
        std::fs::create_dir(work.path().join("Tools")).unwrap();
        let ctx = RecordingOptions(Default::default(), work.path().to_path_buf());
        let input = options_input(serde_json::json!({ "command": "tool", "cwd": "Tools" }));
        ShellCommand::new()
            .process(input, &ProgressReporter::new_noop(), &ctx)
            .unwrap();
        let seen = ctx.0.lock().unwrap();
        assert_eq!(seen[0].dir, Some(work.path().join("Tools")));
    }

    #[test]
    fn test_without_cwd_the_command_runs_in_the_work_dir() {
        let ctx = RecordingOptions(Default::default(), Default::default());
        let input = options_input(serde_json::json!({ "command": "tool" }));
        ShellCommand::new()
            .process(input, &ProgressReporter::new_noop(), &ctx)
            .unwrap();
        assert_eq!(ctx.0.lock().unwrap()[0].dir, None);
    }

    #[test]
    fn test_a_missing_cwd_folder_fails_before_running() {
        let work = tempfile::tempdir().unwrap();
        let ctx = RecordingOptions(Default::default(), work.path().to_path_buf());
        let input = options_input(serde_json::json!({ "command": "tool", "cwd": "Tools" }));
        let err = ShellCommand::new()
            .process(input, &ProgressReporter::new_noop(), &ctx)
            .err()
            .expect("should fail");
        assert!(err.to_string().contains("'cwd' folder 'Tools'"), "{err}");
        assert!(ctx.0.lock().unwrap().is_empty(), "nothing should run");
    }

    #[test]
    fn test_validate_refuses_a_malformed_each_group() {
        let params = serde_json::json!({ "command": "tool", "args": [{ "each": "x" }] });
        let errors = ShellCommand::new().validate(params.as_object().unwrap());
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(errors[0].contains("each"), "{errors:?}");
    }

    #[test]
    fn test_validate_refuses_a_cwd_that_leaves_the_work_dir() {
        let params = serde_json::json!({ "command": "tool", "cwd": "../elsewhere" });
        let errors = ShellCommand::new().validate(params.as_object().unwrap());
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(errors[0].contains("'cwd'"), "{errors:?}");
    }

    #[test]
    fn test_cwd_param_is_internal_and_listed() {
        let meta = ShellCommand::new().metadata();
        let param = meta.parameters.iter().find(|p| p.name == "cwd");
        assert!(!param.expect("cwd should exist").surfaceable);
    }
}
