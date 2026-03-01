# Bnto Cloud + Desktop Strategy

**Date:** 2026-02-06
**Updated:** 2026-03-01
**Status:** Reference Document — Significantly Trimmed
**For current strategy:** See `ROADMAP.md` (source of truth for milestones, direction, and technology decisions)

> **March 2026 Trim:** This document was condensed from 687 lines. Stale sections (Wails, Better Auth, Go-as-primary, MVP phases) removed — git history preserves them. Remaining content: architecture overview, cloud execution topology, and cost analysis.

---

## Architecture: Layered Abstraction

Every layer only knows the layer below. `@bnto/core` is the transport-agnostic API — UI components have zero knowledge of the backend.

```
Apps (web/desktop/CLI)
  → @bnto/core (hooks, types, adapters)
    → Engine (Rust WASM for browser, Tauri native for desktop, Go for cloud)
```

| Package | Role | Implementation |
|---|---|---|
| `@bnto/backend` | Data layer | Convex |
| `@bnto/auth` | Auth client | `@convex-dev/auth` |
| `@bnto/core` | Transport-agnostic API | React Query + adapters |
| `@bnto/nodes` | Node definitions | Pure TypeScript |

**State management:** Zustand (client state) + React Query (server state). Convex adapter preserves real-time subscriptions through React Query's interface.

**Desktop:** Tauri (Rust-native) renders the same React app. `@bnto/core` detects runtime and swaps adapter. Engine runs as native Rust binary — same codebase as WASM, compiled for desktop.

---

## Cloud Execution Topology (M4)

Cloud execution is built and tested. Ready for M4 (premium server-side bntos). Browser execution (M1) doesn't use any of this — it's 100% client-side via WASM.

### Production Services

| Service | Provider | Purpose |
|---|---|---|
| Web app | Vercel | Next.js, SSR, static pages |
| Database + real-time | Convex Cloud | Subscriptions, business logic, presigned URLs |
| Go API | Railway | Cloud workflow execution |
| File transit | Cloudflare R2 | Temporary object storage (upload → process → download → delete) |
| DNS | Cloudflare | bnto.io |

### Cloud Execution Flow

```
Browser → R2 (upload) → Convex action → Railway Go API → R2 (output) → Browser (download)
```

R2 is a transit layer, not storage. Objects auto-delete via lifecycle rules. Three cleanup layers: Go API (immediate), Convex scheduler (2h delay for downloads), R2 lifecycle (1-day safety net).

### Development

| Service | Production | Development |
|---|---|---|
| Web | Vercel | localhost:4000 |
| Database | Convex (gregarious-donkey-712) | Convex (zealous-canary-422) |
| Go API | Railway | localhost:8080 via Cloudflare tunnel |
| File transit | R2 (bnto-transit) | R2 (bnto-transit-dev) |

---

## Cost Analysis

### M1: Browser Execution (Current)

| Service | Monthly Cost | Notes |
|---|---|---|
| Vercel | $0 (Hobby) | Static + SSR |
| Convex | $0 (Free tier) | 1M function calls, 1GB storage |
| Cloudflare | $0 | DNS, R2 not used for browser execution |
| **Total** | **$0** | Entire M1 runs at zero cost |

### M4: Cloud Execution (Future)

| Service | Monthly Cost | Notes |
|---|---|---|
| Railway | ~$5 (usage-based) | Go API, scales to zero |
| R2 | ~$0.015/GB | Transit only, objects live minutes |
| Convex | $0-25 | Depends on function call volume |
| **Total** | **~$5-30** | Only incurred when server-side bntos are used |

### M3: Desktop

| Service | Monthly Cost | Notes |
|---|---|---|
| Code signing | ~$99/year (Apple) | One-time for macOS distribution |
| **Total** | **$0 runtime** | Everything runs locally |

**Cost principle:** The user's browser is a powerful computer. Use it. No always-on compute. Backend is serverless/on-demand. Every architectural decision tested against: "Does this cost $0?"
