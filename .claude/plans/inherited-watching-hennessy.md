# Sprint 11 Revised: Engine-Owned Node Schema (Wave 1) + TUI Schema-Driven Config

**Status:** Awaiting approval
**Target sprint:** Sprint 11 (restructured)
**References:** `.claude/PLAN.md` §Sprint 11, `.claude/strategy/tui-strategy.md` §Param Control Matrix, `.claude/rules/node-responsibilities.md`, `.claude/rules/engine-node-patterns.md`

---

## Context

The node config field schema — the metadata that drives sliders, selects, toggles, groups, suffixes, visibility rules, and every rendered form control in both the web editor and TUI — is currently **split across two sources of truth**:

1. **Engine (Rust)** owns the domain contract for 13 processors: `name`, `label`, `description`, `param_type`, `default`, `constraints`, `placeholder`, `visible_when`, `required_when`, `surfaceable` (10 fields on `ParameterDef` in `engine/crates/bnto-core/src/metadata.rs`). The 7 IO/container/data node types (`input`, `output`, `loop`, `group`, `transform`, `parallel`, `editFields`) have `NodeTypeInfo` but no `ParameterDef` at all.
2. **Web (`@bnto/nodes`)** hand-writes a mix of (a) presentation overlays for engine-backed processors (~228 LOC across 8 files — `presets`, `suffix`, `group`, enum `options` with labels, `control` override, `accept`, `inverted`), (b) full Zod + `NodeParamField` schemas for 7 IO/container/data nodes (~371 LOC), (c) runtime Zod→control inference in `inferFieldType.ts` (~211 LOC), and (d) the `NodeSchema`/`NodeParamField` type shapes in `types.ts`.

### The Core Insight: Nothing in `@bnto/nodes` Is Web-Specific

Every field in `NodeParamField` is platform-agnostic:

- `control: "slider" | "select" | "switch" | "file" | …` — a generic string identifier
- `suffix`, `group`, `presets`, `options`, `inverted`, `visibleWhen`, `accept` — pure data

Platform-specific mapping happens at the **consumer** layer, not in the schema:

- **React consumer (`@bnto/form`):** `controlType → React component` (e.g., `"slider" → <Slider>`, `"select" → <Select>`)
- **TUI consumer (`engine/crates/bnto/src/tui/screens/controls/`):** `controlType → ratatui widget` (e.g., `"slider" → bounded number widget`, `"select" → cycling enum widget`)

Both consumers ingest the **same** engine-generated schema. The schema itself describes "slider, 0–100, preset at 80" once; the mapping to a `<Slider>` component or a `[====o====]` widget is a rendering concern owned by each consumer.

### Why This Split Exists and What It Costs

- **TUI can't consume web metadata.** The TUI renders every param as plain text today because the presentation hints it needs (presets, suffix, bounds, control type, enum labels) live in TypeScript that WASM can't reach.
- **Two (or three) places to change per node.** Adding a processor parameter means editing `metadata()` in Rust AND the overlay in TypeScript AND possibly inferring a control in `inferFieldType.ts`. Adding an IO/container node is TypeScript-only — so the engine doesn't know about `input`/`output`/`loop` params at all.
- **`inferFieldType.ts` is codegen logic running at runtime.** Its Zod→control decision tree (bounded number → slider, `z.enum` → select, etc.) should be computed once at codegen time and baked into the generated schemas as an explicit `control` field.
- **Types are hand-maintained.** `NodeSchema`/`NodeParamField` TypeScript shapes live in `types.ts` and must be kept in lock-step with whatever codegen emits. They're platform-agnostic — they can be engine-derived.

### The Fix

Make the engine the single source of truth for node schemas, end-to-end. `@bnto/nodes` becomes a thin ingestion layer — a barrel that re-exports engine-generated code plus the two hand-written definition types (`Definition`, `Recipe`) that describe recipe file structure.

**Outcome:**

- Engine `ParameterDef` gains 6 new fields — single source of truth for processor and IO/container params
- Engine defines `ParameterDef`s for all 7 IO/container/data node types (currently absent)
- `inferFieldType.ts` decision tree moves to codegen — every generated param has an explicit `control` field
- `NodeSchema`/`NodeParamField` TypeScript types are codegen-generated from the Rust catalog shape (via `ts-rs` on the Rust side, or emitted by the existing codegen script)
- ~810+ LOC deleted from `@bnto/nodes/src/schemas/` (8 processor overlays + 7 IO/container hand-written schemas + `types.ts` + `inferFieldType.ts`)
- `packages/@bnto/nodes/src/schemas/` collapses to `index.ts` barrel + a 5-line `registry.ts` that maps generated entries to a `Map`
- TUI renders type-aware controls (slider, toggle, select, file picker) from the same data as the web
- React consumer (`@bnto/form`) owns `controlType → React component` mapping (already does — verify)
- TUI consumer owns `controlType → ratatui widget` mapping in a new `tui/screens/controls/` module
- Adding a new node = one change in Rust, one `task wasm:codegen`, done

---

## What Changes vs What Stays

**What changes:**

- `engine/crates/bnto-core/src/metadata.rs`:
  - `ParameterDef` gains `group`, `suffix`, `control`, `accept`, `presets`, `inverted` fields (6 new)
  - `ParameterType::Enum` options change from `Vec<String>` to `Vec<OptionEntry { value, label }>`
  - New `ParameterType::Array(Box<ParameterType>)` variant for `z.array(z.string())` (tagPicker)
  - New `ParameterType::Record(Box<ParameterType>)` variant for `z.record(…)` (keyValue editor)
  - `ts-rs` derives on `NodeMetadata`, `ParameterDef`, `ParameterType`, `OptionEntry`, `PresetEntry`, `Constraints`, `ParamCondition`, `NodeTypeInfo` to emit TypeScript type definitions at codegen time
- `engine/crates/bnto-core/src/definition.rs` (new) or equivalent — Rust structs for the `.bnto.json` document shape: `Definition`, `Edge`, `Port`, `Metadata`, `Recipe`, `AcceptSpec`. These are the canonical shapes the engine already parses when running a pipeline (the engine today owns `DEFINITION_JSON_SCHEMA` — this formalizes the types). `ts-rs` derives on each so codegen emits them for TypeScript consumers. The `.bnto.json` file format is engine-owned, so its types are engine-owned.
- `engine/crates/bnto-core/src/metadata.rs` + new IO/container metadata crate:
  - `ParameterDef` entries added for all 7 IO/container/data node types (`input`, `output`, `loop`, `group`, `transform`, `parallel`, `edit-fields`). These nodes have no `NodeProcessor` impl — metadata lives alongside `NodeTypeInfo`.
- Processor `metadata()` impls updated (8 engine processors across 3 crates): `bnto-image` (compress, resize, convert, overlay, strip-exif), `bnto-file` (rename), `bnto-csv` (clean, rename) — populate the new fields and convert enum options to labeled form.
- Codegen script (`packages/@bnto/nodes/scripts/generate-from-catalog.ts`) absorbs `inferFieldType.ts` logic — every generated param has an explicit `control` field, computed at codegen time from `param_type` + `constraints` + the new engine `control` hint.
- Codegen emits `NodeSchema` / `NodeParamField` / `NodeParamControl` / `SelectOption` / `PresetEntry` / `VisibleWhenClause` TypeScript types AND `Definition` / `Edge` / `Port` / `Metadata` / `Recipe` / `AcceptSpec` document-shape types (via `ts-rs`-generated definitions). Hand-written `packages/@bnto/nodes/src/schemas/types.ts`, `definition.ts`, and `recipe.ts` are deleted.
- `packages/@bnto/nodes/src/`:
  - **Deleted (~930+ LOC):** _Processor overlays (8 files, ~228 LOC):_ `schemas/imageCompress.ts`, `schemas/imageConvert.ts`, `schemas/imageOverlay.ts`, `schemas/imageResize.ts`, `schemas/imageStripExif.ts`, `schemas/fileRename.ts`, `schemas/spreadsheetClean.ts`, `schemas/spreadsheetRename.ts`. _IO/container schemas (7 files, ~371 LOC):_ `schemas/input.ts`, `schemas/output.ts`, `schemas/loop.ts`, `schemas/group.ts`, `schemas/transform.ts`, `schemas/parallel.ts`, `schemas/editFields.ts`. _Runtime inference + type declarations (~281 LOC):_ `schemas/inferFieldType.ts`, `schemas/types.ts`, `schemas/engineSchemaEntries.ts`. _Document-shape types (now engine-generated via ts-rs, ~50 LOC):_ `definition.ts`, `recipe.ts`.
  - **Collapsed to barrel:** `schemas/registry.ts` becomes a ~5-line `Map` construction from generated entries; `schemas/index.ts` re-exports generated types.
