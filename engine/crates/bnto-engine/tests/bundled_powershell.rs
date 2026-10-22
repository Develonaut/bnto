// PSScriptAnalyzer over every bundled recipe's PowerShell script-mode steps,
// the PowerShell side of bundled_scripts.rs's ShellCheck gate (SEC-38).
// Warnings and errors fail, including PSAvoidUsingInvokeExpression, the rule
// behind the "never turn a value into code" contract. Without PowerShell or
// the module the checks skip, unless BNTO_REQUIRE_PSSCRIPTANALYZER=1 (set in
// Windows CI), where a missing analyzer fails so the gate can't pass by not
// running.

use std::path::{Path, PathBuf};
use std::process::Command;

#[path = "support/bundled_steps.rs"]
mod bundled_steps;

use bundled_steps::{ScriptStep, bundled_script_steps};

/// Lints `$env:BNTO_LINT_SCRIPT`, printing one line per finding; exits 3
/// when the module isn't installed.
const ANALYZE: &str = "$ErrorActionPreference = 'Stop'
if (-not (Get-Module -ListAvailable -Name PSScriptAnalyzer)) { exit 3 }
$found = Invoke-ScriptAnalyzer -ScriptDefinition $env:BNTO_LINT_SCRIPT -Severity Warning, Error
foreach ($f in $found) { '{0} (line {1}): {2}' -f $f.RuleName, $f.Line, $f.Message }";

/// Exit code `ANALYZE` uses for a missing module.
const NO_MODULE: i32 = 3;

enum Outcome {
    Clean,
    Findings(String),
    Missing,
}

fn powershell_steps() -> Vec<ScriptStep> {
    let steps = bundled_script_steps().into_iter();
    steps.filter(ScriptStep::is_powershell).collect()
}

/// An installed PowerShell, at the fixed paths bnto itself uses.
fn powershell() -> Option<PathBuf> {
    let candidates: Vec<PathBuf> = if cfg!(windows) {
        let root = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".into());
        vec![PathBuf::from(root).join(r"System32\WindowsPowerShell\v1.0\powershell.exe")]
    } else {
        [
            "/usr/bin/pwsh",
            "/opt/microsoft/powershell/7/pwsh",
            "/usr/local/bin/pwsh",
            "/opt/homebrew/bin/pwsh",
        ]
        .iter()
        .map(PathBuf::from)
        .collect()
    };
    candidates.into_iter().find(|p| p.exists())
}

fn analyze(powershell: &Path, script: &str) -> Outcome {
    let output = Command::new(powershell)
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            ANALYZE,
        ])
        .env("BNTO_LINT_SCRIPT", script)
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    match output.status.code() {
        Some(NO_MODULE) => Outcome::Missing,
        Some(0) if text.is_empty() => Outcome::Clean,
        _ => Outcome::Findings(format!("{text}{}", String::from_utf8_lossy(&output.stderr))),
    }
}

fn analyzer_required() -> bool {
    std::env::var("BNTO_REQUIRE_PSSCRIPTANALYZER").is_ok_and(|v| v == "1")
}

/// Skip without the analyzer, or fail when it's required.
fn on_missing() {
    assert!(
        !analyzer_required(),
        "PSScriptAnalyzer isn't available, and BNTO_REQUIRE_PSSCRIPTANALYZER=1 requires it"
    );
    eprintln!("skipping: PSScriptAnalyzer isn't available");
}

#[test]
fn every_bundled_powershell_script_passes_psscriptanalyzer() {
    let steps = powershell_steps();
    assert!(
        steps.iter().any(|s| s.at.starts_with("keep-awake/")),
        "no PowerShell steps found; the walk is broken"
    );
    let Some(powershell) = powershell() else {
        return on_missing();
    };
    let mut failures = Vec::new();
    for step in &steps {
        match analyze(&powershell, &step.script) {
            Outcome::Clean => {}
            Outcome::Findings(text) => failures.push(format!("{}:\n{text}", step.at)),
            Outcome::Missing => return on_missing(),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn invoke_expression_on_a_value_is_a_finding() {
    // Control: the gate can fail, so a pass above means something.
    let Some(powershell) = powershell() else {
        return on_missing();
    };
    match analyze(&powershell, "Invoke-Expression $env:BNTO_ARG_1\n") {
        Outcome::Findings(text) => assert!(text.contains("PSAvoidUsingInvokeExpression"), "{text}"),
        Outcome::Clean => panic!("PSScriptAnalyzer accepted Invoke-Expression"),
        Outcome::Missing => on_missing(),
    }
}
