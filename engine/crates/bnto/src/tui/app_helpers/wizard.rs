// Wizard helpers — open, forward messages, form routing.

use super::super::app::{AppModel, DetailOrigin, Screen};
use super::super::screens::wizard::{
    WizardAction, WizardMessage, WizardModel, update as wizard_update,
};
use super::navigation::back_screen_for_editor;

/// Forward wizard messages, handling actions (complete, back).
pub(crate) fn handle_wizard(model: AppModel, msg: WizardMessage) -> AppModel {
    let from = match &model.screen {
        Screen::Wizard { from } => *from,
        _ => DetailOrigin::Home,
    };
    match model.wizard {
        Some(wizard_model) => {
            let (new_wizard, action) = wizard_update(wizard_model, msg, &model.registry);
            match action {
                WizardAction::None => AppModel {
                    wizard: Some(new_wizard),
                    ..model
                },
                WizardAction::Complete(editor_screen) => AppModel {
                    screen: Screen::Editor { from },
                    editor: Some(*editor_screen),
                    wizard: None,
                    ..model
                },
                WizardAction::Back => {
                    let back = back_screen_for_editor(from);
                    AppModel {
                        screen: back,
                        wizard: None,
                        ..model
                    }
                }
            }
        }
        None => model,
    }
}

/// Route form messages through the wizard's update.
pub(crate) fn handle_wizard_form(model: AppModel, form_msg: tonkotsu::FormMessage) -> AppModel {
    match model.wizard {
        Some(wizard_model) => {
            let msg = WizardMessage::Form(form_msg);
            let (new_wizard, _) = wizard_update(wizard_model, msg, &model.registry);
            AppModel {
                wizard: Some(new_wizard),
                ..model
            }
        }
        None => model,
    }
}

/// Open the wizard for guided recipe creation.
pub(crate) fn handle_open_wizard(model: AppModel) -> AppModel {
    let from = match &model.screen {
        Screen::Home => DetailOrigin::Home,
        Screen::Browser => DetailOrigin::Browser,
        Screen::Library => DetailOrigin::Library,
        _ => DetailOrigin::Home,
    };
    let wizard = WizardModel::new(&model.registry);
    AppModel {
        screen: Screen::Wizard { from },
        wizard: Some(wizard),
        ..model
    }
}
