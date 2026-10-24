# Known Issues

**The pickup list for defects:** bugs, standards violations, stale docs and security gaps found while working. Each was verified when added, with the evidence to find it again. Features and ideas stay in [PLAN.md's backlog](PLAN.md#backlog).

## How this list works

- **Add an issue when you find one,** even mid-task: verify it first (a file and line, a command, a log), then add a row and a section. Don't fix it in an unrelated PR (single-concern rule).
- **Pick one up** by setting its status to `In progress (<branch>)`. A larger fix gets its own plan.
- **The PR that fixes an issue deletes it** from this file. Git history keeps the record.
- **IDs never get reused.** The next one is **ISS-25**.
- **Status values:** `Open`, `Scheduled` (link to the plan that owns it), `In progress (<branch>)`.

| ID     | Issue                                                                     | Area        | Status                                           |
| ------ | ------------------------------------------------------------------------- | ----------- | ------------------------------------------------ |
| ISS-7  | The CLI's telemetry notice links `bnto.io/telemetry`, a 404               | web docs    | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-8  | Most CLI integration tests run `bnto` against the real `~/.bnto`          | cli test    | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-9  | Turbo's lint cache ignores the root `eslint.config.base.mjs`              | tooling     | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-10 | `task check` skips the crate publish checks CI runs                       | tooling     | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-12 | Released CLIs send no telemetry: the PostHog key is never set             | cli         | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-16 | `@bnto/nodes` lint skips `scripts/`; codegen script is 1,381 lines        | tooling     | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-17 | The `/workflow-expert` skill's node inventory is stale                    | docs        | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-18 | A Windows process test times out waiting 20 s for PowerShell              | engine test | Scheduled ([plan](strategy/open-issues-plan.md)) |
| ISS-19 | `/pickup` says hooks may be 30 lines; the limit is 20                     | docs        | Open                                             |
| ISS-20 | 12 broken links in `.claude/` docs; nothing checks links                  | docs        | Open                                             |
| ISS-22 | The output node's `directory` leaves `{{env.*}}` as a literal folder name | cli         | Open                                             |

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

CI's Crate Publish List job runs `scripts/check-crate-publish-list.sh` and `scripts/check-own-version-crates.sh` (`.github/workflows/ci.yml:316-323`), but no task does, so `task check` can't catch them. #740 added `#[expect]` lines to tonkotsu without bumping its own version: `task check` exited 0 and CI failed (`tonkotsu-0.1.2/src/field.rs ... differ`). Add both scripts to `task check` (the own-version one needs network, to download the published crate), or say in pre-commit.md that a change to a crate with its own version needs that bump.

### ISS-12: Released CLIs send no telemetry: the PostHog key is never set

**Found:** October 24, 2026 (planning ISS-7).

`engine/crates/bnto/src/telemetry/mod.rs:57` reads the key from the `BNTO_POSTHOG_KEY` env var at run time, and telemetry is inactive without one (`:60`). Nothing sets it: there's no `option_env!`, no build script, and nothing in `release.yml` (`grep -rn BNTO_POSTHOG_KEY` finds only `mod.rs` and a test). So shipped CLIs collect nothing, while the first-run notice says bnto "collects anonymous usage data" and the code comment calls the key "safe to embed". `bnto telemetry status` also reports "Effective: true" without checking for a key (`mod.rs:134`). Needs Ryan's decision: ship the key, or say telemetry is off.

### ISS-16: `@bnto/nodes` lint skips `scripts/`; codegen script is 1,381 lines

**Found:** October 21, 2026 (dogfood sessions); checked October 24, 2026.

`packages/@bnto/nodes/eslint.config.mjs:6` ignores `scripts/**`, so `scripts/generate-from-catalog.ts` reached 1,381 lines against the 250-line cap, and its functions against the 20-line one, with nothing flagging it. Codegen stays in TypeScript (decided October 24, 2026).

### ISS-17: The `/workflow-expert` skill's node inventory is stale

**Found:** October 21, 2026 (dogfood sessions); checked October 24, 2026.

`.claude/skills/workflow-expert/SKILL.md:30` still says recipes use "the 6 implemented browser operations", and its operations table predates most of today's nodes. The skill reviews every new node and param, so a stale inventory steers it wrong.

### ISS-18: A Windows process test times out waiting 20 s for PowerShell

**Found:** October 24, 2026 (v0.18.2 release run 40096930376).

`terminate_all_kills_orphaned_grandchild_on_windows` (`engine/crates/bnto/src/context.rs:743`) starts `cmd /c start /b powershell … $PID; Start-Sleep 30` and waits once, `recv_timeout(20 s)`, for PowerShell to print its PID (`:758-759`). On the release's Windows runner PowerShell didn't print in time: `called Result::unwrap() on an Err value: Timeout`. The same commit passed the same job on #743's CI an hour earlier, so the failure is timing, not code. It failed the release's CI Gate. Wait on PowerShell's start with a longer bound, or start a lighter process for the grandchild, so a slow runner can't fail a release.

### ISS-19: `/pickup` says hooks may be 30 lines; the limit is 20

**Found:** October 24, 2026 (cold review of #745).

`.claude/skills/pickup/SKILL.md:310-311` tells agents that hooks are "thin reactive wrappers (< 30 lines)" and that a hook is too big at ">30 lines". [code-standards.md](rules/code-standards.md) caps every function at 20 lines (`.tsx` at 60), and ESLint enforces it (#739). `/code-review` had the same stale "hooks get 30" until #745.

### ISS-20: 12 broken links in `.claude/` docs; nothing checks links

**Found:** October 24, 2026 (cold review of #745).

Relative links that don't resolve from the file they're in:

- `.claude/PLAN.md:168, 172, 439, 850, 878`: `.claude/strategy/…` paths written root-relative, the missing `file-node-ecosystem.md`, and `research/reports/Shell%20script%20injection%20safety.md`.
- four in `.claude/plans/inherited-watching-hennessy.md`.
- one each in `.claude/skills/pickup/SKILL.md:202`, `.claude/skills/groom/SKILL.md` and `.claude/strategy/engine-owned-schema.md`.

`task docs:validate` (`scripts/validate-docs.ts`) checks crate READMEs and node docs only, never links. `CLAUDE.md`'s 115 root-relative links (`.claude/rules/…`) work for agents, which read from the repo root, but not on GitHub; keep that convention or not, but say so. Fix the 12 and add a link check to `docs:validate` (a Python sketch that lists them: walk `.claude/**/*.md`, resolve each Markdown link target against the file's folder).

### ISS-22: The output node's `directory` leaves `{{env.*}}` as a literal folder name

**Found:** October 24, 2026 (evaluating [a feature request](requests/2026-10-24-folder-index.md)).

A recipe with `secrets: [{"key": "SOME_KEY"}]` and an output node `directory: "{{env.SOME_KEY}}"`, run with `SOME_KEY` set:

- `bnto check` passes.
- `bnto run` exits 0 and prints `1 file written to {{env.SOME_KEY}}/`.
- The file lands in `./{{env.SOME_KEY}}/note.txt`; the folder `SOME_KEY` names stays empty.

`resolve_effective_output_dir` (`engine/crates/bnto/src/run.rs`) and the TUI's copy (`tui/bridge.rs:252-254`) fill only `{{ctx.*}}` and `{{node.*}}`, and an unknown `{{ctx.*}}` key becomes `""`. So `out-{{ctx.nope}}-{{item.x}}` writes to `out--{{item.x}}/`. With `SOME_KEY` unset, the run is refused, but the folder is created first (`progressive_dir`, `run.rs`). Fill every namespace a run has (`env` from declared secrets, as `file-copy`'s `destination` does), refuse any placeholder left over before consent and before any folder is made (as `{{fields.*}}` are refused since #741 and #742), and have `bnto check` flag the ones no run can fill (`{{item.*}}`, a processing step's `{{node.*}}`). ISS-21, the TUI wipe this could have pointed at a real folder, is fixed (#750).
