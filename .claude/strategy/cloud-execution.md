# Cloud Execution Strategy — Railway

**Created:** April 25, 2026
**Status:** Strategy Spike — ready for review and refinement
**Related:** [cloud-desktop-strategy.md](cloud-desktop-strategy.md), [engine-execution.md](engine-execution.md), [pricing-model.md](pricing-model.md), [architecture.md](../rules/architecture.md), [ROADMAP.md](../ROADMAP.md)

---

## Problem Statement

Bnto's browser execution (Rust->WASM) covers image, CSV, file, and SVG operations — but entire categories of recipes can't run in the browser sandbox:

| Limitation           | Blocked recipes       | Examples                                        |
| -------------------- | --------------------- | ----------------------------------------------- |
| No external binaries | Video, audio, PDF     | `yt-dlp`, `ffmpeg`, `imagemagick`, `blender`    |
| CORS restrictions    | Unrestricted HTTP     | API aggregation, web scraping, webhooks         |
| No filesystem access | Shell commands        | Batch scripts, build tooling, system automation |
| API key exposure     | AI-powered nodes      | LLM inference, classification, extraction       |
| ~2GB memory ceiling  | Large file processing | Video transcoding, large dataset transforms     |

The CLI handles all of these natively. But bnto.io visitors who don't have `cargo install bnto` can't run these recipes. Cloud execution closes that gap — the same Rust engine, compiled natively (not WASM), running on managed infrastructure.

**This is not about replacing the CLI.** The CLI is the product. Cloud execution extends bnto.io's reach to recipes that require system access.

---

## Decision: Railway

**Railway** is our cloud execution platform. It resolves the "Technology TBD" placeholder in every strategy document.

### Why Railway

| Requirement            | Railway                 | Notes                                                                    |
| ---------------------- | ----------------------- | ------------------------------------------------------------------------ |
| Rust binary deployment | Native support          | Multi-stage Docker, Nixpacks auto-detect Rust                            |
| Usage-based billing    | Per-second (vCPU + RAM) | $20/vCPU/mo, $10/GB RAM/mo — billed per second                           |
| Scale to zero          | Serverless mode         | Service sleeps after 10min idle, $0 compute while asleep                 |
| Cold start acceptable  | Yes                     | Recipe execution is not latency-sensitive — user expects processing time |
| Presigned URL I/O      | Standard HTTP           | Service fetches from R2, writes back to R2                               |
| Health checks          | Built-in                | Configurable endpoint, timeout, zero-downtime deploys                    |
| Docker support         | First-class             | Multi-stage builds, private images, env vars                             |
| Existing familiarity   | Used in M1              | Railway hosted Go API during early development (archived)                |
| Cost at rest           | ~$5/mo (Pro plan)       | Pro plan includes $20 credit; idle service = $0 compute                  |
| Cost under load        | Pay-per-second          | Light usage well within Pro credit                                       |

### Why not alternatives

| Alternative          | Reason to skip                                                                                  |
| -------------------- | ----------------------------------------------------------------------------------------------- |
| AWS Lambda / Fargate | Overkill complexity, cold starts worse for large Rust binaries, vendor lock-in                  |
| Fly.io               | Good option but Railway has existing team familiarity + simpler DX                              |
| Self-hosted VPS      | No scale-to-zero, always-on cost, operational burden                                            |
| Cloudflare Workers   | No system access, no external binaries, WASM-only — same limitations as browser                 |
| Render               | Comparable but Railway's per-second billing and serverless mode are better for bursty workloads |

### Cost Model

**Railway Pro plan: $20/month** (includes $20 usage credit).

| Scenario                        | Estimated monthly cost         | Notes                              |
| ------------------------------- | ------------------------------ | ---------------------------------- |
| Idle (no executions)            | $20 (plan) + $0 (compute)      | Service sleeps, no compute charges |
| Light usage (50 executions/day) | $20 (plan) + ~$2-5 (compute)   | Within Pro credit                  |
| Moderate (500 executions/day)   | $20 (plan) + ~$15-30 (compute) | Slightly over credit               |
| Heavy (5000+ executions/day)    | $20 (plan) + $50-150 (compute) | Pro tier revenue should cover      |

Assumptions: Average execution = 5-15 seconds, 0.5 vCPU, 512MB RAM. Video/AI nodes skew higher.

