# Decision: Keeping `engine/target` small

**Created:** November 2, 2026
**Status:** Done (ISS-120): lighter debug info (#833), one test binary for most of the CLI's integration tests (#834), automatic pruning and a size guard (`chore/prune-stale-target`).
**Applies to:** `engine/target` on every contributor's machine, `task check`, CI's Windows engine job.

---

## The problem

On Ryan's Mac `engine/target` reached 57 GB, then 63 GB. Cargo never deletes what it built before: each change of settings (another `cargo test -p` selection, clippy versus test, a profile, a toolchain) writes new copies of the units it touches, and the old ones stay. `debug/deps` held 747,000 object files and 594 test executables, though one build makes about 90. Ryan (November 1, 2026): the repo's workflow should keep this small on any machine, for any contributor, without anyone cleaning by hand.

## What each PR does

1. **Less debug info (#833).** `debug = "line-tables-only"` for the workspace's crates in dev and test builds, none for dependencies. A fresh workspace test build went from 3.0 GB to 2.3 GB.
2. **Fewer test binaries (#834).** The `bnto` crate's 53 integration-test files build as one `cli` binary. `cargo test -p bnto --no-run`: 58 executables and 1362 MB down to 6 and 1143 MB.
3. **Pruning and a guard (this decision).** The first two make each build smaller. Neither stops old builds piling up, which is most of the 63 GB.

## Pruning

`task check` starts with `task clean:stale` (`scripts/target/pruneTarget.ts`). It deletes every build unit and incremental cache in `engine/target` that no build used for 7 days. Every profile counts, cross-compiled ones (`wasm32-unknown-unknown/debug`) too.

- **A unit goes whole.** Cargo names a unit's outputs, objects, fingerprint and build-script folder with one 16-hex-digit hash (`libx-<hash>.rlib`, `x-<hash>.….rcgu.o`, `.fingerprint/x-<hash>`, `build/x-<hash>`). The prune groups entries by that hash and deletes a group only when none of its files was used within the window. So cargo never finds a fingerprint whose outputs are gone: a pruned unit that's needed again is rebuilt from scratch. Each `incremental/<crate>-<id>` cache is its own group. Uplifted binaries (`debug/bnto`), cdylibs without a hash and `.cargo-lock` are never touched.
- **"Used" is a file's newer of atime and mtime.** Cargo reads each unit's fingerprint on every build, so on macOS and Linux (relatime) a unit in use is never stale. Folders don't count: listing one, as the prune does, updates its atime. Where the filesystem doesn't keep atime (Windows turns it off on most volumes), only mtime counts. A dependency nothing rebuilt for a week is then pruned and rebuilt once a week. That's accepted: a fresh build of every test takes about 1.5 minutes on a Mac with 4 jobs.
- **Why 7 days.** It's the window the issue agreed on. On this machine on November 2 it prunes 22.9 GB of 63 GB. 3 days would prune 45.5 GB and 1 day 57.2 GB, but then a long weekend away means rebuilding everything, and a release or WASM build used weekly would rebuild every time. The rest of today's 63 GB is builds from the last week, which ages out.
- **A build running alongside.** A cargo build started elsewhere while the prune runs could pick up a unit the prune is deleting, and fail on the missing file. That takes a unit unused for a week being needed in the same seconds; rerunning the build fixes it. The prune doesn't take cargo's build lock: Node has no portable file lock, and the race is narrow.
- **A file it can't delete** (Windows: a running test binary) is reported and skipped, never fatal.

## The size guard

`task check:test-build-size` (`scripts/target/checkTestBuildSize.ts`) fails when one build of the engine's tests (`cargo test --workspace`) outgrows its OS's budget in `scripts/target/testBuildBudget.ts`.

- **What it measures.** It runs `cargo test --workspace --no-run --message-format=json`. Right after the tests that's a no-op, and cargo still lists every unit of the build. It then adds up what those units hold in the target folder: outputs, objects, build-script folders. Units from other builds don't count, so the guard measures one build, not the folder, with no scratch rebuild. Incremental caches are left out: they're a cache, and the prune bounds them.
- **What it catches.** Full debug info coming back, test binaries multiplying (each links the whole engine), a heavy new dependency.
- **Budgets** are the size measured when set, plus about 15%, one per OS: macOS writes its debug info into object files it keeps, and Windows writes `.pdb` files. Raising one is a commit that says what grew.
- **Where it runs.** In `task check`, after the Rust tests, and in CI's Windows engine job, after `cargo test --workspace`. Both had just built what it measures. CI's Linux job splits its tests (`--workspace --exclude bnto`, then `--package bnto`). Their features differ from `--workspace`'s, so measuring there would rebuild about 50 crates. The per-OS CI workflows (PLAN.md) add the guard to macOS, which will run `cargo test --workspace`. CI runs the script with Node 24's own TypeScript support, so the job needs no `pnpm install`. That's why `scripts/target/` imports name their `.ts` files.

## Options considered

- **`cargo-sweep`** (`cargo sweep --time 7`): it does the age-based prune, but every contributor and CI would need it installed. It also deletes file by file, so a unit can lose its outputs and keep its fingerprint.
- **Cargo's own garbage collection** (`-Zgc`): nightly only, and it cleans `~/.cargo`'s caches, not target folders.
- **`cargo clean` on a schedule, or past a size cap:** this deletes what's in use too, so the next build is a full rebuild. A cap would also punish legitimately working across profiles.
- **A guard that does a fresh build** into a scratch folder: exact, but minutes per `task check`.
- **A shell script**, like `check-test-tmp.sh`: parsing cargo's JSON and grouping by unit is fragile in `sh`. Node is already required for `task check`.
- **A dogfood recipe:** no node filters files by age or groups them by unit, and adding nodes for repo upkeep isn't worth it.
