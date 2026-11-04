// The consent prompt's contents, line by line, which `bnto run`'s terminal
// prompt and the TUI's consent screen both render, so the two always ask
// about the same things. Every piece of recipe text is escaped here
// (`display_text`), so nothing a recipe holds can rewrite the prompt.

use crate::consent_request::ConsentRequest;
use crate::consent_summary::{heading, risk_summary};
use crate::display_text::display_text;

/// One line of the prompt, by what it is; each surface styles its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsentLine {
    /// What the prompt is about, at the top.
    Heading(String),
    /// Its risks in one line, under the heading.
    Summary(String),
    /// The recipe's name and where it came from.
    Detail(String),
    /// A section's title: Steps, Paths, Commands, the files it runs.
    Section(&'static str),
    /// One entry in a section.
    Item(String),
    /// An entry that reaches outside, with why.
    Flagged(String),
    /// A further line of the entry above (a script's lines).
    More(String),
    /// A remark under a section.
    Note(&'static str),
    Blank,
}

/// Under the files its commands run: approval pins the recipe, not them.
pub const FILES_NOTE: &str = "Changes to these files won't ask again. The list may be incomplete.";

/// The prompt's lines for `request`, escaped. A section with nothing in it
/// isn't shown, so a recipe without shell steps lists no commands. Where it
/// reaches and what it runs come first, the steps last, so a short terminal
/// shows what matters before it scrolls.
pub fn consent_body(request: &ConsentRequest) -> Vec<ConsentLine> {
    let mut lines = vec![ConsentLine::Heading(heading(request).to_string())];
    lines.extend(risk_summary(request).map(ConsentLine::Summary));
    lines.extend([
        ConsentLine::Blank,
        ConsentLine::Detail(format!("Recipe: {}", display_text(&request.recipe_name))),
        ConsentLine::Detail(display_text(&request.origin).into_owned()),
        ConsentLine::Blank,
    ]);
    push_section(&mut lines, "Paths", path_lines(request));
    push_section(&mut lines, "Commands", command_lines(request));
    push_files(&mut lines, request);
    let steps = request.steps.iter().map(|s| ConsentLine::Item(shown(s)));
    push_section(&mut lines, "Steps", steps.collect());
    lines
}

/// The files its commands run, and that approving doesn't pin them.
fn push_files(lines: &mut Vec<ConsentLine>, request: &ConsentRequest) {
    let title = if request.project {
        "Project files its commands run or read"
    } else {
        "Files its commands run or read"
    };
    if push_section(lines, title, file_lines(request)) {
        lines.push(ConsentLine::Note(FILES_NOTE));
    }
}

/// Push `title` and `items` when there are any; whether it did.
fn push_section(
    lines: &mut Vec<ConsentLine>,
    title: &'static str,
    items: Vec<ConsentLine>,
) -> bool {
    if items.is_empty() {
        return false;
    }
    lines.push(ConsentLine::Section(title));
    lines.extend(items);
    true
}

/// `file-delete base: /Users/me  (absolute)` for each path param.
fn path_lines(request: &ConsentRequest) -> Vec<ConsentLine> {
    let line = |p: &crate::consent_paths::PathParam| {
        let filled = p
            .filled
            .as_ref()
            .map(|f| format!(" = {f}"))
            .unwrap_or_default();
        let text = format!(
            "{} {}: {}{filled}",
            shown(&p.node_type),
            p.param,
            shown(&p.value)
        );
        match p.flag {
            Some(flag) => ConsentLine::Flagged(format!("{text}  ({})", flag.describe())),
            None => ConsentLine::Item(text),
        }
    };
    request.paths.iter().map(line).collect()
}

/// Each command's first line as an item, a script's lines below it.
fn command_lines(request: &ConsentRequest) -> Vec<ConsentLine> {
    let mut lines = Vec::new();
    for command in &request.commands {
        for (i, part) in command.lines().enumerate() {
            lines.push(if i == 0 {
                ConsentLine::Item(shown(part))
            } else {
                ConsentLine::More(shown(part))
            });
        }
    }
    lines
}

/// `bin  (searchPath)` for each file its commands run.
fn file_lines(request: &ConsentRequest) -> Vec<ConsentLine> {
    let line = |f: &crate::consent_files::RunFile| {
        ConsentLine::Item(format!("{}  ({})", shown(&f.path), f.source))
    };
    request.files.iter().map(line).collect()
}

fn shown(text: &str) -> String {
    display_text(text).into_owned()
}

#[cfg(test)]
mod tests;
