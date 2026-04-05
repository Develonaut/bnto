# Decision: Implicit Iteration — Engine Architecture Standard

**Created:** March 2026  
**Status:** Decided — full codebase audit complete, TDD-first implementation sprint issued  
**Applies to:** All execution paths — browser WASM, CLI, TUI, server, future runtimes  
**Related sprint:** PLAN.md → Sprint 4H

---

## The Principle

**"Process all input files through each pipeline node" is business logic.**

It must live in a runtime-agnostic layer that every runtime inherits automatically — and it must be proven correct by tests that run in pure Node.js with no browser, no WASM, no Web Worker. If the executor is bulletproof at the core level, any runtime can be placed in front of it and trusted.

This is not just about the `processFiles()` loop. It is the general rule for all orchestration logic. Anything that describes _how a recipe executes_ rather than _how a single file gets processed_ is orchestration. Orchestration belongs in one shared, independently-testable layer.

---

## The Problem (General Form)

Bnto runs on multiple runtimes: browser WASM today, CLI and TUI soon, server-side later. Each runtime needs to:

1. Take a recipe definition and a set of input files
2. Walk the nodes in order
3. For each processing node, run every file through it
4. Chain outputs between nodes
5. Report progress

If any of this logic lives inside a runtime adapter (browser worker wrapper, CLI runner, HTTP handler), it must be reimplemented — or forgotten — every time a new runtime is added. Logic that isn't shared isn't a system. It's copy-paste waiting to drift.

---

## Full Codebase Audit — What Lives Where Today

### ✅ Correct: Rust Engine (`bnto-core/src/processor.rs`)

`NodeProcessor::process()` takes exactly one `NodeInput`, returns one `NodeOutput`. Atomic, correct, single-file, always. Never changes. The existing unit tests in `processor.rs` confirm this with `EchoProcessor` and `FailProcessor` mock impls — exactly the pattern `executePipeline.test.ts` must follow.

### ✅ Correct: WASM Bridge (`bnto-image/src/wasm_bridge.rs`)

One exported function per node type, one file in, one result out. No iteration at this level. The existing `wasm.rs` integration tests confirm single-file semantics at the WASM boundary.

### ✅ Correct: `executionInstance.ts`

Lifecycle management (start/complete/fail/abort), progress throttling, store wiring. Scoped to a single execution run. The `execute` function it wraps is injected — it does not own the iteration loop. The existing store tests (`executionInstanceStore.test.ts`) are comprehensive and serve as the model for `executePipeline.test.ts` coverage depth.

### ✅ Correct: `executionService.ts`

Thin Convex adapter. Server-side execution lifecycle mutations and queries. No business logic.

### ✅ Correct: `executionClient.ts`

Unified public API facade. Wraps instances with history recording. Correct abstraction level — delegates to browser service, doesn't reimplement orchestration.

### ⚠️ VIOLATION: `BrowserEngine` interface (`types/browser.ts`)

`processFiles()` is a required method on the engine interface:

```typescript
export interface BrowserEngine {
  processFile(...): Promise<BrowserFileResult>;
  processFiles(...): Promise<BrowserFileResult[]>;  // ← iteration on the interface
  terminate(): void;
  readonly isReady: boolean;
}
```

Multi-file iteration is now a contract every engine implementation must satisfy. A CLI `NodeRunner` would have to implement this, or it breaks the interface. The engine interface must describe single-file processing only.

### ⚠️ VIOLATION: `BntoWorker.ts`

`processFiles()` contains the `for` loop directly in the browser-specific worker wrapper:

```typescript
async processFiles(files: File[], nodeType: string, params, onProgress) {
  const results: ProcessResult[] = [];
  for (let i = 0; i < files.length; i++) {
    const result = await this.processFile(files[i], nodeType, params, fileProgress);
    results.push(result);
  }
  return results;
}
```

This is business logic masquerading as adapter code. A CLI has no `BntoWorker`. It would need to know to write this loop itself — or it silently breaks multi-file behavior.

### ⚠️ VIOLATION: `toBrowserEngine.ts`

Passes `processFiles` through to satisfy the bad interface. Downstream of the `BntoWorker` violation — fixing the root fixes this.

### ⚠️ VIOLATION: `browserExecutionService.ts`

`execute()` calls `browserEngine.processFiles()`:

```typescript
return browserEngine.processFiles(files, nodeType, params, onProgress ? ... : undefined);
```

The slug → node type mapping (correct, stays here) is conflated with the multi-file loop (incorrect, moves to executor).

### 🔴 MISSING: Pipeline Executor

There is no `executePipeline()` function anywhere in the codebase. "A recipe is a sequence of nodes, each applied to all current files" does not exist as a named, testable, runtime-agnostic unit. When the editor wires execution (Sprint 5 Wave 3), it will build on the wrong foundation unless this is extracted first.

---

