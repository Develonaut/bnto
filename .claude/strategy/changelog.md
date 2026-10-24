# Changelog: what changed, everywhere bnto shows up — 4 PRs

**Created:** October 24, 2026
**Status:** Decided by Ryan on October 24, 2026: a curated `CHANGELOG.md`, surfaced by a site badge, links from the CLI and TUI notices, and an RSS/Atom feed. Not started.
**Related:** [releases.md](../scopes/process/releases.md), [cli-upgrade.md](cli-upgrade.md), [feature-requests.md](../scopes/process/feature-requests.md) (this was evaluated as a request)

## Context

**The outcome:** anyone on bnto.io, or using the CLI or TUI, can see what changed in each release. Someone returning after a release learns there's something new.

**What exists today:**

- **No changelog file.**
- **GitHub Releases are auto-generated.** They hold the deploy status plus every merged PR's title, mostly internal work such as lint, refactors and plans.
- **The CLI and TUI announce new versions.** `update_check` covers that, but nothing says what changed.
- **The site shows the version** only in its install demo.
- **The best user-facing notes are buried.** They're the "What users get" sections hand-written in each version-bump PR (#737, #743).

**The web is in maintenance mode** (PLAN.md, the CLI/TUI-first pivot). This plan is a deliberate exception, approved by Ryan.

**What changes:**

- a new curated changelog, written in the bump PR;
- the GitHub Release body;
- a `/changelog` page with a feed;
- a "new" badge on the site;
- links from the CLI and TUI notices, and a new `bnto changelog` command.

**What doesn't change:** no node, recipe or processor counts. Cost stays at $0: everything is static at build time, and the badge state lives in each visitor's own browser storage.

Split into 4 PRs. PR 1 comes first. PRs 2 and 4 depend only on it and are independent of each other; PR 3 depends on PR 2.

---

## PR 1: A curated changelog, written in every release's bump PR

**Branch:** `feat/changelog-file` from `main`
**One sentence:** Each release gets a hand-written changelog entry, which becomes its GitHub Release body.

### What

