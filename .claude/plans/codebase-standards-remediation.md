# Codebase Standards Remediation Plan

**Created:** February 26, 2026
**Source:** Full codebase audit (Sprint 3 Wave 1)
**Total violations:** 149 (33 HIGH, 59 MEDIUM, 57 LOW)

Each domain section is self-contained — a persona agent can pick up their section and fix everything in it without touching other domains.

---

## Domain 1: Core Architect (`packages/core/`)

**Persona:** `/core-architect`
**Violations:** 4 HIGH, 10 MEDIUM, 16 LOW

### HIGH

- [ ] **T1-H1: Move `@bnto/backend` imports out of transforms.** `transforms/workflow.ts`, `transforms/execution.ts`, `transforms/user.ts` all import `Doc<T>` from `@bnto/backend`. Transforms should receive already-typed data from adapters — only adapters import backend types. Define intermediate raw types in the adapter layer and have transforms accept those instead.

- [ ] **T1-H2: Fix `useAuth` hook to use SessionContext.** `hooks/useAuth.ts` imports `useConvexAuth` directly from `convex/react`, duplicating the subscription that `SessionProvider` already manages. Refactor to use the existing `useSessionStatus()` or `useIsAuthenticated()` from `SessionContext`.

- [ ] **T1-H3: Relocate `AuthUser` and `AuthState` types.** These are defined in `hooks/useAuth.ts` but exported as public API types from `index.ts`. Move to `types/auth.ts` where all other public types live.

- [ ] **T1-H4: Add `browserClient` wrapper.** `core.browser` exposes `browserExecutionService` directly without a client wrapper, unlike every other domain (`workflows`, `executions`, `user`, etc.). Create `createBrowserClient()` for architectural symmetry.

### MEDIUM

- [ ] **T1-M1: Eliminate `as` assertions in services.** `executionService.ts`, `workflowService.ts`, `userService.ts` all use `select: (data: unknown) => ... as Parameters<typeof toFoo>[0]`. Root cause: `convexQuery` returns opaque types. Type the adapter return or use a generic query options pattern to eliminate the casts.

- [ ] **T1-M2: Abstract Convex imports in hooks.** `useAnonymousSession.ts` and `useSignOut.ts` import directly from `convex/react` and `@convex-dev/auth/react`. Evaluate whether these can go through a core abstraction (auth adapter) or document them as pragmatic exceptions.

- [ ] **T1-M3: Abstract Convex imports in providers.** `SessionProvider.tsx` imports `useConvexAuth` from `convex/react`. `BntoCoreProvider.tsx` imports `ConvexAuthNextjsProvider` from `@convex-dev/auth/nextjs`. `client.ts` imports `ConvexReactClient` and `ConvexQueryClient`. These are functionally adapter-layer code but live outside `adapters/`. Either relocate to `adapters/convex/` or document them as bootstrap infrastructure.

- [ ] **T1-M4: Extract React hook from browserExecutionStore.** `stores/browserExecutionStore.ts` exports the factory, singleton, AND a React hook (`useBrowserExecutionStore`). The hook should be in its own file per one-export-per-file rule.

- [ ] **T1-M5: Relocate `AuthStatus` type.** Exported from `providers/SessionContext.ts` but is part of the public API surface. Move to `types/`.

### LOW

- [ ] **T1-L1: Multiple exports in authError.ts.** 3 exports (`onAuthError`, `emitAuthError`, `isAuthError`). Consider splitting `isAuthError` to its own file.
- [ ] **T1-L2: Multiple exports in client.ts.** `getConvexClient` and `getQueryClient` share initialization. Acceptable but note.
- [ ] **T1-L3: Multiple exports in transform files.** `toWorkflow`/`toWorkflowListItem`, `toExecution`/`toExecutionLog`. Same-domain co-location is reasonable.
- [ ] **T1-L4: Multiple exports in adapter files.** `workflowAdapter.ts` (4), `executionAdapter.ts` (5), `userAdapter.ts` (2). Single-domain adapter co-location is reasonable.
- [ ] **T1-L5: Multiple exports in browser adapters.** `engineRegistry.ts` (3), `slugCapability.ts` (3). Singleton/map pattern justifies co-location.
- [ ] **T1-L6: Test file size limits.** `setup.ts` (253 lines), `execution.test.ts` (285 lines), `uploadService.test.ts` (257 lines), `browserExecutionService.test.ts` (295 lines). All test files, barely over limit.
- [ ] **T1-L7: Justified `as ArrayBuffer` in createZipBlob.ts.** Trust boundary — no fix needed, just acknowledge.
- [ ] **T1-L8: `@bnto/backend` imports in test files.** `setup.ts` and `transit-helpers.ts`. Integration tests intentionally test Convex directly — acceptable exception.