- `packages/@bnto/form/` gains (or has verified) a `controlType → React component` registry. This is where platform-specific rendering lives.
- `engine/crates/bnto/src/tui/screens/controls/` gains a `controlType → ratatui widget` dispatch. This is where TUI-specific rendering lives.
- TUI `ParamEntry` carries full `ParameterDef` metadata (description, constraints, presets, suffix, group, control, visible_when, placeholder).
- New strategy doc **`.claude/strategy/engine-owned-schema.md`** lands in PR 1 so subsequent PRs (and `/pickup` runs) share the same canonical design. PLAN.md task lines reference both this plan and the strategy doc.

**What does NOT change:**

- Engine processor count (still 13 processors — `11 browser + 2 native-only`)
- Node type count (still 20 total: 13 processors + 7 IO/container/data)
- Recipe count (still 15)
- Golden test outputs (this is a schema-surface change, not an execution change)
- Runtime behavior of any pipeline
- Public TypeScript surface of `@bnto/nodes` — `NodeSchema`, `NodeParamField`, `Definition`, `Edge`, `Port`, `Metadata`, `Recipe`, `AcceptSpec`, etc. still exported from the package, just sourced from `generated/` (emitted by the engine via `ts-rs`) instead of hand-written `types.ts` / `definition.ts` / `recipe.ts`
- Consumers of `@bnto/nodes` (`@bnto/registry`, `@bnto/core`, `@bnto/form`, `@bnto/editor`) — all continue to work without changes because the exported type shape stays identical; only the origin of the types moves (hand-written → engine-generated)

---

## Strategy Document: `.claude/strategy/engine-owned-schema.md`

This is a multi-PR migration spanning 7 PRs across 3 Waves. A single plan file is not enough context for future `/pickup` runs, PR reviewers joining mid-migration, or cross-referencing from `PLAN.md` task lines. We ship a canonical strategy doc in PR 1 so every subsequent PR (and every agent picking up those tasks) shares the same design.

**Location:** `.claude/strategy/engine-owned-schema.md` (follows the same convention as `tui-strategy.md`, `editor-architecture.md`, `engine-execution.md`, `io-nodes.md`, etc.)

**Lands in:** PR 1 (alongside the Rust `ParameterDef` extensions). Subsequent PRs update the doc as they land (e.g., PR 3 flips the "Codegen" section from "planned" to "shipped").

**Contents:**

- **Context & goal** — why `@bnto/nodes` becomes a barrel over engine-generated code; why the engine owns `.bnto.json` document types via `ts-rs`
- **Single source of truth principle** — every field the UI needs (control, presets, suffix, group, options, visible_when) comes from the engine catalog; no web-specific or framework-specific data lives in `@bnto/nodes`
- **Platform-agnostic `control` field** — engine emits a generic string identifier; platform consumers map it to their widget (`@bnto/form` → React component, TUI `controls/` → ratatui widget)
- **Type origin map** — table of every TypeScript type exported from `@bnto/nodes` with its origin (hand-written → engine-generated) and the `ts-rs` source struct
- **Migration plan** — condensed 7-PR / 3-Wave summary with links to this plan doc for detail
- **Responsibility matrix** — what lives in engine / `@bnto/nodes` / `@bnto/form` / TUI `controls/` / editor — cross-reference with `.claude/rules/node-responsibilities.md`
- **Verification** — how to confirm a new engine field surfaces end-to-end in both web and TUI
- **CLAUDE.md integration** — one-line guidance: "`@bnto/nodes` is a barrel over engine-generated code. Never hand-write schemas, document types, or control inference in `@bnto/nodes`."

**PLAN.md cross-reference:** Every Sprint 11 task line in `PLAN.md` references BOTH this plan doc (for PR-level detail) AND `.claude/strategy/engine-owned-schema.md` (for the canonical design). Format: `**Plan doc:** [.claude/plans/inherited-watching-hennessy.md](./plans/inherited-watching-hennessy.md) · **Strategy doc:** [.claude/strategy/engine-owned-schema.md](./strategy/engine-owned-schema.md)` at the top of the Sprint 11 section.

---

## PR Split (7 PRs, Waves 1–3 of Sprint 11)

The work divides into seven single-concern PRs. Wave 1 (PRs 1–4) makes the engine the sole source of truth and deletes ~930 LOC from `@bnto/nodes`. Wave 2 (PRs 5–6) builds the TUI consumer on top of the enriched metadata. Wave 3 (PR 7) ships docs + end-to-end integration.

```
Wave 1: Engine owns schema (PRs 1–4)
  PR 1 ─ Extend ParameterDef + ParameterType (Rust types + 8 processor metadata() updates)
  PR 2 ─ Add ParameterDef metadata for 7 IO/container/data node types (Rust)
  PR 3 ─ Codegen overhaul: absorb inferFieldType logic, emit types, delete ~810 LOC hand-written TS
  PR 4 ─ Web verification (editor config panel, @bnto/form showcase, E2E)

Wave 2: TUI consumer (PRs 5–6)
  PR 5 ─ TUI type-aware controls (boolean, enum, number, presets, reset, description)
  PR 6 ─ TUI visibility, custom recipes, scrolling

Wave 3: Ship (PR 7)
  PR 7 ─ End-to-end integration tests + docs (tui-strategy.md, README, CLAUDE.md, PLAN.md)
```

---

## PR 1: Extend `ParameterDef` + `ParameterType` shape; update 8 processors

**Branch:** `feat/engine-parameter-def-presentation` from `main`
**One sentence:** Extend `ParameterDef` with presentation metadata, add `Array`/`Record` variants to `ParameterType`, refactor `Enum` options to carry labels, and propagate through all 8 processor `metadata()` impls.

### What

Rust-only change to `bnto-core::metadata` and all 8 engine-backed processor `metadata()` impls. Landing this PR extends the engine contract for the processor surface only — IO/container/data nodes (PR 2) and TypeScript codegen (PR 3) come later. Web continues building against the old snapshot because overlays still merge until PR 3.

### Files (~2 new, ~13 modified)

**New:**

- `engine/crates/bnto-core/src/metadata/presets.rs` — `PresetEntry` + `OptionEntry` structs (if metadata.rs crosses size threshold; otherwise inline)

**Modified:**

- `engine/crates/bnto-core/src/metadata.rs` — extend `ParameterDef`, refactor `ParameterType::Enum`, add `ParameterType::Array`/`Record`, add `PresetEntry`/`OptionEntry`, optional `ts-rs` derives
- `engine/crates/bnto-image/src/common.rs` — update `quality_param_def()` to include presets + suffix; `format_param_def()` to use `OptionEntry`
- `engine/crates/bnto-image/src/compress.rs` — quality presets via shared builder
- `engine/crates/bnto-image/src/resize.rs` — width/height group="dimensions" + suffix="px"; quality presets
- `engine/crates/bnto-image/src/convert.rs` — format options with labels; quality presets
- `engine/crates/bnto-image/src/overlay.rs` — `control="file"` for image param; `control="watermarkPreview"` for preview synthetic param; groups + suffixes
- `engine/crates/bnto-image/src/strip_exif.rs` — quality presets + suffix
- `engine/crates/bnto-file/src/rename.rs` — case `OptionEntry` labels (e.g., `{value: "snake", label: "snake_case"}`)
- `engine/crates/bnto-csv/src/clean.rs` — any enum options converted to `OptionEntry`; group/suffix where applicable
- `engine/crates/bnto-csv/src/rename.rs` — same
- `engine/crates/bnto-engine/src/lib.rs` — registry tests: still 11 processors (unchanged)
- `engine/crates/bnto-wasm/src/catalog.rs` — catalog tests: still 11 processors, new fields serialize

