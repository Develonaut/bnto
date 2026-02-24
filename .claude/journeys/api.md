# API — User Journey Test Matrix

**Domain:** Go HTTP API server on Railway — cloud execution, R2 file transit, health
**Status:** Deployed to Railway. Integration tests exist (20+). Execution flow blocked by auth.
**Last Updated:** February 23, 2026

---

## Why This Matters

The API server is the bridge between Convex (orchestration) and the Go engine (execution). Every cloud execution flows through it: Convex action → Railway API → Go engine → R2. If the API is down, slow, or returns wrong data, every user's "Run" button is broken.

---

## Gate Map

API request lifecycle and validation checkpoints.

| Step | What Happens | Gate | Error Behavior |
|------|-------------|------|----------------|
| Health check | `GET /health` | None | 200 OK with version info |
| Start execution | `POST /api/run` | Request validation (body schema) | 400 with validation error details |
| Download input | Fetch input files from R2 | R2 presigned URL validity | 500 with "failed to download input: [reason]" |
| Execute workflow | Run Go engine with input | Engine validation + execution | 500 with engine error details (node ID, context) |
| Upload output | Push output files to R2 | R2 presigned URL validity | 500 with "failed to upload output: [reason]" |
| Report status | Update Convex execution record | Convex mutation reachability | Logs error, retries, eventually fails |
| Cleanup | Delete input files from R2 | Best-effort | Logs warning, doesn't fail execution |

---

## Journey Matrix

### Health & Reachability

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **P1** | Health check responds | `GET /health` | 200 OK. Response includes version and status. |
| **P2** | API reachable from Convex | Convex action calls `GO_API_URL/health` | 200 OK. No timeout. Proves Railway ↔ Convex connectivity. |
| **P3** | Invalid endpoint returns 404 | `GET /nonexistent` | 404 (not 500, not HTML error page). |

### Execution Lifecycle

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **P10** | Valid execution request | `POST /api/run` with valid workflow + R2 input URLs | 200 OK. Engine executes. Output uploaded to R2. Response includes output file keys. |
| **P11** | Execution with multiple input files | `POST /api/run` with 3+ input files | All files downloaded from R2. Engine processes all. All outputs uploaded. |
| **P12** | Execution reports progress | `POST /api/run` → monitor Convex updates | Convex execution record transitions: pending → running → completed. `currentNode` updates as nodes execute. |
| **P13** | Execution failure handled | `POST /api/run` with workflow that will fail (bad node config) | Response indicates failure. Convex execution record status = `"failed"`. Error message includes node context. Input files cleaned up. |

### Request Validation

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **P20** | Missing workflow definition | `POST /api/run` with empty body | 400 error. Message says what's missing. |
| **P21** | Invalid workflow JSON | `POST /api/run` with malformed workflow | 400 error. Message includes validation details. |
| **P22** | Missing input files | `POST /api/run` with workflow referencing files not in R2 | Execution fails gracefully. Error message mentions missing file. No panic. |

### R2 File Transit

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **P30** | Input download from R2 | API downloads file from presigned URL | File downloaded. Size matches. Content intact. |
| **P31** | Output upload to R2 | API uploads file to presigned URL | File accessible via presigned GET URL. Content matches engine output. |
| **P32** | Input cleanup after download | Execution completes → check R2 input path | Input files deleted (best-effort). If deletion fails, logged but execution not affected. |
| **P33** | Expired presigned URL handled | API attempts download with expired URL | Clear error ("presigned URL expired" or similar). No hang, no panic. |

---

## Cross-Domain Dependencies

| Journey | Also Touches | Notes |
|---------|-------------|-------|
| P10-P13 | Engine (execution) | API wraps engine — same fixtures, same execution paths |
| P10-P13 | Auth (Convex actions) | Convex action triggers API call — requires authenticated session to create execution record |
| P30-P32 | Auth (presigned URL generation) | Convex generates presigned URLs — auth gates who can generate them |
| P2 | Infra (Railway + Cloudflare tunnel) | Dev: `api-dev.bnto.io` via tunnel. Prod: `bnto-production.up.railway.app` |

---

## Implementation Notes

- Go integration tests via `httptest` in `apps/api/`
- Existing suite: 20+ tests covering happy path and error cases
- R2 transit tests need real R2 buckets (dev bucket: `bnto-transit-dev`)
- P2 (Convex → API reachability) requires `task dev:all` running
- P12 (progress reporting) requires Convex dev backend for mutation updates
