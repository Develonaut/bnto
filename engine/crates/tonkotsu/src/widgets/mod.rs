//! Pure rendering functions for each field type.
//! Each function returns `Vec<Line>` that callers compose into their layout.

pub mod confirm;
pub mod display;
pub mod file_path;
pub mod note;
pub mod number;
pub mod select;
pub mod text_area;
pub mod text_input;
