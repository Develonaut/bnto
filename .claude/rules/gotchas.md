# Common Gotchas

Known pitfalls and their fixes. Check here before debugging something that feels familiar.

## `convexQuery` + Empty IDs = Silent Subscription Failures

`@convex-dev/react-query`'s `convexQuery()` creates a real Convex WebSocket subscription unless args is the literal string `"skip"`. Passing `{ id: "" }` creates an active subscription that hits the Convex validator and fails with `ArgumentValidationError`. React Query's `enabled: false` does NOT prevent this — it only prevents React Query from surfacing the data, but the subscription still fires server-side.

**Symptoms:** 100% failure rate spikes on `executions:get` (or any query) in the Convex dashboard. Intermittent 500 errors from Next.js dev server under parallel E2E load. Cache hit rate drops to 0%.

**Fix:** Always use `"skip"` in adapter functions when the ID is falsy:

```typescript
export function getExecutionQuery(id: string) {
  return convexQuery(api.executions.get, id ? { id: id as Id<"executions"> } : "skip");
}
```

See [convex.md](../scopes/backend/convex.md#convexquery-skip-guard-critical) for the full rule.

## SSG + Convex Hooks

Pages using Convex hooks (`useMutation`, `useQuery`) crash during static generation because there's no `ConvexProvider` at build time.

**Fix:** Push `"use client"` to the smallest leaf component that needs Convex hooks. The page stays a server component and renders static content (headings, descriptions) on the server. Client leaves handle their own loading state.

```tsx
// page.tsx -- server component, renders heading on the server
import { MyForm } from "./_components/MyForm";
import { Heading } from "@/components/ui/Heading";

export default function Page() {
  return (
    <>
      <Heading level={1}>Page Title</Heading> {/* server-rendered */}
      <MyForm /> {/* client leaf */}
    </>
  );
}

// _components/MyForm.tsx -- client boundary at the leaf
("use client");
export function MyForm() {
  const { data, isLoading } = core.recipes.useRecipes();
  if (isLoading) return <Skeleton />;
  return <form>...</form>;
}
```

**Anti-pattern:** `"use client"` on a page + `dynamic(() => ..., { ssr: false })` for Convex components. This puts the SSR bypass at the trunk level — the page renders nothing on the server, and the dynamic import adds an extra loading waterfall. `ssr: false` belongs at the leaf, not the trunk.

**When `ssr: false` IS correct:** Leaf components that require browser-only APIs with no SSR fallback — ReactFlow (`@xyflow/react`), Canvas, WebGL. These genuinely cannot render on the server. Use `dynamic` + `ssr: false` in the parent component that imports the leaf, keeping the bypass as close to the browser-only code as possible.

## Convex Module Filenames -- No Hyphens

Convex doesn't allow hyphens in module filenames. Path components can only contain alphanumeric characters, underscores, or periods. A file like `enrich-with-data.ts` will fail on deploy with `InvalidConfig`.

**Fix:** Use underscores instead of hyphens for all files inside the `convex/` directory.

```
# BAD -- deploy will fail
convex/_helpers/enrich-with-data.ts

# GOOD
convex/_helpers/enrich_with_data.ts
```

## Convex Schema Migration (Production)

Convex validates all existing documents against the new schema on deploy. If you rename or change a field type, the deploy will fail.

**Fix:**

1. Temporarily relax the schema (make old and new fields optional)
2. Deploy: `npx convex deploy --yes`
3. Run a cleanup mutation: `npx convex run --prod <function>`
4. Restore the strict schema
5. Deploy again

## Tailwind v4 + Monorepo

Tailwind skips `node_modules` by default, so classes defined in shared workspace packages (`@bnto/ui`, `@bnto/editor`, `@bnto/form`) won't generate CSS unless the app points at their sources.

**Fix:** Add a `@source` directive in the app's `globals.css`:

```css
@source "../../../packages/ui/src";
```

Path is relative to the CSS file. `apps/web/app/globals.css` already has `@source` lines for `packages/ui`, `packages/editor` and `packages/@bnto/form`; add one when a new package ships Tailwind classes.

## pnpm 10 Native Dependencies

pnpm 10 requires explicit opt-in for native dependency builds.

**Fix:** Add to root `package.json`:

```json
{
  "pnpm": {
    "onlyBuiltDependencies": ["esbuild", "sharp", "unrs-resolver"]
  }
}
```

## Turbo Hashes Only a Package's Own Files

A Turbo task's cache key covers the files inside its package. A task that also reads a file outside it (the root `eslint.config.base.mjs`, `README.md`, `engine/Cargo.toml`) replays a stale cached result when only that file changes: `task check` passes, and CI, which has no Turbo cache, fails.

**Fix:** list the file in that task's `inputs`, beside `$TURBO_DEFAULT$` (which keeps the package's own files). A file every package reads goes in the root `turbo.json`:

