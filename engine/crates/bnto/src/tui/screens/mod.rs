// Screen modules — each screen is a self-contained TEA unit.
//
// Screens added in subsequent waves:
//   Wave 1: browser
//   Wave 2: detail, picker
//   Wave 3: execution, results

pub mod browser;
pub mod detail;
pub(crate) mod detail_bridge;
pub(crate) mod detail_loader;
pub mod editor;
pub(crate) mod editor_bridge;
pub mod execution;
pub mod home;
pub mod library;
pub mod nav_history;
pub mod picker;
mod picker_loader;
mod picker_update;
pub mod results;
pub mod settings;
pub mod viewport;
pub mod wizard;
