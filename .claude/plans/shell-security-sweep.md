# Security PR sweep: close the shell-command findings (12 PRs)

## Context

The injection research (`.claude/research/reports/Shell script injection safety.md`) produced a security suite (`engine/crates/bnto/tests/shell_security/`, uncommitted on `test/shell-security-suite`). It has 12 passing guards and 14 open findings, each an `#[ignore = "OPEN SEC-xx"]` test that was checked to fail today. Exploration found three more:

- **SEC-17:** second-order template injection. `resolve_string` (`bnto-core/src/executor/template.rs:209-247`) fills `{{fields}}` and then rescans the result for env, ctx, item and node placeholders. A field or CSV value of `{{env.AWS_SECRET_ACCESS_KEY}}` gets the secret filled in.
- **SEC-18:** a hang. The `while find("{{env.")` / `{{item.` / `{{node.` loops (`:251-311`) never end when a value contains its own placeholder.
- **SEC-19:** the TUI has no consent gate. `tui/bridge.rs:101` goes straight to `run_pipeline`, so library recipes with shell steps run unprompted. The badge is hard-coded (`render_detail.rs:66`).

Goal: one single-concern PR per finding. Each one un-ignores its tests, so the suite tracks progress to "0 ignored". Keep-awake is **paused**; its work stays in the `keep-awake PR1 WIP` stash.

User decisions:

- SEC-11 refuses outright, with no warn-first release.
- Script mode is included, as the final PR.
- SEC-08 is included.
- Work one PR at a time and check in before each push or next PR.

## Workflow for every PR

1. Branch from `main`, named as listed below.
2. Remove the `#[ignore]` from that PR's suite tests and add unit tests in the crate (RED).
3. Implement until GREEN.
4. Run `task wasm:lint`, `task wasm:test` and `task cli:test`. Also run `task ui:build` and `task ui:test` when codegen changed.
5. Flip the finding to Guard in `.claude/scopes/security/shell-command.md` and tick it in PLAN.md.
6. Commit, without attribution lines, per the global CLAUDE.md.
7. Check in with the user before pushing.

## PRs, in order

**PR 0, `test/shell-security-suite`: Add the shell-command security suite and findings index.**

- Commit the suite already written, `.claude/scopes/security/shell-command.md`, and `.claude/research/` (report and notes).
- Add ignored RED specs for SEC-17 and SEC-18. For SEC-18, give `support.rs` a run-with-deadline helper that kills `bnto` after 10 s, so a regression can't hang CI.
- Add a "Security sweep" section to `.claude/PLAN.md` listing PRs 1–11.

**PR 1, `test/shell-each-macos-path`: Expect the resolved work dir in `shell_each` on macOS.**