```json
"lint": { "inputs": ["$TURBO_DEFAULT$", "$TURBO_ROOT$/eslint.config.base.mjs"] }
```

A file one package reads goes in that package's own `turbo.json`, which extends the root one and inherits the task's `dependsOn` and `outputs` (see `packages/@bnto/registry/turbo.json`). Its `inputs` **replace** the root task's `inputs` rather than adding to them, so repeat any root input the task needs:

```json
{
  "extends": ["//"],
  "tasks": { "test": { "inputs": ["$TURBO_DEFAULT$", "$TURBO_ROOT$/README.md"] } }
}
```

Don't use `globalDependencies`, which would invalidate every task's cache. `task check:turbo-inputs` (and CI) fail when a task's sources import an unlisted file outside its package, following imports through: `lint` from the package's ESLint config, `test` from all its sources (`@bnto/nodes`' tests import `engine/catalog.snapshot.json`), `build` from its non-test sources (`scripts/turbo/checkTaskInputs.ts`, ISS-51). Nothing checks runtime reads, such as a test's `readFileSync` of a root file (`@bnto/registry` reads `README.md` that way). Add those inputs by hand in the same PR.

**Inside a git hook in a worktree, Turbo drops root inputs.** Git sets `GIT_DIR` for hooks run in a worktree, and with it set Turbo leaves `$TURBO_ROOT$/…` files out of the hash, so the pre-commit lint replayed `main`'s cached pass over a staged rule that fails every file (ISS-39). Hook commands run Turbo through `sh scripts/hooks/turbo.sh`, which unsets `GIT_DIR`; `scripts/hooks/turbo.test.ts` fails on a `turbo` line in `lefthook.yml` that skips it.

## CI Picks Jobs by the Paths a Change Touches

CI runs its Rust and TypeScript jobs only when a change touches a path those jobs read, and the CI Gate accepts the skip. A release always runs its Rust jobs, but publishes crates, runs the TypeScript job and deploys the site and Convex only for the surfaces that changed since the last shipped release. Both workflows decide it with one list, `scripts/ci/changed-surfaces.sh` (ISS-38); a release passes `--release`, which leaves out CI's own files. A job that starts reading a new file outside its folders (a root config, a shared fixture) needs that path on the list, or a change to only that file skips the job and passes.

`scripts/ci/changed-surfaces.test.ts` (in `task check:scripts` and CI) fails when a `$TURBO_ROOT$` input in any `turbo.json` isn't on the web list, a root path the engine's Rust code reads through a `../` literal isn't on the engine list, or a `scripts/` or `recipes/` path a gated `ci.yml` job runs isn't on that job's list. Other reads (a TypeScript test's `readFileSync` of a root file) are a manual step: add the path to the list in the same PR.

## A `loading.tsx` Can Steal a Page's LCP

