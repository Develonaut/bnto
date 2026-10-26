# Release Process

Tag-triggered release pipeline. Tag a commit on `main`, CI deploys a Vercel preview, runs the full test suite against it, and creates a GitHub Release when everything passes.

---

## How to Cut a Release

```bash
task version:bump -- 1.0.0   # in a PR: sets the version every bnto crate shares
task release:tag -- v1.0.0   # after that PR merges
```

`release:tag` checks out `main`, pulls latest, refuses a stable tag that doesn't match the engine release version, creates the tag, and pushes it. The release workflow starts automatically.

**Manual alternative:**

```bash
git checkout main && git pull origin main
git tag v1.0.0
git push origin v1.0.0
```

---

## What the Pipeline Does

```
Tag pushed (v*.*.*)
  │
  ├─ CI Gate (Rust fmt/clippy/test on Linux + Windows + TypeScript build/lint/test)
  │
  ├─ Deploy Vercel Preview (vercel deploy --prebuilt)
  │
  ├─ E2E Tests (browser project only — no auth/editor, max 3 failures)
  │
  ├─ Lighthouse (performance/a11y/best-practices + SEO as warn on preview)
  │
  ├─ Release Gate (quality check — all jobs must pass)
  │
  ├─ Deploy Vercel Production (separate build with prod env vars) — stable only
  │
  ├─ Deploy Convex (functions → production) — stable only
  │
  └─ Create GitHub Release (after production is live)
       └─ Pre-release if tag contains `-`
```

**Stable tags** deploy to production automatically — Vercel first, then Convex. The GitHub Release is created only after both deploys succeed, so "released" means "actually live."

**Pre-release tags** (beta/rc) skip production deploys. The GitHub Release is created after the release gate passes, marked as pre-release.

**Workflow file:** `.github/workflows/release.yml`

**WASM bundle:** the release's Rust job builds the web app's WASM with `wasm-pack build --release` and both Vercel deploys use that artifact, so production never ships the dev build that the lefthook pre-commit commits to `apps/web/public/wasm/`. Because the bundle comes from `engine/`, engine changes count as web changes for deploy gating.

**Change baseline:** a release is diffed against the newest stable tag with a published GitHub Release (the last one that actually shipped), not the previous tag, so a tag whose gate failed never hides its changes from the next one.

---

## Version Numbering

