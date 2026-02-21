# Deployment Strategy

**Last Updated:** February 2026
**Status:** Active

---

## Guiding Principle

Backend and frontend deploy independently but must be coordinated. Convex schema changes must land before the frontend that depends on them. Work happens on feature branches; merging to `main` triggers production frontend deployment via Vercel.

---

## Infrastructure

| Layer | Service | Deploy Trigger |
|---|---|---|
| Database/Backend | Convex Cloud | Manual: `npx convex deploy --prod` |
| Web Frontend | Vercel | Auto: merge to `main` |
| Go API Server | Railway | Manual: deploy on release (Phase 3) |
| Previews | Vercel | Auto: push to feature branch |
| Desktop | Wails v2 build | Manual: release binary (Phase 3) |

---

## Branch Workflow

```
feature branch -> push -> Vercel preview deploy (automatic)
                       -> test against Convex dev deployment
                       -> iterate until ready

ready to ship  -> npx convex deploy --prod   (1. backend first)
               -> merge to main              (2. triggers Vercel production deploy)
```

Feature branches get automatic Vercel preview deployments. This is the standard Vercel Git flow -- `main` is always deployable, and anything that lands on it goes to production.

---

## Release Checklist

### Standard Release (backend + frontend changes)

```bash
# 1. Deploy backend first -- additive schema changes are safe to deploy early
npx convex deploy --prod

# 2. Merge feature branch to main -- Vercel auto-deploys to production
git checkout main && git merge feature-branch && git push
```

**Why backend first:** New tables, fields, and functions can exist before the frontend uses them. But the frontend can't reference things that don't exist yet. Deploying backend first is always safe; deploying frontend first can cause runtime errors.

### Frontend-Only Release

Just merge to `main`. No backend step needed.

### Backend-Only Release

```bash
npx convex deploy --prod
```

No merge needed if frontend doesn't change.

### Go Engine Release (Phase 3+)

```bash
# 1. Build and test engine
task check

# 2. Deploy API server to Railway
# (Railway deployment process TBD -- likely git push to Railway remote or Docker deploy)

# 3. Desktop: build Wails binary
task desktop:build
```

The Go engine powers both the Railway-hosted API server (for cloud execution) and the Wails desktop app (for local execution). Both consume the same engine package.

---

## Convex Deploy Safety

Convex validates all existing documents against the new schema on deploy. This means:

- **Adding a new table** -- always safe
- **Adding a new optional field** -- always safe
- **Adding a new required field** -- will fail if existing docs don't have it
- **Removing/renaming a field** -- will fail if existing docs still have it

For breaking schema changes, use a migration pattern: add new field as optional, backfill existing docs, then make required.

---

## Preview Environments

Vercel automatically creates preview deployments for every push to a non-main branch. These preview URLs are useful for testing UI changes before merging.

Preview deploys currently talk to the **Convex dev deployment** (via `NEXT_PUBLIC_CONVEX_URL` set per environment in Vercel). This means preview testing uses dev data, not production data.

---

## Environment Matrix

| Environment | Web | Backend | Go Engine | Purpose |
|---|---|---|---|---|
| Local dev | `next dev` | Convex dev | `task build && ./bnto` | Development |
| Preview | Vercel preview URL | Convex dev | N/A | PR review |
| Production | Vercel (main) | Convex prod | Railway (Phase 3) | Live users |
| Desktop | Wails webview | N/A (local engine) | Bundled binary | Local execution |

---

## Cost

| Service | Tier | Cost |
|---|---|---|
| Convex Cloud | Free tier | $0 |
| Vercel | Hobby | $0 |
| Railway | Hobby (Phase 3) | $5/mo (when needed) |
| Preview deployments | Included in Vercel | $0 |

No CI/CD pipeline costs since backend deploys are triggered locally. Railway costs begin only when the Go API server is deployed for cloud execution in Phase 3.