## The Correct Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│  Runtime Adapters                                                │
│  browser: BntoWorker → provides NodeRunner via processFile()     │
│  cli: RustCli → provides NodeRunner via process()                │
│  server: HttpRunner → provides NodeRunner via httpClient.post()  │
│  test: vi.fn() → provides NodeRunner via mock                    │
│                                                                  │
│  Each adapter provides exactly ONE thing:                        │
│    NodeRunner: (file, nodeType, params) => Promise<FileResult>   │
└────────────────────────────┬─────────────────────────────────────┘
                             │ inject NodeRunner
┌────────────────────────────▼─────────────────────────────────────┐
│  Pipeline Executor  (packages/core/src/engine/ — runtime-agnostic)│
│                                                                  │
│  executePipeline(recipe, files, runNode, onProgress)             │
│                                                                  │
│  OWNS:                                                           │
│  • Walking nodes in recipe order                                 │
│  • Iterating every file through each processing node (forEach)   │
│  • Chaining output files between nodes                           │
│  • Aggregating progress: (nodeIndex, fileIndex, percent)         │
│  • Explicit loop node override behavior (future)                 │
│                                                                  │
│  DOES NOT OWN:                                                   │
│  • How a single file gets processed (that's NodeRunner)          │
│  • UI state management (that's ExecutionInstance + store)        │
│  • History recording (that's ExecutionClient)                    │
│  • Browser APIs, WASM imports, Worker references (that's the     │
│    browser adapter — zero platform imports allowed here)         │
└────────────────────────────┬─────────────────────────────────────┘
                             │ calls
┌────────────────────────────▼─────────────────────────────────────┐
│  Node Processors  (Rust/WASM, native binary, future runtimes)    │
│                                                                  │
│  NodeProcessor::process(NodeInput) → NodeOutput                  │
│  One file. One result. Always.                                   │
└──────────────────────────────────────────────────────────────────┘
```

### The `NodeRunner` contract

```typescript
// packages/core/src/engine/types.ts — no browser imports, no WASM, no Worker
export type NodeRunner = (
  file: FileInput,
  nodeType: string,
  params: Record<string, unknown>,
  onProgress?: (percent: number, message: string) => void,
) => Promise<FileResult>;
```

The only runtime-specific piece. Every adapter implements this one function. Everything else is shared.

### The `executePipeline` contract

```typescript
// packages/core/src/engine/executePipeline.ts — runtime-agnostic
export async function executePipeline(
  recipe: PipelineDefinition,
  files: FileInput[],
  runNode: NodeRunner,
  onProgress?: PipelineProgressCallback,
): Promise<PipelineResult>;
```

One function. One place. All iteration semantics live here. Every runtime passes its `NodeRunner` and gets the same behavior.

### How runtimes plug in

```typescript
// Browser
const runNode: NodeRunner = (file, nodeType, params, onProgress) =>
  worker.processFile(file, nodeType, params, onProgress);
await executePipeline(recipe, files, runNode, onProgress);

// CLI (future)
const runNode: NodeRunner = (file, nodeType, params, onProgress) =>
  rustCli.process(file, nodeType, params, onProgress);
await executePipeline(recipe, files, runNode, onProgress);

// Test — zero setup, pure TypeScript
const runNode: NodeRunner = vi.fn().mockResolvedValue(mockFileResult);
await executePipeline(recipe, files, runNode);
```

---

## What the `.bnto.json` Format Means

The format is a description of **intent**, not control flow. A recipe without a `loop` node is valid and complete:

```json
{
  "nodes": [
    { "id": "input", "type": "input" },
    { "id": "compress", "type": "compress-images", "params": { "quality": 80 } },
    { "id": "output", "type": "output" }
  ],
  "order": ["input", "compress", "output"]
}
```

The pipeline executor applies implicit forEach over all files at each processing node. **No loop node needed for the default case.**

The explicit `loop` node (future) overrides this for power users who want parallelism, per-item filtering, or custom error handling. Same output semantics, different control flow. Not a requirement — an escape hatch.

---

## TDD Requirement

The pipeline executor is the most critical piece of new business logic in the system. It must have comprehensive tests before any feature is built on top of it.

**Test file:** `packages/core/src/engine/executePipeline.test.ts`

**Test runtime:** `vitest run` — pure Node.js, no browser environment (matches existing `vitest.config.ts` with `environment: "node"`). Zero WASM. Zero Worker setup. Just TypeScript and mock functions.

**Pattern:** Follow the model set by `executionInstanceStore.test.ts` — exhaustive coverage of happy path, error cases, edge cases, and documented boundary behavior.

**Coverage requirements (all must be green before Sprint 5 Wave 3 starts):**

| Area                 | What to prove                                                              |
| -------------------- | -------------------------------------------------------------------------- |
| Single node          | All files processed through the one node, results collected                |
| Multi-node           | Files chained correctly between nodes, intermediate outputs passed forward |
| I/O node skipping    | `input` and `output` nodes do not call `runNode`                           |
| Node failure         | Error in `runNode` rejects the pipeline with clear error                   |
| Empty file array     | Returns empty results cleanly                                              |
| Single file          | Behaves identically to multi-file with one file                            |
| Progress aggregation | `onProgress` fires with correct (nodeIndex, fileIndex, percent)            |
| `runNode` call count | Equals `(processingNodes.length × files.length)` — provably correct        |
| Order guarantee      | Files processed in input order, nodes applied in recipe order              |

---

## Rules (Permanent)

### ✅ Always

1. **`NodeProcessor::process()` is single-file. Always.** The Rust engine processes one item at a time. This is correct and protected.

2. **`NodeRunner` is the single-file contract for all runtimes.** One function, one file, one result. Every runtime adapter implements this. Nothing else.

3. **`executePipeline()` owns all orchestration.** The forEach, the node walk, the output chaining, the progress aggregation. One place. All runtimes delegate here.

4. **`BrowserEngine` interface exposes `processFile` only.** Not `processFiles`. Iteration is not an engine concern.

5. **`BntoWorker` has no `processFiles()` method.** It exposes `processFile()`. The loop lives in `executePipeline()`.

6. **Pipeline executor tests use a mock `NodeRunner`.** No WASM. No Worker. No browser. Pure TypeScript in Node.js. These tests are the source of truth that the executor is correct.

7. **Test coverage gates Sprint 5 Wave 3.** The editor executor wiring does not start until `executePipeline.test.ts` is fully green. This is the dependency, not a suggestion.

### ❌ Never

- Don't put iteration logic in runtime adapters. Not in `BntoWorker`, not in a CLI runner, not in a server handler.
- Don't add `processFiles()` to the `BrowserEngine` interface.
- Don't add implicit loop nodes to `.bnto.json` on save. The flat format is canonical.
- Don't require a loop node for multi-file processing.
- Don't start Sprint 5 Wave 3 before Sprint 4H Wave 3 (all tests green) is complete.

---

## Exact File Changes Required

| File                                                         | Change                                                                                                                |
| ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------- |
| `packages/core/src/engine/types.ts`                          | **NEW** — `NodeRunner`, `FileInput`, `FileResult`, `PipelineDefinition`, `PipelineProgressCallback`, `PipelineResult` |
| `packages/core/src/engine/executePipeline.ts`                | **NEW** — runtime-agnostic pipeline executor                                                                          |
| `packages/core/src/engine/executePipeline.test.ts`           | **NEW** — comprehensive unit tests, mock `NodeRunner`, pure Node.js                                                   |
| `packages/core/src/types/browser.ts`                         | Remove `processFiles()` from `BrowserEngine` interface                                                                |
| `packages/core/src/adapters/browser/BntoWorker.ts`           | Remove `processFiles()` method                                                                                        |
| `packages/core/src/adapters/browser/toBrowserEngine.ts`      | Remove `processFiles` pass-through                                                                                    |
| `packages/core/src/adapters/browser/BntoWorker.test.ts`      | Remove `processFiles` tests; iteration semantics proven by `executePipeline.test.ts`                                  |
| `packages/core/src/services/browserExecutionService.ts`      | Replace `engine.processFiles()` with `executePipeline(singleNodeRecipe, files, runNode)`                              |
| `packages/core/src/services/browserExecutionService.test.ts` | Replace `processFiles` mocks with `processFile` mocks; verify `executePipeline` is called                             |
| `packages/core/src/index.ts`                                 | Export `executePipeline` and `NodeRunner` for Sprint 5 Wave 3 and future CLI                                          |

---

## Layer Summary

| Layer                                       | What it does                     | Owns iteration?               | Tested by                                         |
| ------------------------------------------- | -------------------------------- | ----------------------------- | ------------------------------------------------- |
| Rust engine (`NodeProcessor`)               | Processes one file               | ❌ single-file                | `processor.rs` unit tests, WASM integration tests |
| WASM bridge (`wasm_bridge.rs`)              | Exports Rust fns to JS           | ❌ one call = one file        | `wasm.rs` tests per crate                         |
| Web Worker (`bnto.worker.ts`)               | Runs WASM in background thread   | ❌ one request at a time      | Worker message flow tests                         |
| Browser adapter (`BntoWorker`)              | Worker lifecycle + `processFile` | ❌ single-file only after fix | `BntoWorker.test.ts`                              |
| **Pipeline Executor** (`executePipeline`)   | **Walks recipe, iterates files** | **✅ YES — sole owner**       | **`executePipeline.test.ts`**                     |
| Browser service (`browserExecutionService`) | Slug routing + engine init       | ❌ delegates to executor      | `browserExecutionService.test.ts`                 |
| Execution instance (`executionInstance`)    | Lifecycle + progress throttle    | ❌ wraps execute fn           | `executionInstanceStore.test.ts`                  |
| Execution client (`executionClient`)        | Public API + history             | ❌ delegates to instance      | `executionService.test.ts`                        |
