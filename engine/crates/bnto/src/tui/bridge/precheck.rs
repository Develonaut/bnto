// What a TUI run checks before it asks to allow a shell recipe's commands,
// in `bnto run`'s order (ISS-77, ISS-78): fill `{{vars.*}}`, refuse an empty
// field or a param a step won't take (ISS-74), prepare the inputs, fill the
// output folder. Only then does it ask, so nobody approves a run that is
// refused, and the approval is pinned to the filled values (SEC-29). It runs
// on the bridge thread: preparing inputs walks picked folders.

use std::sync::mpsc;

use super::BridgeEvent;
use crate::catalog::SourceKind;
use crate::fill_with::FillWith;
use crate::input::{self, PreparedInput};
use crate::recipe_output_dir::recipe_output_dir;
use crate::storage::BntoPaths;
use crate::trust::TrustStore;
use crate::{trust_gate, unfilled_fields, vars};

/// What deciding consent needs from the UI thread: where the definition came
/// from (only the catalog knows), and the session's bnto home. Approvals are
/// kept in its trust file, and the run's context is built in it too, so a
/// run and its consent never see two homes (ISS-127).
#[derive(Debug, Clone)]
pub struct ConsentCheck {
    pub kind: SourceKind,
    pub paths: BntoPaths,
}

/// A run that passed its checks and may start.
pub struct Ready {
    pub prepared: PreparedInput,
    /// The recipe's own output folder, filled but not yet made.
    pub recipe_dir: Option<String>,
}

/// The run's inputs as `bnto run` takes them: input args and `-p` overrides.
pub struct RunArgs<'a> {
    pub slug: &'a str,
    pub definition_json: &'a str,
    pub args: &'a [String],
    pub overrides: &'a [String],
}

/// Check the run, then ask for consent if it needs it. `Ok(None)` when the
/// user didn't allow it: the run stops without a word, as on cancel.
pub fn check_then_consent(
    tx: &mpsc::Sender<BridgeEvent>,
    run: &RunArgs,
    consent: &ConsentCheck,
    ctx: &dyn bnto_core::ProcessContext,
) -> Result<Option<Ready>, String> {
    // The TUI has no --vars files: BNTO_VAR_<KEY> fills a var, as for `bnto run`.
    let definition_json = vars::apply(run.definition_json, &[])?;
    // A TUI field is filled on the recipe's form, not with -p (ISS-97).
    let form = FillWith::Form;
    unfilled_fields::refuse_unfilled_with(&definition_json, run.overrides, form)?;
    let prepared = input::prepare_inputs(&definition_json, run.args, run.overrides)?;
    let recipe_dir = recipe_output_dir(&prepared.definition_json, ctx, &prepared.files)?;
    let shown = (definition_json.as_str(), prepared.definition_json.as_str());
    let allowed = allowed(tx, shown, run.slug, consent);
    Ok(allowed.then_some(Ready {
        prepared,
        recipe_dir,
    }))
}

/// Whether the run may go ahead: no consent needed, or the user allowed it.
/// Waits for the answer; a dropped reply (the run was let go) is a no. The
/// prompt shows `shown` (the form's values filled), the approval is pinned
/// to `definition`.
fn allowed(
    tx: &mpsc::Sender<BridgeEvent>,
    (definition, shown): (&str, &str),
    slug: &str,
    consent: &ConsentCheck,
) -> bool {
    let store = TrustStore::load(&consent.paths.trusted_file());
    let kind = &consent.kind;
    let request = trust_gate::consent_needed_shown(definition, shown, slug, kind, &store);
    let Some(request) = request else {
        return true;
    };
    let (reply, answer) = mpsc::channel();
    tx.send(BridgeEvent::ConsentNeeded { request, reply })
        .is_ok()
        && answer.recv().is_ok()
}
