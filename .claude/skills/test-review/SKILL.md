---
name: test-review
description: Review test quality against proven community patterns for the bnto tech stack
---

# Test Review

Review existing and proposed tests against proven community patterns. This skill runs in two modes:

- **Change mode** (invoked during `/pre-commit`): Reviews tests related to pending changes — are the right things tested? Are tests testing behavior or implementation details?
- **Landscape mode** (invoked standalone): Evaluates the overall testing health across every layer

## Philosophy

Three sentences from the community that define how we test:

1. **"Write tests. Not too many. Mostly integration."** — Guillermo Rauch. Integration tests give the highest confidence-to-cost ratio. Don't chase 100% coverage.
2. **"The more your tests resemble the way your software is used, the more confidence they can give you."** — Kent C. Dodds. Test behavior through public APIs, not internal implementation.
3. **"Test state, not interactions. Test behaviors, not methods."** — Google SWE Book. Assert on outcomes the user cares about, not how many times a function was called.

**The testing trophy** (not pyramid): Static analysis (TypeScript/Go compiler) → Unit tests (pure logic) → Integration tests (the bulk) → E2E (critical journeys). The fat middle is integration — not unit tests.

**Bluesky's lesson:** Real services over mocks. Their `@atproto/dev-env` spins up actual PDS instances for testing — they barely mock at all. For bnto: use `convex-test` with real schema, `httptest` with real handlers, fixture `.bnto.json` with real engine execution. Mock only at the boundary you can't control.

---

## Step 0: Read the Standards

Before reviewing ANY tests, read and internalize:

```
.claude/CLAUDE.md                  # Architecture, layering, tech stack
.claude/rules/code-standards.md    # Testing strategy section
.claude/rules/pre-commit.md        # Test coverage requirements
```

## Step 0b: Activate Domain Personas

Identify which packages the tests under review belong to and invoke the relevant persona skill(s):

| Test files in... | Persona skill |
|---|---|
| `engine/` | `/rust-expert` |
| `archive/engine-go/`, `archive/api-go/` | `/go-engineer` |
| `apps/web/` (including `e2e/`) | `/frontend-engineer` |
| `packages/core/` | `/core-architect` |
| `packages/@bnto/backend/`, `packages/@bnto/auth/` | `/backend-engineer` |

**Invoke the matching persona skill(s) now.** Each persona is a domain expert that defines domain-specific testing patterns, anti-patterns, and quality bars — e.g., Rust TDD layers (unit → WASM integration → E2E), Go table-driven tests with race detector, React testing trophy (thin unit + thick integration + targeted E2E). Evaluating test quality requires knowing how that domain's community tests. Activate all relevant personas for the scope under review.

## Step 1: Identify Scope

### Change mode (if invoked from /pre-commit or with pending changes)

Identify what changed:

!`git diff --name-only HEAD 2>/dev/null; git diff --cached --name-only 2>/dev/null; git ls-files --others --exclude-standard`

Map each changed file to the test layer that should cover it (see the value matrix in Step 2).

### Landscape mode (if invoked standalone)

Survey all test files across every layer:

```bash
# Go tests
find archive/engine-go/ archive/api-go/ -name "*_test.go" | head -100

# TypeScript tests
find packages/ apps/web/ -name "*.test.ts" -o -name "*.test.tsx" | head -100

# E2E tests
find apps/web/e2e/ -name "*.spec.ts" | head -50
```

Read representative test files from each layer to assess patterns.

---

## Step 2: The Value Matrix

Not all tests are created equal. This matrix defines what's worth testing at each layer — sourced from community consensus and adapted for bnto's stack.

### Layer 1: Go Engine (`engine/`)

**Source:** Go community (Dave Cheney, Learn Go with Tests), Google SWE Book

