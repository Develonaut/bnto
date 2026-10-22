// CLIXML on a PowerShell step's stderr, decoded back to text. An encoded,
// non-interactive command with stderr redirected writes its error, warning
// and verbose records as `#< CLIXML` + `<Objs>…</Objs>`. pwsh 7 stops when
// given `-OutputFormat Text`; Windows PowerShell 5.1 always does it, and no
// flag changes that. Ansible decodes the same way for its Windows hosts.

/// The line PowerShell writes before a serialized stream.
const MARKER: &str = "#< CLIXML";

/// One streamed stderr line as text: the marker line becomes nothing, an
/// `<Objs>` line becomes the lines its strings hold, anything else stays.
pub(crate) fn output_lines(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    if trimmed == MARKER {
        return Vec::new();
    }
    if !trimmed.starts_with("<Objs") {
        return vec![line.to_string()];
    }
    strings(trimmed)
        .lines()
        .map(str::to_string)
        .filter(|l| !l.trim().is_empty())
        .collect()
}

/// `text` (a failure message) with each `#< CLIXML <Objs>…</Objs>` block
/// replaced by the text it holds. Unchanged when there's none.
pub(crate) fn decode(text: &str) -> String {
    let Some(at) = text.find(MARKER) else {
        return text.to_string();
    };
    let (before, rest) = text.split_at(at);
    let rest = &rest[MARKER.len()..];
    let end = rest
        .find("</Objs>")
        .map_or(rest.len(), |i| i + "</Objs>".len());
    let decoded = strings(&rest[..end]);
    let tail = decode(&rest[end..]);
    format!("{before}{}{tail}", decoded.trim_end())
}

/// Every `<S S="…">…</S>` string in `objs`, unescaped and joined.
fn strings(objs: &str) -> String {
    let mut out = String::new();
    let mut rest = objs;
    while let Some(open) = rest.find("<S S=") {
        let after = &rest[open..];
        let (Some(start), Some(close)) = (after.find('>'), after.find("</S>")) else {
            break;
        };
        if start < close {
            out.push_str(&unescape(&after[start + 1..close]));
        }
        rest = &after[close + "</S>".len()..];
    }
    out.replace("\r\n", "\n")
}

/// XML entities, then PowerShell's `_xHHHH_` escapes for control characters.
fn unescape(text: &str) -> String {
    let xml = text
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&");
    let mut out = String::with_capacity(xml.len());
    let mut rest = xml.as_str();
    while let Some(i) = rest.find("_x") {
        out.push_str(&rest[..i]);
        let code = rest
            .get(i + 2..i + 6)
            .filter(|_| rest.get(i + 6..i + 7) == Some("_"));
        match code
            .and_then(|hex| u32::from_str_radix(hex, 16).ok())
            .and_then(char::from_u32)
        {
            Some(c) => {
                out.push(c);
                rest = &rest[i + 7..];
            }
            None => {
                out.push_str("_x");
                rest = &rest[i + 2..];
            }
        }
    }
    out.push_str(rest);
    strip_ansi(&out)
}

/// Drops ANSI color codes (`ESC[31;1m`); text that only looks like one stays.
fn strip_ansi(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(i) = rest.find("\u{1b}[") {
        out.push_str(&rest[..i]);
        let after = &rest[i + 2..];
        let end = after.find(|c: char| !(c.is_ascii_digit() || c == ';'));
        rest = match end {
            Some(n) if after[n..].starts_with('m') => &after[n + 1..],
            _ => after,
        };
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Verbatim from CI: pwsh 7 on Linux, before `-OutputFormat Text`.
    const PWSH: &str = "<Objs Version=\"1.1.0.1\" xmlns=\"http://schemas.microsoft.com/powershell/2004/04\"><S S=\"Error\">_x001B_[31;1mWrite-Error: _x001B_[31;1mboom-9c1_x001B_[0m_x000A_</S></Objs>";

    // Windows PowerShell 5.1: its error view starts with the command text.
    const WINDOWS: &str = "<Objs Version=\"1.1.0.1\" xmlns=\"http://schemas.microsoft.com/powershell/2004/04\"><S S=\"Error\">Write-Error 'boom-9c1'_x000D__x000A_</S><S S=\"Error\">exit 3 : boom-9c1_x000D__x000A_</S><S S=\"Error\">    + CategoryInfo          : NotSpecified: (:) [Write-Error], WriteErrorException_x000D__x000A_</S></Objs>";

    #[test]
    fn the_marker_line_disappears_and_objs_become_text() {
        assert!(output_lines("#< CLIXML").is_empty());
        assert_eq!(output_lines(PWSH), ["Write-Error: boom-9c1"]);
        assert_eq!(
            output_lines(WINDOWS),
            [
                "Write-Error 'boom-9c1'",
                "exit 3 : boom-9c1",
                "    + CategoryInfo          : NotSpecified: (:) [Write-Error], WriteErrorException",
            ]
        );
    }

    #[test]
    fn plain_lines_pass_through() {
        assert_eq!(output_lines("hello [1m] _x_"), ["hello [1m] _x_"]);
    }

    #[test]
    fn a_failure_message_keeps_its_prefix_and_loses_the_xml() {
        let message = format!("Command 'pwsh' failed (exit 3): #< CLIXML\n{WINDOWS}");
        let decoded = decode(&message);
        assert!(decoded.starts_with("Command 'pwsh' failed (exit 3): Write-Error 'boom-9c1'\n"));
        assert!(decoded.contains("exit 3 : boom-9c1"), "{decoded}");
        assert!(
            !decoded.contains("CLIXML") && !decoded.contains("<Objs"),
            "{decoded}"
        );
        assert_eq!(
            decode("Command 'x' failed (exit 1): boom"),
            "Command 'x' failed (exit 1): boom"
        );
    }

    #[test]
    fn only_real_color_codes_are_dropped() {
        let objs = "<Objs><S S=\"Error\">_x001B_[31mred_x001B_[0m and [31;1m kept</S></Objs>";
        assert_eq!(strings(objs), "red and [31;1m kept");
    }

    #[test]
    fn escapes_are_undone() {
        let objs = "<Objs><S S=\"warning\">a &lt;b&gt; &amp; _x0009_c_x005F_x_</S></Objs>";
        assert_eq!(strings(objs), "a <b> & \tc_x_");
    }
}
