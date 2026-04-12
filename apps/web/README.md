# @bnto/web

Next.js web application. The primary frontend for bnto.

## Overview

The web app is a Next.js App Router application deployed on Vercel. It composes UI from `@bnto/ui` and data access from `@bnto/core`. Static tool pages are SEO-optimized with per-slug metadata and JSON-LD.

## Directory Structure

```
app/
├── layout.tsx              # Root layout (fonts, theme, global providers)
├── not-found.tsx           # Branded 404 page
├── providers/              # BntoCoreProvider wrapper (Convex + auth + React Query)
├── (app)/                  # Main app shell
│   ├── layout.tsx          # App shell (header + main)
│   ├── page.tsx            # Home page (/)
│   ├── [bnto]/             # Dynamic tool pages (/compress-images, /clean-csv, etc.)
│   ├── pricing/            # Pricing page
│   └── privacy/            # Privacy policy
├── (dev)/                  # Dev-only routes (component showcase)
└── editor/                 # Recipe editor (beta)
components/
├── blocks/                 # Business components (Navbar, Footer, RecipeGrid)
├── ThemeProvider.tsx        # next-themes wrapper
└── useTheme.ts             # Theme hook
e2e/                        # Playwright E2E tests
├── fixtures/               # Custom fixtures (enhanced page, error capture)
├── helpers/                # Shared helpers (upload, run, download, assertions)
├── journeys/               # User journey tests (browser, editor)
├── pages/                  # Page-level screenshot tests
└── editor/                 # Editor component tests
lib/
├── bntoRegistry.ts         # SEO slug registry (single source of truth)
├── routes.ts               # Route definitions (ROUTES, editorUrl)
└── stores/                 # App-level Zustand stores
proxy.ts                    # URL normalization middleware
```

## Routing

| Route                                  | Description                        |
| -------------------------------------- | ---------------------------------- |
| `/`                                    | Home - tool grid gallery           |
| `/compress-images`, `/clean-csv`, etc. | Tool pages - SEO-optimized, static |
| `/editor`                              | Visual recipe editor (beta)        |
| `/explore`                             | Recipe & node browser              |
| `/pricing`                             | Pricing info                       |
| `/motorway`                            | Component showcase (dev only)      |

Tool page slugs are registered in `lib/bntoRegistry.ts`. The `[bnto]` dynamic segment matches root-level slugs; unknown slugs return a real 404.

## Development

```bash
task dev              # Start Next.js (port 4000) + Convex dev server
task ui:build         # Production build (all packages)
task ui:test          # Run unit tests
task ui:lint          # Lint
```

### E2E Testing

E2E tests require a running dev server on port 4000:

```bash
# Check if dev server is running
lsof -ti:4000

# If not running, start it
task dev

# Run E2E tests
task e2e              # Both stages: browser (parallel) + editor (serial)
task e2e:browser      # Non-editor tests only (parallel)
task e2e:editor       # Editor tests only (serial, avoids ReactFlow flakiness)
```

**Screenshot updates** (after page layout changes):

```bash
cd apps/web && pnpm exec playwright test --update-snapshots && pnpm exec playwright test
```

## Environment Variables

Required in `.env.local`:

```bash
NEXT_PUBLIC_CONVEX_URL=https://your-project.convex.cloud
```

See `.env.example` for the full list.