### Key API / data structures

```rust
pub struct ParameterDef {
    pub name: String,
    pub label: String,
    pub description: Option<String>,
    pub param_type: ParameterType,
    pub default: Option<Value>,
    pub constraints: Option<Constraints>,
    pub placeholder: Option<String>,
    pub visible_when: Option<ParamCondition>,
    pub required_when: Option<ParamCondition>,
    pub surfaceable: bool,
    // NEW:
    pub group: Option<String>,
    pub suffix: Option<String>,
    pub control: Option<String>,            // "file" | "watermarkPreview" | "textarea" | "positionGrid"
    pub accept: Option<Vec<String>>,        // MIME types when control="file"
    pub presets: Option<Vec<PresetEntry>>,  // slider presets
    pub inverted: Option<bool>,             // flip slider semantics
}

pub struct PresetEntry {
    pub value: Value,  // JSON number or string
    pub label: String,
}

pub struct OptionEntry {
    pub value: String,
    pub label: String,
}

pub enum ParameterType {
    Number,
    String,
    Boolean,
    Enum { options: Vec<OptionEntry> },   // was Vec<String>
    Object,
    File { accept: Vec<String> },
}
```

All new fields are `Option<T>` / `Vec<T>` — serde will omit them via `skip_serializing_if = "Option::is_none"` to keep catalog snapshot diffs minimal.

### RED tests (write first)

- `ParameterDef` with `presets: Some(vec![PresetEntry{value: json!(60), label: "Draft".into()}])` round-trips through serde
- `ParameterDef` with `group: Some("dimensions".into()), suffix: Some("px".into())` round-trips
- `ParameterType::Enum { options: vec![OptionEntry{value: "snake".into(), label: "snake_case".into()}] }` serializes with both fields
- `quality_param_def()` returns preset entries [60/Draft, 80/Balanced, 100/Maximum] and suffix "%"
- `compress.rs` `metadata()` exposes quality presets to engine tests
- `resize.rs` width/height params share `group="dimensions"` and `suffix="px"`
- `overlay.rs` image param has `control="file"` and `accept=["image/*"]`
- `overlay.rs` has a synthetic param with `control="watermarkPreview"` (or on an existing one) — mirror current TS overlay
- `rename.rs` case options include `{value: "snake", label: "snake_case"}` etc.
- `test_browser_registry_has_all_processors()` still passes (count unchanged)
- `test_catalog_serializes_to_valid_json()` still passes; new fields appear when set, are absent when `None`

### Verification

```
task wasm:test
task wasm:lint
task wasm:fmt:check
```

### No count changes

Processor count, node type count, recipe count all unchanged. This PR changes the SHAPE of metadata, not the cardinality.

---

## PR 2: Add `ParameterDef` metadata for 7 IO / container / data node types

**Branch:** `feat/engine-io-container-parameter-defs` from PR 1
**One sentence:** Bring `input`, `output`, `loop`, `group`, `transform`, `parallel`, and `edit-fields` into the engine metadata surface with full `ParameterDef` entries — these currently have only `NodeTypeInfo` and exist purely as hand-written Zod schemas in `@bnto/nodes`.

### What

The 7 IO/container/data node types have no `NodeProcessor` impl — they're structural (graph semantics) rather than executable. Today their schemas live only in TypeScript (`input.ts`, `output.ts`, etc. in `@bnto/nodes/src/schemas/`). Move the param metadata into Rust so codegen can emit them uniformly with the 13 processor types. No `process()` logic added — just metadata.

### Files (~1 new, ~4 modified)

**New:**

- `engine/crates/bnto-core/src/metadata/io_container.rs` — `io_container_param_defs()` returning `HashMap<&'static str, Vec<ParameterDef>>` keyed by node type (`"input"`, `"output"`, `"loop"`, `"group"`, `"transform"`, `"parallel"`, `"edit-fields"`)

**Modified:**

- `engine/crates/bnto-core/src/metadata.rs` — `NodeTypeInfo` gains `params: Option<Vec<ParameterDef>>` (or separate `node_type_params(type_name)` accessor); `all_node_types()` still returns 20 entries (unchanged), but now with params for the 7 non-processor types
- `engine/crates/bnto-wasm/src/catalog.rs` — catalog output includes `params` on non-processor node types (new field in JSON shape). Catalog test count remains 20 node types.
- `engine/catalog.snapshot.json` — regenerated; gains param arrays on 7 node types
- `engine/crates/bnto-engine/src/lib.rs` — no registry change (still 11 browser processors)

### Param coverage (hand-ported from current TS schemas)

| Node type     | Params (from current `@bnto/nodes/src/schemas/*.ts`)                                                                                                                                                               |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `input`       | `mode` (enum: `file-upload`/`text`/`url`), `accept` (array), `extensions` (array w/ 15-value tagPicker), `label`, `multiple`, `maxFileSize`, `maxFiles`, `placeholder` — with `visible_when` rules from `input.ts` |
| `output`      | `mode` (enum), `filename` or equivalent, format hints — matches current `output.ts`                                                                                                                                |
| `loop`        | `iterationMode` enum, `maxIterations`, `collectResults` — matches current `loop.ts`                                                                                                                                |
| `group`       | `label`, `collapsed` (boolean) — matches current `group.ts`                                                                                                                                                        |
| `transform`   | `mode` (enum: `javascript`/`jq`/`jsonpath`), `expression` (textarea) — matches current `transform.ts`                                                                                                              |
| `parallel`    | `concurrency` (number) — matches current `parallel.ts`                                                                                                                                                             |
| `edit-fields` | `fields` (record/keyValue) — matches current `editFields.ts`                                                                                                                                                       |

Param defaults, constraints, `visible_when`, `surfaceable` flags, and enum `OptionEntry` labels are ported verbatim from the existing TS — the engine becomes the source of truth, not a rewrite.

### Key API additions

```rust
// metadata/io_container.rs
pub fn io_container_param_defs() -> &'static HashMap<&'static str, Vec<ParameterDef>> {
    // returns 7 entries, one per IO/container/data node type
}

// metadata.rs
pub fn node_type_params(type_name: &str) -> Option<&'static [ParameterDef]> {
    // looks up params for any node type — processor OR IO/container
}
```

### RED tests (write first)

- `io_container_param_defs()` returns exactly 7 entries with expected keys
- `node_type_params("input")` returns `mode`, `accept`, `extensions`, `label`, `multiple`, `maxFileSize`, `maxFiles`, `placeholder` with correct types and defaults matching current `input.ts`
- `input` `mode` param has `OptionEntry` labels matching current overlay (`"File Upload"`, `"Text"`, `"URL"`)
- `input` `extensions` param has `control="tagPicker"` and 15 default options (`.jpg`, `.png`, …) matching current `input.ts`
- `input` params that are `visible_when: { mode: "file-upload" }` round-trip correctly
- `transform` `expression` param has `control="textarea"`
- `edit-fields` `fields` param has `ParameterType::Record` shape
- `group` `label` param round-trips with correct default
- `all_node_types()` still returns 20 node types (count unchanged)
- Catalog snapshot JSON contains `params` arrays on the 7 non-processor node types

### Verification

```
task wasm:test
task wasm:lint
task wasm:snapshot   # regenerate catalog.snapshot.json and inspect the diff
```

### No count changes

Node type count stays at 20. Registry processor count stays at 11. Recipe count stays at 15. What changes is the SHAPE of `NodeTypeInfo` / catalog JSON: non-processor node types now carry `params`.

---

## PR 3: Codegen overhaul — absorb `inferFieldType`, emit types, delete ~930 LOC

