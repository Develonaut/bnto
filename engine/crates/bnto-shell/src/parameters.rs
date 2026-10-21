// Parameter schema for the shell-run node.
//
// One function per parameter; each sets only what differs from
// `ParameterDef::default()` (no default value, surfaceable, no UI hints).

use bnto_core::metadata::{ParameterDef, ParameterType};

use crate::validate::{DEFAULT_MAX_OUTPUT_MB, DEFAULT_TIMEOUT_SECS};

/// Every parameter the shell-run node accepts. None is a run option: they
/// are the program, and a recipe offers choices through `fields`.
pub(crate) fn build_parameters() -> Vec<ParameterDef> {
    let params = vec![
        command_param(),
        script_param(),
        shell_param(),
        args_param(),
        output_mode_param(),
        timeout_param(),
        max_output_size_param(),
        env_param(),
        search_path_param(),
        cwd_param(),
    ];
    params
        .into_iter()
        .map(|p| ParameterDef {
            run_option: false,
            ..p
        })
        .collect()
}

fn command_param() -> ParameterDef {
    ParameterDef {
        name: "command".to_string(),
        label: "Command".to_string(),
        description: "Binary to execute (e.g., 'ffmpeg', 'yt-dlp'). Must be on PATH and \
                      written literally. Set this or 'script', not both."
            .to_string(),
        param_type: ParameterType::String,
        placeholder: Some("ffmpeg".to_string()),
        ..Default::default()
    }
}

fn script_param() -> ParameterDef {
    ParameterDef {
        name: "script".to_string(),
        label: "Script".to_string(),
        description: "A shell script to run instead of 'command', written literally — no \
                      {{templates}}. Values from 'args' arrive as \"$1\", \"$2\"… (\"$0\" is \
                      'bnto'). Quote every expansion, avoid eval, and don't splice values into \
                      nested -c / -e code; put -- before values a tool could read as options. \
                      The shell starts with a cleared environment and no startup files."
            .to_string(),
        param_type: ParameterType::String,
        control: Some("textarea".to_string()),
        surfaceable: false,
        ..Default::default()
    }
}

fn shell_param() -> ParameterDef {
    ParameterDef {
        name: "shell".to_string(),
        label: "Shell".to_string(),
        description: "Which shell runs 'script': sh, bash, zsh, dash or ksh, by its fixed \
                      path (never PATH). Default: sh."
            .to_string(),
        param_type: ParameterType::String,
        default: Some(serde_json::Value::String("sh".to_string())),
        surfaceable: false,
        ..Default::default()
    }
}

fn args_param() -> ParameterDef {
    ParameterDef {
        name: "args".to_string(),
        label: "Arguments".to_string(),
        description: "Command arguments: text, conditional groups (nested lists, dropped \
                      when any part is empty) and {\"each\": [...]} groups, repeated once \
                      per incoming file with {{path}}, {{file}} and {{item.KEY}} filled."
            .to_string(),
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
        name: "searchPath".to_string(),
        label: "Search Path".to_string(),
        description: "Folders to find the command in, in order, instead of PATH. The command \
                      is still a bare name and runs by its full path; relative folders are \
                      under the working directory."
            .to_string(),
        param_type: ParameterType::Object,
        surfaceable: false,
        ..Default::default()
    }
}

fn cwd_param() -> ParameterDef {
    ParameterDef {
        name: "cwd".to_string(),
        label: "Working Folder".to_string(),
        description: "Folder to run the command in, under the working directory (e.g. \
                      'Tools'). Relative args are relative to it; search_path and \
                      {{output_dir}} are not. Absolute paths and \"..\" are refused."
            .to_string(),
        param_type: ParameterType::String,
        placeholder: Some("Tools".to_string()),
        surfaceable: false,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_param_is_an_option_for_someone_running_the_recipe() {
        // The command, its args and its limits are the program the author
        // wrote. A recipe offers the runner choices through `fields`.
        for param in build_parameters() {
            assert!(!param.run_option, "{} is shown when running", param.name);
        }
    }

    #[test]
    fn the_author_can_still_set_the_command() {
        let command = build_parameters().into_iter().find(|p| p.name == "command");
        assert!(
            command.unwrap().surfaceable,
            "the wizard and editor need it"
        );
    }
}