- Test-only change. On macOS `getcwd` returns `/private/var/...`.
- At `tests/shell_each.rs:76-78`, build the expected root with `if cfg!(unix) { canonicalize } else { as-is }`. On Windows, canonicalizing would yield `\\?\`.

**PR 2, `fix/tui-consent-gate` (SEC-19): Ask for consent in the TUI before running an untrusted shell recipe.**

- Extract the CLI's trust decision from `bnto/src/consent.rs:16` into a pure `trust_decision(raw_json, catalog) -> {Trusted | NeedsConsent{commands}}`, which `check_trust` keeps using.
- The TUI runs it before `bridge::spawn_pipeline` (`tui/mod.rs:216`), shows a confirm modal listing the commands, and on "allow" saves the hash to the same `TrustStore` (`trust.rs`).
- Fix the hard-coded badge so it uses the recipe's real `has_shell`.
- RED: unit tests for `trust_decision` and for the TUI state transition (run → consent modal → declined means no run, approved means run and the hash is stored).

**PR 3, `fix/single-pass-templates` (SEC-17, SEC-18): Fill templates in one pass so substituted values are never read as templates.**

- Replace `resolve_string`'s chain of per-namespace passes with one left-to-right scan. It finds each `{{ns.key}}`, resolves it through the existing `resolve_placeholder` (`template.rs:90`), appends the value, and moves past it.
- Sole-placeholder type preservation stays. Unresolved placeholders keep today's per-namespace behaviour: `fields` left as written; env, ctx, node and item become empty; item stays as written when there is no item map.
- Apply the same rule to bnto-shell's `{{output_dir}}`. `file_mode.rs:97` should substitute only in the recipe's arg templates, not in values that came from substitution.
- RED: unit tests in `template.rs` (a value containing `{{env.X}}` stays literal; a self-referencing value terminates; the existing namespace tests keep passing). Un-ignore the SEC-17 and SEC-18 suite tests.

**PR 4, `fix/shell-raw-param-checks` (SEC-07): Check shell-command params before templates are filled, and require a literal command.**

- Add a defaulted `fn validate_raw(&self, raw: &Params) -> Vec<String>` to `NodeProcessor` (`bnto-core/src/processor.rs`), following the `count_keys` / `resolves_items` pattern. This adds no metadata, so no codegen.
- Call it in two places: in a pre-flight walk in `executor/mod.rs:157` `execute_pipeline` (all nodes, children included, before anything runs), and in `check/mod.rs:99` `processor_findings` on `node.params` before `resolve_fields`.
- Add a public `contains_placeholder(&str)` in `executor/template.rs` that recognises the executor namespaces with identifier keys (`[A-Za-z0-9_.-]+`). Literal text such as `{{fields.*}}`, as in Ryan's download-patreon, is then not flagged.
- Shell-command's `validate_raw` refuses placeholders in `command`. `{{vars.*}}` is unaffected, because the CLI applies it textually before the executor runs.

**PR 5, `fix/interpreter-code-templates` (SEC-11): Refuse templates inside interpreter code args.**

- In shell-command's `validate_raw`: when `command` (lower-cased, with a trailing `.exe` stripped) is a known interpreter and an arg is its code flag, refuse a placeholder in the _following_ arg.
- Interpreter → code flag:
  - `python`, `python3` and `python3.N` → `-c`
  - `node` → `-e` / `--eval` / `-p` / `--print`
  - `deno` → `eval`
  - `perl` → `-e` / `-E`
  - `ruby` → `-e`
  - `php` → `-r`
  - `osascript` → `-e`
  - `Rscript` → `-e`
- Error text names the safe pattern: "pass the value as a trailing argument and read it from sys.argv / process.argv / ARGV".
- Lives in a new `bnto-shell/src/interpreter_code.rs`.

**PR 6, `fix/denylist-windows-spellings` (SEC-05): Make the shell denylist catch Windows spellings.**

- `validate.rs:62` normalises before comparing: lower-case, trim trailing dots and spaces (Windows ignores them), then strip a `.exe`/`.com` suffix. So `bash.exe`, `PowerShell.EXE` and `cmd.` are refused.

**PR 7, `fix/denylist-exec-wrappers` (SEC-08): Refuse exec wrappers that would launch a shell.**

- Add a wrapper list: `env`, `xargs`, `nohup`, `nice`, `timeout`, `stdbuf`, `setsid`, `sudo`, `doas`, `chroot`, `script`, `watch`, `busybox`, `find` with `-exec`/`-execdir` targeting a shell, and `runas`/`start` on Windows.
- Refuse a wrapper whose argv would launch a denied shell. A wrapper running a non-shell tool (`timeout 5 ffmpeg`) stays allowed.
- Shares PR 6's normaliser.

**PR 8, `fix/recipe-env-startup-vars` (SEC-10): Strip shell and interpreter startup variables from a recipe's env.**

- Extend `DENIED_ENV_VARS` / `sanitize_env` (`validate.rs:30,101`). Match case-insensitively.
  - Exact names: `BASH_ENV`, `ENV`, `SHELLOPTS`, `BASHOPTS`, `PS1`–`PS4`, `PROMPT_COMMAND`, `IFS`, `CDPATH`, `GLOBIGNORE`, `ZDOTDIR`, `FPATH`, `NODE_OPTIONS`, `PERL5OPT`, `PERL5LIB`, `PYTHONSTARTUP`, `PYTHONPATH`, `RUBYOPT`.
  - Prefix: `BASH_FUNC_`.
- Stripped silently, consistent with today's `sanitize_env`.

**PR 9, `fix/field-value-validation` (SEC-13): Check field values against their definitions before a run.**

- Add `FieldDef::check_value` (`bnto-core/src/field_def.rs` after `:144`):
  - enum value must be in `options` (an empty `options` means no allowlist, so the existing tests at `input.rs:705,739` and `resolve.rs:223,243` keep passing)
  - numbers must parse and respect `min`/`max`
  - booleans must parse
- Called from:
  - a pre-flight in `executor/mod.rs:157`, covering CLI, TUI, dry-run with inputs, WASM, and values set via vars
  - `input.rs:189` `apply_param_overrides`, for a friendly error listing the allowed values
  - `dry_run/shell.rs:171`, which must return `Result`
- This also stops a bad number or boolean from silently becoming a literal `{{fields.x}}` in args.

**PR 10, `feat/consent-full-commands` (SEC-16): Show full command lines in the consent prompt.**

- Add `trust::collect_shell_command_lines`, a sibling of `collect_shell_command_names`, so the existing unit tests stay valid.
- Each entry is the raw template line, `command arg arg …`: unquoted, conditional groups shown as written, `search_path` noted. Script steps show their shell and script.
- Used by the CLI `prompt_consent` and the PR 2 TUI modal.

**PR 11, `feat/shell-script-mode` (SEC-20–26): Run a literal script through a hardened shell.**

- New shell-command params: `shell` (enum `sh|bash|zsh|dash|ksh`, default `sh`) and `script` (literal).
- Runs as `<abs shell> [-p for bash | -f for zsh] -c <script> bnto <args…>`.
  - The shell path is fixed per OS: `/bin/sh`, then `/bin/bash`, `/usr/bin/zsh`, `/bin/zsh` and so on. It never comes from PATH or `search_path`.
- `validate_raw` (from PR 4) refuses any placeholder in `script` or `shell`.
- Add an executor exemption so those two params are never resolved, using a defaulted `NodeProcessor::literal_params()` honoured at `primitive.rs` resolution.
- Add `CommandOptions.clear_env: bool` (`bnto-core/src/command_options.rs:10`), honoured at `bnto/src/context.rs:260`.
  - With it set, the child starts from an empty env plus an allowlist (`PATH`, `HOME`, `USER`, `LOGNAME`, `LANG`, `LC_*`, `TERM`, `TMPDIR`, and on Windows `SystemRoot`, `PATHEXT`, `COMSPEC`), then the sanitised recipe env.
- `command` and `script` are mutually exclusive.
- Docs: add the safe-script contract to node docs (`"$1"`, no `eval`, no nested `-c`/`-e`, `--` for getopt tools).
- Codegen: run `task nodes:codegen`, which updates the snapshot, `@bnto/nodes` generated files and node docs.
- PowerShell / Windows script mode stays out of scope; it ships with keep-awake on Windows.

## Dependency chain

```
PR 0 suite ── everything un-ignores tests from it
PR 1, 2, 3, 6, 8, 9  independent (any order after PR 0)
PR 4 ──> PR 5 (uses validate_raw)
PR 6 ──> PR 7 (shares normaliser)
PR 2 ──> PR 10 (TUI modal shows the lines)
PR 4, 8, 10 ──> PR 11
```

Suggested order (risk first): 0, 2, 3, 5 (needs 4), 1, 6, 7, 8, 9, 10, 11.

## Verification

- Per PR: `cargo test -p bnto --test shell_security` shows that PR's tests passing, with the ignored count dropping.
- Plus `task wasm:lint`, `task wasm:test`, `task cli:test`. Add `task ui:build` and `task ui:test` for PR 11's codegen.
- After the last PR: `cargo test -p bnto --test shell_security -- --ignored` reports 0 ignored. Windows CI runs SEC-05, SEC-30 and the Windows env allowlist.
- Re-run the mutation check: weaken each fix temporarily and confirm its suite test fails.
- Manual: `bnto tui` on a library shell recipe shows the consent modal (PR 2). `bnto run` on Ryan's `download-patreon` still works after PRs 4, 5 and 9.

## Count changes

- No processor, node-type or recipe count changes.
- PR 11 adds 2 shell-command parameters, so the codegen artifacts change.
