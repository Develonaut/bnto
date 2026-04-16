# Engine-Owned Node Schema

**Status:** PR 1 in flight (Wave 1 of 3)
**Last Updated:** April 16, 2026
**Tracks:** Sprint 11 — Engine-Owned Node Schema + TUI Schema-Driven Config

**Plan doc:** [.claude/plans/inherited-watching-hennessy.md](../plans/inherited-watching-hennessy.md) — full 7-PR split, files, deletion surface, verification.

---

## Context & Goal

The node config field schema — the metadata that drives sliders, selects, toggles, groups, suffixes, visibility rules, and every rendered form control in both the web editor and TUI — is currently split across two sources of truth:

1. **Engine (Rust)** owns the domain contract for 13 processors: `name`, `label`, `description`, `param_type`, `default`, `constraints`, `placeholder`, `visible_when`, `required_when`, `surfaceable` (10 fields on `ParameterDef` in `engine/crates/bnto-core/src/metadata.rs`). The 7 IO/container/data node types (`input`, `output`, `loop`, `group`, `transform`, `parallel`, `editFields`) have `NodeTypeInfo` but no `ParameterDef` at all.
2. **Web (`@bnto/nodes`)** hand-writes a mix of (a) presentation overlays for engine-backed processors (~228 LOC across 8 files — `presets`, `suffix`, `group`, enum `options` with labels, `control` override, `accept`, `inverted`), (b) full Zod + `NodeParamField` schemas for 7 IO/container/data nodes (~371 LOC), (c) runtime Zod→control inference in `inferFieldType.ts` (~211 LOC), and (d) the `NodeSchema`/`NodeParamField` type shapes in `types.ts`.

**Goal:** Make the Rust engine the single source of truth for node config field schemas AND `.bnto.json` document types, end-to-end. `@bnto/nodes` collapses to a barrel over engine-generated code — no hand-written overlays, no runtime control inference, no hand-written document shapes, no hand-written field types.

**Outcome:** ~930 LOC deleted from `@bnto/nodes`. Both web and TUI consume the same engine-generated schema. Adding a new node or param = one change in Rust + `task wasm:codegen`, done.

---

## Single Source of Truth Principle

Every field the UI needs (`control`, `presets`, `suffix`, `group`, `options`, `visible_when`, `accept`, `description`, `constraints`, `placeholder`, `inverted`) comes from the engine catalog. No web-specific or framework-specific data lives in `@bnto/nodes`.

**Nothing in `@bnto/nodes` is web-specific.** Every field is platform-agnostic:

- `control: "slider" | "select" | "switch" | "file" | …` — a generic string identifier
- `suffix`, `group`, `presets`, `options`, `inverted`, `visibleWhen`, `accept` — pure data

Platform-specific mapping happens at the **consumer** layer, not in the schema:

- **React consumer (`@bnto/form`):** `controlType → React component` (e.g., `"slider" → <Slider>`, `"select" → <Select>`)
- **TUI consumer (`engine/crates/bnto/src/tui/screens/controls/`):** `controlType → ratatui widget` (e.g., `"slider" → bounded number widget`, `"select" → cycling enum widget`)

Both consumers ingest the **same** engine-generated schema. The schema itself describes "slider, 0–100, preset at 80" once; the mapping to a `<Slider>` component or a `[====o====]` widget is a rendering concern owned by each consumer.

---

## Platform-Agnostic `control` Field

The engine emits a generic string identifier. Consumers map it to their widget.

| `control` value      | Meaning                                    | `@bnto/form` (React) | TUI `controls/` (Rust)    |
| -------------------- | ------------------------------------------ | -------------------- | ------------------------- |
| `"slider"`           | Bounded number with presets                | `<Slider>`           | bounded number widget     |
| `"select"`           | Constrained enum choice                    | `<Select>`           | cycling `◀ value ▶`       |
| `"switch"`           | Boolean toggle                             | `<Switch>`           | `[x]`/`[ ]`               |
| `"file"`             | File upload / picker                       | `<FileInput accept>` | file picker (scrolling)   |
| `"textarea"`         | Multi-line text                            | `<Textarea>`         | multi-line text input     |
| `"positionGrid"`     | 3x3 alignment picker (overlay)             | `<PositionGrid>`     | 3x3 grid widget (planned) |
| `"watermarkPreview"` | Synthetic preview field (no param binding) | `<WatermarkPreview>` | hidden in TUI             |
| `"tagPicker"`        | Array-of-strings editor                    | `<TagPicker>`        | multi-line text input     |
| `"keyValue"`         | Record-of-strings editor                   | `<KeyValueEditor>`   | TBD                       |

Where no `control` is set, the consumer falls back to a default derived from `param_type`: `Number` → slider (if bounded) else numeric input, `String` → text input, `Boolean` → switch, `Enum` → select, `Array(String)` → tagPicker, `Record(String)` → keyValue. This fallback is computed at codegen time (see Type Origin Map).

---

## Type Origin Map

