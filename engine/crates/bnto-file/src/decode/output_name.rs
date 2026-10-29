// The decoded file's name, from the `filename` template and the input's name.

use bnto_core::split_extension;

/// `filename` with `{{name}}` and `{{ext}}` filled from `input`, or `input`
/// itself when there's no template.
pub(super) fn output_name(input: &str, filename: Option<&str>) -> String {
    let Some(template) = filename else {
        return input.to_string();
    };
    let (name, ext) = split_extension(input);
    template.replace("{{name}}", name).replace("{{ext}}", ext)
}