**Branch:** `feat/codegen-engine-schemas` from PR 2
**One sentence:** Extend codegen to consume PR 1's processor fields and PR 2's IO/container params, absorb `inferFieldType.ts`'s Zod→control decision tree into codegen time, emit platform-agnostic TypeScript types (including `.bnto.json` document shapes), and delete ~930 LOC of hand-written TS from `@bnto/nodes`.

### What

After PR 1 + PR 2, the engine catalog carries every field every consumer needs. This PR rewrites the codegen script to bake in the full mapping at generate time (not runtime) and deletes the hand-written overlays, IO/container schemas, document-shape types, field types, and runtime inference. `@bnto/nodes/src/schemas/` collapses to a barrel + tiny `registry.ts` wrapping the generated `Map`; `definition.ts` and `recipe.ts` at the package root are also deleted (their types become engine-generated).

**Four structural changes in the codegen script:**

1. **`inferFieldType.ts`'s decision tree moves in.** Every generated `NodeParamField` gets an explicit `control` field at codegen time, computed from `param_type` + `constraints` + engine `control` hint. No runtime inference, no Zod introspection at render time.
2. **Field types emitted, not hand-written.** `NodeSchema`, `NodeParamField`, `NodeParamControl`, `SelectOption`, `PresetEntry`, `VisibleWhenClause` are generated from Rust struct shapes via `ts-rs` derives on `bnto-core` types. `schemas/types.ts` is deleted.
3. **Document-shape types emitted, not hand-written.** `Definition`, `Edge`, `Port`, `Metadata`, `Recipe`, `AcceptSpec` are also generated via `ts-rs` — the engine already parses `.bnto.json` and owns `DEFINITION_JSON_SCHEMA`; PR 1/PR 2 add Rust structs in `engine/crates/bnto-core/src/definition.rs` (or equivalent) with `ts-rs` derives, PR 3 consumes them. `definition.ts` and `recipe.ts` at the `@bnto/nodes` package root are deleted.
4. **IO/container schemas come from the catalog.** The 7 hand-written schemas (`input.ts` through `editFields.ts`) are deleted; codegen emits their Zod schemas and `NodeParamField` entries from PR 2's engine metadata.

### Files (~0 new, ~6 modified, ~19 deleted)

**Regenerated (not hand-edited):**

- `engine/catalog.snapshot.json` — via `task wasm:snapshot`
- `packages/@bnto/nodes/src/generated/catalog.ts` — via `task nodes:generate`
- `packages/@bnto/nodes/src/generated/schemas.ts` — Zod schemas for all 20 node types (was 13)
- `packages/@bnto/nodes/src/generated/types.ts` — NEW file: engine-emitted `NodeSchema`, `NodeParamField`, `NodeParamControl`, etc.
- `packages/@bnto/nodes/src/generated/definitionSchema.ts` — enum changes reflected
- `packages/@bnto/backend/convex/_helpers/nodeTypeLabels.ts` — regenerated
- `packages/@bnto/i18n/src/generated/nodes.json` — regenerated

**Modified:**

- `packages/@bnto/nodes/scripts/generate-from-catalog.ts` — absorb `inferFieldType` logic; compute `control` at codegen time from `(param_type, constraints, control hint)`; emit platform-agnostic types; generate Zod schemas for all 20 node types including IO/container
- `packages/@bnto/nodes/src/schemas/registry.ts` — collapses to ~5 lines: `new Map(Object.entries(GENERATED_NODE_SCHEMAS))`. All overlay merge logic deleted.
- `packages/@bnto/nodes/src/schemas/index.ts` — re-exports types from `generated/types.ts` instead of hand-written `types.ts`
- `packages/@bnto/nodes/src/catalogValidation.test.ts` — assertions adjusted to check engine-provided fields (presets, suffix, group, control) per node type; extend coverage to IO/container node types
- `packages/@bnto/nodes/src/nodeTypes.test.ts` — assertions adjusted for new shape (count unchanged at 20)
- `packages/@bnto/registry/src/nodeTypes.test.ts` — unchanged count (20), possibly new shape assertions

**Deleted (~930 LOC):**

_Processor overlays (~228 LOC, 8 files):_

- `packages/@bnto/nodes/src/schemas/imageCompress.ts` (~16 LOC)
- `packages/@bnto/nodes/src/schemas/imageConvert.ts` (~40 LOC)
- `packages/@bnto/nodes/src/schemas/imageOverlay.ts` (~52 LOC)
- `packages/@bnto/nodes/src/schemas/imageResize.ts` (~30 LOC)
- `packages/@bnto/nodes/src/schemas/imageStripExif.ts` (~28 LOC)
- `packages/@bnto/nodes/src/schemas/fileRename.ts` (~26 LOC)
- `packages/@bnto/nodes/src/schemas/spreadsheetClean.ts` (~18 LOC, currently empty)
- `packages/@bnto/nodes/src/schemas/spreadsheetRename.ts` (~18 LOC, currently empty)

_IO/container/data schemas (~371 LOC, 7 files):_

- `packages/@bnto/nodes/src/schemas/input.ts` (~124 LOC)
- `packages/@bnto/nodes/src/schemas/output.ts`
- `packages/@bnto/nodes/src/schemas/loop.ts`
- `packages/@bnto/nodes/src/schemas/group.ts`
- `packages/@bnto/nodes/src/schemas/transform.ts`
- `packages/@bnto/nodes/src/schemas/parallel.ts`
- `packages/@bnto/nodes/src/schemas/editFields.ts`

_Runtime inference + type declarations:_

- `packages/@bnto/nodes/src/schemas/inferFieldType.ts` (~211 LOC) — logic now baked into codegen script
- `packages/@bnto/nodes/src/schemas/types.ts` — `NodeSchema`/`NodeParamField` shapes now emitted from engine
- `packages/@bnto/nodes/src/schemas/engineSchemaEntries.ts` — overlay merge obsolete (no overlays left)

_Document-shape types (~50 LOC, 2 files — engine-generated via `ts-rs`):_

- `packages/@bnto/nodes/src/definition.ts` (~30 LOC) — `Definition`/`Edge`/`Port`/`Metadata` interfaces now emitted from `engine/crates/bnto-core/src/definition.rs`
- `packages/@bnto/nodes/src/recipe.ts` (~20 LOC) — `Recipe`/`AcceptSpec` types now emitted from engine

### Deletion surface table

| File                           | LOC  | Currently owns                               | After migration                                   |
| ------------------------------ | ---- | -------------------------------------------- | ------------------------------------------------- |
| `imageCompress.ts`             | 16   | quality presets, suffix                      | DELETED (engine owns)                             |
| `imageConvert.ts`              | 40   | format options w/ labels, quality presets    | DELETED (engine owns)                             |
| `imageOverlay.ts`              | 52   | file control, watermarkPreview, groups       | DELETED (engine owns)                             |
| `imageResize.ts`               | 30   | dimensions group, px suffix, quality presets | DELETED (engine owns)                             |
| `imageStripExif.ts`            | 28   | quality presets, suffix                      | DELETED (engine owns)                             |
| `fileRename.ts`                | 26   | case options w/ labels                       | DELETED (engine owns)                             |
| `spreadsheetClean.ts`          | 18   | (empty)                                      | DELETED                                           |
| `spreadsheetRename.ts`         | 18   | (empty)                                      | DELETED                                           |
| `input.ts`                     | 124  | mode/accept/extensions/label/…               | DELETED (engine owns — PR 2)                      |
| `output.ts`                    | ~50  | output mode + filename config                | DELETED (engine owns — PR 2)                      |
| `loop.ts`                      | ~50  | iteration mode + max iterations              | DELETED (engine owns — PR 2)                      |
| `group.ts`                     | ~30  | label + collapsed                            | DELETED (engine owns — PR 2)                      |
| `transform.ts`                 | ~50  | mode (js/jq/jsonpath) + expression           | DELETED (engine owns — PR 2)                      |
| `parallel.ts`                  | ~30  | concurrency                                  | DELETED (engine owns — PR 2)                      |
| `editFields.ts`                | ~40  | fields record                                | DELETED (engine owns — PR 2)                      |
| `inferFieldType.ts`            | 211  | Zod → control runtime decision tree          | DELETED (logic in codegen)                        |
| `schemas/types.ts`             | ~30  | `NodeSchema`/`NodeParamField` type decls     | DELETED (emitted by codegen via `ts-rs`)          |
| `engineSchemaEntries.ts`       | ~40  | overlay merge logic                          | DELETED (no overlays left)                        |
| `definition.ts` (package root) | ~30  | `Definition`/`Edge`/`Port`/`Metadata` types  | DELETED (engine-generated via `ts-rs` — PR 1/3)   |
| `recipe.ts` (package root)     | ~20  | `Recipe`/`AcceptSpec` types                  | DELETED (engine-generated via `ts-rs` — PR 1/3)   |
| **Total**                      | ~930 |                                              |                                                   |
| `schemas/registry.ts`          | —    | merges engine + hand-written                 | COLLAPSED to ~5-line `Map` over generated entries |