Follow [semver](https://semver.org/):

| Tag format      | Meaning            | GitHub Release        |
| --------------- | ------------------ | --------------------- |
| `v1.0.0`        | Stable release     | Full release          |
| `v1.0.0-beta.1` | Pre-release (beta) | Marked as pre-release |
| `v1.0.0-rc.1`   | Release candidate  | Marked as pre-release |

**Convention:**

- **MAJOR** — Breaking changes (reserved for post-1.0)
- **MINOR** — New features, new recipes, new capabilities
- **PATCH** — Bug fixes, performance improvements, dependency updates

Pre-1.0 (`v0.x.y`): minor = features, patch = fixes. Breaking changes are expected.

### One number everywhere

The release tag, the bnto crates on crates.io, `bnto --version` and the site's install demo all show the same version. (Decided October 21, 2026. Before that, tags versioned the site and the crates were bumped by hand: v0.16.0 shipped the site while crates.io stayed on 0.2.0.)

- **Source:** `[workspace.package] version` in `engine/Cargo.toml`. Every `bnto*` crate sets `version.workspace = true` and depends on its siblings through `[workspace.dependencies]` (enforced by `bnto-engine/tests/workspace_versions.rs`). tonkotsu, a standalone widget library, keeps its own version.
- **Bump:** `task version:bump -- X.Y.Z` (`scripts/bump-version.sh`) rewrites the workspace version and the sibling dependency versions, then refreshes `Cargo.lock`. Bump in its own PR before tagging.
- **Guard:** `scripts/check-release-version.sh` fails a stable tag that doesn't match. It runs in `task release:tag` and as the release workflow's first step, so crates.io can't silently skip a release ("already exists"). Pre-release tags aren't checked; they don't publish crates.
- **Crates with their own version (tonkotsu).** They don't move with `task version:bump`. When one changes, bump its `version` in its own `Cargo.toml` and in `engine/Cargo.toml` `[workspace.dependencies]`, in the same PR. `scripts/check-own-version-crates.sh` fails `task check` (`check:crates`), CI (the Crate Publish List job) and the release when such a crate's packaged files differ from the version crates.io already has. Without an answer from crates.io it warns and skips locally, and fails in CI and the release. v0.17.0 shipped without this: tonkotsu gained `focused_field_lines` under 0.1.1, the release skipped it as already published, and `bnto` 0.17.0 failed to build against the published tonkotsu, so only the library crates reached crates.io. v0.17.1 bumped tonkotsu to 0.1.2.
- **Every release ships web and CLI together.** A recipe, node or page that isn't ready for bnto.io stays behind a feature flag ([feature-flags.md](../web/feature-flags.md)) rather than waiting for a separate web release.

---

## Promoting to Production

For **stable tags**, deployment is automatic — the release pipeline builds a separate production deployment (with production env vars from Vercel Dashboard) and deploys Convex functions after the release gate passes.

For **pre-release tags** (beta/rc), no production deploy happens. If you need to manually promote a preview:

1. **Vercel Dashboard** — Find the preview deployment, click "Promote to Production"
2. **Or via CLI** — `vercel promote <preview-url> --token=... --scope=<org>`

---

## Rollback

If a production deploy has issues:

1. **Vercel instant rollback** — Vercel Dashboard > Deployments > find the previous production deployment > "Promote to Production"
2. **Or via CLI** — `vercel rollback --token=... --scope=<org>`

Vercel keeps all previous deployments. Rollback is instant (DNS swap, no rebuild).

For Convex rollbacks, redeploy the previous commit's functions:

```bash
git checkout <previous-tag>
cd packages/@bnto/backend && npx convex deploy --yes
```

---

## Hotfix Workflow

1. Fix the bug on `main` via normal PR flow
2. After merge, tag the fix: `task release:tag -- v1.0.1`
3. Pipeline runs — preview + E2E + Lighthouse + auto-deploy to production

No cherry-picking or release branches needed. `main` is always the source of truth.

---

## Required Secrets

| Secret                            | Source                                          | Purpose                              |
| --------------------------------- | ----------------------------------------------- | ------------------------------------ |
| `VERCEL_TOKEN`                    | Vercel Dashboard > Settings > Tokens            | Authenticate CLI in CI               |
| `VERCEL_ORG_ID`                   | `.vercel/project.json` after `vercel link`      | Identify Vercel org                  |
| `VERCEL_PROJECT_ID`               | `.vercel/project.json` after `vercel link`      | Identify Vercel project              |
| `GITHUB_TOKEN`                    | Auto-provided by GitHub Actions                 | Create GitHub Releases               |
| `LHCI_GITHUB_APP_TOKEN`           | Already configured                              | Lighthouse CI                        |
| `VERCEL_AUTOMATION_BYPASS_SECRET` | Vercel Project > Deployment Protection > Bypass | Bypass protection for E2E/Lighthouse |

Setup instructions in [environment-variables.md](../../environment-variables.md).

---

## Monitoring a Release

Watch the workflow: `https://github.com/bntoio/bnto/actions`

When it finishes, check the release actually shipped:

```bash
task release:verify -- 1.0.0   # no v
```

The `release-verify` dogfood recipe (`recipes/release-verify.bnto.json`) checks three things and reports each one even after another fails:

- crates.io serves that version of every crate a release publishes;
- a published GitHub Release (not a draft or prerelease) exists for the tag;
- www.bnto.io's install demo shows the version.

It isn't part of the release workflow yet; it runs by hand until it has proven itself.

Artifacts uploaded on every release:

- `e2e-report-<tag>` — Full Playwright HTML report (14-day retention)
- `e2e-results-<tag>` — Test result traces (14-day retention)
- Lighthouse results — Uploaded to temporary public storage

---

## Environment Variables

`NEXT_PUBLIC_*` values are managed in the **Vercel Dashboard** (Project Settings > Environment Variables), scoped per environment:

| Variable                      | Preview                    | Production                     |
| ----------------------------- | -------------------------- | ------------------------------ |
| `NEXT_PUBLIC_CONVEX_URL`      | `zealous-canary-422` (dev) | `gregarious-donkey-712` (prod) |
| `NEXT_PUBLIC_CONVEX_SITE_URL` | `zealous-canary-422` (dev) | `gregarious-donkey-712` (prod) |

The release workflow uses `vercel pull --environment=preview` and `vercel pull --environment=production` to fetch the correct values at build time. No Convex URLs are hardcoded in the workflow file.

To update: Vercel Dashboard > bnto-web > Settings > Environment Variables.

---

## Rules

1. **Only tag `main`.** Never tag a feature branch.
2. **Tags are immutable.** Once pushed, don't delete and recreate. If a release is broken, fix forward with a new patch tag.
3. **CI must pass before deploy.** The release gate is the quality bar — production deploys only happen after it passes.
4. **Deploy first, release second.** Stable tags auto-deploy Vercel + Convex to production. The GitHub Release is created only after both succeed. Pre-release tags skip production deploys.