**This fits the cost principle.** Railway at rest costs $20/mo (plan fee). Compute scales with actual usage. The $0 aspiration isn't achievable for cloud execution (managed infrastructure always has a base cost), but $20/mo base with pay-per-second compute is the closest thing to it.

---

## Architecture

### High-Level Flow

```
Browser (bnto.io)                    Railway                         Cloudflare R2
      |                                  |                                |
      |-- 1. Upload files ------------->|-- (presigned URL) ----------->|
      |                                  |                                |
      |-- 2. Execute recipe ----------->|                                |
      |   (Convex action dispatches)     |                                |
      |                                  |-- 3. Fetch inputs ----------->|
      |                                  |     (presigned GET)            |
      |                                  |                                |
      |                                  |-- 4. Run pipeline              |
      |                                  |     (native Rust engine)       |
      |                                  |                                |
      |                                  |-- 5. Write outputs ---------->|
      |                                  |     (presigned PUT)            |
      |                                  |                                |
      |<-- 6. Poll completion -----------|                                |
      |   (Convex subscription)          |                                |
      |                                  |                                |
      |-- 7. Download results --------->|-- (presigned GET) ----------->|
      |                                  |                                |
```

### Component Responsibilities

```
bnto-server (new Rust crate)         @bnto/core (existing)            Convex (existing)
----------------------------         --------------------             -----------------
HTTP API (axum)                      Cloud execution adapter          Execution records
Pipeline execution (native)          Upload/download services         Presigned URL generation
R2 file I/O (presigned URLs)         Progress relay to UI             Progress updates
ServerContext (ProcessContext impl)   Recipe/node validation           Completion polling
Health check endpoint                                                 R2 client (existing)
Auth token verification
```

### New Crate: `bnto-server`

A new crate in `engine/crates/bnto-server/` — an HTTP service wrapping the engine for cloud execution.

```
engine/crates/bnto-server/
  src/
    main.rs           # Axum server, PORT from env, health check
    routes/
      execute.rs      # POST /execute — accept recipe + file refs, run pipeline
      health.rs       # GET /health — liveness probe
      progress.rs     # GET /progress/:id — SSE stream for real-time progress
    context/
      server.rs       # ServerContext — ProcessContext impl for cloud
    io/
      r2.rs           # R2 file download/upload via presigned URLs
    auth/
      verify.rs       # Verify execution tokens (Convex-issued)
  Dockerfile          # Multi-stage: build in rust:1.x, ship minimal binary
  Cargo.toml
```

**`bnto-server` links `bnto-engine` directly** — same as the CLI. No WASM boundary. Native Rust binary with full system access.

### ServerContext: ProcessContext for Cloud

```rust
/// Cloud execution context with controlled system access.
pub struct ServerContext {
    /// Temporary directory scoped to this execution (cleaned up after).
    work_dir: PathBuf,
    /// Allowed external binaries (pre-installed in container).
    allowed_binaries: HashSet<String>,
    /// Execution timeout.
    timeout: Duration,
}

impl ProcessContext for ServerContext {
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>> {
        // Verify cmd is in allowed_binaries allowlist
        // Run with timeout enforcement
        // Scoped to work_dir
    }

    fn temp_file(&self, suffix: &str) -> Result<PathBuf> {
        // Create in work_dir (auto-cleaned)
    }

    fn env_var(&self, key: &str) -> Option<String> {
        // Allowlisted env vars only (no leaking Railway secrets)
    }

    fn work_dir(&self) -> &Path {
        &self.work_dir
    }
}
```

**Key difference from NativeContext:** ServerContext is sandboxed. Restricted binary allowlist, scoped temp directory, env var filtering. NativeContext (CLI) gives full access; ServerContext gives controlled access.

### Execution API

```
POST /execute
  Headers:
    Authorization: Bearer <execution-token>
    Content-Type: application/json
  Body:
    {
      "execution_id": "conv_abc123",
      "recipe": { ... },           // PipelineDefinition (same as .bnto.json)
      "inputs": [                   // R2 presigned URLs for input files
        { "name": "photo.jpg", "url": "https://r2.../uploads/sess123/photo.jpg" }
      ],
      "output_prefix": "executions/exec456/output/",
      "callback_url": "https://gregarious-donkey-712.convex.cloud/api/..."
    }

  Response: 202 Accepted
    { "execution_id": "conv_abc123", "status": "running" }

GET /health
  Response: 200 OK
    { "status": "ok", "version": "0.1.0" }

GET /progress/:execution_id
  Response: SSE stream
    data: {"event":"NodeStarted","node_id":"compress","node_type":"image-compress"}
    data: {"event":"FileProgress","node_id":"compress","file_index":0,"progress":0.45}
    data: {"event":"NodeCompleted","node_id":"compress","duration_ms":1200}
    data: {"event":"PipelineCompleted","total_duration_ms":3400}
```

