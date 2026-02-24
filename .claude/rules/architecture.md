# Architecture Rules

## Layered Architecture

```
Apps (web/desktop) -> @bnto/core -> Go Engine
```

Each layer only depends on layers below it. Never skip layers.

> **Co-location note:** UI components and editor features are currently co-located in `apps/web/`. They will be extracted into `@bnto/ui` and `@bnto/editor` packages when the desktop app creates a real second consumer. Engine, core API, and data layer logic stays in `@bnto/core`.

**The key insight:** `@bnto/core` is the transport-agnostic API layer. UI components have ZERO knowledge of whether they're talking to Convex (cloud) or Wails bindings (desktop). Core exposes React hooks that internally detect the runtime environment and route requests to the correct backend.

**Package naming convention:** Internal packages are named by **role**, not by technology. This ensures any technology can be swapped by rewriting the package internals without changing consumers.

| Package | Role | Current Implementation |
|---|---|---|
| `@bnto/backend` | Data layer -- schema, functions, business logic | Convex |
| `@bnto/auth` | Auth client -- sign in, sign up, session | `@convex-dev/auth` |
| `@bnto/core` | Transport-agnostic API -- hooks, types, adapters | React Query + adapters |

**State management:** Zustand handles client-only state (editor content, UI preferences). React Query handles all server state (data fetching, caching, mutations). For the Convex path, `@convex-dev/react-query` preserves real-time subscriptions through React Query's interface.

**Desktop shares the web frontend:** Wails v2 renders the same React app in a system webview. `@bnto/core` detects the runtime (browser vs Wails) and swaps the transport adapter internally -- no separate frontend for desktop. Desktop uses local filesystem directly (no R2 file transit). Engine starts in-process (Wails bindings), designed for later decoupling into a standalone `bnto` binary called via subprocess.

## API Abstraction

**UI code NEVER calls backend or storage APIs directly.** Always go through `@bnto/core` hooks.

```typescript
// CORRECT -- use @bnto/core hooks
import { useWorkflows, useExecution, useRunWorkflow } from "@bnto/core";
const workflows = useWorkflows();
const { mutate: run } = useRunWorkflow();

// WRONG -- direct Convex calls in components
const workflows = useQuery(api.workflows.list);

// WRONG -- direct Wails calls in components
const workflows = window.go.main.App.ListWorkflows();
```

## Cost-First Architecture

**The user's browser is a powerful computer. Use it.**

- Client-side processing where possible (file validation, preview generation)
- No always-on compute services. Backend and hosting are serverless/on-demand
- Every architectural decision should be tested against: "Does this cost $0? If not, can we make it cost $0?"

## Package Responsibilities

### `packages/core/` (`@bnto/core`) -- Transport-agnostic API layer
- React hooks for all data operations (workflows, executions, logs)
- TypeScript types and interfaces shared across the app
- Zustand stores for client-only state (editor content, UI preferences)
- React Query configuration + transport adapters (Convex + R2 for web, Wails + local filesystem for desktop)
- Runtime detection to swap adapters transparently
- NO backend or storage technology imports in public API -- only in internal adapters

### `packages/@bnto/backend/` (`@bnto/backend`) -- Data layer
- Schema definition (tables, indexes, validators)
- Server functions (queries, mutations, actions)
- Business rules and validation logic
- **Currently:** Convex. Named by role so internals can be swapped.
- Consumed by `@bnto/core` internals, NEVER by app code directly

### `packages/@bnto/auth/` (`@bnto/auth`) -- Auth client
- Sign in, sign up, sign out, session management
- OAuth provider configuration
- **Currently:** `@convex-dev/auth`. Named by role so internals can be swapped.
- Consumed by `@bnto/core` internals, NEVER by app code directly

### `apps/web/` -- Next.js application (Vercel)
- Landing page (public, static/SSG routes)
- Authenticated app routes (dashboard, workflows, executions)
- UI components and editor features co-located here (future `@bnto/ui` + `@bnto/editor`)
- Page composition -- imports from `@bnto/core` for data, local components for UI

### `apps/desktop/` -- Wails v2 application
- Same React frontend rendered in system webview
- `@bnto/core` detects Wails runtime and swaps transport adapter
- No separate frontend code -- shares everything with web
- Engine in-process via Wails Go bindings (MVP); designed for later decoupling to standalone `bnto` CLI binary via subprocess
- Files accessed directly from local filesystem -- no R2, no cloud file transit

### `apps/api/` -- Go HTTP API server (Railway)
- HTTP API that wraps the Go engine for cloud execution
- Receives workflow execution requests from Convex actions
- Returns execution results

### `engine/` -- Go engine (CLI + execution)
- CLI binary (`cmd/bnto/`)
- Workflow execution engine (`pkg/engine/`)
- Node type implementations (`pkg/node/`)
- Workflow validation (`pkg/validator/`)
- Node registry (`pkg/registry/`)
- Path resolution and config (`pkg/paths/`)

