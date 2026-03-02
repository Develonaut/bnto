# Environment Variables

All environment variables used across the bnto stack, where they're configured, and what they do.

---

## Convex Dashboard (Server-Side Secrets)

Set in the [Convex Dashboard](https://dashboard.convex.dev) under Settings > Environment Variables. These run server-side in Convex functions and are never exposed to the browser.

**Set per-environment** — dev and prod deployments each have their own env vars.

### Auth (`@convex-dev/auth`)

| Variable | Purpose | Status |
|---|---|---|
| `JWT_PRIVATE_KEY` | Private key for signing JWTs (`@convex-dev/auth`) | **Dev: set** / **Prod: set** |
| `JWKS` | JSON Web Key Set for JWT verification (`@convex-dev/auth`) | **Dev: set** / **Prod: set** |
| `SITE_URL` | Base URL for auth redirects (e.g. `http://localhost:4000` for dev, `https://bnto.io` for prod) | **Dev: set** / **Prod: set** |

### Auth Providers (deferred — not yet enabled in code)

| Variable | Purpose | Status |
|---|---|---|
| `AUTH_GOOGLE_ID` | Google OAuth client ID | Not set |
| `AUTH_GOOGLE_SECRET` | Google OAuth client secret | Not set |
| `AUTH_DISCORD_ID` | Discord OAuth client ID | Not set |
| `AUTH_DISCORD_SECRET` | Discord OAuth client secret | Not set |

### R2 Storage (Transit Layer)

Used by `convex/_helpers/r2_client.ts`, `convex/uploads.ts`, and `convex/downloads.ts` to generate presigned URLs for file transit. R2 is a temp transit layer with 1-hour TTL, not permanent storage.

| Variable | Purpose | Status |
|---|---|---|
| `R2_ACCOUNT_ID` | Cloudflare account ID (used to build the S3-compatible endpoint) | **Dev: set** / **Prod: set** |
| `R2_ACCESS_KEY_ID` | R2 API token key ID (Object Read & Write) | **Dev: set** / **Prod: set** |
| `R2_SECRET_ACCESS_KEY` | R2 API token secret | **Dev: set** / **Prod: set** |
| `R2_BUCKET_NAME` | Bucket name — defaults to `"bnto-transit"` if unset. **Use separate buckets per environment.** | **Dev: set** (`bnto-transit-dev`) / **Prod: set** (`bnto-transit`) |

**Per-environment buckets:** Dev and prod should use separate R2 buckets to prevent test uploads from polluting production storage.

| Environment | `R2_BUCKET_NAME` | Notes |
|---|---|---|
| Dev | `bnto-transit-dev` | Development/testing uploads |
| Prod | `bnto-transit` | Production uploads only |

### Cloud Execution

| Variable | Purpose | Status |
|---|---|---|
| `GO_API_URL` | Go API server URL. Dev: Cloudflare tunnel to localhost. Prod: Railway. | **Dev: set** (`https://api-dev.bnto.io`) / **Prod: set** (`https://bnto-production.up.railway.app`) |
| `ANONYMOUS_RUN_LIMIT` | **Server-side only.** Max server-node executions for anonymous users (defaults to `3` if unset). Browser executions are unlimited — see [pricing-model.md](strategy/pricing-model.md). | Not set (using default) |
| `FREE_PLAN_RUN_LIMIT` | **Server-side only.** Max server-node executions/month for free-tier users (defaults to `25` if unset). Browser executions are unlimited. | Not set (using default) |

---

## Vercel (Frontend)

Set in the [Vercel Dashboard](https://vercel.com) under Project > Settings > Environment Variables. Also in `apps/web/.env.local` for local dev.

**Set per-environment** — Vercel supports Development, Preview, and Production scopes.

| Variable | Purpose |
|---|---|
| `NEXT_PUBLIC_CONVEX_URL` | Convex deployment URL (e.g. `https://zealous-canary-422.convex.cloud`) |
| `NEXT_PUBLIC_CONVEX_SITE_URL` | Convex site URL for auth callbacks (e.g. `https://zealous-canary-422.convex.site`) |
| `NEXT_PUBLIC_POSTHOG_KEY` | PostHog project API key (e.g. `phc_...`). **Production only** — not set in `.env.local` to avoid polluting PostHog with localhost events. |
| `NEXT_PUBLIC_POSTHOG_HOST` | PostHog ingest host (e.g. `https://us.i.posthog.com`). **Production only** — not set in `.env.local`. |

**Dev values** (in `apps/web/.env.local`):
- `NEXT_PUBLIC_CONVEX_URL=https://zealous-canary-422.convex.cloud`
- `NEXT_PUBLIC_CONVEX_SITE_URL=https://zealous-canary-422.convex.site`
- PostHog vars intentionally omitted — telemetry is a silent no-op in dev

**Prod values** (in Vercel):
- `NEXT_PUBLIC_CONVEX_URL` / `NEXT_PUBLIC_CONVEX_SITE_URL` — production Convex deployment URLs
- `NEXT_PUBLIC_POSTHOG_KEY=phc_MM2CZwPL8RWy5nJVH6RyrG5sWnHbtKTxu0aofvDWDKF`
- `NEXT_PUBLIC_POSTHOG_HOST=https://us.i.posthog.com`

---

## Railway (Go API Server)

Set in the [Railway Dashboard](https://railway.com) under Service > Variables.

**Project:** `bnto` (ID: `12cb77eb-df37-4efb-83ef-6c5d49cde433`)
**Service:** `bnto` (ID: `ecc01652-5321-4bb3-9ac6-4f17f0874ec1`)
**URL:** `https://bnto-production.up.railway.app`

| Variable | Purpose | Status |
|---|---|---|
| `PORT` | Server listening port (defaults to `8080`, Railway sets automatically) | Auto-injected |
| `R2_ACCOUNT_ID` | Cloudflare account ID (same as Convex) | **Set** |
| `R2_ACCESS_KEY_ID` | R2 API token key ID | **Set** |
| `R2_SECRET_ACCESS_KEY` | R2 API token secret | **Set** |
| `R2_BUCKET_NAME` | R2 bucket for file transit | **Set** (`bnto-transit`) |

---

## Cloudflare Tunnel (Local Dev → Cloud)

A named Cloudflare tunnel exposes the local Go API server (`localhost:8080`) at `https://api-dev.bnto.io`. This lets Convex dev functions call the Go API the same way Convex prod calls Railway — no code changes between environments.

**Tunnel:** `bnto-dev`
**URL:** `https://api-dev.bnto.io` → `localhost:8080`
**Config:** `~/.cloudflared/config.yml` (local only — credentials never committed)

### Usage

```bash
# Terminal 1: Start Go API locally
task api:dev

# Terminal 2: Start tunnel
task api:tunnel
```

The tunnel requires `cloudflared` (`brew install cloudflared`) and a one-time login (`cloudflared tunnel login`).

### How it fits

| Environment | `GO_API_URL` in Convex | Go API runs on |
|---|---|---|
| Dev | `https://api-dev.bnto.io` | `localhost:8080` via Cloudflare tunnel |
| Prod | `https://bnto-production.up.railway.app` | Railway (Docker) |

---

## Local Dev Only

Auto-generated by Convex CLI in `packages/@bnto/backend/.env.local`. Not used in application code — only by Convex dev tooling.

| Variable | Purpose |
|---|---|
| `CONVEX_DEPLOYMENT` | Convex CLI deployment identifier (e.g. `dev:zealous-canary-422`) |
| `CONVEX_URL` | Convex deployment URL (used by CLI) |
| `CONVEX_SITE_URL` | Convex site URL (used by CLI) |

---

## Test / CI

| Variable | Where | Purpose |
|---|---|---|
| `CI` | GitHub Actions | Auto-set by runner, enables retries in Playwright |
| `NEXT_PUBLIC_CONVEX_URL` | CI workflow | Set to placeholder for build-only checks |

---

## Where to Configure

| Context | Location |
|---|---|
| Local development (frontend) | `apps/web/.env.local` for `NEXT_PUBLIC_*` vars |
| Local development (Convex CLI) | `packages/@bnto/backend/.env.local` (auto-generated) |
| Convex functions (dev) | [Convex Dashboard](https://dashboard.convex.dev) > dev deployment > Environment Variables |
| Convex functions (prod) | [Convex Dashboard](https://dashboard.convex.dev) > prod deployment > Environment Variables |
| Production frontend | [Vercel Dashboard](https://vercel.com) > Project > Environment Variables |
| Go API server | [Railway Dashboard](https://railway.com) > Service > Variables |
| CI/CD | `.github/workflows/` (when created) |

---

## Setup Checklist

### R2 Credentials (Cloudflare)

1. Go to [Cloudflare Dashboard](https://dash.cloudflare.com) > R2 > Manage R2 API Tokens
2. Create an API token with **Object Read & Write** permissions scoped to the bnto buckets
3. Note the **Access Key ID** and **Secret Access Key**
4. Get your **Account ID** from the Cloudflare dashboard URL or R2 overview page
5. Set `R2_ACCOUNT_ID`, `R2_ACCESS_KEY_ID`, `R2_SECRET_ACCESS_KEY` in the Convex dashboard for each environment
6. Set `R2_BUCKET_NAME` per environment (`bnto-transit-dev` for dev, `bnto-transit` for prod)

### Convex Dev Deployment

Currently set:
- [x] `JWT_PRIVATE_KEY`
- [x] `JWKS`
- [x] `SITE_URL` (`http://localhost:4000`)
- [x] `R2_ACCOUNT_ID`
- [x] `R2_ACCESS_KEY_ID`
- [x] `R2_SECRET_ACCESS_KEY`
- [x] `R2_BUCKET_NAME` (`bnto-transit-dev`)
- [x] `GO_API_URL` (`https://api-dev.bnto.io`) — Cloudflare tunnel to local Go API

### Convex Prod Deployment

- [x] `JWT_PRIVATE_KEY` (unique prod key)
- [x] `JWKS` (unique prod keyset)
- [x] `SITE_URL` (`https://bnto.io`)
- [x] `R2_ACCOUNT_ID`
- [x] `R2_ACCESS_KEY_ID`
- [x] `R2_SECRET_ACCESS_KEY`
- [x] `R2_BUCKET_NAME` (`bnto-transit`)
- [x] `GO_API_URL` (`https://bnto-production.up.railway.app`)

### Vercel

- [x] `NEXT_PUBLIC_CONVEX_URL` — dev/preview: `zealous-canary-422`, production: `gregarious-donkey-712`
- [x] `NEXT_PUBLIC_CONVEX_SITE_URL` — dev/preview: `zealous-canary-422`, production: `gregarious-donkey-712`

### Railway

- [x] Project linked: `bnto` (production environment)
- [x] `PORT` — auto-injected by Railway
- [x] `R2_ACCOUNT_ID`, `R2_ACCESS_KEY_ID`, `R2_SECRET_ACCESS_KEY` — set for R2 file transit
- [x] `R2_BUCKET_NAME` (`bnto-transit`) — prod bucket
