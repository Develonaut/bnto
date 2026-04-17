// Type-aware control logic — pure functions for boolean, enum, and number params.
//
// No ratatui dependency. Each module receives param state and returns the next value.

pub mod boolean;
pub mod enum_select;
pub mod number;