---

## Domain 2: Frontend Engineer (`apps/web/`)

**Persona:** `/frontend-engineer`
**Violations:** 8 HIGH, 13 MEDIUM, 8 LOW

### HIGH

- [ ] **T2-H1: Decompose FileUpload.tsx.** 1417 lines — nearly 6x the 500-line max. Contains 10+ components, contexts, reducer, store. Break into a `FileUpload/` folder: store, context, sub-components, assembled via `index.ts` with `Object.assign` dot-notation namespace.

- [ ] **T2-H2: Add dot-notation to FileUpload.** Currently exports flat names (`FileUploadDropzone`, `FileUploadList`, etc.). Assemble as `FileUpload.Dropzone`, `FileUpload.List`, etc.

- [ ] **T2-H3: Add dot-notation to Popover.** Only multi-part primitive without namespace assembly. Create `Object.assign(PopoverRoot, { Trigger, Content, Anchor, ... })`.

- [ ] **T2-H4: Fix FileDropZone flat imports.** Currently imports flat `FileUploadDropzone`, `FileUploadList`, etc. Migrate to `FileUpload.Dropzone`, `FileUpload.List` once T2-H2 is done.

- [ ] **T2-H5: Fix hover/focus parity in not-found.tsx.** `group-hover:-translate-x-1` on line 25 missing `group-focus-within:-translate-x-1`. Add keyboard equivalent.

### MEDIUM

- [ ] **T2-M1: Replace `cva` with `createCn` in Button.tsx.** `cva` from `class-variance-authority` should be `createCn` per component standards.
- [ ] **T2-M2: Replace `cva` with `createCn` in Sheet.tsx.** Same violation.
- [ ] **T2-M3: Replace `cva` with `createCn` in Dropzone.tsx.** Same violation.
- [ ] **T2-M4: Fix direct `next-themes` import in NavThemeMenu.tsx.** Import from `@/components/` wrapper, not `next-themes` directly.
- [ ] **T2-M5: Fix direct `next-themes` import in AnimatedThemeToggle.tsx.** Same violation.
- [ ] **T2-M6: Fix font variable naming in layout.tsx.** Variables are `--font-geist`/`--font-inter` but theming.md says `--font-display`/`--font-sans`. Align with documented convention.
- [ ] **T2-M7: Fix hardcoded `bg-green-500` in ExecutionProgress.tsx.** Replace with semantic success token.
- [ ] **T2-M8: Remove redundant flat exports from Select.tsx.** Has dot-notation via `Object.assign` but ALSO exports flat names. Remove flat exports.
- [ ] **T2-M9: Remove redundant flat exports from Sheet.tsx.** Same pattern.
- [ ] **T2-M10: Remove redundant flat exports from Accordion.tsx.** Same pattern.
- [ ] **T2-M11: Remove redundant flat exports from Tabs.tsx.** Same pattern.
- [ ] **T2-M12: Use `<Heading>` component in BntoPageShell.tsx.** Raw `<h1>` on line 151 should use `<Heading level={1}>`.
- [ ] **T2-M13: Use `<Heading>` component in BntoConfigPanel.tsx.** Raw `<h2>` on line 30 should use `<Heading level={2}>`.

### LOW

