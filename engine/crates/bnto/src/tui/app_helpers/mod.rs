// App helper modules — organized by domain.
//
// Each module contains pure-ish helper functions used by `update()` in `app.rs`.
// Re-exported here so `app.rs` imports from `super::app_helpers::*`.

mod editor;
mod home_detail;
mod library;
mod navigation;
mod settings;
mod wizard;

pub(crate) use editor::{
    handle_editor, handle_editor_form, handle_open_editor_from_browser,
    handle_open_editor_from_library, load_editor_from_json,
};
pub(crate) use home_detail::{
    handle_config_confirmed, handle_detail_form, handle_execution, handle_execution_complete,
    handle_files_selected, handle_home_confirm, handle_preview_confirm, handle_preview_requested,
    handle_recipe_selected,
};
pub(crate) use library::{
    handle_add_to_library, handle_add_to_library_write, handle_library, handle_library_confirm,
    handle_open_library,
};
pub(crate) use navigation::handle_back;
pub(crate) use settings::{
    handle_open_settings_picker, handle_settings_path_confirmed, handle_telemetry_toggled,
    handle_theme_changed,
};
pub(crate) use wizard::{handle_open_wizard, handle_wizard, handle_wizard_form};
