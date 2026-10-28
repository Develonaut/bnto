// How a refusal made before a run starts says to fill what's missing: the
// command line's `-p`, or the TUI's recipe form (ISS-97).

/// How a refusal says to fill a field or param.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillWith {
    /// The command line: `-p ...`.
    Flag,
    /// The TUI: the recipe's form, which shows each field by its label.
    Form,
}