- [ ] **T2-L1: Add Geist Mono font loading in layout.tsx.** `theming.md` specifies three fonts including monospace. Currently only Geist + Inter are loaded.
- [ ] **T2-L2: Fix hardcoded `text-green-600` in ExecutionProgress.tsx.** Line 122 — use semantic token.
- [ ] **T2-L3: Fix hardcoded `text-green-600` in ExecutionResults.tsx.** Line 46 — use semantic token.
- [ ] **T2-L4: Fix hardcoded `text-green-600` in BrowserExecutionResults.tsx.** Line 35 — use semantic token.
- [ ] **T2-L5: Fix hardcoded `text-green-600` in UploadProgress.tsx.** Line 79 — use semantic token.
- [ ] **T2-L6: Fix hardcoded `text-white/60` in MobileNavMenu.tsx.** Lines 69, 116, 130 — use `text-primary-foreground/60`.
- [ ] **T2-L7: Extract geometry helpers from RadialSlider.tsx.** 394 lines — math helpers (lines 70-160) could move to a `utils/` file.
- [ ] **T2-L8: Multiple exports in layoutTypes.ts.** `resolveGap`, `alignMap`, `justifyMap` — cohesive layout primitives, borderline acceptable.

---

## Domain 3: Backend Engineer (`packages/@bnto/backend/`)

**Persona:** `/backend-engineer`
**Violations:** 3 HIGH, 12 MEDIUM, 10 LOW

### HIGH

- [ ] **T3-H1: Add auth gate to `generateUploadUrls`.** `uploads.ts` — public action generates R2 presigned URLs without rejecting unauthenticated callers. If `user` is null, it defaults to `plan: "free"` and proceeds. Add auth check that rejects null users.

- [ ] **T3-H2: Fix `resetRunCounters` scaling.** `users.ts` — `.collect()` on entire users table. Add a `by_runsResetAt` index to `schema.ts` and use `.withIndex()` to only fetch users due for reset. Paginate if needed.

- [ ] **T3-H3: Decompose `executeWorkflow` handler.** `executions.ts` lines 175-278 — 103 lines. Extract into helper functions: `callGoApi()`, `pollExecution()`, `handleCompletion()`, `handleFailure()`.

### MEDIUM

- [ ] **T3-M1: Decompose `start` handler.** `executions.ts` lines 27-76 — 49 lines. Extract shared start logic into a helper.
- [ ] **T3-M2: Decompose `startPredefined` handler.** `executions.ts` lines 86-131 — 45 lines. Uses same pattern as `start` — deduplicate via shared helper.
- [ ] **T3-M3: Decompose `createOrUpdateUser` callback.** `auth.ts` lines 103-161 — 58 lines. Extract anonymous upgrade, existing user patch, and new user creation into sub-functions.
- [ ] **T3-M4: Use `ConvexError` consistently.** `workflows.ts` lines 62, 100, 103 use plain `Error` instead of `ConvexError`. Also `executions.ts` line 37. Standardize all mutations to `ConvexError`.
- [ ] **T3-M5: Add missing index for `executions.status/startedAt`.** `cleanup_stale.ts` does `.filter()` on status + startedAt without an index. Add `by_status_startedAt` compound index.
- [ ] **T3-M6: Add missing index for `users.runsResetAt`.** Needed by T3-H2. Add `by_runsResetAt` index to schema.
- [ ] **T3-M7: Guard `.collect()` in `aggregateBySlug`.** `execution_analytics.ts` lines 20-23 — loads ALL user events into memory. Add `.take()` limit or document scaling boundary.
- [ ] **T3-M8: Review fingerprint query visibility.** `listByFingerprint` and `countByFingerprint` are public queries that accept arbitrary fingerprints. Evaluate whether these should be `internalQuery` to prevent enumeration.
- [ ] **T3-M9: Reduce `executions.ts` file size.** 416 lines. After decomposing handlers (T3-H3, T3-M1, T3-M2), extract internal action + poll logic to `execution_engine.ts`.
- [ ] **T3-M10: File size reduction for `executions.ts`.** Target: split into `executions.ts` (public mutations/queries) + `execution_engine.ts` (internal action + poll).
- [ ] **T3-M11: Inconsistent `Error` in `workflows.ts`.** Convert 3 instances of `new Error(...)` to `new ConvexError(...)` with appropriate error codes.
- [ ] **T3-M12: Stale cleanup `.filter()`.** `cleanup_stale.ts` lines 25-36 — full table scan on `executions` for status + startedAt. Use index from T3-M5.

### LOW

