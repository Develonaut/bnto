// The terminal consent prompt for `bnto run` and `bnto dry-run`: the shared
// consent body (`consent_body`) as plain lines on stderr, then a y/N answer
// read from stdin.

use crate::consent_body::{ConsentLine, consent_body};
use crate::consent_request::ConsentRequest;
use crate::consent_summary::risk_summary;

/// Print the prompt for `request` to stderr and read the answer. True only
/// for `y` or `yes`; anything else, or no stdin, declines.
pub fn prompt_consent(request: &ConsentRequest) -> bool {
    use std::io::{BufRead, Write};

    for line in prompt_lines(request) {
        eprintln!("{line}");
    }
    eprint!("  Allow execution? [y/N] ");
    let _ = std::io::stderr().flush();

    let mut line = String::new();
    if std::io::stdin().lock().read_line(&mut line).is_err() {
        return false;
    }
    matches!(line.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Every line above the question, already escaped, ending with the risk
/// summary again so it sits right above `Allow execution?`.
pub fn prompt_lines(request: &ConsentRequest) -> Vec<String> {
    let mut lines = vec![String::new()];
    lines.extend(consent_body(request).iter().map(cli_line));
    let hint = if request.commands.is_empty() {
        "  Run `bnto dry-run <recipe>` without input files to see what it does first."
    } else {
        "  Run `bnto dry-run <recipe>` without input files to inspect the full commands."
    };
    lines.extend([String::new(), hint.to_string(), String::new()]);
    // The risks again, right above the question, after a long listing.
    lines.extend(risk_summary(request).map(|summary| format!("  {summary}")));
    lines
}

fn cli_line(line: &ConsentLine) -> String {
    match line {
        ConsentLine::Heading(text) => format!("  Warning: {text}."),
        ConsentLine::Summary(text) => format!("  {text}"),
        ConsentLine::Detail(text) => format!("  {text}"),
        ConsentLine::Section(title) => format!("  {title}:"),
        ConsentLine::Item(text) => format!("    - {text}"),
        ConsentLine::Flagged(text) => format!("    ! {text}"),
        // A script's lines carry their own indent (`consent_lines`).
        ConsentLine::More(text) => text.clone(),
        ConsentLine::Note(text) => format!("    {text}"),
        ConsentLine::Blank => String::new(),
    }
}

#[cfg(test)]
mod tests;
