# Environment Variables

All environment variables used across the bnto stack, where they're configured, and what they do.

---

## Convex Dashboard (Server-Side Secrets)

Set in the [Convex Dashboard](https://dashboard.convex.dev) under Settings > Environment Variables. These run server-side in Convex functions and are never exposed to the browser.

**Set per-environment** — dev and prod deployments each have their own env vars.

### Auth (`@convex-dev/auth`)

| Variable          | Purpose                                                                                        | Status                       |
| ----------------- | ---------------------------------------------------------------------------------------------- | ---------------------------- |
| `JWT_PRIVATE_KEY` | Private key for signing JWTs (`@convex-dev/auth`)                                              | **Dev: set** / **Prod: set** |
| `JWKS`            | JSON Web Key Set for JWT verification (`@convex-dev/auth`)                                     | **Dev: set** / **Prod: set** |
| `SITE_URL`        | Base URL for auth redirects (e.g. `http://localhost:4000` for dev, `https://bnto.io` for prod) | **Dev: set** / **Prod: set** |

### Auth Providers (deferred — not yet enabled in code)

| Variable              | Purpose                     | Status  |
| --------------------- | --------------------------- | ------- |
| `AUTH_GOOGLE_ID`      | Google OAuth client ID      | Not set |
| `AUTH_GOOGLE_SECRET`  | Google OAuth client secret  | Not set |
| `AUTH_DISCORD_ID`     | Discord OAuth client ID     | Not set |
| `AUTH_DISCORD_SECRET` | Discord OAuth client secret | Not set |

### R2 Storage (Transit Layer)

Used by `convex/_helpers/r2_client.ts`, `convex/uploads.ts`, and `convex/downloads.ts` to generate presigned URLs for file transit. R2 is a temp transit layer with 1-hour TTL, not permanent storage.

| Variable               | Purpose                                                                                        | Status                                                             |
| ---------------------- | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `R2_ACCOUNT_ID`        | Cloudflare account ID (used to build the S3-compatible endpoint)                               | **Dev: set** / **Prod: set**                                       |
| `R2_ACCESS_KEY_ID`     | R2 API token key ID (Object Read & Write)                                                      | **Dev: set** / **Prod: set**                                       |
| `R2_SECRET_ACCESS_KEY` | R2 API token secret                                                                            | **Dev: set** / **Prod: set**                                       |
| `R2_BUCKET_NAME`       | Bucket name — defaults to `"bnto-transit"` if unset. **Use separate buckets per environment.** | **Dev: set** (`bnto-transit-dev`) / **Prod: set** (`bnto-transit`) |

**Per-environment buckets:** Dev and prod should use separate R2 buckets to prevent test uploads from polluting production storage.

| Environment | `R2_BUCKET_NAME`   | Notes                       |
| ----------- | ------------------ | --------------------------- |
| Dev         | `bnto-transit-dev` | Development/testing uploads |
| Prod        | `bnto-transit`     | Production uploads only     |

### Cloud Execution (M4, archived)

> **Note:** The Go API on Railway was used for cloud execution during M1 development. It is now archived (`archive/` deleted Feb 2026). The `GO_API_URL` env var can be removed from Convex deployments. Cloud execution technology for M4 is TBD.

---

## Vercel (Frontend)

Set in the [Vercel Dashboard](https://vercel.com) under Project > Settings > Environment Variables. Also in `apps/web/.env.local` for local dev.

**Set per-environment** — Vercel supports Development, Preview, and Production scopes.

| Variable                      | Purpose                                                                                                                                   |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `NEXT_PUBLIC_CONVEX_URL`      | Convex deployment URL (e.g. `https://zealous-canary-422.convex.cloud`)                                                                    |
| `NEXT_PUBLIC_CONVEX_SITE_URL` | Convex site URL for auth callbacks (e.g. `https://zealous-canary-422.convex.site`)                                                        |
| `NEXT_PUBLIC_POSTHOG_KEY`     | PostHog project API key (e.g. `phc_...`). **Production only** — not set in `.env.local` to avoid polluting PostHog with localhost events. |
| `NEXT_PUBLIC_POSTHOG_HOST`    | PostHog ingest path — set to `/ingest` (reverse proxy via Next.js rewrites). **Production only** — not set in `.env.local`.               |

**Dev values** (in `apps/web/.env.local`):

- `NEXT_PUBLIC_CONVEX_URL=https://zealous-canary-422.convex.cloud`
- `NEXT_PUBLIC_CONVEX_SITE_URL=https://zealous-canary-422.convex.site`
- PostHog vars intentionally omitted — telemetry is a silent no-op in dev

**Prod values** (in Vercel):

- `NEXT_PUBLIC_CONVEX_URL` / `NEXT_PUBLIC_CONVEX_SITE_URL` — production Convex deployment URLs
- `NEXT_PUBLIC_POSTHOG_KEY` — set in Vercel, get from PostHog project settings
- `NEXT_PUBLIC_POSTHOG_HOST` — set to `/ingest` in Vercel (reverse proxy — bypasses ad blockers)

---

---

## Private Docs Path

Set in the **root** `.env.local` (not `apps/web/.env.local`). Points to a local directory containing private business docs (pricing strategy, competitive analysis, revenue projections, etc.) that are NOT committed to the repo.

| Variable                 | Purpose                                                                                                       | Status                 |
| ------------------------ | ------------------------------------------------------------------------------------------------------------- | ---------------------- |
| `BNTO_PRIVATE_DOCS_PATH` | Absolute path to private business docs directory. Agents read from this path when business context is needed. | **Set** (Google Drive) |

---

## Local Dev Only

Auto-generated by Convex CLI in `packages/@bnto/backend/.env.local`. Not used in application code — only by Convex dev tooling.

| Variable            | Purpose                                                          |
| ------------------- | ---------------------------------------------------------------- |
| `CONVEX_DEPLOYMENT` | Convex CLI deployment identifier (e.g. `dev:zealous-canary-422`) |
| `CONVEX_URL`        | Convex deployment URL (used by CLI)                              |
| `CONVEX_SITE_URL`   | Convex site URL (used by CLI)                                    |

---

## GitHub Actions (CI/CD)

| Variable                          | Where                 | Purpose                                                                                                                                                                                                | Status                |
| --------------------------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------- |
| `CI`                              | GitHub Actions        | Auto-set by runner, enables retries in Playwright                                                                                                                                                      | Auto-injected         |
| `NEXT_PUBLIC_CONVEX_URL`          | CI workflow           | Set to dev deployment URL for build-only checks                                                                                                                                                        | Hardcoded in workflow |
| `CONVEX_DEPLOY_KEY`               | GitHub Actions secret | Production deploy key for `convex deploy`. Generated from the [Convex Dashboard](https://dashboard.convex.dev) > Settings > Deploy Keys. Only used on merge to `main`.                                 | **Required**          |
| `LHCI_GITHUB_APP_TOKEN`           | GitHub Actions secret | Lighthouse CI GitHub App token for PR status badges. Get from [Lighthouse CI App](https://github.com/apps/lighthouse-ci) authorization.                                                                | **Set**               |
| `VERCEL_TOKEN`                    | GitHub Actions secret | Vercel CLI authentication for preview deployments in the release pipeline. Create at Vercel Dashboard > Settings > Tokens.                                                                             | **Required**          |
| `VERCEL_ORG_ID`                   | GitHub Actions secret | Vercel org identifier. Found in `.vercel/project.json` after running `vercel link` in `apps/web/`.                                                                                                     | **Required**          |
| `VERCEL_PROJECT_ID`               | GitHub Actions secret | Vercel project identifier. Found in `.vercel/project.json` after running `vercel link` in `apps/web/`.                                                                                                 | **Required**          |
| `PLAYWRIGHT_BASE_URL`             | CI workflow           | Overrides Playwright `baseURL` to test against a Vercel preview instead of localhost. Only set in `release.yml`.                                                                                       | Set in workflow       |
| `VERCEL_AUTOMATION_BYPASS_SECRET` | GitHub Actions secret | Bypasses Vercel Deployment Protection for automated E2E and Lighthouse tests against preview deployments. Set in Vercel Project > Settings > Deployment Protection > Protection Bypass for Automation. | **Required**          |
| `INDEXNOW_KEY`                    | GitHub Actions secret | Self-generated key for IndexNow protocol (Bing, Yandex). Also served as a verification file at `/{key}.txt`.                                                                                           | Not set               |
| `GOOGLE_INDEXING_KEY`             | GitHub Actions secret | GCP service account JSON key for Google Indexing API v3. The service account email must be added as Owner in Google Search Console.                                                                    | Not set               |

### Setting up Search Engine Indexing Secrets

| Secret                | How to generate                                                                                                                                                       |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `INDEXNOW_KEY`        | Generate: `uuidgen \| tr -d '-' \| tr '[:upper:]' '[:lower:]'`. Add as GitHub Actions secret. The release pipeline writes `{key}.txt` to `apps/web/public/`.          |
| `GOOGLE_INDEXING_KEY` | GCP Console: create a service account, enable Indexing API, download JSON key. Add the service account email as Owner in Google Search Console. Paste JSON as secret. |

### Setting up Vercel Secrets (Release Pipeline)

1. Install Vercel CLI locally: `pnpm add -g vercel`
2. Run `vercel link` in `apps/web/` to connect the project (creates `.vercel/project.json`)
3. Get credentials:
   - `VERCEL_TOKEN`: Vercel Dashboard > Settings > Tokens > Create
   - `VERCEL_ORG_ID`: From `.vercel/project.json` → `orgId`
   - `VERCEL_PROJECT_ID`: From `.vercel/project.json` → `projectId`
4. Add all three as GitHub repo secrets: [GitHub Settings > Secrets](https://github.com/bntoio/bnto/settings/secrets/actions)

### Setting up `CONVEX_DEPLOY_KEY`

1. Go to [Convex Dashboard](https://dashboard.convex.dev) > select the **production** deployment (`gregarious-donkey-712`)
2. Navigate to **Settings > Deploy Keys**
3. Create a new **Production Deploy Key**
4. Copy the key value
5. Go to [GitHub repo Settings](https://github.com/bntoio/bnto/settings/secrets/actions) > **Secrets and variables > Actions**
6. Add a new repository secret named `CONVEX_DEPLOY_KEY` with the key value

---

## Where to Configure

| Context                        | Location                                                                                   |
| ------------------------------ | ------------------------------------------------------------------------------------------ |
| Local development (frontend)   | `apps/web/.env.local` for `NEXT_PUBLIC_*` vars                                             |
| Local development (Convex CLI) | `packages/@bnto/backend/.env.local` (auto-generated)                                       |
| Convex functions (dev)         | [Convex Dashboard](https://dashboard.convex.dev) > dev deployment > Environment Variables  |
| Convex functions (prod)        | [Convex Dashboard](https://dashboard.convex.dev) > prod deployment > Environment Variables |
| Production frontend            | [Vercel Dashboard](https://vercel.com) > Project > Environment Variables                   |
| CI/CD                          | `.github/workflows/` (when created)                                                        |

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

### Convex Prod Deployment

- [x] `JWT_PRIVATE_KEY` (unique prod key)
- [x] `JWKS` (unique prod keyset)
- [x] `SITE_URL` (`https://bnto.io`)
- [x] `R2_ACCOUNT_ID`
- [x] `R2_ACCESS_KEY_ID`
- [x] `R2_SECRET_ACCESS_KEY`
- [x] `R2_BUCKET_NAME` (`bnto-transit`)

### Vercel

- [x] `NEXT_PUBLIC_CONVEX_URL` — dev/preview: `zealous-canary-422`, production: `gregarious-donkey-712`
- [x] `NEXT_PUBLIC_CONVEX_SITE_URL` — dev/preview: `zealous-canary-422`, production: `gregarious-donkey-712`
