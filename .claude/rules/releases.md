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
  ├─ Release Gate
  │    ├─ All jobs must pass
  │    └─ Creates GitHub Release (pre-release if tag contains `-`)
  │
  └─ Convex Deploy (prod) — stable releases only, skipped for beta/rc
```

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

Promotion is manual. After the release gate passes:

1. **Vercel Dashboard** — Find the preview deployment, click "Promote to Production"
2. **Or via CLI** — `vercel promote <preview-url> --token=... --scope=<org>`

Convex production deploys happen as part of this pipeline (stable tags only — skipped for beta/rc).

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
3. Pipeline runs — preview + E2E + Lighthouse
4. Promote to production when green

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

## Rules

1. **Only tag `main`.** Never tag a feature branch.
2. **Tags are immutable.** Once pushed, don't delete and recreate. If a release is broken, fix forward with a new patch tag.
3. **CI must pass before promotion.** The release gate is the quality bar.
4. **Convex deploys with the release.** Convex production functions deploy after the release gate passes (stable tags only — skipped for beta/rc). Pre-release tags verify against dev Convex.
