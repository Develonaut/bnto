// Control nodes — processors that steer a run rather than transform files.

/// Assert node — fail the run when a comparison doesn't hold.
pub mod assert;

pub use assert::Assert;
