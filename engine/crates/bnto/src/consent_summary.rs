// The top of the consent prompt: a heading by what the recipe is and does,
// and one line naming its risks ("2 flagged paths · deletes files · runs 1
// command"), so they show before anything scrolls out of view.

use crate::consent_request::ConsentRequest;
use crate::path_flag::PathFlag;

/// The heading when the recipe has shell steps.
pub const RUNS_COMMANDS: &str = "This recipe runs commands on your computer";
/// The heading for a project recipe without them.
pub const NOT_APPROVED: &str = "This project recipe hasn't been approved yet";
/// The heading for any other recipe that asks.
pub const NEEDS_OK: &str = "This recipe needs your OK before it runs";

/// The heading for `request`, by its steps and where it came from.
pub fn heading(request: &ConsentRequest) -> &'static str {
    if has_step(request, bnto_shell::NODE_TYPE) {
        RUNS_COMMANDS
    } else if request.project {
        NOT_APPROVED
    } else {
        NEEDS_OK
    }
}

/// The risks in a line, or `None` when it has none to name.
pub fn risk_summary(request: &ConsentRequest) -> Option<String> {
    let flagged = request
        .paths
        .iter()
        .filter(|p| p.flag.is_some_and(|f| !replaces(f)))
        .count();
    let overwrites = request.paths.iter().any(|p| p.flag.is_some_and(replaces));
    let parts: Vec<String> = [
        (flagged > 0).then(|| counted(flagged, "flagged path")),
        has_step(request, "file-delete").then(|| "deletes files".into()),
        has_step(request, "file-move").then(|| "moves files".into()),
        overwrites.then(|| "overwrites files".into()),
        (!request.commands.is_empty())
            .then(|| format!("runs {}", counted(request.commands.len(), "command"))),
    ]
    .into_iter()
    .flatten()
    .collect();
    (!parts.is_empty()).then(|| parts.join(" · "))
}

fn replaces(flag: PathFlag) -> bool {
    matches!(
        flag,
        PathFlag::Overwrite | PathFlag::ReplacesExisting | PathFlag::MayReplace
    )
}

fn has_step(request: &ConsentRequest, node_type: &str) -> bool {
    request.steps.iter().any(|s| s.trim() == node_type)
}

/// `1 command`, `2 commands`.
fn counted(n: usize, noun: &str) -> String {
    if n == 1 {
        format!("1 {noun}")
    } else {
        format!("{n} {noun}s")
    }
}