Every page renders per request (`next build` marks them `ƒ`, recipe pages included, ISS-99), so a route's `loading.tsx` is streamed first: the server sends the layout with the fallback in the page's place, and the page follows in a hidden `<div id="S:…">` that a script swaps in. A first paint before the swap shows the skeleton and the layout's footer, whose mascot then becomes the LCP image. Lighthouse's `lcp-discovery-insight` fails on it (it's lazy, not high priority), and only on some runs: the v0.19.0 release failed on `/compress-images` this way.

**Fix:** give a route a `loading.tsx` only when its page awaits real work. Recipe pages read a static registry and have none (removed in #801). `e2e/pages/recipe-page-loading.spec.ts` fails on a streamed fallback (`<template id="B:`) or a hero `<img>` that comes after the footer's in the HTML.

## Turbopack + Node.js Subpath Imports on Vercel

Turbopack on Vercel does NOT support Node.js subpath imports (`#components/*` via `package.json` `imports` field). Builds fail with `Module not found: Can't resolve '#components/...'`. Setting `turbopack.root` to the monorepo root makes it worse — breaks the `imports` field resolution entirely. `resolveAlias` workarounds also fail.

**Fix:** Use standard `@/` path aliases via `tsconfig.json` `paths` instead of subpath imports. Do NOT set `turbopack.root` in `next.config.ts` — `transpilePackages` handles monorepo resolution.

## Git Case-Sensitivity on macOS (Vercel Deploys)

macOS filesystems are case-insensitive by default. Renaming `container.tsx` to `Container.tsx` on macOS won't be detected by git — it keeps tracking the old lowercase name. On Vercel (Linux, case-sensitive), imports for `@/components/ui/Container` fail because git checked out `container.tsx`.

**Fix:** Use a two-step `git mv` to force git to recognize the case change:

```bash
git mv components/ui/container.tsx components/ui/container_tmp.tsx
git mv components/ui/container_tmp.tsx components/ui/Container.tsx
```

**Detection:** Run this to find case mismatches:

```bash
git ls-files apps/web/components/ | while read gitpath; do
  dir=$(dirname "$gitpath"); base=$(basename "$gitpath")
  actual=$(ls -1 "$dir" 2>/dev/null | grep -ix "$base")
  [ -n "$actual" ] && [ "$actual" != "$base" ] && echo "MISMATCH: git=$gitpath disk=$dir/$actual"
done
```

## Playwright Screenshot Updates

`--update-snapshots` won't overwrite existing screenshots that already match. If you need to force full regeneration, delete the `__screenshots__/` directory first.

## Stale Symlinks After Moving Packages

After renaming or moving packages in the monorepo, old `node_modules` symlinks can persist and cause confusing import errors.

**Fix:** Delete `node_modules` in the affected packages and run `pnpm install`.

## FileData::Bytes on Large Files Causes OOM

Engine processors that read files into `FileData::Bytes` when they only need to pass them through the pipeline cause unnecessary memory pressure. A user processing a directory of 2 GB video files with `file-collect` + `file-rename` would load every file into RAM if the collect step uses `FileData::Bytes` — easily exhausting memory.

**Symptoms:** OOM kills or excessive memory usage when processing large files or directories with many files. The pipeline succeeds on small test fixtures but fails on real workloads.

**Fix:** Use `FileData::Path(path)` instead of `FileData::Bytes(std::fs::read(&path)?)` when the processor doesn't modify file content. `FileData::Path` defers reading until a downstream processor actually needs the bytes, and `write_to()` uses `rename()` (O(1)) instead of writing bytes to disk.

```rust
// BAD — reads entire file into memory for no reason
let data = std::fs::read(&path)?;
OutputFile { data: FileData::Bytes(data), .. }

// GOOD — zero-copy path reference
OutputFile { data: FileData::Path(path), .. }
```

**Rule of thumb:** If your processor's `process()` function doesn't call `input.data.into_bytes()`, its output should use `FileData::Path`. See [engine-node-patterns.md](../scopes/rust/engine-node-patterns.md#filedata-selection) for the full guide.

**The CLI's input files are `FileData::Path` too.** `bnto run` argument files enter the pipeline as references to the user's own files (`read_pipeline_file` in `bnto/src/io.rs`). So code that writes results out must use `copy_to()`, never `write_to()`: `write_to()` renames a `Path`, which would move the user's original into the output folder. Only `overwrite` mode renames sources, on purpose.

**A copy can land on an input the run hasn't read yet.** Writing into a folder that holds the run's inputs, `a` renamed to `b.txt` replaced `b.txt` before `b` was copied out, and `b`'s content was gone (ISS-48). The final write goes through `bnto/src/ordered_write/`: each write waits until no later write still reads its destination (matched by file identity, not name), and a swap sets one file aside first. Two destinations that are one file can't both be kept, so that plan is refused before any write. A new path that writes several results must go through `ordered_write` too.

- **One file, two names:** an existing file through a symlink or hard link is matched by identity, and a new one through a folder symlink or `..` by the path it lands on. Any other names a filesystem treats as one (Unicode NFC/NFD on APFS, trailing dots and spaces or 8.3 short names on Windows) are caught by creating every new destination exclusively (`create_new`) before the first write. Don't add a name-normalization table. Each empty placeholder is removed again just before its own write, so the copy creates the file: copying onto an existing file can't clone on APFS (a 1 GB copy went from 0.4 ms to 244 ms). A write that creates its file and then fails (disk full, file too large) has that empty file removed. SIGKILL or power loss after claiming can leave empty files under output names. A rerun replaces one at a name the recipe gives, but not at a made-up name like `b_1.txt`: `output_names` never takes a name a file holds, and an empty file doesn't hold the result, so the rerun writes `b_2.txt`. Such stray empty files are safe to delete.
- **The staging folder** is `.bnto-staged-*` inside the output folder: hidden on Unix, not on Windows. The file a swap replaces is copied into it, not moved, so the original stays in place until that write replaces it: a write that fails, runs out of space or is stopped first loses nothing. The cost, accepted: swapping a large file copies it once. From that write until the copy's last reader has written, the copy is the only one. A finished write removes it (a failed removal is an error). Anything else keeps it, registered: a failed write names it in its error, which the TUI shows wrapped in full. Ctrl+C in `bnto run`, or quitting the TUI mid-run, stops writes at their next step (`ordered_write::GATE`) before scratch dirs go, then names every kept folder on stderr. A panic keeps it too. `file-collect` skips any folder with that prefix, so a later run never takes those files as inputs.
- **Parked files:** `bnto_file::relocate` (overwrite-mode output, `file-move`) breaks a swap of renames by parking one file under `.bnto-rename-<n>-<name>` in its own folder until its final rename, and registers it meanwhile. Ctrl+C in `bnto run`, or quitting the TUI mid-run, names every registered parked file still on disk (`exit_report.rs`), never deleting it: it may hold the only copy. Moves don't wait on `GATE`, so one can still finish after the report (a stale line at worst), but none parks a file after it: each park renames with the list locked, and taking the list refuses every later park. A failed run names them in its error. `file-move` moves folders too, so a parked entry can be a whole folder: `file-collect` skips any file or folder with that prefix.
- **Progressive writes** can't see later iterations' sources (`file-list` in the pipeline finds them at run time), so they leave any destination that already holds a file to the final write (besides the held names `output_names` already skips). The cost: such a file is written only by the final write. A failed or cancelled run doesn't write it, so a rerun keeps the previous version and an input stays untouched. Accepted, because a progressive copy that replaces a later iteration's unread input loses that input for good. For the same reason a conflict between two destinations is only seen at the final write, after iterations wrote some files: its refusal names them (in the TUI too: its progressive copies run on the bridge thread, before the final write). Progressive writes go through `GATE` as well, so Ctrl+C or a TUI quit waits for the file being copied and starts no more; with a large file that wait lasts until its copy ends. A held file (#759) is never written by either write, and isn't a clash unless another write would replace it.

**Never copy a file onto itself.** `std::fs::copy(p, p)` opens the destination for writing first, so it empties the file and returns `Ok(0)` (an output folder that is the input's own folder did this, ISS-36). `copy_to()` and `write_to()` check `FileData::is_at(dest)`, which compares file identity (`same-file`), not names, so `..`, symlinks, hard links and another case on macOS/Windows all count, and leave such a file as is. Any new code that opens a destination for writing from a `Path` source goes through `copy_to()` or makes the same check.

## Recipe Edges Route Files

The engine honors a recipe's `edges` (see [engine-execution.md](../strategy/engine-execution.md#edges-and-data-flow)). Three things catch people out:

- **Edge order sets merge order.** A step fed by several edges gets their files in the order the edges are listed in `edges`, not `nodes` order. List `notes → join` before `listing → join` to put the notes first.
- **A step with no incoming edge starts empty.** No more drop-everything `file-filter` before a fresh `file-list`; leave it unwired from the step before. A step wired to nothing after it contributes no files to the output.
- **Fan-out shares files on disk.** One output feeding two steps hands both the same `FileData::Path`. A step that moves or deletes it (`file-move`, `file-delete`, an overwriting output) takes it from the branches that run after it, and they fail naming the missing file. A `file-copy` over it is refused (`ProcessContext::held_elsewhere`). A new node that replaces files on disk should ask the same, and must be a batch node: only a batch node's context gets the run's answer (`executor/run_scope.rs`); any other node's always says `None`.

## Windows Portability (Engine)

CI runs the engine's fmt, clippy, and tests on Windows as well as Linux (`rust-windows` job, required by the CI Gate). Code and tests that assume Unix break there. Rules:

- **Output names from relative paths use `bnto_core::slash_path`.** File names travel between nodes and downstream nodes split them on `/` (e.g. `file-filter` matches against `filename.rsplit('/')` in `bnto-file/src/filter.rs`). `rel.to_string_lossy()` yields `a\b.mp4` on Windows, so the filter sees the whole string as the name. `file-collect` and `shell-run` already use `slash_path`.

  ```rust
  // BAD — `\` separators on Windows
  let name = rel.to_string_lossy().into_owned();
  // GOOD — `/` on every OS
  let name = bnto_core::slash_path(rel);
  ```

- **Names that differ only in case are one file.** On Windows and macOS `Logo.svg` and `logo.svg` share a path, so code that puts several files in one folder compares names case-folded on every OS, so a recipe acts the same everywhere. Moves and renames go through `bnto_file::relocate` (refuses the plan). The CLI's `--output` writes go through `bnto/src/output_names.rs` (later file becomes `logo_1.svg`, or the next number not already in the folder; a file with exactly that name already holding exactly the same bytes is taken and left as is, see [output-suffix-skips-existing.md](../decisions/output-suffix-skips-existing.md)). Don't key a "taken names" set on the raw name.
- **No `/nonexistent`-style absolute paths in tests.** On Windows `/nonexistent` resolves to `C:\nonexistent` and may be creatable, so "can't be created" tests write outside the sandbox and pass or fail by accident. To get a path that can't be created, put it under a regular file in a temp dir:

  ```rust
  let tmp = tempfile::tempdir().unwrap();
  let file = tmp.path().join("not-a-dir");
  std::fs::write(&file, b"").unwrap();
  let bad_path = file.join("logs"); // cannot be created on any OS
  ```

  For a path that just doesn't exist, use `tmp.path().join("missing")`, and for a recipe catalog with no library, `RecipeCatalog::bundled_only()`. `task check:nonexistent-paths` (and CI's engine job) fails on any `"/…nonexistent…"` string, in any case, in a Rust file under `engine/crates`.

- **Tests don't spawn `sh` or `echo` directly.** On Windows `echo` is a `cmd` built-in, not a program, and `sh` is not guaranteed on PATH. Run the script through the platform shell — see the `shell()` test helper in `bnto/src/context.rs` (`cmd /c` on Windows, `sh -c` elsewhere). Trim captured lines before asserting: `cmd` keeps the space before a redirect (`echo err >&2` prints `err `).
- **Gate items that only one OS reads.** A constant or function used only inside a `#[cfg(unix)]` path is dead code on Windows, and clippy `-D warnings` fails there. Move it inside the `#[cfg(unix)]` function that reads it, or gate its declaration to match its readers (e.g. `#[cfg(any(unix, test))]`).

## No Object.assign Compound Components (RSC Incompatible)

`Object.assign` dot-notation (`<Dialog.Content>`, `<Card.Header>`) does NOT work in React Server Components. Server components get "client reference" objects — sub-properties assigned via `Object.assign` are undefined at render time.

**Fix (already applied repo-wide):** Use flat named exports with prefixed names instead:

```tsx
// GOOD -- flat exports, RSC compatible
import { Dialog, DialogContent, DialogTitle } from "@bnto/ui";

<Dialog open={open}>
  <DialogContent>
    <DialogTitle>Title</DialogTitle>
  </DialogContent>
</Dialog>

// BAD -- Object.assign dot-notation, breaks in RSC
<Dialog.Content>
  <Dialog.Title>Title</Dialog.Title>
</Dialog.Content>
```

**If you see `Object.assign` compound patterns anywhere in the codebase, convert them to flat exports.**

## Rust Toolchain Is Pinned

`engine/rust-toolchain.toml` pins an exact stable release (e.g. `channel = "1.98"`), not the floating `stable` channel. It is the **single source of truth**: local `cargo` picks it up automatically, and the CI and release workflows install it with `rustup toolchain install` (no version in any workflow file). Every Rust release can add clippy lints that fail the `-D warnings` gate on untouched code, so new lints arrive only through a deliberate bump PR, never as a surprise on an unrelated one.

**Bump checklist (one PR):**

1. Change `channel` in `engine/rust-toolchain.toml`. Nothing else holds the version.
2. Run `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` on **Linux and Windows** (CI runs both). Some lints and `cfg` paths are platform-specific.
3. Fix every new lint in the same PR.

**MSRV vs pinned toolchain.** `rust-version` in `engine/Cargo.toml` `[workspace.package]` is the _minimum_ compiler the crates build on, which is what `cargo install bnto` users need. The pinned toolchain is what contributors and CI _use_, and it's usually newer. Bumping the toolchain doesn't change the MSRV. Raise `rust-version` only when code starts using a newer language feature or std API (verify with `rustup toolchain install <ver> --profile minimal && cargo +<ver> check --workspace --all-targets`).