| What to test | Value | Pattern | Example |
|---|---|---|---|
| Node type `Execute()` — happy path + errors | **Critical** | Table-driven subtests for multi-case, narrative for complex flows | `image_test.go` |
| Full workflow execution with fixture `.bnto.json` | **Critical** | Fixture-driven integration test | `csv_pipeline_test.go` |
| Output structure stability | **High** | Golden tests (`*_golden_test.go`) | `progress_golden_test.go` |
| Validator rules — valid + invalid inputs | **High** | Paired valid/invalid fixtures (Bluesky interop pattern) | `omakase_test.go` |
| Error wrapping and context messages | **Medium** | Assert error string contains context | — |
| Context cancellation in loops | **Medium** | Cancel context mid-execution, verify early exit | — |

**Anti-patterns to flag:**

- [ ] **Testing Go stdlib behavior** — don't test that `json.Marshal` works or `os.ReadFile` reads files
- [ ] **Over-mocking internal packages** — if `engine` can call `registry` directly in tests, let it. Mock only external I/O (network, filesystem writes to real paths)
- [ ] **Table-driven tests with complex per-row setup** — if each row needs a different setup function, use separate subtests. Tables are for functions with many input/output pairs, not for orchestrating different scenarios
- [ ] **Testing unexported functions** — test through the public API. If you can't test behavior through the public API, the design needs refactoring
- [ ] **Ignoring the race detector** — every `go test` must run with `-race`

**High-value patterns to verify:**

- [ ] Every node type in `engine/pkg/node/library/` has both unit tests (isolated `Execute()`) and integration tests (fixture workflow)
- [ ] Fixture files in `engine/tests/fixtures/workflows/` cover the recipe catalog — every recipe in `engine/pkg/menu/data/` should have a matching fixture test
- [ ] Golden tests exist for output formats that external consumers depend on (progress messages, execution results)
- [ ] Error paths tested — malformed input, missing files, invalid config, cancelled context

### Layer 2: Go API (`archive/api-go/`)

**Source:** Go community (httptest patterns), Speedscale, Martin Fowler

| What to test | Value | Pattern | Example |
|---|---|---|---|
| Handler request/response contract | **Critical** | `httptest.NewRecorder` + table-driven for auth states | `handler_test.go` |
| Auth enforcement — missing/invalid/valid token | **Critical** | Three subtests per endpoint | `handler_test.go` |
| R2 upload/download flow | **High** | Integration with mock R2 or real temp dir | `run_transit_test.go` |
| CORS headers | **Medium** | Assert headers present on response | `contract_test.go` |
| Error response shape | **Medium** | Assert JSON error body matches contract | — |

**Anti-patterns to flag:**

- [ ] **Testing the HTTP framework** — don't test that `http.StatusOK` equals 200 or that Go can parse JSON
- [ ] **Starting a real server with `ListenAndServe`** — use `httptest.NewServer` or `httptest.NewRecorder` instead
- [ ] **Testing internal handler logic separately from HTTP** — the handler IS the integration. Test it through HTTP

### Layer 3: Convex Backend (`packages/@bnto/backend/`)

**Source:** Convex docs, Convex Stack blog (testing patterns)

| What to test | Value | Pattern | Example |
|---|---|---|---|
| Auth enforcement — mutation rejects unauthenticated/wrong user | **Critical** | `convex-test` without/with wrong identity | `auth.test.ts` |
| Quota boundary — free tier limit (run N ok, run N+1 rejected) | **Critical** | Seed user near limit, test the boundary | `quota.test.ts` |
| Execution state machine — valid and invalid transitions | **High** | Seed execution in state X, attempt transition to Y | `executions.test.ts` |
| Business rules in mutations (validation, side effects) | **High** | `t.mutation()` with controlled inputs | — |
| Schema migration safety — new fields work with existing docs | **Medium** | Insert old-shape doc, run migration, verify | — |

**Anti-patterns to flag:**

