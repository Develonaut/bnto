// Parameter schema for the shell-command node.
//
// One function per parameter; each sets only what differs from
// `ParameterDef::default()` (no default value, surfaceable, no UI hints).

use bnto_core::metadata::{ParameterDef, ParameterType};

use crate::validate::{DEFAULT_MAX_OUTPUT_MB, DEFAULT_TIMEOUT_SECS};

/// Every parameter the shell-command node accepts.
pub(crate) fn build_parameters() -> Vec<ParameterDef> {
    vec![
        command_param(),
        args_param(),
        output_mode_param(),
        timeout_param(),
        max_output_size_param(),
        env_param(),
        search_path_param(),
    ]
}

fn command_param() -> ParameterDef {
    ParameterDef {
        name: "command".to_string(),
        label: "Command".to_string(),
        description: "Binary to execute (e.g., 'ffmpeg', 'yt-dlp'). Must be on PATH.".to_string(),
        param_type: ParameterType::String,
        placeholder: Some("ffmpeg".to_string()),
        ..Default::default()
    }
}

fn args_param() -> ParameterDef {
    ParameterDef {
        name: "args".to_string(),
        label: "Arguments".to_string(),
        description: "Command arguments as an array of strings.".to_string(),
        param_type: ParameterType::String,
        control: Some("tagPicker".to_string()),
        ..Default::default()
    }
}

fn output_mode_param() -> ParameterDef {
    ParameterDef {
        name: "outputMode".to_string(),
        label: "Output Mode".to_string(),
        description: "How to collect output. 'stdout' captures command output. \
            'file' reads files written by the command to a temp directory \
            (use {{output_dir}} in args to inject the path)."
            .to_string(),
        param_type: ParameterType::String,
        default: Some(serde_json::Value::String("stdout".to_string())),
        ..Default::default()
    }
}

fn timeout_param() -> ParameterDef {
    ParameterDef {
        name: "timeout".to_string(),
        label: "Timeout".to_string(),
        description: "Maximum execution time in seconds. Default: 300.".to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::Value::Number(DEFAULT_TIMEOUT_SECS.into())),
        suffix: Some("seconds".to_string()),
        ..Default::default()
    }
}

fn max_output_size_param() -> ParameterDef {
    ParameterDef {
        name: "maxOutputSize".to_string(),
        label: "Max Output Size".to_string(),
        description: "Maximum size per output file in megabytes. Default: 100 MB.".to_string(),
        param_type: ParameterType::Number,
        default: Some(serde_json::Value::Number(DEFAULT_MAX_OUTPUT_MB.into())),
        suffix: Some("MB".to_string()),
        ..Default::default()
    }
}

fn env_param() -> ParameterDef {
    ParameterDef {
        name: "env".to_string(),
        label: "Environment".to_string(),
        description: "Additional environment variables for the command.".to_string(),
        param_type: ParameterType::Object,
        surfaceable: false,
        ..Default::default()
    }
}

fn search_path_param() -> ParameterDef {
    ParameterDef {
        name: "search_path".to_string(),
        label: "Search Path".to_string(),
        description: "Folders to find the command in, in order, instead of PATH. The command                       is still a bare name and runs by its full path; relative folders are                       under the working directory."
            .to_string(),
        param_type: ParameterType::Object,
        surfaceable: false,
        ..Default::default()
    }
}