The changelog follows the [Keep a Changelog](https://keepachangelog.com/) format: `## [0.18.2] - 2026-10-24`, then `### Added`, `### Changed`, `### Fixed` and `### Security`. It's written in plain words for users. Internal PRs are left out unless they change what a user sees.

- **Location:** the canonical file is `engine/crates/bnto/CHANGELOG.md`. It ships inside the published crate, so the CLI can read it offline (PR 4). The root `CHANGELOG.md` is a one-line pointer.
- **Backfill:** 0.18.0, 0.18.1 and 0.18.2, from their bump PRs and release notes.
- **Gate:** `task version:bump` adds an empty `## [X.Y.Z] - <date>` stub. `scripts/check-release-version.sh` (run by `task release:tag` and the release workflow) refuses a stable tag whose entry is missing or empty.
- **Release body:** `release.yml` puts the entry first, then GitHub's generated "All changes" list and the deploy status as now.

### Files (~3 new, ~4 modified)

**New:**

- `engine/crates/bnto/CHANGELOG.md`
- `CHANGELOG.md` (the pointer)
- `scripts/changelog-entry.sh`: prints one version's entry; shared by the check and `release.yml`

**Modified:**

- `scripts/bump-version.sh`
- `scripts/check-release-version.sh`
- `.github/workflows/release.yml` (release body, around l.720)
- `.claude/scopes/process/releases.md`

### Key function

`scripts/changelog-entry.sh <version>` prints the section between `## [<version>]` and the next `## [`, and exits 1 when it's missing or blank.

### RED tests

- `changelog-entry.sh 0.18.2` prints the 0.18.2 entry. An unknown version exits 1, and so does a stub with no items.
- `check-release-version.sh v9.9.9` fails, naming the missing entry, even when the workspace version matches.
- `bump-version.sh` adds a stub dated today, and adds it only once.

Shell scripts are tested with a bats or plain-bash test file under `scripts/tests/`, run by `task check`. If none exists, add `task check:scripts`.

### Verification

```bash
bash scripts/tests/changelog.sh && task check
```

### No count changes

---

## PR 2: A `/changelog` page and feed on bnto.io

**Branch:** `feat/web-changelog` from `main`, after PR 1
**One sentence:** bnto.io renders the changelog at `/changelog`, with an anchor per version and an Atom feed.

### What

The page is a statically generated server component. It reads `engine/crates/bnto/CHANGELOG.md` at build time, the same way the site already reads `engine/Cargo.toml` for the install demo (`apps/web/lib/getBntoCliVersion.ts`). A pure parser turns the file into versions, dates and sections. `/changelog.xml` is an Atom feed generated from the same parse.

The page is linked from the footer and listed in:

- the sitemap (`buildStaticSitemapEntries.ts`);
- Lighthouse (`scripts/generate-lighthouserc.ts`);
- `scripts/seo/collectUrls.ts`.

### Files (~6 new, ~5 modified)

**New:**

- `apps/web/lib/changelog/parseChangelog.ts` and its test
- `apps/web/app/(app)/changelog/page.tsx` and `layout.tsx` (metadata)
- `apps/web/app/changelog.xml/route.ts` (static)
- `apps/web/e2e/pages/changelog.spec.ts`

**Modified:**

- the sitemap, Lighthouse and `collectUrls` files above
- `FooterLinks.tsx`
- `site-navigation.spec.ts`

### Key function

```ts
export function parseChangelog(markdown: string): ChangelogRelease[]; // { version, date, sections: { kind, items[] }[] }
```

### RED tests

**Unit:**

- The three backfilled releases parse in order, newest first.
- Section kinds are preserved.
- An empty section is dropped.
- Malformed headings are skipped, not thrown on.

**E2E (`@browser`):**

- `/changelog` shows each version as a heading with an `id` anchor (`#0.18.2`).
- `not-found-heading` is absent.
- `/changelog.xml` is valid Atom with one entry per release.

### Verification

```bash
cd apps/web && pnpm exec vitest run lib/changelog && pnpm exec playwright test e2e/pages/changelog.spec.ts
task e2e && task seo:audit && task check
```

### Count changes

- Sitemap static entries: +1
- Lighthouse static pages: +1
- `collectUrls`: +1

None of these counts is asserted exactly.

---

## PR 3: A "New in vX" badge on bnto.io

**Branch:** `feat/web-new-release-badge` from `main`, after PR 2
**One sentence:** Returning visitors see a dismissible "New in vX →" badge linking to that version's changelog entry.

### What

The badge is a small client leaf in the site header. It reads the latest version at build time (from PR 2's parser, as a prop from a server parent).

- **When it shows:** the visitor's `localStorage` `bnto:last-seen-version` is older than the latest.
- **When it hides:** after the visitor follows it or dismisses it.
- **First-time visitors:** if they've never seen any version, it sets the key silently without showing, so the badge means "new since you were last here".
- **Storage:** every read and write is wrapped in try/catch, so blocked storage just means no badge.
- **Telemetry:** none beyond the existing pageview.

### Files (~3 new, ~2 modified)

**New:**

- `apps/web/components/blocks/NewReleaseBadge.tsx`
- `apps/web/lib/changelog/shouldShowBadge.ts` and its test

**Modified:**

- the header component
- an e2e spec

### RED tests

**Unit, `shouldShowBadge(lastSeen, latest)`:**

- no last-seen version → false;
- older → true;
- equal → false;
- a newer local value (from a pre-release) → false.

**E2E:**

- a seeded old version shows the badge;
- clicking it lands on `/changelog#<latest>`;
- after a reload, the badge is gone.

### Verification

```bash
task e2e && task check
```

Page-level screenshots are regenerated (two passes) if the header shifts.

### No count changes

---

## PR 4: The CLI and TUI point to what changed

**Branch:** `feat/cli-changelog` from `main`, after PR 1
**One sentence:** `bnto changelog` prints a version's notes from the changelog shipped in the binary, and the update notices link to them.

### What

- **`bnto changelog [version]`** reads `include_str!("../CHANGELOG.md")`. It defaults to the running version and works offline. `--all` lists every version.
- **The update notices** (`update_check/notice.rs`, and the TUI's) add `What's new: https://bnto.io/changelog#<version>`.
- **After a successful install,** `bnto upgrade` runs the new binary's `bnto changelog <new version>`, so the user sees the headline items immediately.

### Files (~2 new, ~5 modified)

**New:**

- `engine/crates/bnto/src/changelog.rs`
- `engine/crates/bnto/tests/changelog_command.rs` (sandboxed)

**Modified:**

- `cli.rs` and `dispatch.rs` (the command)
- `update_check/notice.rs`
- the TUI's notice render
- `upgrade/` (print after install)
- the crate README

### Key function

```rust
pub fn entry(changelog: &str, version: &str) -> Option<&str>
```

### RED tests

**Unit:**

- `entry` finds 0.18.2;
- an unknown version gives `None`;
- the last entry ends at end of file.

**Integration, with `Sandbox`:**

- `bnto changelog 0.18.2` prints its items and exits 0;
- an unknown version exits 1, naming the versions it knows;
- the update notice text contains the link (a unit test on `notice()`).

**TUI:** the snapshot of the screen showing the update notice updates in two passes.

### Verification

```bash
cd engine && cargo test -p bnto changelog && task tui:test && task check
```

### Count changes

None to nodes or recipes. `bnto --help` gains a command, so update any CLI help golden or snapshot.

---

## Dependency Chain

```
PR 1 (curated changelog + release gate)
├── PR 2 (/changelog page + Atom feed)   depends on PR 1
│   └── PR 3 (site "New in vX" badge)    depends on PR 2
└── PR 4 (bnto changelog + notice links) depends on PR 1
```

Release after PRs 1–4, so 0.18.3 (or the next minor) is the first release whose notes appear on the site, in the CLI and in GitHub.
