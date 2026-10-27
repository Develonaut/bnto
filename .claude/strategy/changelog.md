# Changelog: what changed, everywhere bnto shows up — 5 PRs

**Created:** October 24, 2026
**Status:** Decided by Ryan on October 24, 2026: a curated `CHANGELOG.md`, surfaced by a site page and Atom feed, a "New in vX" badge, `bnto changelog`, and links from the CLI and TUI update notices. Not started.
**Related:** [releases.md](../scopes/process/releases.md), [cli-upgrade.md](cli-upgrade.md)

## Context

**The outcome:** anyone on bnto.io, or using the CLI or TUI, can see what changed in each release. Someone returning after a release learns there's something new.

**What exists today:**

- **No changelog file.**
- **GitHub Releases are auto-generated.** They hold the deploy status plus every merged PR's title, mostly internal work such as lint, refactors and plans.
- **The CLI and TUI announce new versions.** `update_check` covers that, but nothing says what changed.
- **The site shows the version** only in its install demo.
- **The best user-facing notes are buried.** They're the "What users get" sections hand-written in each version-bump PR (#737, #743).

**The web is in maintenance mode** (PLAN.md, the CLI/TUI-first pivot). PRs 2 and 3 are a deliberate exception, approved by Ryan on October 24, 2026, and recorded in PLAN.md.

**What changes:**

- a new curated changelog, written in the bump PR and checked in CI and at tag time;
- the GitHub Release body;
- a `/changelog` page with an Atom feed;
- a "new" badge on the site;
- a new `bnto changelog` command;
- the CLI and TUI update notices, and `bnto upgrade`'s output.

**What doesn't change:** no node, recipe or processor counts. No new service: everything on the site is static at build time, the badge state lives in each visitor's own browser storage, and `bnto changelog` reads a file compiled into the binary. Cost stays at $0.

Split into 5 PRs. PR 1 comes first. PRs 2 and 4 depend only on it. PR 3 depends on PR 2. PR 5 depends on PR 4, and links to the page PR 2 builds.

## Design