- [ ] **Testing that Convex stores and retrieves data** — `t.mutation(insert)` then `t.query(get)` on a pass-through CRUD function is testing Convex, not your code
- [ ] **Testing validator shapes with valid data** — the schema already validates this. Test that *invalid* inputs are rejected if you have custom validation beyond schema validators
- [ ] **Mocking `convex-test` itself** — `convex-test` IS the mock. Using `vi.mock` on top of it means you're testing nothing
- [ ] **Testing simple pass-through queries** — a query that just calls `ctx.db.get(id)` has zero test value
- [ ] **Testing `.withIndex()` ordering** — that's Convex's responsibility

**Convex's own guidance:** "Start by testing the logic that is core to your value proposition, and anything to do with security or accounting."

### Layer 4: Core API (`packages/core/`)

**Source:** TkDodo (React Query maintainer), TanStack docs

| What to test | Value | Pattern | Example |
|---|---|---|---|
| Transform functions (doc -> API type mappers) | **Critical** | Pure function tests — input doc, assert output shape | `execution.test.ts` |
| Service orchestration — correct args passed to adapter | **Medium** | Mock adapter, verify call args | `executionService.test.ts` |
| Error handling paths — adapter throws, transform receives null | **Medium** | Mock adapter to throw, assert service behavior | — |
| Cache invalidation — save() invalidates the right query keys | **Low** | Verify `invalidateQueries` called with correct key | — |

**Anti-patterns to flag:**

