# Schema-Driven Config Panel — Detailed Task Breakdown

**Archived from:** PLAN.md backlog (Feb 24, 2026)
**Status:** Backlog — not yet started
**Prior art:** Atomiton project (`~/Code/atomiton`)

---

## Problem

The frontend currently hardcodes per-node configuration shapes (quality sliders, format selectors, column mappings) in `app/[bnto]/_components/configs/`. This creates two sources of truth — the Go engine knows what each node expects, and the frontend independently guesses. Every new node type requires new hardcoded UI code. Constraints can drift (engine says max quality is 100, frontend slider goes to 95).

## Solution

Define node parameter schemas once (in Go), expose them as structured metadata, and auto-derive the config panel UI from the schema. The frontend renders controls dynamically based on schema introspection. Only UI-specific concerns that can't be inferred from the schema (e.g., "this string field should render as a code editor") require explicit overrides.

## The Pattern (proven in atomiton)

```
Schema (single source of truth)
  │
  ├── Used in Go engine for validation at execution time
  │
  └── Exposed as structured metadata to frontend
        │
        └── createFieldsFromSchema(schema, overrides)
              │
              ├── Auto-derived (~70-80% of fields need zero configuration):
              │   - controlType: string→text, number→number, enum→select, boolean→checkbox, url→url, email→email
              │   - label: camelCase→"Title Case" (e.g., maxRetries→"Max Retries")
              │   - required: derived from optional/required in schema
              │   - min/max: derived from .min()/.max() constraints
              │   - options: derived from enum values
              │   - helpText: derived from .describe() on the schema field
              │   - placeholder: derived from .default() value (shows "Default: X")
              │
              └── Selective overrides (~20-30% of fields):
                  - controlType override (string→"code", string→"textarea")
                  - custom option labels (enum value "GET" → label "GET - Retrieve data")
                  - rows for textarea
                  - step for number inputs
                  - placeholder text
```

## Control Type Taxonomy

| Schema type | Auto-derived control | When to override |
|---|---|---|
| `string` | `text` | Override to `textarea`, `code`, `markdown`, `password` |
| `string` with url validation | `url` | Rarely |
| `string` with email validation | `email` | Rarely |
| `number` | `number` | Override to `range` (slider) for bounded values |
| `boolean` | `checkbox` / `switch` | Rarely |
| `enum` | `select` (dropdown) | Override option labels for clarity |
| `object` | `json` | Override to `code` for complex objects |
| `array` | `json` | Override to `textarea` for simple string arrays |
| `date` | `date` | Rarely |

## Implementation Layers

### Layer 1: Go engine schema declarations
- [ ] `engine` — Define a `ParameterSchema` struct that each node type can declare: field name, Go type (string/number/bool/enum/object/array), default value, constraints (min/max, allowed values), description text, and whether it's required
- [ ] `engine` — Each node type in `pkg/node/library/` registers its `ParameterSchema` alongside its `Executable` (co-located, same pattern as atomiton's schema + definition + executable per node type)
- [ ] `engine` — Schema registry: `pkg/registry/` exposes `GetParameterSchema(nodeType string) *ParameterSchema`
- [ ] `engine` — Unit tests: verify every registered node type has a parameter schema, and schema constraints match execution validation

### Layer 2: API exposure
- [ ] `apps/api` — `GET /nodes/{type}/schema` endpoint returns the parameter schema as JSON
- [ ] `apps/api` — `GET /nodes/schemas` endpoint returns all node type schemas in one request (for frontend caching)
- [ ] `apps/api` — Integration tests: verify schema responses match expected shapes

### Layer 3: TypeScript schema consumption
- [ ] `@bnto/core` — TypeScript types for `ParameterSchema` and `FieldConfig` (mirror the Go structs)
- [ ] `@bnto/core` — `createFieldsFromSchema(schema, overrides?)` utility — introspects the parameter schema and returns UI field configs. Port the proven logic from atomiton's implementation (see `~/Code/atomiton/packages/@atomiton/nodes/src/core/utils/createFieldsFromSchema.ts`)
- [ ] `@bnto/core` — Unit tests for `createFieldsFromSchema` — test every auto-derivation case: type inference, label formatting, constraint extraction, default value placeholder, optional detection, enum option generation, and override merging (port test cases from atomiton's test suite)
- [ ] `@bnto/core` — Hook: `useNodeSchema(nodeType)` fetches and caches the parameter schema via React Query

### Layer 4: Dynamic config panel
- [ ] `apps/web` — Generic `ConfigPanel` component that renders a form from `FieldConfig[]` — maps each `controlType` to the appropriate shadcn input component (Input, Select, Slider, Switch, Textarea, etc.)
- [ ] `apps/web` — Per-bnto override files (only for UI hints that can't be inferred): custom option labels, control type overrides, field grouping/ordering. These are thin — ~20-30% of fields at most
- [ ] `apps/web` — Integration tests: verify the schema-to-UI pipeline (every schema field produces a UI field, every UI field traces back to a schema field — no orphans in either direction)
- [ ] `apps/web` — Remove hardcoded per-bnto config components once the generic renderer covers all Tier 1 cases

### Layer 5: Pipeline integrity tests
- [ ] `apps/web` — E2E test: load a bnto tool page, verify the config panel renders the correct controls for that node type's schema (e.g., `/compress-images` shows a quality slider with min=1 max=100)
- [ ] `engine` + `apps/web` — Contract test: when a new node type is added to the Go engine with a parameter schema, the frontend can render a config panel for it with zero new UI code (the test adds a mock node type and verifies the pipeline end-to-end)

## Design Decisions (resolved by studying atomiton)

- **Schema format:** Use a lean custom format (not JSON Schema). A simple struct with `{ field, type, default, min, max, enum, required, description }` is sufficient. Go engine can declare schemas with a builder pattern similar to Zod.
- **UI hints in schema vs. registry:** The schema declares constraints and descriptions. The bnto registry (or per-bnto override files) declares purely visual concerns (grouping, display order, custom labels). This keeps the engine UI-agnostic.
- **Fixtures and defaults:** The `.bnto.json` fixture embeds the *current* parameter values. The schema provides the *defaults* and *constraints*. When the config panel loads, it reads parameter values from the fixture and uses the schema for validation and control rendering.

## Key Atomiton Reference Files

- `packages/@atomiton/nodes/src/core/utils/createFieldsFromSchema.ts` — the auto-derive utility
- `packages/@atomiton/nodes/src/core/utils/createFieldsFromSchema.test.ts` — thorough test suite
- `packages/@atomiton/nodes/src/core/types/parameters.ts` — `NodeFieldConfig` and `NodeFieldControlType`
- `packages/@atomiton/nodes/src/schemas/image/index.ts` — example schema with `.describe()`, `.min()`, `.max()`, `.default()`, `.enum()`, `.optional()`
- `packages/@atomiton/nodes/src/definitions/image/fields.ts` — example of `createFieldsFromSchema(imageSchema, overrides)` with ~30% selective overrides
