//! `bnto-form` — TEA-native ratatui form widgets.
//!
//! Standalone crate with zero bnto dependency. Provides TextInput, Select,
//! Confirm, and Number field types with pure-function state transitions
//! and rendering (returns `Vec<Line>`, not Widget trait impls).
//!
//! # Usage
//!
//! ```rust
//! use bnto_form::{FormModel, text, select, confirm, number, update, render_form, map_key_event};
//!
//! let form = FormModel::new(vec![
//!     text("name").label("Recipe Name").placeholder("My Recipe").build(),
//!     number("quality").label("Quality").range(1.0, 100.0).suffix("%").value("80").build(),
//! ]);
//! ```

pub mod controls;
pub mod field;
pub mod form;
pub mod keys;
pub mod render;
pub mod theme;
pub mod widgets;

// --- Public API re-exports ---

pub use field::{
    Field, FieldBuilder, FieldKind, FieldState, SelectOption, ValidatorFn, confirm, number, select,
    text,
};
pub use form::{FormMessage, FormModel, update};
pub use keys::map_key_event;
pub use render::render_form;
pub use theme::{DefaultTheme, FormTheme};
