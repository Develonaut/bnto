# Known Issues

**The pickup list for defects:** bugs, standards violations, stale docs and security gaps found while working. Each was verified when added, with the evidence to find it again. Features and ideas stay in [PLAN.md's backlog](PLAN.md#backlog).

## How this list works

- **Add an issue when you find one,** even mid-task: verify it first (a file and line, a command, a log), then add a row and a section. Don't fix it in an unrelated PR (single-concern rule).
- **Pick one up** by setting its status to `In progress (<branch>)`. A larger fix gets its own plan.
- **The PR that fixes an issue deletes it** from this file. Git history keeps the record.
- **IDs never get reused.** The next one is **ISS-12**.
- **Status values:** `Open`, `Scheduled` (link to the plan that owns it), `In progress (<branch>)`.

| ID     | Issue                                                                  | Area     | Status |
| ------ | ---------------------------------------------------------------------- | -------- | ------ |
| ISS-7  | The CLI's telemetry notice links `bnto.io/telemetry`, a 404            | web docs | Open   |
| ISS-8  | Most CLI integration tests run `bnto` against the real `~/.bnto`       | cli test | Open   |
| ISS-9  | Turbo's lint cache ignores the root `eslint.config.base.mjs`           | tooling  | Open   |
| ISS-10 | `task check` skips the crate publish checks CI runs                    | tooling  | Open   |
| ISS-11 | The consent prompt suggests `bnto dry-run <path>`, which refuses paths | cli      | Open   |

---

### ISS-7: The CLI's telemetry notice links `bnto.io/telemetry`, a 404

**Found:** October 23, 2026 (fixing ISS-2).

The first-run notice (`engine/crates/bnto/src/telemetry/consent.rs:27`) says bnto "collects anonymous usage data" and sends people to `https://bnto.io/telemetry` for more. That URL renders the `[bnto]` route's "Page Not Found" (`curl -sL https://www.bnto.io/telemetry | grep -o not-found-heading`), and no page says what the CLI collects: `privacy/privacy.mdx` lists PostHog without mentioning the CLI, and the CLI recipe FAQ (`apps/web/lib/cliRecipeFaq.ts:9`) has one line. Add a `/telemetry` page listing the four events (`cli_command`, `cli_recipe_run`, `cli_error`, `cli_tui_session`), their properties (bundled slugs only, else `"custom"`; bucketed counts, sizes and durations), the anonymous id and the three opt-outs, or point the notice at a page that has them.

### ISS-8: Most CLI integration tests run `bnto` against the real `~/.bnto`

**Found:** October 23, 2026 (fixing ISS-3).

`cli_commands.rs` and the run helpers in `tests/helpers/mod.rs` now use `helpers::Sandbox` (temp HOME, BNTO_HOME, config dir and working directory, telemetry off). About 17 other files in `engine/crates/bnto/tests/` still spawn `Command::new(bnto_bin())` with no HOME or BNTO_HOME: `assert_files`, `check_edges`, `check_folders`, `check_run_time`, `edges`, `edit_fields`, `file_delete`, `file_inspect`, `file_join`, `progressive_output`, `report_file_names`, `run_args`, `run_json`, `run_temp_files`, `shell_each`, `shell_options`, and some runs in `dry_run_temp_files` and `run_timeout`. Each run writes a session log into the developer's `~/.bnto/state/logs`: `cargo test -p bnto --test edges --test run_json` added 3. A run that resolves a slug would also read their library. Switch them to `Sandbox::new().command()`.

### ISS-9: Turbo's lint cache ignores the root `eslint.config.base.mjs`

**Found:** October 23, 2026 (ISS-5, PR #739).

Every package's `eslint.config.mjs` imports `../../eslint.config.base.mjs` (or `../../../`), but `turbo.json` has no `globalDependencies` and the `lint` task declares no inputs, so the shared config isn't part of any package's lint hash. After #739 lowered `max-lines-per-function` in the base config, `task check` replayed cached lint results for `@bnto/auth` and `@bnto/i18n` (`cache hit, replaying logs`), whose own files hadn't changed. A rule change that breaks an unchanged package passes `task check` and the pre-commit hook locally and fails only in CI, which has no Turbo cache. Add the base config (and any other shared root config the tasks read) to `globalDependencies`.

### ISS-10: `task check` skips the crate publish checks CI runs

**Found:** October 23, 2026 (ISS-5, PR #740).

CI's Crate Publish List job runs `scripts/check-crate-publish-list.sh` and `scripts/check-own-version-crates.sh` (`.github/workflows/ci.yml:382-389`), but no task does, so `task check` can't catch them. #740 added `#[expect]` lines to tonkotsu without bumping its own version: `task check` exited 0 and CI failed (`tonkotsu-0.1.2/src/field.rs ... differ`). Add both scripts to `task check` (the own-version one needs network, to download the published crate), or say in pre-commit.md that a change to a crate with its own version needs that bump.

### ISS-11: The consent prompt suggests `bnto dry-run <path>`, which refuses paths

**Found:** October 23, 2026 (ISS-1 part 2).

When `bnto run` asks for consent, it suggests running `bnto dry-run {recipe_arg}` to inspect the commands (`engine/crates/bnto/src/consent.rs:42`), and declining says so again. For a recipe run by path, that command fails: `show_dry_run` resolves only catalog slugs (`engine/crates/bnto/src/inspect.rs`, `catalog.resolve(slug)`), so `bnto dry-run recipes/release-verify.bnto.json` prints `Error: Unknown recipe: …` and exits 1. Either let `bnto dry-run` read a recipe file as `bnto run` does (`run.rs` `read_recipe`), or suggest it only for slugs.
