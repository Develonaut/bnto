# Decision: PowerShell Script Mode

**Created:** October 22, 2026
**Status:** Decided. Ships in keep-awake PR 9 (`feat/powershell-script-mode`, [tracker](../strategy/keep-awake-recipe.md)).
**Applies to:** `shell-run` script mode (`bnto-shell`), the CLI's consent and dry-run previews, the shell-run security specs.
**Evidence:** [Shell script injection safety report](../research/reports/Shell%20script%20injection%20safety.md), recommendation 8.

---

## The problem

Script mode (`{ "shell": "sh", "script": "…", "args": [...] }`) runs a literal POSIX script with values as `"$1"…"$N"` (SEC-20..26). Windows has no POSIX shell, so script mode refuses to run there, and keep-awake on Windows (PR 11) needs `SetThreadExecutionState`, which only PowerShell (or a compiled helper) can reach without installing anything.

PowerShell can't reuse the POSIX shape. `-Command <script> <args…>` **appends every following argument to the code** ([about_Pwsh](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_pwsh)), so trailing values would recreate exactly the injection script mode exists to prevent. `-CommandWithArgs` (which fills `$args`) exists only in pwsh 7.4+; Windows PowerShell 5.1, the one every Windows ships with, has no equivalent.

## The decision

### Shells

| `shell`      | What it is                                 | Runs on               | Absolute paths tried, in order                                                                                                                                                                                                 |
| ------------ | ------------------------------------------ | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `powershell` | Windows PowerShell 5.1, built into Windows | Windows only          | `%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe`, then `C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe`                                                                                                |
| `pwsh`       | PowerShell 7, installed separately         | Windows, macOS, Linux | Windows `C:\Program Files\PowerShell\7\pwsh.exe`; macOS `/usr/local/bin/pwsh`, `/opt/homebrew/bin/pwsh`, `/usr/local/microsoft/powershell/7/pwsh`; Linux `/usr/bin/pwsh`, `/opt/microsoft/powershell/7/pwsh`, `/snap/bin/pwsh` |