Every TypeScript type exported from `@bnto/nodes` with its origin and `ts-rs` source struct.

| TypeScript type     | Before (hand-written)                  | After (engine-generated)        | `ts-rs` source struct       |
| ------------------- | -------------------------------------- | ------------------------------- | --------------------------- |
| `NodeSchema`        | `schemas/types.ts`                     | `generated/types.ts`            | `NodeMetadata` (derived)    |
| `NodeParamField`    | `schemas/types.ts`                     | `generated/types.ts`            | `ParameterDef`              |
| `NodeParamControl`  | inferred in `inferFieldType.ts`        | `generated/types.ts` (explicit) | emitted from `control` enum |
| `SelectOption`      | `schemas/types.ts`                     | `generated/types.ts`            | `OptionEntry`               |
| `PresetEntry`       | `schemas/types.ts`                     | `generated/types.ts`            | `PresetEntry`               |
| `VisibleWhenClause` | `schemas/types.ts`                     | `generated/types.ts`            | `ParamCondition`            |
| `Constraints`       | `schemas/types.ts`                     | `generated/types.ts`            | `Constraints`               |
| `NodeTypeInfo`      | mix (generated + hand-written overlay) | `generated/types.ts`            | `NodeTypeInfo`              |
| `Definition`        | `definition.ts` (hand-written)         | `generated/definition.ts`       | `Definition` (new struct)   |
| `Edge`              | `definition.ts`                        | `generated/definition.ts`       | `Edge`                      |
| `Port`              | `definition.ts`                        | `generated/definition.ts`       | `Port`                      |
| `Metadata`          | `definition.ts`                        | `generated/definition.ts`       | `Metadata`                  |
| `Recipe`            | `recipe.ts` (hand-written)             | `generated/definition.ts`       | `Recipe`                    |
| `AcceptSpec`        | `recipe.ts`                            | `generated/definition.ts`       | `AcceptSpec`                |

**Deleted entirely after Wave 1:**

- `packages/@bnto/nodes/src/schemas/imageCompress.ts`, `imageConvert.ts`, `imageOverlay.ts`, `imageResize.ts`, `imageStripExif.ts`, `fileRename.ts`, `spreadsheetClean.ts`, `spreadsheetRename.ts` (8 files, ~228 LOC — processor overlays)
- `packages/@bnto/nodes/src/schemas/input.ts`, `output.ts`, `loop.ts`, `group.ts`, `transform.ts`, `parallel.ts`, `editFields.ts` (7 files, ~371 LOC — IO/container/data schemas)
- `packages/@bnto/nodes/src/schemas/inferFieldType.ts` (~211 LOC — runtime Zod→control inference, moves to codegen)
- `packages/@bnto/nodes/src/schemas/types.ts` — type shapes now engine-emitted
- `packages/@bnto/nodes/src/schemas/engineSchemaEntries.ts` — overlay merge obsolete
- `packages/@bnto/nodes/src/definition.ts` (~30 LOC) and `packages/@bnto/nodes/src/recipe.ts` (~20 LOC) — document-shape types now engine-emitted via `ts-rs`

**Collapsed to a barrel:**

- `packages/@bnto/nodes/src/schemas/registry.ts` — 5-line `Map` over generated entries
- `packages/@bnto/nodes/src/schemas/index.ts` — re-exports from `generated/types.ts`

---

## Migration Plan (7-PR / 3-Wave Summary)

See `.claude/plans/inherited-watching-hennessy.md` for per-PR detail (files, RED tests, verification commands, dependency chain).

```
Wave 1: Engine owns schema (PRs 1–3, sequential)
  PR 1 ─ Extend ParameterDef + ParameterType (Rust types + 8 processor metadata() updates)   ← in flight
  PR 2 ─ Add ParameterDef metadata for 7 IO/container/data node types (Rust)
  PR 3 ─ Add document-shape Rust types + Codegen overhaul: absorb inferFieldType logic,
         emit types via ts-rs, delete ~930 LOC hand-written TS

Wave 2: Consumers (PRs 4–6, partial parallelism)
  PR 4 ─ Web verification (editor config panel, @bnto/form showcase, E2E)
  PR 5 ─ TUI type-aware controls (boolean, enum, number, presets, reset, description)
  PR 6 ─ TUI visibility, custom recipes, scrolling

Wave 3: Ship (PR 7)
  PR 7 ─ End-to-end integration tests + docs (tui-strategy.md, README, CLAUDE.md, PLAN.md)
```

**Dependency chain:** PR1 → PR2 → PR3 → (PR4 ∥ PR5) → PR6 → PR7. PR3 depends on PRs 1 and 2 landing (codegen needs both the enriched `ParameterDef` shape and IO/container metadata). PRs 4 and 5 may run in parallel once PR3 ships. PR6 depends on PR5's control scaffolding. PR7 depends on PR6.

---

## Responsibility Matrix

Cross-reference: [.claude/rules/node-responsibilities.md](../rules/node-responsibilities.md).

