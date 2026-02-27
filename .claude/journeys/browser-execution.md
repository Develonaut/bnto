# Browser Execution Testing Strategy

**Goal:** "It just works." Confidence comes from layered coverage that tests the same pipeline at increasing levels of integration — from pure Rust functions to full user journeys.

**Last Updated:** February 24, 2026

---

## The Stack (M1 Browser-Only)

```
User drops files
  → FileDropZone (React component)
    → RecipeShell (orchestrator)
      → useBrowserExecution (React hook)
        → browserExecutionService (pure TS service)
          → BntoWorker (main-thread wrapper)
            → bnto.worker.ts (Web Worker script)
              → bnto_wasm.js (wasm-pack glue)
                → bnto_wasm_bg.wasm (Rust compiled)
                  → bnto-image crate (compression)
                    → image crate (decode/encode)
```

Every layer is a boundary. Every boundary is a potential failure point. The testing strategy covers each boundary and the full chain.

---

## Testing Trophy (not pyramid)

We follow the Testing Trophy model. Most value comes from integration tests that exercise real boundaries. Unit tests cover business logic edge cases. E2E tests prove the user journey works.

```
        🏆
       /  \
      / E2E \        ← 6-10 tests: full user journeys (Playwright)
     /--------\
    /Integration\    ← 37 tests: WASM boundary (wasm-bindgen-test)
   /--------------\
  /  Unit Tests    \ ← 60+ tests: Rust logic + TS services
 /------------------\
/ Static Analysis    \ ← clippy + eslint + tsc
```

---

## Layer 1: Rust Unit Tests

**What:** Pure Rust logic tested natively. No WASM, no JS, no browser.
**Command:** `task wasm:test:unit`
**Speed:** < 2 seconds

### Currently Covered (44 tests)

| Crate | File | Tests | Coverage |
|-------|------|-------|----------|
| bnto-core | lib.rs | 1 | Version semver |
| bnto-core | errors.rs | Tests | Error formatting, Display impl |
| bnto-core | progress.rs | Tests | Noop reporter, callback counting |
| bnto-core | processor.rs | Tests | NodeProcessor trait basics |
| bnto-image | compress.rs | Tests | JPEG/PNG/WebP compression, quality params |
| bnto-image | format.rs | Tests | Magic byte detection, MIME mapping |
| bnto-wasm | lib.rs | 1 | Setup idempotence |

### Gaps to Fill

| ID | Test | Why it matters | Priority |
|----|------|----------------|----------|
| R1 | Truncated file (< 10 bytes) | Users drop partial downloads | HIGH |
| R2 | Corrupt magic bytes (valid start, garbage body) | Common with interrupted saves | HIGH |
| R3 | Zero-byte file | Edge case that should error, not panic | HIGH |
| R4 | Quality param bounds (0, 1, 100, 101, -1) | Users can set any slider value | MEDIUM |
| R5 | Very small valid image (1x1 pixel) | Minimum viable input | MEDIUM |
| R6 | EXIF-heavy JPEG (large metadata, small image) | Common phone photos | MEDIUM |
| R7 | Animated WebP/PNG (multi-frame) | Should compress or error cleanly | LOW |
| R8 | Progressive JPEG handling | Common web format | LOW |

---

## Layer 2: WASM Integration Tests

**What:** Tests the Rust↔JS boundary. Proves types cross correctly, callbacks fire, errors propagate.
**Command:** `task wasm:test`
**Speed:** ~5 seconds (runs in Node.js via wasm-bindgen-test)

### Currently Covered (37 tests across 3 suites)

| Crate | Tests | Coverage |
|-------|-------|----------|
| bnto-core | 8 | Version, error formatting, progress noop in WASM context |
| bnto-image | 16 | JPEG/PNG/WebP compression + bytes, unsupported format error, invalid JSON params |
| bnto-wasm | 13 | Unified entry point: setup, version, greet, compress (JSON + bytes), defaults, errors |

### Gaps to Fill

| ID | Test | Why it matters | Priority |
|----|------|----------------|----------|
| W1 | All codec combinations (JPEG→JPEG, PNG→PNG, WebP→WebP) | Prove each path works end-to-end | HIGH |
| W2 | Progress callback fires with increasing percentages | E2E UI depends on this | HIGH |
| W3 | Large file (1MB+) doesn't OOM in WASM heap | Memory is limited in WASM | HIGH |
| W4 | Concurrent calls don't interfere | Worker processes sequentially but worth verifying | MEDIUM |
| W5 | Output is always smaller or equal (lossless edge case) | Core promise to users | MEDIUM |
| W6 | `compress_image_bytes` output has correct magic bytes for all formats | Download integrity | MEDIUM |