### Progress Reporting

Two options for real-time progress (decide during implementation):

| Approach             | Pros                                                                 | Cons                                                                        |
| -------------------- | -------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| **SSE from Railway** | Real-time, low latency, browser subscribes directly                  | Keeps Railway service awake during execution (fine — it's actively running) |
| **Convex polling**   | Existing pattern (execution_engine.ts), no direct Railway connection | 2-second polling interval, higher Convex function calls                     |

**Recommendation: SSE for progress, Convex for completion.** The browser opens an SSE connection to Railway for real-time progress during execution, then Convex records the final execution result for history and persistence. This gives the best UX (real-time progress bars) without overloading Convex with high-frequency progress mutations.

### Docker Image

```dockerfile
# Stage 1: Build
FROM rust:1.78 AS builder
WORKDIR /app
COPY engine/ ./engine/
RUN cargo build --release --bin bnto-server

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    ffmpeg \
    imagemagick \
    && rm -rf /var/lib/apt/lists/*

# Install yt-dlp (latest stable)
ADD https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp /usr/local/bin/yt-dlp
RUN chmod +x /usr/local/bin/yt-dlp

COPY --from=builder /app/engine/target/release/bnto-server /usr/local/bin/
EXPOSE 3000
CMD ["bnto-server"]
```

**Pre-installed binaries in the container image.** External dependencies (`ffmpeg`, `yt-dlp`, `imagemagick`) are baked into the Docker image. The dependency checker (`bnto doctor` logic) still runs at pipeline start — if a recipe requires a binary not in the image, it fails fast with a clear error rather than mysterious runtime failures.

---

## Convex Orchestration

### Updated Execution Flow

The existing `execution_engine.ts` (Convex internal action) is the orchestrator. It currently targets the dead Go API URL — it needs to be updated to target Railway.

```typescript
// packages/@bnto/backend/convex/execution_engine.ts (updated)

export const executeOnServer = internalAction({
  args: {
    executionId: v.id("executions"),
    recipeDefinition: v.any(),
    sessionId: v.string(),
    inputFiles: v.array(v.object({ name: v.string(), key: v.string() })),
  },
  handler: async (ctx, args) => {
    // 1. Generate presigned GET URLs for inputs (existing R2 client)
    const inputUrls = await generatePresignedGets(args.inputFiles);

    // 2. Generate presigned PUT prefix for outputs
    const outputPrefix = `executions/${args.executionId}/output/`;

    // 3. Generate execution token (short-lived, scoped to this execution)
    const token = await generateExecutionToken(args.executionId);

    // 4. POST to Railway service
    const response = await fetch(process.env.RAILWAY_API_URL + "/execute", {
      method: "POST",
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        execution_id: args.executionId,
        recipe: args.recipeDefinition,
        inputs: inputUrls,
        output_prefix: outputPrefix,
        callback_url: process.env.CONVEX_SITE_URL + "/api/execution-complete",
      }),
    });

    // 5. Railway returns 202 — execution is async
    // 6. Railway calls back to Convex on completion (or Convex polls)

    await ctx.runMutation(internal.executions.updateProgress, {
      executionId: args.executionId,
      status: "running",
      message: "Pipeline dispatched to cloud execution",
    });
  },
});
```

### Environment Variables

| Variable                   | Where                    | Value                                           |
| -------------------------- | ------------------------ | ----------------------------------------------- |
| `RAILWAY_API_URL`          | Convex env               | `https://bnto-server-production.up.railway.app` |
| `RAILWAY_EXECUTION_SECRET` | Convex env + Railway env | Shared HMAC secret for token verification       |
| `R2_*` vars                | Convex env (existing)    | Already configured for both dev and prod        |

---

## Node Classification Update

With Railway as the execution target, the `platforms` taxonomy becomes concrete:

| Platform    | Execution target   | ProcessContext   | External binaries             |
| ----------- | ------------------ | ---------------- | ----------------------------- |
| `"browser"` | WASM in Web Worker | NoopContext      | None                          |
| `"cli"`     | Native Rust binary | NativeContext    | User's system                 |
| `"desktop"` | Tauri native (M4)  | SandboxedContext | User's system (scoped)        |
| `"server"`  | Railway container  | ServerContext    | Pre-installed in Docker image |

### Recipes unlocked by cloud execution

| Recipe          | Node types               | Why server           | CLI alternative                           |
| --------------- | ------------------------ | -------------------- | ----------------------------------------- |
| Download Video  | `shell-command` (yt-dlp) | Needs yt-dlp binary  | `bnto run download-video` (already works) |
| Extract Audio   | `shell-command` (ffmpeg) | Needs ffmpeg binary  | `bnto run extract-audio`                  |
| Video Thumbnail | `shell-command` (ffmpeg) | Needs ffmpeg binary  | `bnto run video-thumbnail`                |
| API to CSV      | `http-request`           | No CORS restrictions | `bnto run api-to-csv`                     |
| AI Classify     | `ai`                     | API key proxy        | BYOK via CLI                              |
| AI Summarize    | `ai`                     | API key proxy        | BYOK via CLI                              |
| AI Transform    | `ai`                     | API key proxy        | BYOK via CLI                              |
| Batch Shell     | `shell-command`          | Needs system access  | `bnto run <recipe>`                       |

---

## Security Model

### Threat Surface

Cloud execution introduces new attack vectors not present in CLI or browser execution:

| Threat                      | Mitigation                                                                   |
| --------------------------- | ---------------------------------------------------------------------------- |
| Arbitrary command execution | `ServerContext` binary allowlist — only pre-installed binaries               |
| Path traversal              | All file I/O scoped to per-execution temp dir                                |
| Resource exhaustion         | Per-execution timeout (configurable, default 5min) + Railway resource limits |
| Secrets leakage             | `env_var()` returns only allowlisted vars, no Railway/Convex secrets         |
| Unauthorized execution      | Execution tokens: short-lived, single-use, Convex-issued                     |
| Multi-tenant data leakage   | Each execution gets isolated temp dir, cleaned after completion              |
| Denial of service           | Railway rate limiting + per-user execution quotas (Convex-enforced)          |
| Shell injection             | Existing protection: explicit `command + args` split, no `sh -c`             |

### Binary Allowlist

The `ServerContext` only permits execution of pre-installed, vetted binaries:

```rust
const ALLOWED_BINARIES: &[&str] = &[
    "ffmpeg",
    "ffprobe",
    "yt-dlp",
    "magick",     // ImageMagick
    "rsvg-convert", // SVG operations
];
```

Any `shell-command` node requesting a binary not in this list fails with a clear error. This is the single most important security boundary — it prevents arbitrary code execution.

### Execution Tokens

Convex issues short-lived tokens for each execution:

- Scoped to a single `execution_id`
- Expires after 15 minutes (longer than any expected execution)
- HMAC-signed with shared secret (`RAILWAY_EXECUTION_SECRET`)
- Verified by `bnto-server` before pipeline execution begins

### Future: Recipe Trust Levels

When community recipes arrive, cloud execution needs a trust model. This is a future concern (triage item in PLAN.md) but the architecture should not preclude it:

- **Built-in recipes** (signed, trusted) — execute immediately
- **Community recipes** (unsigned) — require explicit user consent before cloud execution
- **User-created recipes** — restricted to nodes/binaries in the allowlist

---

## Implementation Phases

### Phase 1: Foundation (1 sprint)

Ship the minimum viable cloud execution path.

1. **Create `bnto-server` crate** — Axum HTTP server, health check, `/execute` endpoint
2. **Implement `ServerContext`** — binary allowlist, scoped temp dirs, timeout enforcement
3. **R2 I/O** — download inputs from presigned URLs, upload outputs to presigned URLs
4. **Dockerfile** — multi-stage build, pre-installed binaries
5. **Railway deployment** — Pro plan, serverless mode, env vars configured
6. **Update `execution_engine.ts`** — point at Railway instead of dead Go URL
7. **Integration test** — end-to-end: upload file -> Convex dispatches -> Railway executes -> download result

**Exit criteria:** `download-video` recipe runs on bnto.io via cloud execution. User uploads a URL, Railway runs yt-dlp, result downloads.

### Phase 2: Progress & Polish (1 sprint)

Make cloud execution feel like a first-class experience.

1. **SSE progress endpoint** — real-time progress events streamed to browser
2. **`@bnto/core` cloud adapter** — browser subscribes to SSE during execution, falls back to polling
3. **Execution history** — cloud executions recorded in Convex with timing, status, node-level results
4. **Error handling** — structured errors from Railway, surfaced in UI with actionable messages
5. **Timeout & cancellation** — per-execution timeout, user-initiated cancellation via Convex -> Railway

### Phase 3: AI Nodes & Secrets (1 sprint)

Unlock the highest-value cloud-only recipes.

1. **`ai` node type** — new processor in `bnto-engine`, platforms: `["server"]`
2. **Secret management** — `ServerContext` resolves secrets from Railway env vars for AI API keys
3. **API key proxy** — server owns the API key, user doesn't need their own (Pro feature)
4. **`http-request` node type** — unrestricted HTTP from server (no CORS)
5. **Usage metering** — track compute time per execution, per user (Convex schema)

### Phase 4: Hardening & Scale (ongoing)

1. **Rate limiting** — per-user execution quotas (Convex-enforced before dispatch)
2. **Queue system** — if Railway is at capacity, queue executions with estimated wait time
3. **Multi-region** — Railway multi-region replicas for lower latency (when usage warrants)
4. **Monitoring** — execution success/failure rates, p50/p95 duration, Railway resource utilization
5. **Auth reactivation** — cloud execution requires authentication (Pro tier enforcement)

---

## Open Questions

| Question                 | Options                                                 | Notes                                                                            |
| ------------------------ | ------------------------------------------------------- | -------------------------------------------------------------------------------- |
| Progress delivery        | SSE vs Convex polling vs WebSocket                      | SSE recommended — see Progress Reporting section                                 |
| Auth for cloud execution | Reactivate `@bnto/auth` vs new lightweight token system | Auth was stripped in open-source pivot. Cloud execution needs it back for quotas |
| Pro tier timing          | Ship cloud execution free first, then gate?             | Aligns with "show value first" principle. Free beta -> Pro gate when stable      |
| AI API key strategy      | bnto-managed keys vs BYOK-only                          | Managed keys = simpler UX but operational cost. BYOK = $0 but worse UX           |
| Execution size limits    | Max file size, max execution time                       | Need benchmarks. Start conservative: 100MB input, 10min timeout                  |
| Railway plan             | Pro ($20/mo) vs Hobby ($5/mo)                           | Pro for production (higher limits, collaboration). Hobby for dev/staging         |

---

## Cost Projection (Updated)

| Service                    | Monthly cost | Notes                          |
| -------------------------- | ------------ | ------------------------------ |
| Railway Pro                | $20 (plan)   | Includes $20 usage credit      |
| Railway compute (light)    | $0-5         | Covered by credit at low usage |
| Railway compute (moderate) | $10-30       | Video/AI recipes use more CPU  |
| R2 transit                 | ~$0.02/GB    | Transit only, 1-hour TTL       |
| Convex                     | $0-25        | Depends on execution volume    |
| **Total (light)**          | **~$20**     | Mostly just Railway plan fee   |
| **Total (moderate)**       | **~$30-55**  | Scales linearly with usage     |

---

## What This Changes in Existing Docs

| Document                    | Change needed                                                                                     |
| --------------------------- | ------------------------------------------------------------------------------------------------- |
| `cloud-desktop-strategy.md` | Replace "TBD (M4)" with "Railway" in production services table                                    |
| `ROADMAP.md`                | Update M4 description: "Technology for cloud execution TBD" -> "Railway (see cloud-execution.md)" |
| `engine-execution.md`       | Update server row: "bnto-server (planned)" -> link to this doc                                    |
| `architecture.md`           | Update "Cloud Execution (M4)" section                                                             |
| `pricing-model.md`          | Add Railway cost to server node execution                                                         |
| `environment-variables.md`  | Add `RAILWAY_API_URL`, `RAILWAY_EXECUTION_SECRET`                                                 |
| `PLAN.md`                   | Add cloud execution work items when sprint is scheduled                                           |

---

_This document is the cloud execution strategy. For the broader architecture, see [cloud-desktop-strategy.md](cloud-desktop-strategy.md). For engine execution internals, see [engine-execution.md](engine-execution.md). For sprint tasks, see [PLAN.md](../PLAN.md)._
