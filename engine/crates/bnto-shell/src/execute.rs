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
use bnto_core::processor::{NodeInput, NodeOutput};
use bnto_core::{
    BntoError, NodeCategory, NodeMetadata, NodeProcessor, ProcessContext, ProgressReporter,
};

pub use crate::args::flatten_conditional_args;
use crate::args::resolve_input_placeholders;
use crate::file_mode::process_file_mode;
use crate::parameters::build_parameters;
use crate::stdout_mode::process_stdout_mode;
use crate::validate::{self, DEFAULT_MAX_OUTPUT_MB, DEFAULT_TIMEOUT_SECS};

// Items the tests below exercise directly.
#[cfg(test)]
use crate::file_mode::OUTPUT_DIR_PLACEHOLDER;
#[cfg(test)]
use crate::output_files::{collect_output_files, mime_from_extension};
#[cfg(test)]
use crate::stdout_mode::make_output_filename;
#[cfg(test)]
use bnto_core::processor::FileData;

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
        let params = &input.params;
        let command = required_command(params)?;
        let mut args = flattened_args(params);
        // Parsed (env sanitized) but not yet applied to the spawned command.
        let _timeout = u64_param(params, "timeout", DEFAULT_TIMEOUT_SECS);
        let _env = sanitized_env(params);
        // Security check: reject shell interpreters and path-based commands
        validate::validate_command(command).map_err(BntoError::InvalidInput)?;
        // Resolve {{url}} and {{input}} placeholders from injected params.
        // The CLI injects "url" or "text" params for URL/Text mode recipes.
        resolve_input_placeholders(&mut args, params);
        let max_output_mb = u64_param(params, "maxOutputSize", DEFAULT_MAX_OUTPUT_MB);
        match params.get("outputMode").and_then(serde_json::Value::as_str) {
            Some("file") => process_file_mode(command, &args, &input, progress, ctx, max_output_mb),
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

/// The required `command` param.
fn required_command(params: &Params) -> Result<&str, BntoError> {
    params
        .get("command")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| BntoError::InvalidInput("'command' parameter is required".to_string()))
}

/// The `args` param with conditional groups flattened (empty when unset).
fn flattened_args(params: &Params) -> Vec<String> {
    params
        .get("args")
        .and_then(serde_json::Value::as_array)
        .map(|arr| flatten_conditional_args(arr))
        .unwrap_or_default()
}

/// A numeric param, or `default` when unset.
fn u64_param(params: &Params, key: &str, default: u64) -> u64 {
    params
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(default)
}

/// Env vars with dangerous entries stripped silently.
fn sanitized_env(params: &Params) -> Params {
    params
        .get("env")
        .and_then(serde_json::Value::as_object)
        .map(validate::sanitize_env)
        .unwrap_or_default()
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

        // Verify FileData::Path variant — files stay on disk, not read into memory.
        for file in &files {
            assert!(
                matches!(&file.data, FileData::Path(_)),
                "collect_output_files should return FileData::Path, got Bytes for {}",
                file.filename,
            );
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_collect_output_files_preserves_subdirectory_in_filename() {
        let dir = std::env::temp_dir().join("bnto-test-collect-subdirs");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("subdir")).unwrap();
        std::fs::write(dir.join("file.txt"), b"data").unwrap();
        std::fs::write(dir.join("subdir").join("video.mp4"), vec![0u8; 50]).unwrap();

        let files = collect_output_files(&dir, DEFAULT_MAX_OUTPUT_MB).unwrap();
        assert_eq!(files.len(), 2);

        let names: Vec<&str> = files.iter().map(|f| f.filename.as_str()).collect();
        assert!(names.contains(&"file.txt"));
        assert!(names.contains(&"subdir/video.mp4"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_collect_output_files_preserves_deeply_nested_path() {
        let dir = std::env::temp_dir().join("bnto-test-collect-deep");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("a").join("b")).unwrap();
        std::fs::write(dir.join("a").join("b").join("deep.mp4"), vec![0u8; 10]).unwrap();

        let files = collect_output_files(&dir, DEFAULT_MAX_OUTPUT_MB).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].filename, "a/b/deep.mp4");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_collect_output_files_multiple_subdirs_preserve_structure() {
        let dir = std::env::temp_dir().join("bnto-test-collect-multi-subdirs");
        let _ = std::fs::remove_dir_all(&dir);
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
    fn test_collect_output_files_rejects_oversized_file() {
        let dir = std::env::temp_dir().join("bnto-test-collect-oversized");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
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

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_collect_output_files_accepts_file_under_limit() {
        let dir = std::env::temp_dir().join("bnto-test-collect-under-limit");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // 512 KB file with a 1 MB limit — should succeed.
        std::fs::write(dir.join("small.mp4"), vec![0u8; 512 * 1024]).unwrap();

        let files = collect_output_files(&dir, 1).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].filename, "small.mp4");

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
}
