# Decision: Enforcing the 20-line function limit

**Created:** October 23, 2026
**Status:** Decided by Ryan (ISS-5). Enforced for TypeScript in #739 (`feat/ts-function-length`) and for Rust in #740 (`feat/rust-function-length`).
**Applies to:** every TypeScript package's ESLint config and every engine crate.

---

## The problem

[code-standards.md](../rules/code-standards.md) caps functions at 20 lines, but tooling didn't hold code to it:

- **TypeScript:** ESLint's `max-lines-per-function` was set to 30 for `.ts` and 60 for `.tsx`, not 20.
- **Rust:** nothing checked function length. `rust-file-size` (#731) checks only file length.

The worst case, the TUI's 277-line `run_loop`, was split in #738 (ISS-4). Ryan's call: "our tooling should be catching that and not letting work go through."

## What measuring found

All counts skip blank lines and comments.

| Code                                                                  | Over 20 lines | Notes                                                                                                                                                                                                                        |
| --------------------------------------------------------------------- | ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Rust, production (`cargo clippy --workspace`, `too_many_lines` at 20) | 220           | 132 are 21-30 lines, 27 are over 60, and the longest is 193 (`tui/screens/picker_update.rs`). Of the 220, 81 are TUI render, keys and screens or tonkotsu, 15 are metadata or param tables, and 124 are engine and CLI logic |
| Rust, tests (`--all-targets` minus production)                        | 385           |                                                                                                                                                                                                                              |
| TypeScript `.ts`, tests excluded                                      | 98            | None over 30, the old cap                                                                                                                                                                                                    |
| TypeScript `.tsx`, tests excluded                                     | 187           | 44 over 30, none over 60                                                                                                                                                                                                     |

## Options considered

1. **Ratchet (chosen).** Enforce 20 now and list today's offenders in a baseline that can only shrink. New long functions fail at once, and old ones get paid down in later PRs.
2. **Fix everything first.** That's more than 300 functions, many PRs before anything blocks, and new debt can land meanwhile.
3. **Ratchet the cap.** Enforce today's worst per language and lower it over time. Simpler, but new functions up to the cap still pass.

## The decision

- **Tests are exempt.** This matches the existing ESLint config (test files are off) and `rust-file-size`. Long tests are mostly fixture setup.
- **TypeScript:**
  - `max-lines-per-function` is 20 for `.ts`.
  - `.tsx` keeps 60, because code-standards.md sizes a component at 50-100 lines of JSX and logic. Dropping it to 20 would add 187 baseline entries; that's a separate call. (Since ISS-62, code-standards.md sizes the file at 50-100 lines and the component function at most 60, with logic past ~20 lines in a hook; the 60 stands on its own.)
  - Each package's `eslint-suppressions.json` (ESLint 9.24+ bulk suppressions) lists the 98 existing offenders by file and count.
  - The root `scripts/` folder, which belongs to no package, got the same setup in ISS-65: a root `eslint.config.mjs` on `baseConfig` and a root `eslint-suppressions.json`, run by `task check:scripts` and CI's Scripts job. Its baseline also holds three kebab-case file names and two complexity findings.
  - ESLint fails when a file has more offenders than its count. It also fails when the file has fewer ("suppressions left that do not occur anymore"), so fixing a function means lowering its count, or deleting the entry when the count reaches zero.
- **Rust:**
  - `#[expect(clippy::too_many_lines)]` on each existing offender, with `too-many-lines-threshold = 20` in `engine/clippy.toml`.
  - A production-only run (`cargo clippy --workspace -- -W clippy::too_many_lines`) fails on any long function without the marker.
  - The marker fails once its function is under the limit (`unfulfilled_lint_expectations`), so the baseline only shrinks.
  - The normal `--all-targets` run leaves the lint off, so tests aren't checked.

## Trade-offs

- **Growth isn't caught.** A function already on the baseline can grow without failing. Only new offenders and fixed ones are caught.
- **ESLint counts per file.** A file with a count of 2 could swap a fixed function for a new long one. The count still can't rise.
- **Regenerating the baseline:** for TypeScript, run `pnpm exec eslint . --prune-suppressions` in the package once a function is fixed. Never use `--suppress-rule` to add one.
- **Regenerating the Rust baseline** is never needed: a fixed function's `#[expect]` fails `task wasm:lint` until it's deleted. The Rust baseline started at 220 functions in 141 files.
- **tonkotsu has its own crates.io version,** so its markers change its published files. #740 bumped it to 0.1.3 ([releases.md](../scopes/process/releases.md#one-number-everywhere)); splitting a tonkotsu function later needs a bump too.