| Question                      | Choice                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Curated or generated          | **Curated**, a few plain lines per release. Generating it from commits (Conventional Commits with git-cliff or release-please) gives what GitHub's notes already give: every PR title, mostly internal. It would also need every squash title to follow a format. Changesets (a file per PR) adds a step to every PR, for a repo with one release number. The bump PR already has a hand-written "What users get", so curating costs no more. |
| Where the file lives          | **`engine/crates/bnto/CHANGELOG.md`**, with a one-line root `CHANGELOG.md` pointing at it. `cargo publish` packages only the files inside the crate's folder, so `include_str!` can't reach a root file from a published crate. Inside the crate, it ships in the published package, so the CLI reads it offline.                                                                                                                             |
| Format                        | [Keep a Changelog](https://keepachangelog.com/): `## [0.18.2] - 2026-10-24`, then `### Added`, `### Changed`, `### Deprecated`, `### Removed`, `### Fixed` and `### Security`, each a `- ` list. No `[Unreleased]` section: entries are written in the bump PR. Older releases (before 0.18.0) are left to GitHub Releases.                                                                                                                   |
| When it's enforced            | In CI on every PR (the release version must have a non-empty entry), and again when a stable tag is cut (`task release:tag`, `release.yml`). A bump PR with an empty stub fails CI before it merges.                                                                                                                                                                                                                                          |
| Pre-release versions          | `bump-version.sh` accepts them (`scripts/bump-version.sh:12`), but pre-release tags don't publish crates and aren't version-checked (`check-release-version.sh:14-17`). They get no stub and need no entry. The stable release's entry covers what its pre-releases shipped.                                                                                                                                                                  |
| GitHub Release body           | The entry first, then the current deploy and surfaces block, then GitHub's generated "What's Changed". With `generate_release_notes: true`, `softprops/action-gh-release` calls GitHub's generate-notes endpoint and builds `${body}\n\n${notes}`, so the generated list always comes last. If `body_path` can't be read, the action only warns and falls back to `body:`, so the step that writes the file fails the job instead.            |
| Site                          | Read the file at build time, the way the install demo reads `engine/Cargo.toml` (`apps/web/lib/getBntoCliVersion.ts:12-15`). Fetching GitHub Releases at runtime would add a network call, a rate limit and a render cost, for notes that aren't curated.                                                                                                                                                                                     |
| Badge state                   | The visitor's `localStorage`. A cookie or Convex would mean server state, and Convex would need an account.                                                                                                                                                                                                                                                                                                                                   |
| What the CLI can show offline | Only the versions compiled into the running binary. An older bnto can't know a newer release's notes, so anything that announces a newer version links to `https://bnto.io/changelog#<version>`.                                                                                                                                                                                                                                              |
| TUI notice                    | `bnto changelog`, not the URL. The notice sits on the right of the status line and the key hints get what's left (`tui/render.rs:201-222`). At 80×24 only `Tab pane` fits beside today's notice, so a URL would push out every hint. Running `bnto changelog` in a shell prints the CLI notice with the link to the new release, then the installed version's notes.                                                                          |

---

## PR 1: A curated changelog, written in every release's bump PR

**Branch:** `feat/changelog-file` from `main`
**One sentence:** Each release gets a hand-written changelog entry, which becomes its GitHub Release body.

### What

The changelog is written in plain words for users. Internal PRs are left out unless they change what a user sees.

- **Backfill:** 0.18.0, 0.18.1 and 0.18.2, from their bump PRs and release notes.
- **Stub:** `task version:bump` adds `## [X.Y.Z] - <date>` above the newest entry, dated the day the bump runs (UTC). If the tag will go out on a later day, the author sets the date by hand before merging the bump PR; no check compares the date with the tag's. Running the bump twice adds one stub. A pre-release version gets none.
- **Gate:** `scripts/check-changelog.sh` fails when the release version's entry is missing or has no `- ` item. CI runs it on every PR. `check-release-version.sh` runs it too, so `task release:tag` and the release workflow (`release.yml:32-33`) refuse a stable tag with no entry.
- **Release body:** a new step before "Create GitHub Release" (`release.yml:649`) writes `release-body.md` with `scripts/release-body.sh`. The step runs under `set -euo pipefail` and ends with `test -s release-body.md`, so a missing or empty file fails the job. Without that, the action would only warn and fall back to `body:`. The action reads it through `body_path:` in place of the inline `body:` (`release.yml:656-670`). The body is:
  1. `## What's new`, then the entry (a pre-release has none);
  2. the "Deployed" and "Surfaces" block, unchanged;
  3. GitHub's appended "What's Changed" list, as now.
- **Script tests** run in `task check:scripts`, which `task check` runs, and in CI's Scripts job, which the CI Gate requires. Both already exist (the ISS-54 and ISS-65 fix added them, running every `scripts/**/*.test.ts`), so this PR adds `bash scripts/tests/changelog.sh` to each rather than creating them. `scripts/tests/` doesn't exist yet; this PR adds it.

### Files (~7 new, ~6 modified)

**New:**

- `engine/crates/bnto/CHANGELOG.md`
- `CHANGELOG.md`: the pointer
- `scripts/release-version.sh`: prints `[workspace.package] version`. The awk moves here from `check-release-version.sh:19-23`
- `scripts/changelog-entry.sh`: prints one version's entry
- `scripts/check-changelog.sh`: the gate
- `scripts/release-body.sh`: the release body
- `scripts/tests/changelog.sh`: plain-bash tests for all of the above

**Modified:**

- `scripts/bump-version.sh`: adds the stub
- `scripts/check-release-version.sh`: uses `release-version.sh`, then runs `check-changelog.sh`
- `.github/workflows/release.yml`: `create-release` job (l.619). A new "Write the release body" step after "Summarize deploy results" (l.629-647), failing the job when the file is missing or empty; `body:` (l.656-670) becomes `body_path: release-body.md`
- `.github/workflows/ci.yml`: a "Changelog script tests" step in the `scripts` job, and a "Changelog has the release version's entry" step in `crate-publish-check`
- `Taskfile.yml`: `bash scripts/tests/changelog.sh` in the existing `check:scripts`
- `.claude/scopes/process/releases.md`: "How to Cut a Release" and "One number everywhere" say the bump PR writes the entry

### Key functions

```bash
scripts/changelog-entry.sh <version>  # the lines between `## [<version>]` and the next `## [`; exit 1 if missing or no `- ` item
scripts/check-changelog.sh [version]  # default: release-version.sh; a pre-release passes; else changelog-entry.sh or fail
scripts/release-body.sh <tag>         # stdout: entry (stable tags), then the deploy block from VERCEL, CONVEX, CARGO, ENGINE_CHANGED, WEB_CHANGED, CONVEX_CHANGED
```

### RED tests

In `scripts/tests/changelog.sh`. Each case runs in a temp dir holding a small `engine/Cargo.toml`, a fixture changelog and a no-op `cargo` first on `PATH`, so nothing touches the repo or runs `cargo update`.

- **`changelog-entry.sh`:**
  - `0.18.2` prints that entry's sections, without its heading, and stops at the next `## [`;
  - the last entry ends at end of file;
  - an unknown version exits 1, naming the version and the file;
  - a stub with no `- ` item exits 1.
- **`check-changelog.sh`:**
  - a stable version with an entry passes;
  - a stub fails, naming the version and `engine/crates/bnto/CHANGELOG.md`;
  - `1.0.0-beta.1` passes.
- **`check-release-version.sh`:**
  - `v9.9.9` with a matching workspace version but no entry fails, naming the missing entry;
  - with the entry it passes;
  - a pre-release tag still passes untouched.
- **`bump-version.sh`:**
  - `0.18.3` adds `## [0.18.3] - <today, UTC>` directly above `## [0.18.2]`;
  - a second run adds no second stub;
  - `1.0.0-beta.1` adds no stub.
- **`release-body.sh`:**
  - `v0.18.2` prints `## What's new`, then the entry, then `## Deployed`;
  - `v1.0.0-beta.1` starts with `## Deployed` and has no entry.
- **The real file:** `check-changelog.sh` passes on the repo, since the backfill holds 0.18.2.

### Verification

```bash
bash scripts/tests/changelog.sh
./scripts/check-changelog.sh
./scripts/check-release-version.sh v0.18.2
VERCEL=Deployed CONVEX=Skipped CARGO=Deployed ENGINE_CHANGED=true WEB_CHANGED=true CONVEX_CHANGED=false ./scripts/release-body.sh v0.18.2
task check:scripts && task check
```

The `body_path` change itself first runs on the next tag. Check that release's page: the entry first, "What's Changed" last.

### No count changes

---

## PR 2: A `/changelog` page and feed on bnto.io

**Branch:** `feat/web-changelog` from `main`, after PR 1
**One sentence:** bnto.io publishes each release's notes at `/changelog`, anchored by version, with an Atom feed beside it.

### What

The page is a statically generated server component. `readChangelog()` reads `../../engine/crates/bnto/CHANGELOG.md` from `apps/web` at build time, as `getBntoCliVersion.ts:12-15` does. A pure parser turns the file into releases, dates, sections and items. Items keep backtick code as code segments, and are rendered as text, never as HTML.

- **The page** is `app/(app)/changelog/page.tsx`, so it gets the Navbar and Footer. A static segment wins over `(app)/[bnto]`, and `changelog` is already in `RESERVED_PATHS` (`apps/web/lib/bntoRegistry.ts:86`). Each release is an `h2` whose `id` is its version (`id="0.18.2"`), newest first.
- **Metadata:** `page.tsx` exports `metadata`, as `explore/page.tsx:16` does, with `alternates.types` for `application/atom+xml` pointing at `/changelog.xml` (feed discovery). There's no layout file.
- **The feed:** `/changelog.xml` is a `force-static` route (like `app/llms.txt/route.ts:4`) serving Atom built from the same parse. Each entry's `id` and link is `https://bnto.io/changelog#<version>`. The feed's `updated` is the newest release's date, not the build time.
- **Linked from:** the footer's Company section (`FooterLinks.tsx:12-19`). **Listed in:**
  - the sitemap (`buildStaticSitemapEntries.ts:6-27`);
  - Lighthouse (`scripts/generate-lighthouserc.ts:29`);
  - `scripts/seo/collectUrls.ts:11`.

### Files (~9 new, ~7 modified)

**New:**

- `apps/web/lib/changelog/readChangelog.ts`: server-only file read. It throws when the file is missing, so a build without it fails
- `apps/web/lib/changelog/parseChangelog.ts`
- `apps/web/lib/changelog/buildAtomFeed.ts`
- `apps/web/lib/changelog/__tests__/parseChangelog.test.ts`
- `apps/web/lib/changelog/__tests__/buildAtomFeed.test.ts`
- `apps/web/app/(app)/changelog/page.tsx`
- `apps/web/app/(app)/changelog/_components/ChangelogRelease.tsx`
- `apps/web/app/changelog.xml/route.ts`
- `apps/web/e2e/pages/changelog.spec.ts`

**Modified:**

- `apps/web/app/buildStaticSitemapEntries.ts`
- `scripts/generate-lighthouserc.ts`
- `scripts/seo/collectUrls.ts`
- `scripts/seo/collectUrls.test.ts`: asserts `https://bnto.io/changelog`, beside the existing page asserts (l.13-16)
- `apps/web/turbo.json`: `$TURBO_ROOT$/engine/crates/bnto/CHANGELOG.md` in both the `build` and `test` inputs, beside `$TURBO_ROOT$/engine/Cargo.toml`. The page, the feed and the live-file test read it from outside `apps/web`, and Turbo hashes only a package's own files (gotchas.md, "Turbo Hashes Only a Package's Own Files"). Without it, a changelog-only edit would replay a cached build
- `apps/web/components/blocks/Footer/FooterLinks.tsx`
- `apps/web/e2e/pages/site-navigation.spec.ts`: `/changelog` in `PUBLIC_ROUTES` (l.12-24), and the footer link (`footer-link-changelog`, from `FooterSectionColumn.tsx:36`)

### Key functions

```ts
type Segment = { text: string; code: boolean };
type ChangelogRelease = {
  version: string;
  date: string;
  sections: { kind: SectionKind; items: Segment[][] }[];
};
export function parseChangelog(markdown: string): ChangelogRelease[];
export function buildAtomFeed(releases: ChangelogRelease[], siteUrl: string): string;
```

### RED tests

**Unit, `parseChangelog`:**

- The three backfilled releases parse newest first, with their versions and dates.
- The six section kinds are kept in file order. An unknown `### Kind` is skipped.
- An empty section is dropped, and so is a release with no items (a stub).
- Malformed headings and the link-reference lines at the bottom are skipped, not thrown on.
- `` `bnto upgrade` `` becomes a code segment. An unclosed backtick stays text.
- The live file's newest release equals `getBntoCliVersion()`. It's skipped when the workspace version is a pre-release, which has no entry (see Design), as PR 4's equivalent is.

**Unit, `buildAtomFeed`:**

- One `<entry>` per release, newest first, each with `id` `https://bnto.io/changelog#<version>`.
- The feed's `updated` is the newest release's date.
- `&` and `<` in an item are escaped.

**E2E (`@browser`, `apps/web/e2e/pages/changelog.spec.ts`):**

- `/changelog` returns under 400 and shows an `h2` per release, newest first. `page.locator('[id="0.18.2"]')` is visible: `#0.18.2` isn't a valid CSS selector, though it's a fine URL fragment.
- `not-found-heading` is absent.
- `/changelog#0.18.0` scrolls that heading into view.
- The head has `link[rel="alternate"][type="application/atom+xml"]` ending in `/changelog.xml`.
- `request.get("/changelog.xml")`: 200, `application/atom+xml`, and as many `<entry>` as the page has headings.

### Verification

```bash
(cd apps/web && pnpm exec vitest run lib/changelog)
pnpm exec tsx --test scripts/seo/collectUrls.test.ts scripts/seo/indexNowOutcome.test.ts
pnpm turbo run build --filter=@bnto/web | grep changelog   # /changelog and /changelog.xml both listed as static (○)
(cd apps/web && pnpm exec playwright test e2e/pages/changelog.spec.ts e2e/pages/site-navigation.spec.ts)
task e2e && task seo:audit && task check
```

`collectUrls.test.ts` is a top-level `node:assert` script, so `tsx --test` fails the file when an assert throws. No CI or Taskfile change is needed for it: `task check:scripts` and CI's Scripts job already run every `scripts/**/*.test.ts` (the ISS-54 and ISS-65 fix).

To check the Turbo input: run `pnpm turbo run test --filter=@bnto/web` twice, edit `engine/crates/bnto/CHANGELOG.md`, and run it again. The third run must not replay the cache.

### Count changes

- Sitemap static entries: 5 → 6
- Lighthouse static pages: 4 → 5
- `collectUrls` static pages: 5 → 6

None of these counts is asserted exactly.

---

## PR 3: A "New in vX" badge on bnto.io

**Branch:** `feat/web-new-release-badge` from `main`, after PR 2
**One sentence:** Returning visitors see a dismissible "New in vX →" badge linking to that version's changelog entry.

### What

The header is `apps/web/components/blocks/Navbar.tsx`, a server component. `app/(app)/layout.tsx:11` renders it, and so do `(dev)/layout.tsx`, `editor/layout.tsx` and `not-found.tsx`. Navbar reads the latest version with `getLatestChangelogVersion()` (PR 2's parser) and passes it to a client leaf, `NewReleaseBadge`. Today every one of those routes is prerendered, so the read happens at build: `(app)/[bnto]` sets `dynamicParams = false` (`page.tsx:17`), so an unknown slug gets the prerendered 404. A route rendered on demand would run the read on Vercel, where the file isn't deployed. So `getLatestChangelogVersion()` returns `null` when the file can't be read or parsed, and Navbar then renders no badge. Unlike `readChangelog()`, which the page uses, it never throws. The `apps/web/turbo.json` build input that PR 2 adds covers this read too.

- **When it shows:** the visitor's `localStorage` `bnto:last-seen-version` is older than the latest.
- **When it hides:** after the visitor follows it or dismisses it. Both write the latest version.
- **First-time visitors:** with no stored version, it stores the latest without showing, so the badge means "new since you were last here".
- **No flash, no shift:** the badge renders nothing on the server and on the first client render, and reads storage in a `useEffect` after mount. That keeps hydration clean. It's positioned `absolute` under the logo's `ToolbarGroup` (`Navbar.tsx:14-32`, made `relative`), so it takes no layout space when it appears (no CLS).
- **Storage:** every read and write is wrapped in try/catch, so blocked storage just means no badge. `apps/web/lib/useDismissible.ts` isn't reused or copied: it reads `localStorage` in its initial state, with no try/catch (l.12-15).
- **Telemetry:** none beyond the existing pageview.

### Files (~7 new, ~1 modified)

**New:**

- `apps/web/components/blocks/NewReleaseBadge.tsx`
- `apps/web/components/blocks/useNewReleaseBadge.ts`: mount-time read, follow/dismiss write
- `apps/web/lib/changelog/getLatestChangelogVersion.ts`
- `apps/web/lib/changelog/__tests__/getLatestChangelogVersion.test.ts`
- `apps/web/lib/changelog/shouldShowBadge.ts`
- `apps/web/lib/changelog/__tests__/shouldShowBadge.test.ts`
- `apps/web/e2e/pages/new-release-badge.spec.ts`

**Modified:**

- `apps/web/components/blocks/Navbar.tsx`

### Key functions

```ts
export function shouldShowBadge(lastSeen: string | null, latest: string): boolean;
export function getLatestChangelogVersion(path?: string): string | null; // never throws
```

### RED tests

**Unit, `shouldShowBadge(lastSeen, latest)`:**

- no last-seen version → false;
- older → true, compared numerically (`0.9.0` is older than `0.10.0`);
- equal → false;
- a newer local value (from a pre-release) → false;
- an unparseable value → false.

**Unit, `getLatestChangelogVersion`:**

- the live file gives its newest version;
- a path that doesn't exist gives `null`, and so does a file with no release; neither throws.

**E2E (`@browser`, `apps/web/e2e/pages/new-release-badge.spec.ts`).** Storage is seeded with `page.addInitScript`.

- A seeded `0.0.1` shows "New in v<latest>".
- Clicking it lands on `/changelog#<latest>` with `[id="<latest>"]` in view.
- After a reload, the badge is gone.
- A first visit shows no badge, and afterwards stores the latest.
- With `localStorage` throwing, there's no badge and no captured page error.
- The server HTML (`request.get("/")`) has no badge.
- The logo's bounding box is the same with and without the badge.

### Verification

```bash
(cd apps/web && pnpm exec vitest run lib/changelog)
(cd apps/web && pnpm exec playwright test e2e/pages/new-release-badge.spec.ts)
task e2e && task seo:audit && task check
```

Playwright and Lighthouse start with empty storage, so they see no badge, and page-level screenshots shouldn't change. If one does, regenerate it in two passes.

### No count changes

---

## PR 4: A `bnto changelog` command

**Branch:** `feat/cli-changelog` from `main`, after PR 1
**One sentence:** `bnto changelog [version]` prints a release's notes from the changelog compiled into the binary.

### What

- **The source:** `include_str!("../CHANGELOG.md")` in `src/changelog.rs`. It works offline and costs no network call.
- **Which version:** it defaults to the running version. `v0.18.2` and `0.18.2` both work. The notes go to stdout, and errors to stderr.
- **A version this bnto doesn't have:**
  - Newer than the running one: it exits 1 with "0.19.0 is newer than this bnto (0.18.2). Read it at https://bnto.io/changelog#0.19.0, or run `bnto upgrade`."
  - Any other: it exits 1, naming the versions this bnto covers.
- **The update notice:** it runs before `bnto changelog` as before any command but `tui` and `upgrade` (`dispatch.rs:39-43`).
- **Telemetry:** the command reports itself as `changelog` (`Command::name()`, `cli.rs:178-193`).

### Files (~2 new, ~5 modified)

**New:**

- `engine/crates/bnto/src/changelog.rs`
- `engine/crates/bnto/tests/changelog_command.rs`: uses `tests/helpers/sandbox.rs`

**Modified:**

- `engine/crates/bnto/src/main.rs`: `mod changelog;`
- `engine/crates/bnto/src/cli.rs`: `Command::Changelog { version: Option<String> }`, and its `name()`
- `engine/crates/bnto/src/dispatch.rs`: the `run_command` arm (l.48-66)
- `engine/crates/bnto/README.md`: the Usage block
- `.claude/journeys/cli.md`: rows CJ10 and CJ11 (below)

### Key API

```rust
/// One version's entry, without its heading; None when missing or it has no item.
pub fn entry<'a>(changelog: &'a str, version: &str) -> Option<&'a str>;
/// Every version with an entry, newest first.
pub fn versions(changelog: &str) -> Vec<&str>;
/// Where bnto.io shows a version's notes: https://bnto.io/changelog#<version>.
pub fn notes_url(version: Version) -> String;
/// `bnto changelog [version]`.
pub fn run_changelog(version: Option<&str>);
```

### RED tests

**Unit (`changelog.rs`, on a fixture string):**

- `entry` finds 0.18.2 and stops at the next heading;
- the last entry ends at end of file;
- an unknown version, and a stub with no item, give `None`;
- `versions` is newest first;
- `notes_url(Version(0, 19, 0))` is `https://bnto.io/changelog#0.19.0`;
- the shipped changelog has the running version's entry. This test is skipped for a pre-release build.

**Integration (`tests/changelog_command.rs`, sandboxed):**

- `cj10_changelog_prints_the_installed_versions_notes`: `bnto changelog` prints the running version's notes and exits 0;
- `cj10_changelog_takes_an_older_version_with_or_without_v`: `bnto changelog 0.18.0` and `bnto changelog v0.18.0` print 0.18.0's notes and exit 0;
- `cj11_an_unknown_version_names_the_ones_covered`: `bnto changelog 0.0.1` exits 1, naming the versions it covers;
- `cj11_a_newer_version_links_its_notes`: `bnto changelog 99.0.0` exits 1 with `https://bnto.io/changelog#99.0.0` and `bnto upgrade`, and prints nothing on stdout.

### Verification

```bash
(cd engine && cargo test -p bnto --bin bnto changelog::)
(cd engine && cargo test -p bnto --test changelog_command)
task wasm:lint && task cli:test
```

### Count changes

None to nodes or recipes. `bnto --help` gains a command, and there's no help golden: `test_help_flag` (`tests/cli_commands.rs:15-21`) checks one substring.

---

## PR 5: Version announcements say what changed

**Branch:** `feat/notices-whats-new` from `main`, after PRs 2 and 4
**One sentence:** Every message that announces a new bnto version also says where to read what changed in it.

### What

Four places announce a new version. Each gains a pointer to the notes:

- **The CLI notice** (`update_check/notice.rs:5-8`, printed by `update_check/mod.rs:36`) gains a second line: `What's new: https://bnto.io/changelog#0.19.0`. The first line is unchanged. It stays on stderr.
- **The TUI notice** (`tui_notice()`, `notice.rs`) becomes `bnto 0.19.0: U upgrade · bnto changelog`. It keeps the U key (added October 27, 2026, after this plan). That's 3 columns longer than today's `bnto 0.19.0 available, U to upgrade` and 1 shorter than the old `bnto 0.19.0 available, run bnto upgrade`, so the 80×24 status line still has room for the `Tab pane` hint. A URL wouldn't fit (see Design).
- **`bnto upgrade --check`** prints `notice()` instead of its own copy of the first line (`upgrade/mod.rs:30`), so it gets the link too.
- **After a successful install,** `bnto upgrade` shows the new version's notes. It runs `<exe> changelog <new version>`, with the path resolved before the install (`upgrade/mod.rs:38`). On Linux, `current_exe()` read after the install would end in `(deleted)`. This step is best-effort and never changes the exit code:
  - It runs with `BNTO_NO_UPDATE_CHECK=1`, `BNTO_TELEMETRY_DISABLED=1` and null stdin, with its output captured. The telemetry opt-out keeps it from sending a `changelog` command event the user didn't type.
  - On success, its stdout is printed.
  - If it can't start or exits non-zero, `bnto upgrade` prints `What's new in 0.19.0: https://bnto.io/changelog#0.19.0` instead, never the child's error, and still exits 0.

### Files (~1 new, ~15 modified)

**New:**

- `engine/crates/bnto/src/upgrade/whats_new.rs`

**Modified:**

- `engine/crates/bnto/src/update_check/notice.rs`: both forms, and the doc comments that quote them (l.5, l.10). Its tests change: `names_both_versions_and_the_command` (l.27-33) and `the_tui_form_fits_a_status_line` (l.19-25)
- `engine/crates/bnto/src/update_check/mod.rs`: `pub use notice::tui_notice`, for the snapshots
- `engine/crates/bnto/src/upgrade/mod.rs`: `--check` uses `notice()`; `install()` calls `whats_new` after "Upgraded …" (l.49-50)
- `engine/crates/bnto/src/tui/screen_snapshots.rs`, two places:
  - `update_available()` (l.277-282) hard-codes the notice string on l.280. It builds it with `update_check::tui_notice(Version(99, 0, 0))` instead, so the snapshot follows the real text.
  - `the_bottom_bar_status_is_never_overwritten` (l.327-353) finds the notice by `" available, U to upgrade"`, which the new text doesn't contain. It finds the start of `tui_notice(Version(99, 0, 0))` instead.
- `engine/crates/bnto/src/tui/app.rs`: the `update_notice` doc comment (l.114) quotes the new text
- `engine/crates/bnto/tests/tui_journeys.rs`: TJ10 (`tj10_a_newer_release_shows_in_the_status_line`) and TJ15 wait for `bnto 99.0.0: U upgrade · bnto changelog` instead of the old text (l.264)
- `engine/crates/bnto/src/tui/snapshots/bnto__tui__screen_snapshots__home_update_available_80x24.snap`
- `engine/crates/bnto/src/tui/snapshots/bnto__tui__screen_snapshots__home_update_available_120x36.snap`
- `engine/crates/bnto/tests/update_notice.rs`: `a_newer_cached_release_is_mentioned` (l.73-83). Its first-line assert still passes (`contains`); add one for the link line
- `engine/crates/bnto/tests/upgrade.rs`: the cases below, and a `fake_cargo` variant that installs a stub `bnto`. The stub is written to a temp file in `cargo/bin`, made executable, then `mv`'d over `cargo/bin/bnto`, which is the running binary. Writing into a running executable fails on Linux (ETXTBSY), and a write fd left open while spawning causes the same error (`tests/upgrade.rs:13-16`)
- `engine/crates/bnto/README.md`: the "Updates" notice example (l.19-27)
- `.claude/journeys/cli.md`: rows CJ12 and CJ13
- `.claude/journeys/tui.md`: TJ10's pass criteria (l.57) give the new text and say the TUI points at `bnto changelog`
- `.claude/strategy/cli-upgrade.md`: a follow-up line in its Status (the notice is now two lines)

### Key API

```rust
/// "bnto 0.19.0 is available (you have 0.18.2). Run `bnto upgrade`.\nWhat's new: https://bnto.io/changelog#0.19.0"
pub fn notice(current: Version, latest: Version) -> String;
/// "bnto 0.19.0: bnto upgrade · bnto changelog"
pub fn tui_notice(latest: Version) -> String;
/// After an upgrade: the new binary's notes for `latest`, else the link. Never fails.
pub fn show_whats_new(exe: &Path, latest: Version);
```

### RED tests

**Unit (`notice.rs`):**

- `notice()` is the unchanged first line, then `What's new: https://bnto.io/changelog#0.17.0`;
- `tui_notice(Version(0, 17, 0))` is exactly `bnto 0.17.0: bnto upgrade · bnto changelog`. This replaces `the_tui_form_fits_a_status_line` and fails today.
- A width guard (it passes today too): `Line::from(tui_notice(Version(99, 99, 99))).width()` is at most 44. That's display width; `.len()` would count `·` as 2 bytes.

**TUI:**

- Both `home_update_available` snapshots change, in two passes. The 80×24 one still shows `Tab pane`.
- `the_bottom_bar_status_is_never_overwritten` still passes on every screen.
- TJ10 sees the new text in a real pseudo-terminal.

**Integration (`tests/update_notice.rs`):** with a cached 99.0.0, `bnto list` in a terminal prints both notice lines (`a_newer_cached_release_is_mentioned`). The opt-out, CI and piped cases still print neither.

**Integration (`tests/upgrade.rs`, unix):**

- `check_reports_a_newer_release_and_caches_it` (l.121-134) also shows the link line.
- **The new binary has the notes** (`cj13_an_upgrade_shows_the_new_binarys_notes`): the fake cargo moves a stub that echoes `notes for $2` over `cargo/bin/bnto`. The output contains `notes for 99.0.0`, has no fallback line, and the exit is 0.
- **The new binary lacks them** (`cj13_an_upgrade_links_notes_the_new_binary_lacks`, set up like `a_cargo_install_is_upgraded_with_cargo`, l.137-152): the fake cargo leaves the copied binary, which doesn't know 99.0.0. The upgrade still exits 0 and prints `What's new in 99.0.0: https://bnto.io/changelog#99.0.0`. The child's "newer than this bnto" error doesn't appear.
- **A failed install** (`a_failed_cargo_install_says_so`, l.155-164) prints no notes and no link.

### Verification

```bash
(cd engine && cargo test -p bnto --bin bnto -- update_check:: upgrade::)
(cd engine && cargo test -p bnto --test update_notice --test upgrade)
task tui:snapshots:update && git diff engine/crates/bnto/src/tui/snapshots && task tui:test
task wasm:lint && task cli:test
```

### Count changes

None to nodes or recipes. Two TUI snapshot files change.

---

## Dependency Chain

```
PR 1 (curated changelog + release gate)
├── PR 2 (/changelog page + Atom feed)          depends on PR 1
│   └── PR 3 (site "New in vX" badge)           depends on PR 2
└── PR 4 (bnto changelog)                       depends on PR 1
    └── PR 5 (announcements say what changed)   depends on PR 4; links to PR 2's page
```

PRs 2 and 4 can run in parallel. **Merge PR 2 before PR 5.** PR 5's links point at `bnto.io/changelog#<version>`, and bnto.io only deploys to production from a stable release (`deploy-production`, `release.yml:441-445`). A release that shipped PR 5 without PR 2 would link to a 404.

Once both have shipped, every stable release keeps the page current. The bump changes `engine/`, which counts as a web change (`release.yml:68`), so the site redeploys. `cargo-publish` and `deploy-production` both start after the release gate (l.444, l.517). The notice only appears after a once-a-day check sees the new crate, so the anchor is live by then in practice.

Release after PRs 1–5, so the next release is the first whose notes appear on the site, in the CLI and TUI, and in GitHub.

## Journeys

**Add to `.claude/journeys/cli.md`'s Journey Matrix,** after CJ9:

| #    | Journey                                  | Steps                                                                                                               | Pass Criteria                                                                                                                                                                          | Test / PR                                                                                                                                     |
| ---- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| CJ10 | Read what changed, offline               | `bnto changelog`, then `bnto changelog 0.18.0` and `bnto changelog v0.18.0`                                         | The running version's notes, then 0.18.0's, on stdout; exit 0; no network                                                                                                              | `cj10_*` in `tests/changelog_command.rs` (PR 4)                                                                                               |
| CJ11 | Ask for a version this bnto doesn't have | `bnto changelog 0.0.1`, then `bnto changelog 99.0.0`                                                                | Both exit 1 with nothing on stdout. The older one names the versions covered; the newer one gives `https://bnto.io/changelog#99.0.0` and `bnto upgrade`                                | `cj11_*` in `tests/changelog_command.rs` (PR 4)                                                                                               |
| CJ12 | The update notice links what changed     | With a cached 99.0.0, `bnto list` in a terminal; then `bnto upgrade --check`                                        | Both print the unchanged first line, then `What's new: https://bnto.io/changelog#99.0.0`. Opt-out, CI and piped runs print neither                                                     | `a_newer_cached_release_is_mentioned` in `tests/update_notice.rs`; `check_reports_a_newer_release_and_caches_it` in `tests/upgrade.rs` (PR 5) |
| CJ13 | `bnto upgrade` shows what changed        | `bnto upgrade` on a cargo install (fake cargo), with a new binary that has 99.0.0's notes and with one that doesn't | After "Upgraded …": the new binary's notes, or `What's new in 99.0.0: https://bnto.io/changelog#99.0.0` and never the child's error; exit 0 both ways. A failed install prints neither | `cj13_*` and `a_failed_cargo_install_says_so` in `tests/upgrade.rs` (PR 5)                                                                    |

**Update `.claude/journeys/tui.md`'s TJ10** (Upgrade notice, l.57). Its pass criteria become: the status line shows `bnto X: bnto upgrade · bnto changelog`. Its test, `tj10_a_newer_release_shows_in_the_status_line` (`tests/tui_journeys.rs:260-265`), waits for that text (PR 5).
