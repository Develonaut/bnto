# Release Process

Tag-triggered release pipeline. Tag a commit on `main`, CI deploys a Vercel preview, runs the full test suite against it, and creates a GitHub Release when everything passes.

---

## How to Cut a Release

```bash
task release:tag -- v1.0.0
```

This checks out `main`, pulls latest, creates the tag, and pushes it. The release workflow starts automatically.

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
  ├─ CI Gate (Rust fmt/clippy/test + TypeScript build/lint/test)
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

Setup instructions in [environment-variables.md](../environment-variables.md).

---

## Monitoring a Release

Watch the workflow: `https://github.com/bntoio/bnto/actions`

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
