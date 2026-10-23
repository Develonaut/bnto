// `shell-run` argument preparation — conditional arg groups and the
// `{{url}}` / `{{input}}` placeholders injected by input preparation.

/// Placeholder in args for URL/text input injected by the CLI's input preparation.
const URL_PLACEHOLDER: &str = "{{url}}";
const INPUT_PLACEHOLDER: &str = "{{input}}";
const OUTPUT_DIR_TOKEN: &str = "{{output_dir}}";

/// Resolve `{{url}}` and `{{input}}` placeholders in args from injected params.
///
/// The CLI's input preparation injects `"url"` or `"text"` into the node's
/// params for URL/Text mode recipes. If no placeholder exists in the args
/// but a `url` param is present, append the URL as the last argument.
#[expect(clippy::too_many_lines)]
pub(crate) fn resolve_input_placeholders(
    args: &mut Vec<String>,
    params: &serde_json::Map<String, serde_json::Value>,
) -> Result<(), String> {
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
    // SEC-27: `{{output_dir}}` is filled after this, so input carrying it
    // would have it filled too.
    if input_val.contains(OUTPUT_DIR_TOKEN) {
        return Err(format!(
            "the input carries '{OUTPUT_DIR_TOKEN}', which shell-run fills itself"
        ));
    }

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
    Ok(())
}

/// Flatten a resolved `args` array that may contain conditional groups.
///
/// After template resolution, the args array can contain:
/// - **Flat string**: included if non-empty, dropped if `""`
/// - **Nested array**: ALL elements must be non-empty strings → flattened
///   into parent; if ANY element is `""` → entire group dropped
/// - **Other types**: dropped (backward compat)
///
/// This enables conditional flag groups in recipes:
/// ```json
/// "args": ["--always", ["--cookies-from-browser", "{{fields.browser}}"], "{{fields.thumbnail}}"]
/// ```
/// When `browser=""`: `["--always"]`
/// When `browser="chrome"`: `["--always", "--cookies-from-browser", "chrome"]`
pub fn flatten_conditional_args(args: &[serde_json::Value]) -> Vec<String> {
    let mut result = Vec::new();
    for item in args {
        match item {
            serde_json::Value::String(s) if !s.is_empty() => {
                result.push(s.clone());
            }
            serde_json::Value::Array(group) => {
                let strings: Vec<&str> =
                    group.iter().filter_map(serde_json::Value::as_str).collect();
                // All elements must be present strings AND non-empty.
                let all_non_empty =
                    strings.len() == group.len() && strings.iter().all(|s| !s.is_empty());
                if all_non_empty {
                    result.extend(strings.into_iter().map(String::from));
                }
            }
            _ => {}
        }
    }
    result
}
