// What shell-command refuses in its params as the recipe wrote them, before
// templates fill data in (`NodeProcessor::validate_raw`).

use serde_json::{Map, Value};

/// Problems in the unfilled params: a `command` that data could choose.
pub(crate) fn raw_problems(raw: &Map<String, Value>) -> Vec<String> {
    let mut problems = Vec::new();
    if let Some(command) = raw.get("command").and_then(Value::as_str)
        && bnto_core::placeholders::mentions_data(command)
    {
        // SEC-07: like Home Assistant, the program name is written literally
        // so a file name or field value can never pick what runs.
        problems.push(format!(
            "'command' must be written literally, but '{command}' lets data choose \
             the program. Name the tool and pass values as arguments"
        ));
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(json: Value) -> Map<String, Value> {
        json.as_object().unwrap().clone()
    }

    #[test]
    fn a_literal_command_passes() {
        assert!(raw_problems(&params(serde_json::json!({ "command": "ffmpeg" }))).is_empty());
    }

    #[test]
    fn a_command_from_data_is_refused() {
        for command in ["{{fields.tool}}", "{{item.bin}}", "yt-{{env.X}}", "{{url}}"] {
            let problems = raw_problems(&params(serde_json::json!({ "command": command })));
            assert_eq!(problems.len(), 1, "{command}");
            assert!(problems[0].contains("literal"), "{}", problems[0]);
        }
    }

    #[test]
    fn a_command_from_run_vars_is_allowed() {
        let raw = params(serde_json::json!({ "command": "{{vars.Tool}}" }));
        assert!(raw_problems(&raw).is_empty());
    }
}
