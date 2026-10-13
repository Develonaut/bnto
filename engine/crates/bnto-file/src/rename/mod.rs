// Rename Files Node — transform filenames in the browser.
//
// Browser version is a filename transformer: takes a file's current name,
// applies transformation rules, and returns the same file data with the new name.
//
// Transformation order: sanitize -> find/replace -> case -> prefix -> suffix -> pattern.
// Sanitize runs first (if set) to clean the raw filename before other transforms.
// Pattern has "final say" — it overrides everything using template variables.
// Counter: {{counter}} auto-increments per-file in batch mode.

mod naming;
mod param_access;
mod params;
mod processor;
mod result_metadata;
mod transform;

pub use processor::RenameFiles;