---

## Layer 3: TypeScript Unit Tests

**What:** Tests the JS service layer, state machine, and worker protocol in isolation.
**Command:** `task ui:test`
**Speed:** < 1 second

### Currently Covered (39 tests across 4 files)

| File | Tests | Coverage |
|------|-------|----------|
| slugCapability.test.ts | 3 | Slug registry mapping |
| browserExecutionService.test.ts | 11 | Service orchestration (mock engine) |
| browserExecutionReducer.test.ts | 11 | State machine transitions |
| BntoWorker.test.ts | 14 | Worker message protocol |

### Gaps to Fill

| ID | Test | Why it matters | Priority |
|----|------|----------------|----------|
| T1 | Batch progress: fileIndex increments across files | Multi-file UX correctness | HIGH |
| T2 | Service: engine init failure → clean error state | WASM load can fail (network, CORS) | HIGH |
| T3 | Worker: process after terminate → clear error | Lifecycle edge case | MEDIUM |
| T4 | Reducer: rapid fire start/complete/start | Race condition in user mashing Run | MEDIUM |
| T5 | downloadBlob: creates object URL and triggers click | Download actually works | MEDIUM |

---

## Layer 4: E2E Tests (Playwright, No Backend)

**What:** Full user journeys in a real browser. WASM loads, processes, UI updates, downloads work.
**Command:** `task e2e` (runs against Next.js dev server, no Convex/Go needed)
**Speed:** ~5-10 seconds

### Currently Covered (6 tests)

| Test | What it proves |
|------|---------------|
| Detects browser execution mode | `data-execution-mode="browser"` attribute set |
| Single image via WASM | Drop → Run → results with download button |
| Download produces valid file | Download event fires, file is smaller than input |
| Multiple images with Download All | 2 files → "2 files ready" → Download All button |
| Run Again resets | Completed → Run Again → idle state, files cleared |
| Non-browser slug = cloud mode | resize-images still shows `data-execution-mode="cloud"` |

### Gaps to Fill

| ID | Test | Why it matters | Priority |
|----|------|----------------|----------|
| E1 | Unsupported file shows user-friendly error | Drop a .txt → "not supported" | CRITICAL |
| E2 | PNG compression produces valid PNG download | Codec correctness for all formats | HIGH |
| E3 | WebP compression produces valid WebP download | Codec correctness for all formats | HIGH |
| E4 | Quality slider affects output size | Lower quality → smaller file | HIGH |
| E5 | 5+ files batch processing | Real-world batch with progress | HIGH |
| E6 | Corrupt image shows error, doesn't crash | Resilience | HIGH |
| E7 | Error state → retry with new files | Recovery flow | MEDIUM |
| E8 | Large image (1MB+) completes within timeout | Performance under load | MEDIUM |
| E9 | WASM init failure shows clear error | Network/loading failure | MEDIUM |
| E10 | Progress updates visible during processing | UX polish proof | LOW |

### Screenshot Convention: 4-Phase Capture

Every bnto E2E test captures the **full execution lifecycle** with screenshots at each phase. This gives visual proof that the pipeline works and catches regressions at every state.

```
Phase 1: BEFORE    → Page loaded, files selected, ready to run
Phase 2: PROGRESS  → Execution in progress (1+ screenshots capturing transient state)
Phase 3: FINISH    → Execution complete, results displayed
Phase 4: VERIFY    → Output downloaded and validated (file size, type, content)
```

All screenshots use `fullPage: true` to capture the complete page including results section.

#### Per-Bnto Screenshot Matrix

**compress-images (JPEG):**

| Phase | Screenshot | What it proves |
|-------|-----------|----------------|
| BEFORE | `compress-jpg-before.png` | File selected, Run enabled, quality slider visible |
| PROGRESS | (captured if transient) | Progress bar, "Processing file 1 of 1..." |
| FINISH | `compress-jpg-finish.png` | "1 file ready", privacy message, download button |
| VERIFY | Programmatic assertion | Downloaded file is valid JPEG, size <= input |

**compress-images (PNG):**

| Phase | Screenshot | What it proves |
|-------|-----------|----------------|
| BEFORE | `compress-png-before.png` | PNG file selected |
| FINISH | `compress-png-finish.png` | Compressed PNG result with size |
| VERIFY | Programmatic assertion | Downloaded file has PNG magic bytes, size <= input |

**compress-images (WebP):**

