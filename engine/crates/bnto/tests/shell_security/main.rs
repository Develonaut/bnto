// shell-run security suite — attacks the real `bnto` binary with the
// findings in `.claude/research/reports/Shell script injection safety.md`.
//
// Each test is tagged SEC-xx (index: `.claude/scopes/security/shell-run.md`).
// Guards that hold today run normally and block regressions. Open findings
// are written as the behaviour we want and `#[ignore = "OPEN SEC-xx ..."]`d;
// list them with `cargo test -p bnto --test shell_security -- --ignored`.
// A hardening PR un-ignores the tests it makes pass.

#[path = "../helpers/mod.rs"]
mod helpers;
mod support;

mod argv_values;
mod command_policy;
mod consent;
mod enum_values;
mod env_policy;
mod input_modes;
mod interpreter_code;
mod script_mode;
mod template_values;
mod windows_batch;
