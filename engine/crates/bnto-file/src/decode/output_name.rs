// The decoded file's name, from the `filename` template and the input's name.

/// `filename` with `{{name}}` and `{{ext}}` filled from `input`, or `input`
/// itself when there's no template.
pub(super) fn output_name(input: &str, filename: Option<&str>) -> String {
    let Some(template) = filename else {
        return input.to_string();
    };
    let (name, ext) = split_extension(input);
    template.replace("{{name}}", name).replace("{{ext}}", ext)
}

/// Split a name on the last dot of its last path segment: `sub/a.csv.b64`
/// gives (`sub/a.csv`, `b64`). A leading dot (`.env`) is no extension.
fn split_extension(name: &str) -> (&str, &str) {
    let segment_start = name.rfind('/').map_or(0, |i| i + 1);
    match name[segment_start..].rfind('.') {
        Some(dot) if dot > 0 => {
            let at = segment_start + dot;
            (&name[..at], &name[at + 1..])
        }
        _ => (name, ""),
    }
}
