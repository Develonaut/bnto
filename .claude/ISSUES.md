# Known Issues

**The pickup list for defects:** bugs, standards violations, stale docs and security gaps found while working. Each was verified when added, with the evidence to find it again. Features and ideas stay in [PLAN.md's backlog](PLAN.md#backlog).

## How this list works

- **Add an issue when you find one,** even mid-task: verify it first (a file and line, a command, a log), then add a row and a section. Don't fix it in an unrelated PR (single-concern rule).
- **Pick one up** by setting its status to `In progress (<branch>)`. A larger fix gets its own plan.
- **The PR that fixes an issue deletes it** from this file. Git history keeps the record.
- **IDs never get reused.** The next one is **ISS-8**.
- **Status values:** `Open`, `Scheduled` (link to the plan that owns it), `In progress (<branch>)`.

| ID    | Issue                                                            | Area     | Status                                              |
| ----- | ---------------------------------------------------------------- | -------- | --------------------------------------------------- |
| ISS-1 | An unresolved `{{fields.*}}` template reaches its step as text   | engine   | Open                                                |
| ISS-3 | `test_list_command` reads the developer's real `~/.bnto`         | cli test | Scheduled ([dogfooding 6b](strategy/dogfooding.md)) |
| ISS-4 | `run_loop` in `tui/mod.rs` is 277 lines (limit: 20 per function) | tui      | Open                                                |
| ISS-5 | Nothing enforces the 20-line function limit                      | tooling  | Open                                                |
| ISS-6 | `pre-commit.md` says Lighthouse CI runs on every PR; it's manual | docs     | Open                                                |
| ISS-7 | The CLI's telemetry notice links `bnto.io/telemetry`, a 404      | web docs | Open                                                |

---

### ISS-1: An unresolved `{{fields.*}}` template reaches its step as text

**Found:** before October 2026 (moved from PLAN.md's backlog); hit again October 23, 2026.

Fields resolve per node (`collect_field_values(&node.fields, …)`). A template that names a field its node can't see, or a field left unset with no default, reaches the step as the literal placeholder. The step then runs with it and can no-op with exit 0: the download-patreon library recipe's flatten and upload steps did. `release-verify` has to treat any `{` in its `version` argument as "unset" for the same reason (`recipes/release-verify.bnto.json`). `bnto dry-run` already refuses unresolved `{{fields.*}}`, but a live run doesn't.

**Fix options:** resolve fields recipe-wide (a group-level merge), or have `bnto run` stop before the first step and name the missing field (`-p version=...`).

`engine/crates/bnto-core/src/executor/primitive.rs`, `engine/crates/bnto-core/src/executor/resolve.rs`, `engine/crates/bnto/src/dry_run/mod.rs`

### ISS-3: `test_list_command` reads the developer's real `~/.bnto`

**Found:** October 23, 2026 (dogfooding evaluation).

It runs `bnto list` without setting `HOME`, `BNTO_HOME` or a working directory, so it lists the developer's own library and can pass or fail depending on what's in it. Give it a temp `HOME`, `BNTO_HOME` and working directory, like the other CLI tests.

`engine/crates/bnto/tests/cli_commands.rs:40`

### ISS-4: `run_loop` in `tui/mod.rs` is 277 lines

**Found:** October 23, 2026 (dogfooding PR 3b).

`run_loop` (`engine/crates/bnto/src/tui/mod.rs:99-375`) is the TUI's event loop, against code-standards.md's 20-line function limit. Moving the terminal setup out (#729) brought the file under 400 lines, but the function itself is unchanged. Split it into per-event handlers. `task tui:test` (snapshots and journeys) covers it.

### ISS-5: Nothing enforces the 20-line function limit

**Found:** October 23, 2026 (dogfooding PR 4).

code-standards.md says functions stay under 20 lines, "no escape hatch". `rust-file-size` (#731) checks Rust file length and ESLint `max-lines` checks TypeScript file length, but nothing checks function length in either language. ISS-4 is the worst known case. Measure first (for example, clippy's `too_many_lines` with `too-many-lines-threshold = 20` in `clippy.toml`, and ESLint's `max-lines-per-function`), then decide: enforce with an allowlist that only shrinks, or change the rule.

### ISS-6: `pre-commit.md` says Lighthouse CI runs on every PR

**Found:** October 23, 2026 (CI review).

`.claude/rules/pre-commit.md:30` says `lighthouse.yml` runs on every PR and blocks merge. It's `workflow_dispatch` only, and Lighthouse runs inside `release.yml`; its last PR run was March 18, 2026. Correct the doc: on a PR, `task seo:audit` runs locally, and the release gate runs Lighthouse.

### ISS-7: The CLI's telemetry notice links `bnto.io/telemetry`, a 404

**Found:** October 23, 2026 (fixing ISS-2).

The first-run notice (`engine/crates/bnto/src/telemetry/consent.rs:27`) says bnto "collects anonymous usage data" and sends people to `https://bnto.io/telemetry` for more. That URL renders the `[bnto]` route's "Page Not Found" (`curl -sL https://www.bnto.io/telemetry | grep -o not-found-heading`), and no page says what the CLI collects: `privacy/privacy.mdx` lists PostHog without mentioning the CLI, and the CLI recipe FAQ (`apps/web/lib/cliRecipeFaq.ts:9`) has one line. Add a `/telemetry` page listing the four events (`cli_command`, `cli_recipe_run`, `cli_error`, `cli_tui_session`), their properties (bundled slugs only, else `"custom"`; bucketed counts, sizes and durations), the anonymous id and the three opt-outs, or point the notice at a page that has them.