### RED tests (write first)

- `NODE_SCHEMAS.get("image-compress").params.quality.control === "slider"` (computed at codegen from bounded number + presets)
- `NODE_SCHEMAS.get("image-compress").params.quality.presets` equals `[{value: 60, label: "Draft"}, {value: 80, label: "Balanced"}, {value: 100, label: "Maximum"}]`
- `NODE_SCHEMAS.get("image-resize").params.width.group === "dimensions"` and `suffix === "px"`
- `NODE_SCHEMAS.get("image-convert").params.format.options` is `[{value: "jpeg", label: "JPEG"}, …]` with labels
- `NODE_SCHEMAS.get("image-overlay").params.image.control === "file"` and `accept === ["image/*"]`
- `NODE_SCHEMAS.get("file-rename").params.case.options` matches the current TS overlay shape
- `NODE_SCHEMAS.get("input").params.mode.options` matches `[{value: "file-upload", label: "File Upload"}, …]` (engine-derived, not hand-written)
- `NODE_SCHEMAS.get("input").params.extensions.control === "tagPicker"` with 15 default options
- `NODE_SCHEMAS.get("input").params.accept.visibleWhen` evaluates mode=file-upload (engine-derived)
- `NODE_SCHEMAS.get("transform").params.expression.control === "textarea"`
- `NODE_SCHEMAS.get("edit-fields").params.fields.control === "keyValue"`
- Every `NodeParamField` has an explicit `control` field (no fallback inference at runtime)
- `packages/@bnto/nodes/src/generated/types.ts` exports `NodeSchema`, `NodeParamField`, `NodeParamControl`, `SelectOption`, `PresetEntry`, `VisibleWhenClause`
- `packages/@bnto/nodes/src/schemas/` contains NO hand-written schema files — only `registry.ts` (~5 lines) + `index.ts`
- `catalogValidation.test.ts` covers all 20 node types (was 13)
- `@bnto/form`, `@bnto/editor`, and `@bnto/registry` all build against the new shape without changes

### Verification

```
task wasm:codegen
task ui:build
task ui:lint
task ui:test
```

### No count changes

20 node types. 11 browser processors. 15 recipes. Only the internal shape of schema entries changes, and the public TypeScript surface (`NodeSchema`, `NodeParamField`) stays identical — just emitted instead of hand-written.

---

## PR 4: Web verification — editor config panel, form showcase, E2E

**Branch:** `chore/verify-engine-schema-parity` from PR 3
**One sentence:** Verify the web editor, Motorway `@bnto/form` showcase, and SchemaForm rendering are unchanged after the ~930 LOC deletion.

### What

Pure verification PR — no code changes expected beyond fixing any consumer that relied on overlay-only fields, hand-written IO/container schemas, or hand-written document-shape types. This PR's job is to catch any regression in the editor config panel, Motorway form showcase, or SchemaForm rendering and to confirm `@bnto/form` already owns the `controlType → React component` registry correctly (or extract one if it doesn't). Consumers of `Definition`/`Edge`/`Port`/`Metadata`/`Recipe`/`AcceptSpec` should continue working unchanged — the types are now emitted by the engine via `ts-rs` but the public TypeScript surface stays identical.

### Files

- `apps/web/` — if any regression surfaces, fix at the consumer (not by re-introducing overlays)
- `packages/@bnto/form/` — verify (or add) a `controlType → React component` registry; snapshot tests may need updates if field order changes
- `packages/editor/` — editor config panel should render identically; any consumer that `import`ed from the deleted schema files gets redirected to `@bnto/core` (which re-exports from `@bnto/registry`)

### RED tests (write first)

- E2E test asserting `image-compress` detail panel still shows quality slider with Draft/Balanced/Maximum presets
- E2E test asserting `image-resize` still groups width/height under "dimensions"
- E2E test asserting `image-overlay` still shows file picker for image param
- E2E test asserting `input` node config still renders mode select + extensions tagPicker + visible_when filtering
- E2E test asserting `transform` node config still renders textarea
- E2E test asserting `edit-fields` node config still renders keyValue editor
- Playwright screenshot of editor config panel matches post-deletion baseline
- `@bnto/form` showcase renders every `control` type (slider, select, switch, number, text, textarea, tagPicker, keyValue, file, positionGrid, watermarkPreview)

### Verification

```
task ui:build
task ui:test
task e2e:editor          # requires task dev on :4000
```

### No count changes

---

## PR 5: TUI type-aware controls — boolean / enum / number / reset / description

**Branch:** `feat/tui-schema-driven-controls` from PR 3 (can start after PR 3 merges; does not depend on PR 4)
**One sentence:** Render type-aware TUI controls driven by engine metadata — boolean toggles, enum selects, bounded number controls with presets, reset-to-default, and inline description text.

### What

Original Sprint 11 Waves 1–2 (metadata enrichment + controls) collapsed — now possible in one PR because the engine already carries every field the TUI needs. Introduces `engine/crates/bnto/src/tui/screens/controls/` as the TUI consumer's `controlType → ratatui widget` dispatch layer, mirroring `@bnto/form`'s role on the web.

### Files (~3 new, ~6 modified)

**New:**

- `engine/crates/bnto/src/tui/screens/controls/boolean.rs` — toggle renderer
- `engine/crates/bnto/src/tui/screens/controls/enum_select.rs` — cycling select renderer
- `engine/crates/bnto/src/tui/screens/controls/number.rs` — bounded number input with preset shortcuts

**Modified:**

- `engine/crates/bnto/src/tui/screens/detail.rs` — `ParamEntry` gains `constraints`, `description`, `placeholder`, `group`, `suffix`, `presets`, `control`, `visible_when`; `DetailMessage` gains `ToggleBool`, `EnumNext`, `EnumPrev`, `NumberIncrement`, `NumberDecrement`, `ResetDefault`, `ClearError`; `DetailModel` gains `error: Option<String>`
- `engine/crates/bnto/src/tui/screens/detail_loader.rs` — maps all new `ParameterDef` fields into `ParamEntry`; uses `node_type_params()` so IO/container node types also get controls
- `engine/crates/bnto/src/tui/screens/render_detail.rs` — dispatch on `control` field to control-specific renderers; render suffix annotation; render inline description
- `engine/crates/bnto/src/tui/keys.rs` — route keys to Detail controls (`Space`/`Enter` for toggle, `←`/`→` for enum/number, `d` for reset)
- `engine/crates/bnto/src/tui/screens/detail_test.rs` — unit tests for all controls

### RED tests (write first)

- `ParamEntry` carries full metadata from `ParameterDef` (description, constraints, presets, suffix, group, control)
- Boolean toggle: `Space` flips value; ignored when non-boolean focused
- Enum cycle: `→` advances through `OptionEntry` list (displaying `label`, storing `value`); wraps at boundary
- Number increment: `+`/`→` step up (1 for int, 0.1 for float); clamp at max; preset shortcut keys jump to preset values
- Number decrement: `-`/`←` mirror behavior; clamp at min
- Reset: `d` restores `default`; no-op when value already equals default
- Validation: invalid commit sets `error`; next keystroke clears error
- Description: focused param renders description in help area
- Suffix: renders after value (e.g., "80%", "512px")
- Integration: loading `compress-images` renders quality as bounded Number with presets, format as Enum, strip_metadata as Boolean (when applicable)

