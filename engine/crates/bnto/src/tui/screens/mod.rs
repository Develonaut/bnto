// Screen modules — each screen is a self-contained TEA unit.
//
// Screens added in subsequent waves:
//   Wave 1: browser
//   Wave 2: detail, picker
//   Wave 3: execution, results

pub mod browser;
pub mod detail;
mod detail_loader;
pub mod execution;
pub mod picker;
mod picker_loader;
pub mod results;
