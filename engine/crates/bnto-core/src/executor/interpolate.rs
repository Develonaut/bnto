// Single-pass placeholder filling. The template is scanned once, left to
// right, and each filled value is appended as plain text that is never
// scanned again — so a value can't pull in a secret through a placeholder
// it carries (SEC-17) or reproduce itself forever (SEC-18).

/// Replace each `{{inner}}` in `template` with `fill(inner)`. `None` keeps
/// the placeholder as written; an unclosed `{{` is kept as text.
pub(crate) fn interpolate(template: &str, fill: impl Fn(&str) -> Option<String>) -> String {
    let filled = try_interpolate(template, |inner| {
        Ok::<_, std::convert::Infallible>(fill(inner))
    });
    match filled {
        Ok(text) => text,
        Err(never) => match never {},
    }
}

/// `interpolate` with a fill that can fail: the first error stops the scan.
#[expect(clippy::too_many_lines)]
pub(crate) fn try_interpolate<E>(
    template: &str,
    mut fill: impl FnMut(&str) -> Result<Option<String>, E>,
) -> Result<String, E> {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else {
            out.push_str(&rest[start..]);
            return Ok(out);
        };
        let inner = &after[..end];
        if inner.contains("{{") {
            // `{{a {{fields.x}}`: the first `{{` is text; scan on from the next.
            out.push_str("{{");
            rest = after;
            continue;
        }
        match fill(inner)? {
            Some(value) => out.push_str(&value),
            None => out.push_str(&rest[start..start + end + 4]),
        }
        rest = &after[end + 2..];
    }
    out.push_str(rest);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn upper(inner: &str) -> Option<String> {
        inner.strip_prefix("x.").map(str::to_uppercase)
    }

    #[test]
    fn fills_each_placeholder_once() {
        assert_eq!(interpolate("a {{x.b}} c {{x.d}}", upper), "a B c D");
    }

    #[test]
    fn filled_text_is_never_rescanned() {
        let fill = |_: &str| Some("{{x.again}}".to_string());
        assert_eq!(interpolate("{{x.a}}", fill), "{{x.again}}");
    }

    #[test]
    fn unknown_and_unclosed_placeholders_stay_as_written() {
        assert_eq!(interpolate("{{y.b}} {{x.c", upper), "{{y.b}} {{x.c");
    }

    #[test]
    fn a_stray_open_brace_is_text() {
        assert_eq!(interpolate("{{ {{x.b}}", upper), "{{ B");
    }

    #[test]
    fn a_failed_fill_stops_the_scan() {
        let fill = |inner: &str| {
            if inner == "bad" {
                Err(inner.to_string())
            } else {
                Ok(None)
            }
        };
        assert_eq!(
            try_interpolate("{{ok}} {{bad}}", fill),
            Err("bad".to_string())
        );
    }

    #[test]
    fn text_without_placeholders_is_unchanged() {
        assert_eq!(interpolate("plain } { text", upper), "plain } { text");
    }
}