### Verification

```
task wasm:test
task wasm:lint
bnto tui --theme tokyo    # manual spot-check quality slider, format select, file rename case select
```

### Test count updates

- `detail_test.rs` grows substantially — no exact-count asserts elsewhere depend on this

---

## PR 6: TUI visibility + custom recipes + viewport scrolling

**Branch:** `feat/tui-visibility-scrolling` from PR 5
**One sentence:** Add conditional `visible_when` evaluation, custom `.bnto.json` loading via `bnto tui <path>`, and auto-scroll viewport for long parameter lists.

### What

Original Sprint 11 Wave 3. Docs removed from this PR — they live in PR 7 (the final ship PR).

### Files (~0 new, ~5 modified)

- `engine/crates/bnto/src/tui/screens/detail.rs` — `visible_params()` filter; scroll offset
- `engine/crates/bnto/src/tui/screens/detail_loader.rs` — custom recipe path handling
- `engine/crates/bnto/src/tui/app.rs` — skip browser when custom recipe provided
- `engine/crates/bnto/src/main.rs` — clap arg for optional recipe path
- `engine/crates/bnto/src/tui/screens/render_detail.rs` — scroll indicator

### RED tests (write first)

- `visible_params()` filters based on `visible_when` evaluating against current values
- `FocusNext`/`FocusPrev` skip hidden params
- Confirm omits hidden params from overrides payload
- `bnto tui recipe.bnto.json` loads custom recipe and skips browser
- Invalid file produces clear error
- Scroll offset auto-advances to keep focused param visible
- Scroll indicator appears when content overflows

### Verification

```
task wasm:test
task cli:test
bnto tui ~/path/to/some.bnto.json   # manual spot check custom recipe path
```

### No count changes

---

## PR 7: End-to-end integration tests + docs

**Branch:** `chore/sprint-11-docs-and-integration` from PR 6
**One sentence:** Ship the end-to-end integration test covering schema-driven controls, update `tui-strategy.md`, README, CLAUDE.md, and mark Sprint 11 complete in `PLAN.md`.

### What

Final Sprint 11 PR. Adds the top-level integration test that loads a recipe and asserts every control type renders correctly, then updates the canonical docs so the next sprint has an accurate starting point.

### Files (~1 new, ~4 modified)

**New:**

- `engine/crates/bnto/tests/tui_schema_controls_integration.rs` — integration test loading `compress-images` via `bnto tui`, asserting quality renders bounded Number with presets, format renders Enum select, case renders Enum, description lines rendered, visible_when filtering active

**Modified:**

- `.claude/strategy/tui-strategy.md` — mark Param Control Matrix entries shipped; update "Current state" narrative to reflect schema-driven controls
- `README.md` — regenerate TUI section via `task readme:generate` if needed; highlight schema-driven controls
- `.claude/CLAUDE.md` — update node-responsibilities.md references if migration changes guidance ("`@bnto/nodes` is a barrel over generated code")
- `.claude/PLAN.md` — mark Sprint 11 complete; sketch next sprint (file picker UX overhaul + node ecosystem)

### RED tests (write first)

- Integration test: `compress-images` recipe renders quality as bounded Number, format as Enum, strip_metadata (if present) as Boolean
- Integration test: `rename-files` renders case as Enum with `snake_case` / `kebab-case` labels
- Integration test: `overlay-watermark` renders image param as file picker, triggers watermarkPreview synthetic control
- Integration test: focused param description renders in help area
- Doc check: every shipped control listed in `tui-strategy.md` Param Control Matrix

### Verification

```
task check                             # full gate: wasm + ui
bnto tui --theme tokyo                 # manual walkthrough
bnto tui engine/recipes/compress-images.bnto.json
```

### No count changes

---

## Dependency Chain

```
Wave 1 — Engine owns schema (sequential)
  PR 1 (processor ParameterDef + ParameterType shape; 8 processor metadata updates)
    └── PR 2 (IO/container ParameterDef metadata — 7 non-processor node types)
          └── PR 3 (codegen overhaul — absorb inferFieldType, emit types, delete ~810 LOC)

Wave 2 — Consumers build atop unified surface (parallel)
                ├── PR 4 (web verification — editor panel, form showcase, E2E)
                └── PR 5 (TUI type-aware controls — boolean/enum/number/reset/description)
                      └── PR 6 (TUI visibility + custom recipes + scrolling)

Wave 3 — Ship (sequential)
                            └── PR 7 (integration tests + docs: tui-strategy.md, README, CLAUDE.md, PLAN.md)
```

- **PR 1** is self-contained in Rust — no web changes yet. Web continues building against the old snapshot because overlays still merge until PR 3.
- **PR 2** extends the engine surface to the 7 IO/container/data node types so codegen can emit them uniformly. Web still builds against overlays.
- **PR 3** is the big bang — regenerates catalog + TS types, deletes ~810 LOC of hand-written schemas, overlays, and runtime Zod→control inference. Consumers flip to the engine-generated surface in one atomic change.
- **PR 4 and PR 5 run in parallel.** Both depend only on PR 3 — PR 4 verifies web consumers still work after the deletion, PR 5 builds TUI consumers on top of the enriched metadata. Neither blocks the other.
- **PR 6** depends on PR 5's control scaffolding (ParamEntry metadata, control dispatch) before adding visibility/scroll/custom-recipe behavior.
- **PR 7** is the final ship PR — integration tests + canonical docs — and depends on PR 6 landing.

---

## PLAN.md Integration

Replace the current `Sprint 11: TUI Schema-Driven Config — NEXT` section (lines 648–690) with the revised wave structure below. All task lines MUST reference this plan document so future `/pickup` runs can find the full context.

**Proposed PLAN.md update (paste this into PLAN.md in place of the existing Sprint 11 section):**

