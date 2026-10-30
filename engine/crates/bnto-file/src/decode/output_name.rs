// The decoded file's name, from the `filename` template and the input's name.

use bnto_core::split_extension;

/// `filename` with `{{name}}` and `{{ext}}` filled from `input` in one pass
/// (a name that reads like a token is kept as written), or `input` itself
/// when there's no template.
pub(super) fn output_name(input: &str, filename: Option<&str>) -> String {
    let Some(template) = filename else {
        return input.to_string();
    };
    let (name, ext) = split_extension(input);
    bnto_core::control::placeholder::fill_tokens(template, |token| match token {
        "name" => Some(name.to_string()),
        "ext" => Some(ext.to_string()),
        _ => None,
    })
}