| Question                                                                                | Answer                                                                                                                 |
| --------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| What fields does a parameter have (control, presets, suffix, group, options, inverted)? | **Engine** — `ParameterDef` in `bnto-core::metadata`                                                                   |
| Which node types exist, their labels, categories, icons?                                | **Engine** — `NodeTypeInfo` in `bnto-core::metadata`                                                                   |
| Parameters for IO/container/data node types (input, output, loop, group, etc.)?         | **Engine** — `node_type_params()` (new in PR 2) alongside `NodeTypeInfo`                                               |
| `.bnto.json` document shape (`Definition`, `Edge`, `Port`, `Metadata`, `Recipe`)?       | **Engine** — Rust structs in `bnto-core::definition` with `ts-rs` derives (new in PR 3)                                |
| How to convert `control: "slider"` into a React `<Slider>` component?                   | **`@bnto/form`** — `controlType → React component` registry                                                            |
| How to convert `control: "slider"` into a ratatui bounded-number widget?                | **TUI `controls/`** — `engine/crates/bnto/src/tui/screens/controls/` dispatch                                          |
| Zod schema per node type for editor validation?                                         | **Codegen** — `packages/@bnto/nodes/src/generated/schemas.ts` derived from engine metadata                             |
| `control` field value for a parameter when the engine doesn't set one explicitly?       | **Codegen** — `generate-from-catalog.ts` infers from `param_type` + `constraints` at build time (no runtime inference) |
| Node type classification (isContainer, isIoNode)?                                       | **`@bnto/registry`** — helpers that read from engine-generated `NODE_TYPE_INFO`                                        |
| Visual editor state (selection, position, undo)?                                        | **`@bnto/editor`** — Zustand store                                                                                     |

**Golden rule for this migration:**

> If it describes what a node CAN do or what a `.bnto.json` document looks like → **Engine**.
> If it maps a generic control identifier to a React component → **`@bnto/form`**.
> If it maps a generic control identifier to a ratatui widget → **TUI `controls/`**.
> `@bnto/nodes` is a barrel over engine-generated code. Never hand-write schemas, document types, or control inference in `@bnto/nodes`.

---

## Verification

After each Wave, confirm the engine remains the single source of truth.

### Wave 1 (engine owns schema)

```
task wasm:test         # Rust unit + WASM integration tests
task wasm:lint         # clippy
task wasm:fmt:check    # Rust formatting
task wasm:snapshot     # Catalog snapshot reflects new fields
task nodes:generate    # Codegen regenerates TypeScript
task ui:build          # TypeScript compiles against generated types
task ui:test           # TypeScript tests pass
```

Expected after Wave 1:

- `ParameterDef` carries 16 fields (was 10)
- `ParameterType::Enum` options use `Vec<OptionEntry { value, label }>`
- `ParameterType::Array(Box<ParameterType>)` and `ParameterType::Record(Box<ParameterType>)` exist
- `engine/catalog.snapshot.json` has `params` arrays on 7 non-processor node types
- `packages/@bnto/nodes/src/schemas/` contains ONLY `registry.ts` (5-line Map) + `index.ts`
- Every `NodeParamField` carries an explicit `control` field — no runtime Zod→control inference
- `Definition`, `Edge`, `Port`, `Metadata`, `Recipe`, `AcceptSpec` types are emitted from `engine/crates/bnto-core/src/definition.rs` via `ts-rs`
- Count assertions unchanged: 11 browser processors, 20 node types, 15 recipes
- ~930 LOC net deleted from `@bnto/nodes`

### Wave 2 (consumers)

```
task e2e:editor        # Editor config panel renders identically
task wasm:test         # TUI control tests pass
bnto tui --theme tokyo # Quality slider shows presets, format renders select, case renders select
```

### Wave 3 (ship)

```
task check             # Full quality gate (lint + test + build for Rust + TypeScript)
```

### End-to-end "new field surfaces in both consumers" test

1. Add a new optional field to `ParameterDef` in Rust (e.g., `tooltip: Option<String>`)
2. Run `task wasm:codegen`
3. Verify the field appears in `generated/types.ts` on `NodeParamField`
4. Web (`@bnto/form`) reads it from the generated type — no hand-written changes
5. TUI (`engine/crates/bnto/src/tui/screens/controls/`) reads it from `ParamEntry` — no hand-written TS changes
6. Zero code changes in `packages/@bnto/nodes/src/schemas/`

If any step requires a hand-written change in `@bnto/nodes`, the invariant is broken.

---

## CLAUDE.md Integration

After Wave 1 ships, update the root `.claude/CLAUDE.md` documentation index with:

> `@bnto/nodes` is a barrel over engine-generated code. Never hand-write schemas, document types, or control inference in `@bnto/nodes`. Add new parameters in Rust (`engine/crates/bnto-core/src/metadata.rs`), run `task wasm:codegen`, done.

Also link this strategy doc under the "Before You Write Any Code" table:

> | Node system schema changes | [engine-owned-schema.md](.claude/strategy/engine-owned-schema.md) |