- [ ] **Mocking `useQuery` or `useMutation` directly** — this removes all confidence in the integration. Mock at the network/adapter layer instead (TkDodo's explicit recommendation)
- [ ] **Testing that React Query refetches on mount** — that's React Query's behavior, not yours
- [ ] **Testing that `useQuery` returns `isLoading`** — framework behavior
- [ ] **Testing React hooks that just wire `useQuery` to query options** — the service test covers the logic; the hook is pure plumbing
- [ ] **Over-asserting on mock call counts** — `expect(mock).toHaveBeenCalledTimes(1)` is testing interaction, not behavior. Assert on the outcome instead

**TkDodo's rule:** "Don't mock React Query. Mock the network layer."

### Layer 5: Web App Components (`apps/web/`)

**Source:** Kent C. Dodds (Testing Library), Bluesky (thin unit + thick E2E)

| What to test | Value | Pattern | Example |
|---|---|---|---|
| Pure utility functions (formatters, validators) | **High** | Input/output, edge cases | `formatFileSize.test.ts` |
| Route protection logic (proxy/middleware) | **High** | Mock auth state, assert redirects | `middleware.test.ts` |
| Slug registry validation | **High** | Assert all slugs resolve, no collisions | `bntoRegistry.test.ts` |
| Complex hook logic (if extracted) | **Medium** | `renderHook` with controlled inputs | — |
| Simple presentational components | **Zero** | TypeScript validates props. E2E validates rendering | — |
| shadcn primitive wrappers | **Zero** | Radix already tested these | — |
| CSS class application | **Zero** | Visual regression via E2E screenshots | — |

**Anti-patterns to flag:**

- [ ] **Unit-testing every component** — a `<Badge variant="secondary">Draft</Badge>` does not need a test
- [ ] **Testing `useState` behavior** — asserting that clicking a button sets `isOpen` to `true` is testing React, not your code
- [ ] **Snapshot tests of entire component trees** — they break on every change and nobody reads the diffs. E2E screenshots are strictly better
- [ ] **Testing shadcn primitives** — don't test that `<Dialog>` opens and closes. That's Radix's job
- [ ] **Testing CSS** — visual correctness is an E2E concern, not a unit test concern

**Bluesky's pattern:** They unit-test pure utility functions (string manipulation, email validation, number formatting) and don't unit-test React components at all. E2E covers the visual layer.

### Layer 6: E2E (`apps/web/e2e/`)

**Source:** Bluesky (Maestro flows), Playwright docs, Google Testing Blog

| What to test | Value | Pattern | Example |
|---|---|---|---|
| Critical user journeys (drop files -> run -> download) | **Critical** | Journey-based spec with screenshot assertions | `execution-flow.spec.ts` |
| Auth redirects (unauth on protected route) | **High** | Navigate, assert redirect | — |
| SEO (metadata, JSON-LD, sitemap) | **High** | Assert meta tags present and correct | `seo-metadata.spec.ts` |
| Tool page initial state | **High** | Assert config panel, drop zone, disabled run button | `bnto-config.spec.ts` |
| File interactions (add, remove, clear) | **Medium** | Programmatic file input, assert UI updates | `file-drop.spec.ts` |
| Every intermediate UI state | **Low** | Transient states (uploading, running) are flaky to screenshot | — |

**Anti-patterns to flag:**

- [ ] **E2E tests for unit-testable logic** — if a function can be tested with a pure input/output test, don't spin up a browser for it
- [ ] **Flaky screenshot assertions** — screenshots of animated content, dynamic timestamps, or randomly ordered lists. Mask dynamic content, disable animations, sort lists
- [ ] **Testing every state transition** — screenshot the initial state and final state. Intermediate transient states are flaky
- [ ] **Missing `reducedMotion: "reduce"`** — animations make screenshots non-deterministic
- [ ] **Missing error capture** — not using the shared fixture from `fixtures.ts` that captures console/page errors
- [ ] **Page Object Model prematurely** — with 5 spec files, direct semantic selectors are cleaner. Add POM when duplication across 5+ specs demands it

**Bluesky's pattern:** E2E tests read like user stories — setup, actions, assertions. Each flow tests one complete user journey, not individual features. Their mock server spins up a real backend instance per test suite.

**High-value E2E patterns to verify:**

- [ ] Tests use `getByRole()` and `getByText()` (semantic selectors), not CSS classes
- [ ] Tests use `data-testid` for state machine assertions (`data-phase`)
- [ ] Tests import from `./fixtures` (shared error capture), not directly from `@playwright/test`
- [ ] Screenshot assertions exist for primary UI states
- [ ] Tests don't depend on a running backend — UI-only mode with expected failures caught

---

## Step 3: Evaluate Test Quality (Both Modes)

For each test file in scope (changed files in change mode, all tests in landscape mode), evaluate:

### 3a: Is this test testing behavior or implementation?

**Behavior test (GOOD):** "When I execute this workflow with this input, I get this output"
**Implementation test (BAD):** "The engine calls registry.Get() exactly once, then calls node.Execute()"

```
For each test:
  -> Does it assert on OUTCOMES (return values, side effects, output state)?
     YES -> behavior test (keep)
  -> Does it assert on HOW the code works (mock call counts, internal state, private methods)?
     YES -> implementation test (flag for rewrite or deletion)
```

### 3b: What happens if this test is deleted?

Ask for each test: **If I delete this test and the codebase has a real bug, would I notice?**

- If another test (unit, integration, or E2E) would catch the same bug -> this test is **redundant**
- If the compiler/type system would catch it -> this test is **testing the compiler**
- If no other test would catch it -> this test is **valuable**

### 3c: Is this test at the right level?

| Signal | Correct level |
|---|---|
| Testing pure logic with many edge cases | Unit test |
| Testing that two systems work together correctly | Integration test |
| Testing what the user sees and interacts with | E2E test |
| Testing that a button has the right CSS class | **Wrong level — delete or move to E2E screenshot** |
| Testing that a Convex mutation stores data | **Wrong level — this tests Convex** |
| Testing that `useQuery` returns loading state | **Wrong level — this tests React Query** |

### 3d: Is the test isolated?

- [ ] Each test can run independently (no test ordering dependencies)
- [ ] Test data is created fresh per test or per suite, not shared global state
- [ ] Cleanup happens in `afterEach`/`afterAll`/`t.Cleanup()` — no leaked state
- [ ] Go tests use `t.TempDir()` or explicit cleanup for file artifacts
- [ ] Convex tests use fresh `convexTest(schema, modules)` per test suite
- [ ] E2E tests don't depend on state from a previous test

### 3e: Is the test readable?

**Bluesky's standard:** Their Maestro flows read like user stories. Their integration tests use named seed data (`sc.dids.alice`, not `"did:plc:abc123"`).

- [ ] Test names describe behavior, not implementation — "rejects unauthenticated user" not "returns 401"
- [ ] Test data uses descriptive names, not opaque IDs
- [ ] Setup is clear — what state is the world in before the action?
- [ ] Assertion is obvious — what outcome are we verifying?
- [ ] No unnecessary mocks — every mock exists because the real thing can't be used in tests

---

## Step 4: Coverage Gaps (Landscape Mode Only)

In landscape mode, also evaluate structural coverage:

### 4a: Recipe coverage

Every recipe in `engine/pkg/menu/data/` should have a corresponding fixture test in `engine/tests/integration/`. List any recipes without integration tests.

### 4b: Auth enforcement coverage

Every Convex mutation that modifies user data should have a test verifying rejection without auth. List any mutations missing this test.

### 4c: Critical path coverage

These user journeys MUST have E2E coverage. Flag any that are missing:

- [ ] Landing on a tool page (each predefined bnto slug)
- [ ] Dropping files and seeing them listed
- [ ] Clicking Run and observing state transitions
- [ ] Execution failure with "Try Again"
- [ ] Auth redirect (unauth on protected route)

### 4d: Paired valid/invalid test vectors

For the Go engine's validators, verify paired test cases exist:

- [ ] Valid `.bnto.json` fixtures that execute cleanly
- [ ] Invalid `.bnto.json` fixtures that produce specific, tested error messages
- [ ] Each validation rule has both a passing and failing case

---

## Step 5: Review Summary

Present findings organized by severity:

### Wasteful Tests (recommend deletion or rewrite)

Tests that add maintenance cost without catching real bugs:

- **File**: path and line range
- **Issue**: what the test is actually testing (framework behavior, implementation detail, etc.)
- **Recommendation**: delete, rewrite as behavior test, or move to correct layer

### Missing High-Value Tests

Tests that should exist based on the value matrix but don't:

- **Area**: which layer and what code
- **What to test**: specific behavior
- **Pattern**: recommended approach from the value matrix
- **Priority**: Critical / High / Medium

### Anti-Patterns Found

Specific anti-pattern instances from the checklists above:

- **File**: path and line range
- **Anti-pattern**: which pattern from the matrix
- **Fix**: specific recommendation

### Test Health by Layer

```
Go Engine:        X tests | Y fixtures | Coverage: STRONG / ADEQUATE / WEAK
Go API:           X tests | Coverage: STRONG / ADEQUATE / WEAK
Convex Backend:   X tests | Coverage: STRONG / ADEQUATE / WEAK
Core API:         X tests | Coverage: STRONG / ADEQUATE / WEAK
Web App:          X tests | Coverage: STRONG / ADEQUATE / WEAK
E2E:              X specs | Y screenshots | Coverage: STRONG / ADEQUATE / WEAK
```

### Top 3 Actions

The three highest-impact testing improvements, ordered by value-per-effort.

---

## Sources

This skill's guidelines are sourced from:

- **Go:** Dave Cheney (table-driven tests), Learn Go with Tests (anti-patterns, working without mocks), Go Wiki
- **Convex:** Official `convex-test` docs, Convex Stack blog (testing patterns)
- **React Query:** TkDodo's blog (testing React Query), TanStack docs
- **General:** Kent C. Dodds (testing trophy), Guillermo Rauch (write tests, not too many, mostly integration), Google SWE Book (test behaviors not methods), Martin Fowler (practical test pyramid)
- **Bluesky:** `@atproto/dev-env` (real services over mocks), SeedClient (named test data), Maestro flows (declarative E2E), paired valid/invalid test vectors, thin unit + thick integration pattern
- **Anti-patterns:** Codepipes (software testing anti-patterns), DZone (unit testing anti-patterns full list)