```markdown
### Sprint 11: Engine-Owned Node Schema + TUI Schema-Driven Config — NEXT

**Plan doc:** [.claude/plans/inherited-watching-hennessy.md](./plans/inherited-watching-hennessy.md) — full context, 7-PR split, deletion surface, verification.

**Goal:** Make the Rust engine the single source of truth for node config field schemas AND `.bnto.json` document types, end-to-end. `@bnto/nodes` collapses to a barrel over engine-generated code. Both web (`@bnto/form`) and TUI (`engine/crates/bnto/src/tui/screens/controls/`) consume the same platform-agnostic `control` field — mapping to React component or ratatui widget happens at the consumer layer. Deletes ~930 LOC of hand-written TypeScript from `@bnto/nodes` (processor overlays, IO/container schemas, runtime Zod→control inference, hand-written field types, and document-shape types — the latter now emitted by the engine via `ts-rs`).

**Strategy doc:** [tui-strategy.md](strategy/tui-strategy.md) (§ Param Control Matrix)

**Persona ownership:**

| Package                   | Persona              |
| ------------------------- | -------------------- |
| `engine/crates/bnto-core` | `/rust-expert`       |
| `engine/crates/bnto-*`    | `/rust-expert`       |
| `packages/@bnto/nodes`    | `/core-architect`    |
| `packages/@bnto/form`     | `/frontend-engineer` |
| `engine/crates/bnto`      | `/rust-expert`       |

#### Wave 1 — Engine owns schema (sequential, see plan doc PRs 1–3)

- [ ] `engine/crates/bnto-core` — **Extend ParameterDef + ParameterType shape** (plan doc PR 1): add `group`, `suffix`, `control`, `accept`, `presets`, `inverted` to `ParameterDef`; refactor `ParameterType::Enum` options to `Vec<OptionEntry { value, label }>`; add `ParameterType::Array` and `ParameterType::Record` variants; add `PresetEntry`/`OptionEntry` structs; optional `ts-rs` derives. Update `common.rs` shared builders. Update all 8 processor `metadata()` impls across `bnto-image` (compress, resize, convert, overlay, strip-exif), `bnto-file` (rename), `bnto-csv` (clean, rename). Serde tests, processor-level metadata tests. Processor count unchanged.
- [ ] `engine/crates/bnto-core` — **Add ParameterDef metadata for 7 IO/container/data node types** (plan doc PR 2): new `metadata/io_container.rs` with `io_container_param_defs()` for `input`, `output`, `loop`, `group`, `transform`, `parallel`, `edit-fields`. Port param defaults, constraints, `visible_when`, `surfaceable`, enum `OptionEntry` labels verbatim from existing `@bnto/nodes/src/schemas/*.ts`. Catalog snapshot gains `params` arrays on 7 non-processor node types. `all_node_types()` still returns 20 entries.
- [ ] `engine/crates/bnto-core` — **Add document-shape Rust types** (plan doc PR 3 prerequisite): new `engine/crates/bnto-core/src/definition.rs` (or equivalent) with Rust structs for `Definition`, `Edge`, `Port`, `Metadata`, `Recipe`, `AcceptSpec` — the `.bnto.json` document shape. Add `ts-rs` derives so codegen can emit matching TypeScript. The engine already parses `.bnto.json` and owns `DEFINITION_JSON_SCHEMA`; this formalizes the types so `@bnto/nodes` can ingest them instead of hand-writing them.
- [ ] `packages/@bnto/nodes` — **Codegen overhaul + delete ~930 LOC** (plan doc PR 3): extend `generate-from-catalog.ts` to (a) absorb `inferFieldType.ts`'s Zod→control decision tree — every generated param gets an explicit `control` field at codegen time; (b) emit `NodeSchema`/`NodeParamField`/`NodeParamControl`/`SelectOption`/`PresetEntry`/`VisibleWhenClause` TypeScript types (via `ts-rs` or hand-emitted); (c) generate Zod schemas for all 20 node types including IO/container; (d) emit `Definition`/`Edge`/`Port`/`Metadata`/`Recipe`/`AcceptSpec` document-shape types via `ts-rs` from `engine/crates/bnto-core/src/definition.rs`. Collapse `schemas/registry.ts` to ~5-line Map over generated entries. Delete 8 processor overlays (~228 LOC), 7 IO/container hand-written schemas (~371 LOC), `inferFieldType.ts` (~211 LOC), `schemas/types.ts`, `engineSchemaEntries.ts`, `definition.ts` (~30 LOC — now engine-generated via `ts-rs`), `recipe.ts` (~20 LOC — same). Update `catalogValidation.test.ts`, `nodeTypes.test.ts` to cover all 20 node types.

#### Wave 2 — Consumers (parallel, see plan doc PRs 4–6)

_Web verification (plan doc PR 4)_

- [ ] `apps/web` + `packages/@bnto/form` + `packages/editor` — **Web verification** (plan doc PR 4): run `task e2e:editor`; verify editor config panel, Motorway form showcase, SchemaForm render identically after the ~930 LOC deletion. Verify (or add) `controlType → React component` registry in `@bnto/form`. Fix any consumer regressions; do NOT re-introduce overlays.

_TUI type-aware controls (plan doc PRs 5–6)_

- [ ] `engine/crates/bnto` — **Enrich ParamEntry with full metadata** (plan doc PR 5): carry `constraints`, `description`, `placeholder`, `group`, `suffix`, `presets`, `control`, `visible_when` from engine into `ParamEntry`. Use `node_type_params()` so IO/container node types also get controls. Update `detail_loader.rs`, `from_test_data()`, all test fixtures.
- [ ] `engine/crates/bnto` — **TUI controls module** (plan doc PR 5): new `src/tui/screens/controls/` with `boolean.rs`, `enum_select.rs`, `number.rs`. Dispatch on `control` field in `render_detail.rs` — mirrors `@bnto/form`'s role on the web. `Space`/`Enter` toggles bool, `←`/`→` cycles enum (displays `label`, stores `value`) / steps number (clamped to constraints), preset shortcut keys jump to preset values, `d` resets to default. `DetailMessage` gains `ToggleBool`, `EnumNext`, `EnumPrev`, `NumberIncrement`, `NumberDecrement`, `ResetDefault`, `ClearError`. `DetailModel.error: Option<String>` clears on next keystroke. Render suffix annotation and inline description.
- [ ] `engine/crates/bnto` — **TUI visibility + custom recipes + scrolling** (plan doc PR 6): evaluate `visible_when` against current values — hidden params skip rendering and focus; `FocusNext`/`FocusPrev` skip hidden params; confirm omits hidden params. `bnto tui recipe.bnto.json` loads a custom recipe and skips browser; invalid file produces clear error. Detail screen auto-scrolls focused param into view; overflow indicator appears when content scrolls.

#### Wave 3 — Ship (sequential, see plan doc PR 7)

- [ ] `engine/crates/bnto` — **End-to-end integration test** (plan doc PR 7): `tests/tui_schema_controls_integration.rs` loading `compress-images`, asserting quality renders bounded Number with presets, format renders Enum select, case renders Enum with labels, `overlay-watermark` renders image as file picker + watermarkPreview synthetic control, description lines render in help area, `visible_when` filtering active.
- [ ] Update **tui-strategy.md** Param Control Matrix with shipped status. Update **README** TUI section. Update **CLAUDE.md** (`@bnto/nodes is a barrel over engine-generated code`). Mark Sprint 11 complete in **PLAN.md**.

**After Sprint 11:** File picker UX overhaul (ratatui-explorer, directory tree, breadcrumb, scroll). Then file node ecosystem expansion, more node types, recipe expansion.
```

---

## Verification (end-to-end)

After all 7 PRs merge, run the full quality gate:

```
task check
```

Manual spot-checks:

```
bnto tui --theme tokyo                             # quality slider with presets, format select, case select
bnto tui engine/recipes/compress-images.bnto.json  # custom recipe path
task dev                                           # web editor, verify image-overlay file picker, resize dimensions grouping, input node extensions tagPicker
```

Expected state:

- `engine/crates/bnto-core/src/metadata.rs` `ParameterDef` has 16 fields (was 10 — added `group`, `suffix`, `control`, `accept`, `presets`, `inverted`)
- `ParameterType` enum gains `Array` + `Record` variants; `Enum` options are `Vec<OptionEntry { value, label }>`
- Engine owns `ParameterDef` metadata for all 20 node types (13 processors + 7 IO/container/data)
- `packages/@bnto/nodes/src/schemas/` contains ONLY `registry.ts` (~5-line `Map` over generated entries) + `index.ts` barrel — every hand-written schema file deleted (8 processor overlays + 7 IO/container schemas + `inferFieldType.ts` + `types.ts` + `engineSchemaEntries.ts`)
- Package-root `packages/@bnto/nodes/src/definition.ts` (~30 LOC) and `recipe.ts` (~20 LOC) deleted — `Definition` / `Edge` / `Port` / `Metadata` / `Recipe` / `AcceptSpec` types now engine-generated via `ts-rs` from `engine/crates/bnto-core/src/definition.rs`
- `packages/@bnto/nodes/src/generated/types.ts` emits `NodeSchema` / `NodeParamField` / `NodeParamControl` / `SelectOption` / `PresetEntry` / `VisibleWhenClause` (via `ts-rs` or hand-emitted)
- Every `NodeParamField` carries an explicit `control` field — no runtime Zod→control inference
- TUI `bnto tui` renders type-aware controls (boolean toggle, enum select, bounded number with presets, file picker) dispatched through `engine/crates/bnto/src/tui/screens/controls/`
- `@bnto/form` owns `controlType → React component` registry (mirror of TUI `controls/` module)
- Web editor renders identically (no visual regression in editor config panel, Motorway showcase, or SchemaForm)
- Catalog snapshot (`engine/catalog.snapshot.json`) carries new fields on each processor plus `params` arrays on the 7 non-processor node types
- Count assertions unchanged: 11 browser processors, 20 node types, 15 recipes
- ~930 LOC net deleted from `@bnto/nodes`

---

## Risk & Open Questions

**Low risk:**

- `ParameterType::Enum` variant shape change — internal to engine; codegen mapping changes at one site (`generate-from-catalog.ts`). Hand-written web overlays being deleted already expect labeled options, so the downstream TS shape is preserved.
- New `Option<T>` / `Vec<T>` fields on `ParameterDef` — backwards-compatible serde (`skip_serializing_if = "Option::is_none"` keeps catalog snapshot diffs minimal when fields are absent).
- Deleting `inferFieldType.ts` — its Zod→control decision tree is moving to codegen time; the generated output has an explicit `control` field for every param, so no runtime inference is needed.

**Medium risk:**

- `watermarkPreview` control in `imageOverlay.ts` isn't tied to a Rust param today — it's a synthetic overlay field. **Resolution options:** (a) add a virtual parameter to the overlay processor's `metadata()` with `control="watermarkPreview"` (preferred, keeps single source of truth); (b) keep a minimal overlay in `imageOverlay.ts` just for the preview field. Preferred path: (a). Decided in PR 1 during processor review.
- `ts-rs` adoption for Rust→TS type emission — if it proves fragile for the `NodeSchema` / `NodeParamField` shapes, fall back to hand-emitting the TypeScript declarations in the codegen script. Either way, `packages/@bnto/nodes/src/schemas/types.ts` is deleted.

**To revisit during PR 1:**

- Which IO/container schemas benefit from the same enrichments (presets, suffix, group)? Likely the non-processor node types stay minimal — they're mostly structural. PR 2 ports them verbatim from existing TS.
- Confirm `accept` moves off `ParameterType::File { accept }` and onto `ParameterDef.accept` so the file accept list is reusable when a string param is given `control="file"` (overlay's `image` param path).

---

## Appendix: Critical Files

### Wave 1 — Engine owns schema (PRs 1–3)

| File                                                                                          | Role in migration                                                                                                             |
| --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `engine/crates/bnto-core/src/metadata.rs`                                                     | Source of truth for `ParameterDef` shape (PR 1). Also hosts `node_type_params()` (PR 2)                                       |
| `engine/crates/bnto-core/src/metadata/io_container.rs`                                        | NEW (PR 2) — `io_container_param_defs()` for 7 non-processor node types                                                       |
| `engine/crates/bnto-image/src/common.rs`                                                      | Shared builders (`quality_param_def`, `format_param_def` w/ labels) (PR 1)                                                    |
| `engine/crates/bnto-image/src/{compress,resize,convert,overlay,strip_exif}.rs`                | Per-processor `metadata()` impls — add presets, suffix, group, control, accept (PR 1)                                         |
| `engine/crates/bnto-file/src/rename.rs`                                                       | Case enum converted to `OptionEntry` with labels (PR 1)                                                                       |
| `engine/crates/bnto-csv/src/{clean,rename}.rs`                                                | Any enum options converted to `OptionEntry`; group/suffix (PR 1)                                                              |
| `engine/crates/bnto-wasm/src/catalog.rs`                                                      | Catalog test updates for new fields + IO/container params (PRs 1–2)                                                           |
| `engine/catalog.snapshot.json`                                                                | Serialized catalog — regenerated via `task wasm:snapshot` (PRs 1–3)                                                           |
| `packages/@bnto/nodes/scripts/generate-from-catalog.ts`                                       | Codegen script — absorb `inferFieldType` logic; emit types; compute `control` at codegen time (PR 3)                          |
| `packages/@bnto/nodes/src/generated/types.ts`                                                 | NEW (PR 3) — emitted `NodeSchema`/`NodeParamField`/`NodeParamControl` types                                                   |
| `packages/@bnto/nodes/src/generated/schemas.ts`                                               | Regenerated — Zod schemas for all 20 node types (PR 3)                                                                        |
| `packages/@bnto/nodes/src/schemas/registry.ts`                                                | Collapsed to ~5-line `Map` over generated entries (PR 3)                                                                      |
| `packages/@bnto/nodes/src/schemas/index.ts`                                                   | Re-exports from `generated/types.ts` instead of hand-written `types.ts` (PR 3)                                                |
| `packages/@bnto/nodes/src/schemas/{image*,fileRename,spreadsheet*}.ts`                        | DELETE in PR 3 (8 files, ~228 LOC — processor overlays)                                                                       |
| `packages/@bnto/nodes/src/schemas/{input,output,loop,group,transform,parallel,editFields}.ts` | DELETE in PR 3 (7 files, ~371 LOC — IO/container schemas, now engine-owned)                                                   |
| `packages/@bnto/nodes/src/schemas/inferFieldType.ts`                                          | DELETE in PR 3 (~211 LOC — logic moved to codegen)                                                                            |
| `packages/@bnto/nodes/src/schemas/types.ts`                                                   | DELETE in PR 3 — type shapes now emitted from engine                                                                          |
| `packages/@bnto/nodes/src/schemas/engineSchemaEntries.ts`                                     | DELETE in PR 3 — overlay merge obsolete with no overlays left                                                                 |
| `engine/crates/bnto-core/src/definition.rs`                                                   | NEW (PR 1/3 prerequisite) — Rust structs for `Definition`/`Edge`/`Port`/`Metadata`/`Recipe`/`AcceptSpec` with `ts-rs` derives |
| `packages/@bnto/nodes/src/definition.ts`                                                      | DELETE in PR 3 (~30 LOC — `Definition`/`Edge`/`Port`/`Metadata` types now engine-generated via `ts-rs`)                       |
| `packages/@bnto/nodes/src/recipe.ts`                                                          | DELETE in PR 3 (~20 LOC — `Recipe`/`AcceptSpec` types now engine-generated via `ts-rs`)                                       |

### Wave 2 — Consumers (PRs 4–6)

| File                                                         | Role in migration                                                                      |
| ------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `packages/@bnto/form/`                                       | Web consumer — owns/verifies `controlType → React component` registry (PR 4)           |
| `engine/crates/bnto/src/tui/screens/controls/boolean.rs`     | NEW (PR 5) — TUI boolean toggle renderer                                               |
| `engine/crates/bnto/src/tui/screens/controls/enum_select.rs` | NEW (PR 5) — TUI cycling select renderer (displays `label`, stores `value`)            |
| `engine/crates/bnto/src/tui/screens/controls/number.rs`      | NEW (PR 5) — TUI bounded number input with preset shortcuts                            |
| `engine/crates/bnto/src/tui/screens/detail.rs`               | `ParamEntry` gains full metadata; `DetailMessage` gains control variants (PR 5)        |
| `engine/crates/bnto/src/tui/screens/detail_loader.rs`        | Maps `ParameterDef` → `ParamEntry`; uses `node_type_params()` for IO/container (PR 5)  |
| `engine/crates/bnto/src/tui/screens/render_detail.rs`        | Dispatch on `control` field to control-specific renderers; suffix + description (PR 5) |
| `engine/crates/bnto/src/tui/keys.rs`                         | Route keys to Detail controls (Space/Enter, ←/→, preset shortcuts, `d` reset) (PR 5)   |
| `engine/crates/bnto/src/main.rs`                             | clap arg for optional recipe path; skip browser when custom recipe provided (PR 6)     |

### Wave 3 — Ship (PR 7)

| File                                                          | Role in migration                                                                             |
| ------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `engine/crates/bnto/tests/tui_schema_controls_integration.rs` | NEW (PR 7) — end-to-end integration test covering every control type                          |
| `.claude/strategy/tui-strategy.md`                            | Mark Param Control Matrix entries shipped (PR 7)                                              |
| `.claude/CLAUDE.md`                                           | Update node-responsibilities guidance: "`@bnto/nodes` is a barrel over generated code" (PR 7) |
| `.claude/PLAN.md`                                             | Replace Sprint 11 section with new wave split; mark Sprint 11 complete (PR 7)                 |
| `README.md`                                                   | Regenerate TUI section via `task readme:generate`; highlight schema-driven controls (PR 7)    |