| Phase | Screenshot | What it proves |
|-------|-----------|----------------|
| BEFORE | `compress-webp-before.png` | WebP file selected |
| FINISH | `compress-webp-finish.png` | Compressed WebP result |
| VERIFY | Programmatic assertion | Downloaded file has RIFF/WEBP header, size <= input |

**compress-images (batch):**

| Phase | Screenshot | What it proves |
|-------|-----------|----------------|
| BEFORE | `compress-batch-before.png` | Multiple files selected, count shown |
| FINISH | `compress-batch-finish.png` | All results listed, Download All button |
| VERIFY | Programmatic assertion | All downloads valid, each <= its input |

**compress-images (error):**

| Phase | Screenshot | What it proves |
|-------|-----------|----------------|
| BEFORE | `compress-error-before.png` | Unsupported file selected |
| FINISH | `compress-error-finish.png` | Error card with friendly message |
| VERIFY | Programmatic assertion | No crash, Run Again available |

**compress-images (quality comparison):**

| Phase | Screenshot | What it proves |
|-------|-----------|----------------|
| FINISH (q=50) | `compress-quality-low.png` | Low quality result with size |
| FINISH (q=90) | `compress-quality-high.png` | High quality result with size |
| VERIFY | Programmatic assertion | q=50 output < q=90 output |

### Existing Screenshots (already captured)

| Screenshot | Phase | Status |
|-----------|-------|--------|
| `browser-exec-completed.png` | FINISH (single JPEG) | ✅ |
| `browser-exec-multi-completed.png` | FINISH (batch JPEG+PNG) | ✅ |

---

## Layer 5: Visual Regression

Screenshots capture the UI at each execution state. These are generated by E2E tests and verified visually. They catch CSS regressions, layout shifts, and broken rendering.

**Convention:** `fullPage: true` on all screenshots to capture the complete page including results section.

**4-Phase rule:** Every bnto E2E test must capture BEFORE, PROGRESS (if applicable), FINISH, and VERIFY. Progress screenshots are best-effort (transient state may pass too quickly for small files). VERIFY is always programmatic (file size, magic bytes, MIME type), not visual.

---

## Confidence Matrix

When all layers are complete, we can say "it just works" because:

| Concern | How it's tested | Layer |
|---------|----------------|-------|
| Rust compression logic is correct | Unit tests with all codec combinations | L1 |
| Edge cases don't panic | Unit tests for truncated/corrupt/zero-byte files | L1 |
| WASM boundary doesn't corrupt data | Integration tests verify magic bytes | L2 |
| Progress callbacks fire correctly | Integration tests + E2E observation | L2 + L4 |
| JS service orchestrates correctly | Unit tests with mock engine | L3 |
| State machine transitions are correct | Reducer unit tests | L3 |
| Worker protocol is reliable | BntoWorker tests with mock worker | L3 |
| User can drop files and see them | E2E with real browser | L4 |
| User clicks Run and sees progress | E2E screenshot assertions | L4 |
| User downloads a valid compressed file | E2E download verification | L4 |
| Errors show friendly messages, not crashes | E2E with bad inputs | L4 |
| Batch processing works for real | E2E with 5+ files | L4 |
| Quality setting affects output | E2E comparing file sizes | L4 |
| UI looks correct at every state | Visual regression screenshots | L5 |

---

## Test Commands Cheat Sheet

```bash
# Run all browser stack tests (fast → slow)
task wasm:test:unit     # Rust unit tests (~2s)
task wasm:test          # WASM integration tests (~5s)
task ui:test            # TS unit tests (~1s)
task e2e                # E2E user journeys (~10s)

# Run everything
task wasm:test && task ui:test && task e2e

# Update E2E screenshots after visual changes
cd apps/web && pnpm exec playwright test e2e/browser-execution.spec.ts --update-snapshots
```

---

## Priority Order for Filling Gaps

### Wave 1: Critical (must have before launch)
- E1: Unsupported file error handling E2E
- R1-R3: Rust edge cases (truncated, corrupt, zero-byte)
- W1-W2: All codec combinations + progress callback verification
- T1-T2: Batch progress + init failure

### Wave 2: High (strong confidence)
- E2-E6: All codec E2E, quality slider, batch, corrupt image
- W3, W5-W6: Large file, output size, magic bytes
- R4-R6: Quality bounds, tiny images, EXIF

### Wave 3: Medium (polish)
- E7-E9: Error recovery, large file perf, WASM init failure
- T3-T5: Worker lifecycle, rapid-fire, downloadBlob
- W4: Concurrent calls

### Wave 4: Low (nice to have)
- E10: Progress visibility
- R7-R8: Animated images, progressive JPEG