- [ ] **T3-L1: Guard `.collect()` in `summaryByDateRange`.** `execution_analytics.ts` — wide date ranges could produce large result sets. Add `.take()` safety limit.
- [ ] **T3-L2: Guard `.collect()` in `countByFingerprint`.** `execution_events.ts` — could accumulate for heavy anonymous users. Only count is needed.
- [ ] **T3-L3: Rename `executionLogs.ts` to `execution_logs.ts`.** Inconsistent with codebase snake_case convention for multi-word Convex files.
- [ ] **T3-L4: Remove redundant return type in `downloads.ts`.** Line 36 — `Promise<DownloadUrlsResult>` can be inferred.
- [ ] **T3-L5: Remove redundant return type in `cleanup_stale.ts`.** Line 70 — `Promise<{ cleaned: number }>` can be inferred.
- [ ] **T3-L6: `v.any()` on definition/result fields.** `schema.ts` lines 38, 60 — justified for opaque blobs but worth noting.
- [ ] **T3-L7: Duplicate event logging paths.** `executions.start/startPredefined` insert events directly AND `execution_events.logStart` exists as a separate public mutation. Document which path is for which flow.
- [ ] **T3-L8: `any` casts in PasswordWithAnonymousUpgrade.** `auth.ts` lines 47, 53, 56, 59 — all have eslint-disable + justification. Trust boundary, accepted.
- [ ] **T3-L9: `Record<string, any>` in auth.ts.** Line 122 — could be `Partial<{email, name, image, isAnonymous, runLimit}>`.
- [ ] **T3-L10: `complete` handler slightly over limit.** `executions.ts` lines 318-341 — 23 lines. Within 30-line justification window.

---

## Domain 4: Rust Expert (`engine/crates/`)

**Persona:** `/rust-expert`
**Violations:** 14 HIGH, 17 MEDIUM, 12 LOW

### HIGH

- [ ] **T4-H1: Make `ProgressReporter` target-agnostic.** `bnto-core/src/progress.rs` wraps `js_sys::Function` directly. All node crates inherit this WASM dependency. Replace with a generic callback trait (`Fn(f32)` or custom trait). Move JS-specific implementation to `bnto-wasm`.

- [ ] **T4-H2: Decompose `clean.rs` process().** 313 lines (15.6x limit). Extract: `parse_params()`, `parse_csv()`, `apply_transforms()` (trim, dedup, remove empty), `encode_output()`, `build_metadata()`.

- [ ] **T4-H3: Decompose `rename_columns.rs` process().** 217 lines (10.8x limit). Extract: `parse_params()`, `parse_csv()`, `apply_renames()`, `encode_output()`, `build_metadata()`.

- [ ] **T4-H4: Decompose `rename.rs` process().** 158 lines (7.9x limit). Extract: `parse_params()`, `apply_pattern()`, `apply_transforms()` (case, prefix, suffix, date, counter), `build_metadata()`.

- [ ] **T4-H5: Decompose `resize.rs` process().** 123 lines (6.1x limit). Extract: `parse_params()`, `decode_image()`, `calculate_and_resize()`, `encode_output()`, `build_metadata()`.

- [ ] **T4-H6: Decompose `convert.rs` process().** 94 lines (4.7x limit). Extract: `parse_params()`, `decode_image()`, `encode_to_format()`, `build_metadata()`.

- [ ] **T4-H7: Decompose `compress.rs` process().** 87 lines (4.3x limit). Extract: `parse_params()`, `decode_image()`, `compress_to_format()`, `build_metadata()`.

- [ ] **T4-H8: Split `compress.rs` tests.** 1,965 total lines (1,517 test). Extract `#[cfg(test)]` module into `compress_tests.rs` or a `tests/` submodule.

- [ ] **T4-H9: Split `resize.rs` tests.** 1,233 total lines (702 test). Library code at 531 lines exceeds 500-line hard max even without tests.

- [ ] **T4-H10: Split `convert.rs` tests.** 1,070 total lines (581 test). Library code at 489 lines near hard max.

- [ ] **T4-H11: Split `clean.rs` tests.** 1,038 total lines (573 test). Library code at 465 lines.

- [ ] **T4-H12: Split `rename.rs` tests.** 1,011 total lines (492 test). Library code at 519 lines exceeds 500-line hard max.

### MEDIUM

