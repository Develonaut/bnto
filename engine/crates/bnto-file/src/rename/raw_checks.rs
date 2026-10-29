// What file-rename refuses in its params as the recipe wrote them
// (`NodeProcessor::validate_raw`): a param the pattern leaves no token for
// would run and be thrown away (ISS-107).

use serde_json::{Map, Value};

use super::param_access::get_string_param;

/// Params that feed `{{name}}`. `separator`, `maxLength` and `replace` only
/// act through `sanitize` and `find`.
const NAME_PARAMS: [&str; 5] = ["sanitize", "find", "case", "prefix", "suffix"];

/// One problem per token the pattern leaves out while a param feeds it.
pub(super) fn raw_problems(raw: &Map<String, Value>) -> Vec<String> {
    let Some(pattern) = set(raw, "pattern") else {
        return Vec::new();
    };
    // A field is a value the user gives per run, which may be a whole pattern
    // holding the tokens. An `{{item.*}}` name from data is a whole new name
    // (renaming from a table's rows), so a prefix beside it is still dropped.
    if pattern.contains("{{fields.") {
        return Vec::new();
    }
    let pattern = pattern.as_str();
    let mut problems = Vec::new();
    let name_params: Vec<&str> = NAME_PARAMS
        .into_iter()
        .filter(|p| set(raw, p).is_some())
        .collect();
    if !name_params.is_empty() && !pattern.contains("{{name}}") {
        problems.push(left_out(pattern, "{{name}}", &name_params));
    }
    if set(raw, "extension").is_some() && !pattern.contains("{{ext}}") {
        problems.push(left_out(pattern, "{{ext}}", &["extension"]));
    }
    problems
}

/// The param's value when it is set and not empty, read as the rename reads
/// it (text or a number).
fn set(raw: &Map<String, Value>, key: &str) -> Option<String> {
    get_string_param(raw, key).filter(|value| !value.is_empty())
}

fn left_out(pattern: &str, token: &str, params: &[&str]) -> String {
    let named: Vec<String> = params.iter().map(|p| format!("'{p}'")).collect();
    let (them, verb) = if params.len() == 1 {
        ("it", "does")
    } else {
        ("them", "do")
    };
    format!(
        "'pattern' \"{pattern}\" has no {token}, so {} {verb} nothing: \
         put {token} in the pattern, or remove {them}",
        named.join(" and ")
    )
}
