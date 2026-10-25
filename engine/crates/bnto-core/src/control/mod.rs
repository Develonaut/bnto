// Control nodes — processors that steer a run rather than transform files.

/// Assert node — fail the run when a comparison doesn't hold.
pub mod assert;
/// Report node — print a templated line to the run's output.
pub mod report;

mod pass_through;
/// `{{item.*}}` and `{{file}}` filled per file by a node that fills its own.
pub mod placeholder;

pub use assert::Assert;
pub use report::Report;
