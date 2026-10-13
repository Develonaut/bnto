// Wizard step transitions — Confirm advances a step, Back retreats one.

use bnto_core::registry::NodeRegistry;

use super::super::editor::EditorScreenModel;
use super::catalog::build_operations;
use super::config::{build_config_form, build_editor_model, generate_name};
use super::{WizardAction, WizardModel, WizardStep};

/// Advance from the current step.
pub(super) fn handle_confirm(
    mut model: WizardModel,
    registry: &NodeRegistry,
) -> (WizardModel, WizardAction) {
    match model.step {
        WizardStep::Category => confirm_category(&mut model),
        WizardStep::Operation => confirm_operation(&mut model, registry),
        WizardStep::Configure => finish(&mut model),
        WizardStep::Complete => {
            let editor = build_editor_model(&model, registry);
            let screen = EditorScreenModel::new(editor);
            return (model, WizardAction::Complete(Box::new(screen)));
        }
    }
    (model, WizardAction::None)
}

/// Pick the highlighted category and list its operations.
fn confirm_category(model: &mut WizardModel) {
    let cat = &model.categories[model.category_cursor];
    model.selected_category = Some(cat.category.clone());
    let types = bnto_core::metadata::all_node_types();
    model.operations = build_operations(&types, &cat.category);
    model.operation_cursor = 0;
    model.step = WizardStep::Operation;
}

/// Pick the highlighted operation; configure it, or skip straight to
/// Complete when it has no configurable params.
fn confirm_operation(model: &mut WizardModel, registry: &NodeRegistry) {
    let op = &model.operations[model.operation_cursor];
    model.selected_operation = Some(op.name.clone());

    let (params, form) = build_config_form(&op.name, registry);
    if params.is_empty() {
        finish(model);
    } else {
        model.params = params;
        model.form = Some(form);
        model.step = WizardStep::Configure;
    }
}

/// Name the recipe and move to the Complete summary.
fn finish(model: &mut WizardModel) {
    model.recipe_name = generate_name(model);
    model.step = WizardStep::Complete;
}

/// Retreat one step, or exit the wizard from the first step.
pub(super) fn handle_back(mut model: WizardModel) -> (WizardModel, WizardAction) {
    match model.step {
        WizardStep::Category => return (model, WizardAction::Back),
        WizardStep::Operation => {
            model.step = WizardStep::Category;
            model.selected_category = None;
            model.operations.clear();
        }
        WizardStep::Configure => {
            model.step = WizardStep::Operation;
            model.selected_operation = None;
            model.params.clear();
            model.form = None;
        }
        WizardStep::Complete => back_from_complete(&mut model),
    }
    (model, WizardAction::None)
}

/// Back from Complete goes to Configure if there were params,
/// otherwise to Operation.
fn back_from_complete(model: &mut WizardModel) {
    if model.params.is_empty() {
        model.step = WizardStep::Operation;
        model.selected_operation = None;
    } else {
        model.step = WizardStep::Configure;
    }
    model.recipe_name.clear();
}
