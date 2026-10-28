// What a TUI run checks before it asks to allow a shell recipe's commands,
// in `bnto run`'s order (ISS-77, ISS-78): fill `{{vars.*}}`, refuse an empty
// field or a param a step won't take (ISS-74), prepare the inputs, fill the
// output folder. Only then does it ask, so nobody approves a run that is
// refused, and the approval is pinned to the filled values (SEC-29). It runs
// on the bridge thread: preparing inputs walks picked folders.

use std::path::PathBuf;
use std::sync::mpsc;

use super::BridgeEvent;
use crate::fill_with::FillWith;
use crate::input::{self, PreparedInput};
use crate::recipe_output_dir::recipe_output_dir;
use crate::trust::TrustStore;
use crate::{trust_gate, unfilled_fields, vars};

/// What deciding consent needs from the UI thread: whether the definition is
/// the shipped one (only the catalog knows), and where approvals are kept.
#[derive(Debug, Clone)]
pub struct ConsentCheck {
    pub bundled: bool,
    pub trusted_file: PathBuf,
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
    let allowed = allowed(tx, &definition_json, run.slug, consent);
    Ok(allowed.then_some(Ready {
        prepared,
        recipe_dir,
    }))
}

/// Whether the run may go ahead: no consent needed, or the user allowed it.
/// Waits for the answer; a dropped reply (the run was let go) is a no.
fn allowed(
    tx: &mpsc::Sender<BridgeEvent>,
    definition_json: &str,
    slug: &str,
    consent: &ConsentCheck,
) -> bool {
    let store = TrustStore::load(&consent.trusted_file);
    let Some(request) = trust_gate::consent_needed(definition_json, slug, consent.bundled, &store)
    else {
        return true;
    };
    let (reply, answer) = mpsc::channel();
    tx.send(BridgeEvent::ConsentNeeded { request, reply })
        .is_ok()
        && answer.recv().is_ok()
}