- [ ] **T4-M1: Move `From<BntoError> for JsValue` out of bnto-core.** `errors.rs` line 107 — this couples the error type to WASM. Move to `bnto-wasm` or a bridge module.
- [ ] **T4-M2: Deduplicate WASM bridge boilerplate.** Three `wasm_bridge.rs` files (image, csv, file) follow identical patterns: parse JSON, build NodeInput, call process(), serialize. Create a generic bridge function or macro in `bnto-wasm` to replace ~1,071 lines with ~200.
- [ ] **T4-M3: Decompose `convert.rs` validate().** 52 lines (2.6x limit). Nested match statements.
- [ ] **T4-M4: Decompose `resize.rs` validate().** 48 lines (2.4x limit).
- [ ] **T4-M5: Decompose `resize.rs` calculate_dimensions().** ~80 lines. Heavily commented but oversized.
- [ ] **T4-M6: Decompose `format.rs` from_magic_bytes().** 65 lines (3.25x). Large match on byte signatures.
- [ ] **T4-M7: Decompose `orientation.rs` decode_with_orientation().** 39 lines.
- [ ] **T4-M8: Decompose `orientation.rs` extract_orientation().** 36 lines. EXIF parsing.
- [ ] **T4-M9: Reduce `wasm_bridge.rs` (image) file size.** 484 lines, no tests. Near 500-line hard max.
- [ ] **T4-M10: Decompose `wasm_bridge.rs` compress_image().** 73 lines.
- [ ] **T4-M11: Decompose `wasm_bridge.rs` resize_image().** 46 lines.
- [ ] **T4-M12: Decompose `wasm_bridge.rs` convert_image_format().** 46 lines.
- [ ] **T4-M13: Reduce `rename_columns.rs` file size.** 835 total (418 library). Above 250-line soft limit.
- [ ] **T4-M14: Reduce `wasm_bridge.rs` (csv) file size.** 352 lines, above 250-line soft limit.
- [ ] **T4-M15: Decompose `wasm_bridge.rs` (csv) clean_csv().** 73 lines.

### LOW

- [ ] **T4-L1: `processor.rs` slightly over file limit.** 308 lines total (183 library + 125 test). Acceptable.
- [ ] **T4-L2: `format.rs` total over 500.** 571 lines but library (233) is within limit. Tests are thorough.
- [ ] **T4-L3: `orientation.rs` total over 250.** 441 lines but library (207) within limit.
- [ ] **T4-L4: `compress.rs` compress_png() slightly over.** 32 lines. Mostly comments.
- [ ] **T4-L5: `compress.rs` get_quality() at 29 lines.** Within justification window. 19 lines are comments.
- [ ] **T4-L6: `test_utils.rs` inject_exif_orientation() at 87 lines.** Test-only utility, binary construction. Splitting would reduce readability.
- [ ] **T4-L7: `test_utils.rs` create_test_jpeg() at 29 lines.** Test utility, slightly over.
- [ ] **T4-L8: `rename.rs` split_filename() at 26 lines.** Slightly over, well-commented.
- [ ] **T4-L9: `rename.rs` to_title_case() at 22 lines.** Slightly over.
- [ ] **T4-L10: `rename.rs` get_current_date() at 25 lines.** Slightly over.
- [ ] **T4-L11: `wasm_bridge.rs` (csv) rename_csv_columns() at 46 lines.** Bridge boilerplate.
- [ ] **T4-L12: `wasm_bridge.rs` (file) rename_file() at 73 lines.** Bridge boilerplate, addressed by T4-M2.

---

## Domain 5: Auth + Nodes (`packages/@bnto/auth/`, `packages/@bnto/nodes/`)

**Persona:** `/backend-engineer` (auth) + general (nodes)
**Violations:** 4 HIGH, 7 MEDIUM, 11 LOW

### HIGH

- [ ] **T5-H1: Add tests to `@bnto/auth`.** Zero test coverage on security-critical code. Write tests for `useSession`, `useSignIn`, `useSignUp`, `useSignOut` hooks.

- [ ] **T5-H2: Clean stale `dist/` artifacts in `@bnto/auth`.** `dist/` contains functions (`useAuth`, `useCurrentUser`, `useIsWhitelisted`) that don't exist in source, plus a direct `@bnto/backend` import. Delete stale dist and ensure build produces clean output.