## Execution Model: Async & Long-Running Node Support

The execution engine must support nodes that take 2-30+ seconds (AI API calls, large HTTP requests, complex transforms). This is a prerequisite for the `ai` node type (see [bntos.md Tier 4](../strategy/bntos.md#tier-4-ai-powered-nodes-backlog--requires-async-execution)) but also benefits `http-request` and any future external API integration.

**Requirements agents must preserve when working on execution infrastructure:**

- **Progress reporting** — nodes must be able to report incremental progress (e.g., "Classifying image... 4.2s"). The `node.Output` type and execution loop must support progress callbacks, not just final results
- **Per-node timeouts** — configurable per node type. A 30-second AI call is normal; a 30-second image resize is a bug. Default timeout per type, overridable in node config
- **Cancellation** — the existing `context.Context` propagation pattern is the right foundation. Ensure all node `Execute()` implementations check `ctx.Err()` before expensive operations and respect cancellation mid-operation
- **Retry/fallback** — AI and HTTP calls fail transiently more often than local operations. The execution model should support per-node retry config (max attempts, backoff) without requiring every node type to implement its own retry logic
- **Streaming output** — some nodes produce useful intermediate output (AI summarization, large file processing). The execution log model should support appending output incrementally, not just final results

**What this does NOT mean:** Don't build any of this speculatively. These are constraints for agents to keep in mind when designing execution infrastructure. If you're modifying `engine/pkg/engine/` or `node.Output` or the execution loop, check that your changes don't make the above harder to add later.

---

## Data Flow

### Abstraction Layer (how code sees it)

Components never know which backend they're talking to. `@bnto/core` detects the runtime and swaps adapters transparently.

```
+--------------------------------------------------------------+
|                    Apps (same React code)                      |
|  +--------------+  +--------------+  +--------------+         |
|  |  Next.js Web |  | Wails Desktop|  |   CLI        |         |
|  |  (Vercel)    |  | (webview)    |  |   (Terminal) |         |
|  +------+-------+  +------+-------+  +------+-------+         |
|         |                  |                  |                |
|         +--------+---------+                  |                |
|                  v                            |                |
|  +---------------------------------------+    |                |
|  |         @bnto/core                    |    |                |
|  |  +-------------+ +-------------+     |    |                |
|  |  |   Zustand    | | React Query |     |    |                |
|  |  |(client state)| |(server state)|    |    |                |
|  |  +-------------+ +------+------+     |    |                |
|  |          +---------------+            |    |                |
|  |          v               v            |    |                |
|  |  +------------+  +------------+       |    |                |
|  |  |  Convex    |  |   Wails    |       |    |                |
|  |  |  adapter   |  |   adapter  |       |    |                |
|  |  +-----+------+  +-----+------+       |    |                |
|  +--------+---------------+----------+    |                |
|           v               v               v                |
|    +----------+    +----------+    +----------+             |
|    | Convex   |    | Go Engine|    | Go Engine|             |
|    | (cloud)  |    | (local)  |    |  (CLI)   |             |
|    +----------+    +----------+    +----------+             |
+--------------------------------------------------------------+
```

### Cloud Execution: Production Topology

This is what actually runs in production. The abstract diagram above shows the code layers; this shows the deployed services and how data moves between them.

```
Browser
  |
  |--- Vercel (Next.js) --- static pages, SSR, auth UI
  |
  |--- Convex Cloud (gregarious-donkey-712)
  |         |--- real-time subscriptions, DB, business logic
  |         |--- generates R2 presigned URLs (upload + download)
  |         +--- calls Go API to trigger workflow execution
  |
  |--- Cloudflare R2 (bnto-transit) --- file transit only
  |         |--- browser uploads input files via presigned URL
  |         +--- browser downloads output files via presigned URL
  |
  +--- Railway (Go API: bnto-production.up.railway.app)
            |--- downloads input from R2
            |--- runs Go engine (workflow execution)
            +--- uploads output to R2
```

**Full cloud execution sequence:**

1. Browser uploads input file to R2 via Convex-generated presigned URL
2. Convex action calls Railway Go API with execution request
3. Railway downloads input from R2, runs Go engine, uploads output to R2
4. Convex mutation updates execution status (real-time to browser)
5. Browser downloads output from R2 via presigned URL
6. R2 objects deleted (or expire via 1-hour TTL)

### Local Development Topology

`task dev:all` starts everything needed for local development in parallel:

```
task dev:all
  |--- task dev          --- Next.js (localhost:4000) + Convex dev (zealous-canary-422)
  |--- task api:dev      --- Go API server (localhost:8080)
  +--- task api:tunnel   --- Cloudflare Named Tunnel (bnto-dev)
                               exposes localhost:8080 at https://api-dev.bnto.io
```

**Why the Cloudflare tunnel?** Convex dev is cloud-hosted (not local). When a Convex action needs to call the Go API, it can't reach `localhost:8080` because Convex is running on Convex's servers, not your machine. The Cloudflare Named Tunnel (`bnto-dev`) exposes the local Go API at `https://api-dev.bnto.io`, giving Convex dev a stable HTTPS endpoint to call. Dev mirrors prod exactly -- same code paths, different URLs.

### Service Topology Summary

| Service | Production | Development |
|---|---|---|
| Web app | Vercel | localhost:4000 |
| Database + real-time | Convex (gregarious-donkey-712) | Convex (zealous-canary-422) |
| Go API | Railway (bnto-production.up.railway.app) | localhost:8080 via tunnel (api-dev.bnto.io) |
| File transit | R2 (bnto-transit) | R2 (bnto-transit-dev) |
| DNS / tunnel | Cloudflare (bnto.io) | Cloudflare Named Tunnel (bnto-dev) |

## Run Quota Infrastructure

Every execution must:
- Increment `runsUsedThisMonth` in the user's Convex record
- Record `slug`, `timestamp`, and `durationMs`
- Be associated with an authenticated user OR an anonymous browser fingerprint

| Sprint | What gets built |
|--------|----------------|
| Sprint 2 | Execution events logged (userId or fingerprint, slug, timestamp, durationMs) |
| Sprint 3 | `runsUsedThisMonth`, `runResetDate`, `totalRunsAllTime` per user. Dashboard shows usage. |
| Sprint 7 | Stripe integration. Quota enforced server-side. |

File size limits are enforced at the **R2 presigned URL generation step in Convex -- not client-side**. For tier limits, see Notion (`SEO & Monetization Strategy`).

## R2 Storage: Cloud-Only Transit Layer

R2 is a **cloud-only** transit layer, not a storage product. Files exist for minutes. Desktop execution does NOT use R2 -- files stay on the user's local filesystem.

- **Cloud path:** Browser → R2 (upload) → Railway API → Go Engine → R2 (output) → Browser (download)
- **Desktop path:** Webview → Wails adapter → Go Engine → local filesystem (no R2, no network)
- Upload → process → download → delete
- Storage stays near zero at all times
- Never repurpose R2 as long-term storage without an explicit product decision
- The Wails adapter passes file paths directly to the engine. The Convex adapter manages R2 presigned URLs. `@bnto/core` handles this transparently -- components never know which path they're on

### R2 Object Cleanup (Defense in Depth)

Three layers ensure transit objects don't accumulate:

| Layer | Where | What | When |
|---|---|---|---|
| **1. Go API** | `r2.CleanupSessionBestEffort` in `run.go` | Deletes input files after download | Immediately after successful download (best-effort, won't fail execution) |
| **2. Convex scheduler** | `scheduleR2Cleanup` in `executions.ts` | Schedules `cleanup.deleteByPrefix` | Input: immediately on complete/fail. Output: 2-hour delay for user downloads |
| **3. R2 lifecycle rules** | Cloudflare dashboard | Auto-deletes objects past TTL | Catches anything layers 1-2 missed |

**R2 lifecycle rules (configured in Cloudflare dashboard):**

R2 lifecycle rules have a **1-day minimum granularity**. This is fine — layers 1-2 handle the fast path (minutes). Lifecycle rules are the final safety net.

| Prefix | Auto-delete after | Rationale |
|---|---|---|
| `uploads/` | 1 day | Input files consumed within minutes. 1-day catches stragglers |
| `executions/` | 1 day | Output files get 2h scheduled cleanup. 1-day is the final safety net |

Apply these rules to both `bnto-transit` (prod) and `bnto-transit-dev` (dev) buckets.

---

## Content Model: Workflows and Executions

**Workflows are the atomic unit of content.** Users define workflows as `.bnto.json` files that orchestrate tasks.

```
Workflow (atomic unit)
  |-- name, description, version
  |-- nodes[] (task definitions)
  |     +-- Node
  |           |-- type, id, config
  |           +-- connections (inputs/outputs)
  +-- executions (queried via by_workflowId index)
        +-- Execution
              |-- status, startedAt, completedAt
              +-- ExecutionLog[] (per-node results)
```

### Key Principles

- **Workflow-first:** A workflow defines what to do. Executions track runs of that workflow.
- **Nodes are typed:** Each node has a type (image, file, http, transform, etc.) registered in the Go engine's registry.
- **Execution logs are per-node:** Each node in an execution produces its own log entry with status, output, and timing.
- **CLI is the stable API:** Every operation maps to a CLI command. The Go engine is the source of truth for execution behavior.
