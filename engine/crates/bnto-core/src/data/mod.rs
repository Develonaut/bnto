// Data nodes — processors that work on the fields items carry, not their files.

/// Edit Fields node — set fields on each item from templates over its others.
pub mod edit_fields;
/// Summarize node — group items by fields, with each group's count and totals.
pub mod summarize;

pub use edit_fields::EditFields;
pub use summarize::Summarize;