- [ ] **T5-H3: Deduplicate validation constants in `@bnto/nodes`.** `validateTypeSpecific.ts` defines `VALID_HTTP_METHODS`, `VALID_LOOP_MODES`, `VALID_FILE_OPERATIONS` as `Set` objects, duplicating values from schema files. Import from schemas instead.

- [ ] **T5-H4: Fix `fileSystem.ts` conditional rules.** `source` and `dest` are only conditionally required/visible for `copy`, but `move` also requires them. Fix `visibleWhen`/`requiredWhen` to cover both operations. May require extending the `requiredWhen` type to support arrays.

### MEDIUM

- [ ] **T5-M1: Delete dead `middleware.ts` in `@bnto/auth`.** Duplicates exports from `server.ts`. Nothing imports from `@bnto/auth/middleware`. Remove file and `./middleware` entry from `package.json`.
- [ ] **T5-M2: Evaluate `@bnto/auth` direct imports from app code.** `proxy.ts` and `layout.tsx` import directly from `@bnto/auth/server`. Architecture says core internals only. These are Next.js server integrations that arguably MUST be at app level. Document as pragmatic exception or route through core.
- [ ] **T5-M3: Reduce exports in `nodeTypes.ts`.** 11 exports from one file. Extract the 4 functions (`isNodeType`, `getNodeTypeInfo`, `getBrowserCapableTypes`, `getContainerTypes`) into separate files.
- [ ] **T5-M4: Reduce exports in `validateTypeSpecific.ts`.** 6 exported functions. Each validator could be its own file.
- [ ] **T5-M5: Reduce exports in `schemas/index.ts`.** 15+ exports including 4 helper functions. Extract helpers to own files.
- [ ] **T5-M6: Fix `requiredWhen`/`visibleWhen` type limitation.** `schemas/types.ts` only supports single conditions. Extend to support arrays for multi-condition rules (needed by T5-H4).
- [ ] **T5-M7: Add parameters for `batch` image operation.** `schemas/image.ts` includes `"batch"` in operations but has no batch-specific parameters. Either add parameters or remove from enum.

### LOW

- [ ] **T5-L1: `server.ts` exports 5 items.** Re-export barrel, acceptable.
- [ ] **T5-L2: `useSession` returns `{ user: null }` placeholder.** Misleading shape but functional.
- [ ] **T5-L3: `index.ts` barrel exports 6 items.** Standard barrel pattern.
- [ ] **T5-L4: `@bnto/nodes/src/index.ts` barrel exports 30+ items.** Large barrel, worth noting for bundle-conscious consumers.
- [ ] **T5-L5: Multiple type exports in `definition.ts`.** 6 interfaces forming a cohesive `.bnto.json` schema. Reasonable co-location.
- [ ] **T5-L6: Multiple exports in `validate.ts`.** 3 functions + 1 type. `validateWorkflow` is a thin wrapper for `validateDefinition`.
- [ ] **T5-L7: `validate.test.ts` at 423 lines.** Test file, within 500-line justification.
- [ ] **T5-L8: `recipes.ts` re-exports 6 individual recipes.** Convenience re-exports alongside function + constant.
- [ ] **T5-L9: Magic numbers in recipe files.** `quality: 80`, `width: 200`, position coordinates. Configuration data, tolerable.
- [ ] **T5-L10: No linter configured for `@bnto/auth` or `@bnto/nodes`.** `lint` script is `echo "No linter yet"`.
- [ ] **T5-L11: No linter for `@bnto/nodes`.** Same.

---

## Execution Strategy

Each domain can be worked in parallel by an agent with the appropriate persona. Dependencies:

- **T5-H4 depends on T5-M6** (need array support in `requiredWhen` type before fixing fileSystem schema)
- **T2-H4 depends on T2-H2** (FileDropZone can't use dot-notation until FileUpload has it)
- **T4-H2 through T4-H7 should be done before T4-H8 through T4-H12** (decompose process() before splitting test files)
- **T3-H2 depends on T3-M6** (add index before using it)
- **T3-M12 depends on T3-M5** (add index before using it)

All other tasks within each domain are independent.
