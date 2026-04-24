# Recipe Fields — User-Facing Controls for Nodes

**Status:** Implemented (MVP — node-level fields)
**Created:** 2026-04-23
**Updated:** 2026-04-24
**Depends on:** shell-command processor (PR #444), recipe-level `requires` (PR #442)

---

## Problem

When we migrated `download-video` from a dedicated processor to `shell-command`, we lost the user-facing parameter surface. The old `bnto-video` processor declared domain-specific params (format, quality, codec) that the TUI rendered as proper form controls. Now the TUI shows raw shell-command internals:

```
Command     yt-dlp
Arguments   ["--no-playlist","--no-warnings","--merge-output-format","mp4",...]
Output Mode file
Timeout     300
```

Users should see:

```
Video URL       [https://youtube.com/watch?v=...]
Output Format   mp4  ▾
Video Codec     h264 ▾
Audio Codec     m4a  ▾
```

This isn't shell-command-specific. **Any node** should be able to declare user-facing fields — named, typed, labeled controls that hook up to node parameters via templates.

---

## Core Concept

**Every node has `parameters` — the internal config that the processor consumes.** `parameters` is the operational layer: command, args, outputMode, quality, format, etc.

**`fields` is an optional interface layer on top of `parameters`.** It declares user-facing controls that hook up to parameters via `{{fields.*}}` template substitution. Not every node needs fields — it's opt-in for when you want to offer a curated user experience instead of exposing raw parameters.

```
fields (what the user sees)     →  {{fields.*}} templates  →  parameters (what the processor consumes)
  "Output Format: mp4 ▾"           "{{fields.format}}"          "args": ["--merge-output-format", "mp4"]
```

**`fields` lives on nodes** — each node declares its own user-facing controls. The discovery hierarchy:

1. **Nodes have `fields`** — discover from individual nodes, show those
2. **No `fields` anywhere** — fall back to current behavior (surfaceable processor params from metadata)

Node-level fields are the building blocks. Recipe-level fields (composed, curated surface) are deferred until there's a real use case for cross-node field binding.

---

## Design Goals

1. **Universal** — same `fields` concept works on any node, not just shell-command
2. **Declarative** — JSON declares fields, engine + TUI/editor discover and render them
3. **Namespaced templates** — `{{fields.format}}` makes it clear where values come from, extensible to `{{env.HOME}}`, `{{ctx.output_dir}}` etc.
4. **Type-safe** — fields have types (string, number, enum, boolean) with validation constraints
5. **Progressive** — nodes without `fields` work exactly as today; adding `fields` is opt-in
6. **Cross-platform** — same field declarations render in TUI, web editor, and future desktop

---

## Schema

### `fields` on a Node

A node declares its own user-facing fields directly. The fields template into that same node's `parameters`:

```json
{
  "id": "download",
  "type": "shell-command",
  "fields": {
    "format": {
      "label": "Output Format",
      "description": "Video container format",
      "type": "enum",
      "options": [
        { "value": "mp4", "label": "MP4 (H.264)" },
        { "value": "webm", "label": "WebM (VP9)" },
        { "value": "mkv", "label": "MKV (Matroska)" }
      ],
      "default": "mp4",
      "order": 1
    },
    "videoCodec": {
      "label": "Video Codec",
      "type": "enum",
      "options": [
        { "value": "h264", "label": "H.264" },
        { "value": "vp9", "label": "VP9" },
        { "value": "av1", "label": "AV1" }
      ],
      "default": "h264",
      "order": 2
    }
  },
  "parameters": {
    "command": "yt-dlp",
    "args": [
      "--no-playlist",
      "--merge-output-format",
      "{{fields.format}}",
      "-S",
      "vcodec:{{fields.videoCodec}}"
    ],
    "outputMode": "file"
  }
}
```

### Why `fields`

`fields` is the natural name — it's what users think of ("form fields"), what recipe authors write, and what templates reference (`{{fields.format}}`). The JSON key and the template namespace match perfectly: declare `fields`, reference as `{{fields.*}}`. No mental translation needed.

### Field Definition Schema

```rust
/// A single user-facing field declaration.
///
/// Fields are the interface layer — they declare controls that hook up
/// to node parameters via `{{fields.*}}` template substitution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum FieldDef {
    String { label, description?, default?, placeholder?, order? },
    Number { label, description?, default?, min?, max?, step?, suffix?, order? },
    Boolean { label, description?, default?, order? },
    Enum { label, options: Vec<FieldOption>, description?, default?, order? },
}
```

| Type        | Additional Fields              | Control                |
| ----------- | ------------------------------ | ---------------------- |
| `"string"`  | `placeholder`                  | Text input             |
| `"number"`  | `min`, `max`, `step`, `suffix` | Slider or number input |
| `"boolean"` | —                              | Switch/checkbox        |
| `"enum"`    | `options: [{ value, label }]`  | Select/dropdown        |

### Rust Types

```rust
/// Ordered map of field name → definition.
pub type FieldDefs = BTreeMap<String, FieldDef>;
```

On `PipelineNode`:

```rust
/// Node-level field declarations — user-facing controls that map to
/// `{{fields.*}}` templates in this node's parameters.
#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
pub fields: BTreeMap<String, FieldDef>,
```

On `Definition` (authoring format — supports fields on any node in the tree):

```rust
#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
pub fields: BTreeMap<String, FieldDef>,
```

---

## Template Variable Namespaces

All template variables use Handlebars-style double-brace syntax (`{{...}}`). This avoids collisions with JSON object syntax, JS template literals, and shell expansion.

### Active Namespaces

| Namespace        | Source                                     | Example                   | Resolved By                   |
| ---------------- | ------------------------------------------ | ------------------------- | ----------------------------- |
| `{{fields.*}}`   | Field values (user-configured or defaults) | `{{fields.format}}`       | Engine, before node execution |
| `{{output_dir}}` | Engine execution context                   | `{{output_dir}}/file.mp4` | Shell-command processor       |
| `{{url}}`        | Input node URL value                       | `{{url}}`                 | Shell-command processor       |
| `{{input}}`      | Input node text value                      | `{{input}}`               | Shell-command processor       |

### Future Namespaces (Not Implemented)

| Namespace         | Source                | Example               | Use Case              |
| ----------------- | --------------------- | --------------------- | --------------------- |
| `{{env.*}}`       | Environment variables | `{{env.HOME}}`        | User-specific paths   |
| `{{meta.*}}`      | Recipe metadata       | `{{meta.name}}`       | Dynamic output naming |
| `{{node.<id>.*}}` | Cross-node output     | `{{node.step1.path}}` | Multi-step pipelines  |

---

## Engine Resolution

### When Resolution Happens

Template variables are resolved **at pipeline execution time**, after the input node provides its value but before the processing node runs. Each node's fields resolve into its own params — nodes are self-contained.

```
1. Parse recipe JSON → extract `fields` declarations from each node
2. Collect user overrides (from TUI form, editor, or CLI flags)
3. Merge: user overrides + defaults → resolved field values
4. For each node with fields: substitute `{{fields.*}}` templates in its parameters
5. Execute node with resolved parameters
```

### Where Resolution Lives

Resolution happens in the `PipelineExecutor` (engine-level), not in individual node processors. This keeps processors simple — they receive fully resolved parameters and don't know about templates. The shell-command processor sees `"--merge-output-format", "mp4"`, not `"--merge-output-format", "{{fields.format}}"`.

This means **any node type** benefits from fields, not just shell-command. An `image-compress` node could use `{{fields.quality}}` in its `parameters.quality` value if the recipe author wants to surface quality as a user control.

---

## TUI Integration

### Detail Screen Changes

The TUI detail screen (`detail_loader.rs`) walks recipe nodes and collects field-based params:

```
1. Walk all nodes in the definition tree (including nested children)
2. For each node with `fields`, convert to ParamEntry list with owning node_id
3. If no `fields` anywhere, fall back to current behavior (surfaceable processor params from metadata)
4. For input node: always surface the primary input field (URL, file, text) regardless
```

When `fields` is present, the shell-command's raw params (command, args, outputMode) are **not shown**. The author has explicitly declared what users should see.

### Download Video — Before/After

**Before (current):**

```
PARAMETERS
Command      yt-dlp
Arguments    ["--no-playlist","--no-warnings",...]
Output Mode  file
Timeout      300
```

**After (with fields):**

```
INPUT
Video URL    [https://youtube.com/watch?v=...]

PARAMETERS
Output Format   mp4  ▾
Video Codec     h264 ▾
Audio Codec     m4a  ▾
```

---

## CLI Override Routing

CLI param overrides use `nodeId:key=value` format to route field changes to the correct node:

```bash
bnto run download-video --override "download:format=webm"
```

The override system:

1. Parses `nodeId:key=value` format
2. Finds the target node in the definition tree
3. If `key` matches a declared field, updates the field's default value
4. If `key` doesn't match a field, falls back to raw param injection

Empty node_id (`:key=value`) is rejected — recipe-level fields are deferred.

---

## Implementation Status

### Phase 0: Prerequisite Rename — COMPLETE (PR #445)

- Moved edit-fields `{ values, keepOnlySet }` from `fields` into `parameters`
- Deleted `FieldsConfig` type
- Updated edit-fields processor to read from `parameters`
- Updated edit-fields recipe JSON files

### Phase 1: Schema + Engine Resolution — COMPLETE

- Added `FieldDef` enum with string/number/boolean/enum variants (`field_def.rs`)
- Added `fields` to `PipelineNode` and `Definition`
- Implemented `resolve_fields()` in executor (`resolve.rs`)
- Double-brace template syntax (`{{fields.*}}`)
- Unit tests for resolution, field conversion, type checking

### Phase 2: TUI Integration — COMPLETE

- Updated `detail_loader.rs` to discover fields from individual nodes
- `detail_fields.rs` converts `FieldDef` → `ParamEntry` with owning node_id
- Fields render as TUI form controls (enum → select, number → slider, etc.)

### Phase 3: First Recipe Migration — COMPLETE

- `download-video.bnto.json` uses node-level fields on the shell-command node
- Template syntax: `{{fields.format}}`, `{{fields.videoCodec}}`, `{{fields.audioCodec}}`
- Shell-command placeholders: `{{output_dir}}`, `{{url}}`, `{{input}}`

### Phase 4: Web Editor — FUTURE

- Render `fields` in the editor's config panel via `@bnto/form`
- Allow recipe authors to define/edit fields in the visual editor
- Store field values in editor state, export in `.bnto.json`

### Recipe-Level Fields — DEFERRED

- Recipe root `fields` that template into child node params
- Deferred until there's a real use case for cross-node field binding

---

## Design Decisions

| Decision                             | Choice                                  | Rationale                                                                                           |
| ------------------------------------ | --------------------------------------- | --------------------------------------------------------------------------------------------------- |
| JSON key name                        | `fields`                                | Natural name — matches template namespace `{{fields.*}}`, matches user mental model ("form fields") |
| Fields location (MVP)                | Node-level only                         | Building blocks first — recipe-level composition deferred until real use case                       |
| `parameters` vs `fields`             | Distinct roles                          | `parameters` = internal config. `fields` = optional user-facing controls that hook up via templates |
| Template syntax                      | `{{double braces}}`                     | Handlebars-style avoids collision with JSON objects, JS template literals, shell expansion          |
| Resolution layer                     | PipelineExecutor                        | All node types benefit. Processors receive fully resolved params                                    |
| Field type system                    | Tagged union (`#[serde(tag = "type")]`) | Clean JSON, exhaustive matching in Rust, easy to extend                                             |
| Processor params when fields present | Hidden from TUI                         | Author opted into a curated surface. Raw params are implementation details                          |

---

## Non-Goals (This Design)

- **Expression evaluation** — `{{fields.format}}` is literal substitution, not an expression language. No `{{fields.width * 2}}` or conditionals.
- **Cross-node references** — `{{node.step1.output}}` is a future namespace, not part of this design.
- **Runtime field discovery** — fields are statically declared in recipe JSON, not computed at runtime.
- **Nested fields** — no `{{fields.video.codec}}`. Flat namespace only.