- Never a PATH or `searchPath` lookup (SEC-24's rule). `%SystemRoot%` comes from bnto's own environment, which only the user controls; data never reaches it.
- `powershell` off Windows, and the POSIX shells on Windows, fail with a message naming what does run there. A recipe that needs one guards the step with `when {{ctx.platform}} equals windows`, as keep-awake's macOS group does.
- **Why `pwsh` too:** it costs a path list, it lets the same specs run on Linux CI (ubuntu-latest ships pwsh) as well as Windows, and it serves people who already script in PowerShell 7 on any OS.

### The invocation

```
<abs path> -NoLogo -NoProfile -NonInteractive -OutputFormat Text -EncodedCommand <base64(UTF-16LE(prelude + script))>
```

- **`-EncodedCommand`, and nothing after it.** No value is ever on the command line, so no PowerShell or Windows command-line parsing can turn one into code or into an extra argument.
- **`-NoProfile`** so no profile script runs (the analogue of `bash -p` / `zsh -f`); **`-NonInteractive`** so a prompt fails instead of hanging the run.
- **`-OutputFormat Text`, explicitly, and CLIXML decoded.** An encoded, non-interactive command whose stderr is redirected writes its error, warning and verbose records as `#< CLIXML <Objs…>`. pwsh 7 stops when the output format is given on the command line; Windows PowerShell 5.1 does it regardless. The first CI runs caught both: a failing step's message arrived as XML. So bnto also decodes it (`bnto-shell/src/clixml.rs`, PowerShell steps only): the marker line is dropped and each `<S S="…">` string becomes text, in streamed output and in the failure message. This is how Ansible reads PowerShell's stderr. Wrapping the script in `& { … } *>&1` to reformat records was rejected: `using namespace` must open a script, so it can't sit inside a wrapper, and `exit` and `$?` would change.
- **The prelude** is fixed engine text, the same for every step, prepended before encoding:
  - `$ProgressPreference = 'SilentlyContinue'`, so progress records never reach stderr;
  - `[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)`, so stdout is UTF-8 like every other step (5.1 defaults to the OEM code page);
  - `if ($PSStyle) { $PSStyle.OutputRendering = 'PlainText' }`, so error text carries no ANSI color codes (`$PSStyle` exists from pwsh 7.2).

  The prelude is a single line, because 5.1's error view quotes the command text it ran. It never sets `$ErrorActionPreference`: a POSIX script doesn't get `set -e` either, so failure handling stays the author's choice. Consent shows the author's script, not the prelude, the way it doesn't show `-p`.

- **Exit status:** the script's `exit N` is the step's exit code; a terminating error exits 1. Same success/failure rules as every command.
- **Why not `-File <temp.ps1>`:** script files are subject to the execution policy, and a machine policy set by Group Policy overrides `-ExecutionPolicy Bypass`, so the step would fail on managed machines. `-EncodedCommand` isn't subject to the execution policy. A temp file would also add a window to swap the script after consent.
- **Why not `-Command -` (script on stdin):** PowerShell reads stdin like an interactive session, so multi-line constructs need trailing blank lines; and `CommandOptions` has no stdin channel today.

### Values: `BNTO_ARG_1`…`BNTO_ARG_N`, `BNTO_ARG_COUNT`

The step's `args` (after `each` expansion and conditional groups, exactly as for POSIX scripts) are set as environment variables on the child only: `BNTO_ARG_1`…`BNTO_ARG_N` and `BNTO_ARG_COUNT=N`. The script reads `$env:BNTO_ARG_1`. This is the only channel that works the same on 5.1 and 7.x and never touches the Windows command line (report recommendation 8; GitHub Actions and Jenkins `withEnv` use the same pattern).

- They're added after the cleared environment and after the step's own `env`, so nothing can shadow them. A step's `env` may not name `BNTO_ARG_*` in script mode at all (refused by `validate`), so an author can't mistake one for the other.
- An empty arg drops before it gets a number, exactly as it does for a POSIX script's `$1…` (conditional args), so `BNTO_ARG_COUNT` counts only the values that are set.

### Environment

Script mode's cleared environment and allowlist (`bnto/src/child_env.rs`, SEC-26) apply unchanged, plus one name: **`__PSLockdownPolicy`**, so a machine that forces Constrained Language Mode through it doesn't lose that by running a step. **`PSModulePath` is dropped on purpose:** PowerShell falls back to its built-in module paths, so an exported path can't plant a module that shadows a command the script autoloads.

### Length limit

A Windows command line holds at most 32,767 characters, and the encoded script is about 2.7 characters per script character. `validate` refuses a PowerShell script whose command line would pass 30,000 characters (roughly 11,000 script characters), naming the length, on every OS so a recipe behaves the same everywhere. `bnto check` reports it before a run.

### The safe-script contract (PowerShell)

The engine guarantees the script is the one consent showed, PowerShell is the real one, no profile runs, and values arrive only as environment variables. How the script uses them is up to the author. Bundled and user scripts follow these rules (added to [shell-run.md](../scopes/security/shell-run.md#script-mode-the-safe-script-contract)):

- Read values only as `$env:BNTO_ARG_n`. Interpolating one into a string (`"…$env:BNTO_ARG_1…"`) is safe: it's text, not code.
- Never turn a value into code: no `Invoke-Expression` / `iex`, `[scriptblock]::Create(…)`, `Add-Type` with a value in its source, `powershell -Command "…$env:BNTO_ARG_1…"`, `cmd /c "…"`, `Start-Process … -ArgumentList "…"` built from a value.
- Pass values to native programs as separate arguments (`& tool -- $env:BNTO_ARG_1`), and **validate them first on Windows PowerShell 5.1**: its legacy native-argument passing doesn't escape embedded `"`, so a value containing quotes can split into extra arguments for the program (pwsh 7.3+ fixed this with `$PSNativeCommandArgumentPassing = 'Standard'`). Prefer cmdlets and .NET calls, which take values as objects.
- Validate enum-like values with `switch` / `-in` before use, as the POSIX contract asks for `case`.

### Display

- **Consent** shows `powershell <script> $env:BNTO_ARG_n: <args as written>` with the whole script below it, so what's approved is what runs.
- **`bnto dry-run`** shows `powershell '<script>' BNTO_ARG_1=… BNTO_ARG_2=…` with the values filled, never the base64.

## Security specs

New rows in [shell-run.md](../scopes/security/shell-run.md#findings), each a test against the real binary in `bnto/tests/shell_security/powershell_mode.rs`. They run wherever a PowerShell is installed: Windows CI (`powershell` and `pwsh`) and Linux CI (`pwsh`, with `BNTO_REQUIRE_PWSH=1` so a missing pwsh fails instead of skipping, like ShellCheck). Locally they skip without one.

| ID     | Spec                                                                                                                                       |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| SEC-32 | Values arrive only as `$env:BNTO_ARG_n`: `'; New-Item canary; '`, `$(New-Item canary)`, `"; …`, a backtick and a newline payload stay data |
| SEC-33 | Nothing follows `-EncodedCommand`; a value never appears on the command line                                                               |
| SEC-34 | A planted profile never runs (`-NoProfile`, temp HOME/USERPROFILE)                                                                         |
| SEC-35 | PowerShell runs by fixed absolute path; a `pwsh`/`powershell.exe` planted on PATH or in `searchPath` never runs                            |
| SEC-36 | The cleared environment applies: an exported secret isn't visible; `PSModulePath` can't plant a module                                     |
| SEC-37 | A step's `env` can't set `BNTO_ARG_*`, and `BNTO_ARG_COUNT` matches the args                                                               |

SEC-21 (templates in `script`/`shell` refused) and SEC-25 (only known shells) extend to the new shells with one more case each.

## Trade-offs

- **`-EncodedCommand` is a known attacker technique**, so endpoint security (Defender for Endpoint and others) records encoded PowerShell launches and some rules alert on them. It isn't blocked by default, and AMSI scans the decoded script either way. Accepted: it's the only channel that is both injection-proof and immune to execution policy on 5.1. If it causes trouble in practice, pwsh 7.4+'s `-CommandWithArgs` is the escape hatch for machines that have it.
- **No PowerShell lint in CI yet.** POSIX bundled scripts pass ShellCheck (SEC-31). The first bundled PowerShell script (keep-awake Windows, PR 11) adds the equivalent: PSScriptAnalyzer on Windows CI, including `PSAvoidUsingInvokeExpression`.
- **A recipe can still use `shell: powershell` without a Windows guard**, and it fails on macOS/Linux with a clear message. `bnto check` doesn't flag it. Revisit if it bites.
- **Script length** is capped by the Windows command line. Keep-awake's Windows script is expected well under the cap.

## Not in this PR

- The keep-awake Windows variant (PR 11) and its PSScriptAnalyzer gate.
- `-CommandWithArgs` for pwsh 7.4+.
