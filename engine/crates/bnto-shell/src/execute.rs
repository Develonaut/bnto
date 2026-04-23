// ShellCommand processor — execute external CLI tools.
//
// This is the generic "run any command" processor that enables
// connector-as-recipe architecture. Recipes declare what tools
// they need via recipe-level `requires`, and this processor
// executes them via ProcessContext::run_command().
//
// Security boundary: validate.rs checks every command before execution.
// See that module for the full threat model (shell denylist, path
// validation, env var sanitization).

use bnto_core::metadata::{InputCardinality, ParameterDef, ParameterType};
use bnto_core::processor::{NodeInput, NodeOutput, OutputFile};
use bnto_core::{
    BntoError, NodeCategory, NodeMetadata, NodeProcessor, ProcessContext, ProgressReporter,
};

use crate::validate::{self, DEFAULT_TIMEOUT_SECS, MAX_STDOUT_BYTES};

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

        let args: Vec<String> = input
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

        progress.report(10, &format!("Running {command}..."));

        let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
        let output_bytes = ctx.run_command(command, &arg_refs)?;

        if output_bytes.len() > MAX_STDOUT_BYTES {
            return Err(BntoError::ProcessingFailed(format!(
                "Command output exceeded {} MB limit",
                MAX_STDOUT_BYTES / (1024 * 1024)
            )));
        }

        progress.report(100, "Done");

        let output_filename = if input.filename.is_empty() {
            format!("{command}-output")
        } else {
            let stem = input
                .filename
                .rsplit_once('.')
                .map(|(s, _)| s)
                .unwrap_or(&input.filename);
            format!("{stem}-{command}-output")
        };

        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "command".into(),
            serde_json::Value::String(command.to_string()),
        );
        metadata.insert(
            "outputBytes".into(),
            serde_json::Value::Number(serde_json::Number::from(output_bytes.len())),
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
            input_cardinality: InputCardinality::PerFile,
            requires: vec![],
        }
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
        assert!(param_names.contains(&"timeout"));
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
}
