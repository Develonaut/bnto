# Recipe Fields — User-Facing Controls for Recipes and Nodes

**Status:** Design
**Created:** 2026-04-23
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

This isn't shell-command-specific. **Any recipe or node** should be able to declare user-facing fields — named, typed, labeled controls that hook up to node parameters via templates.

### Two Gaps

1. **No field declarations** — recipe/node authors can't define custom user-facing controls
2. **Input node params not surfaced** — the TUI detail screen skips input/output nodes entirely, so the URL field for URL-mode recipes is invisible until execution time

---

## Core Concept

**Every node has `parameters` — the internal config that the processor consumes.** `parameters` is the operational layer: command, args, outputMode, quality, format, etc.

**`fields` is an optional interface layer on top of `parameters`.** It declares user-facing controls that hook up to parameters via `{fields.*}` template substitution. Not every node needs fields — it's opt-in for when you want to offer a curated user experience instead of exposing raw parameters.

```
fields (what the user sees)     →  {fields.*} templates  →  parameters (what the processor consumes)
  "Output Format: mp4 ▾"           "{fields.format}"          "args": ["--merge-output-format", "mp4"]
```

**`fields` can live on any `Definition`** — recipe root or individual node. The discovery hierarchy:

1. **Recipe root has `fields`** — those are the user surface, template into child node `parameters`
2. **No recipe-level fields, but nodes have `fields`** — discover from individual nodes, show those
3. **No `fields` anywhere** — fall back to current behavior (surfaceable processor params from metadata)

Node-level fields are the building blocks. Recipe-level fields are the composed, curated surface.

---

## Design Goals

1. **Universal** — same `fields` concept works on any recipe or node, not just shell-command
2. **Declarative** — JSON declares fields, engine + TUI/editor discover and render them
3. **Namespaced templates** — `{fields.format}` makes it clear where values come from, extensible to `{env.HOME}`, `{ctx.output_dir}` etc.
4. **Type-safe** — fields have types (string, number, enum, boolean) with validation constraints
5. **Progressive** — recipes without `fields` work exactly as today; adding `fields` is opt-in
6. **Cross-platform** — same field declarations render in TUI, web editor, and future desktop

---

## Prerequisite: Rename Existing `Definition.fields`

`Definition.fields` currently exists as `Option<FieldsConfig>` where `FieldsConfig = { values, keepOnlySet }`. This belongs to the `edit-fields` node type — it's the node's operational config for data record manipulation ("set these key-value pairs on each record, optionally drop all other columns").

**This is actually `parameters`, not fields.** The edit-fields node's `{ values, keepOnlySet }` tells the processor what to do — it's internal config, not a user-facing declaration. It should move into `parameters` where it belongs.

**Migration:**

- `FieldsConfig` → delete (its data moves into edit-fields `parameters`)
- `Definition.fields: Option<FieldsConfig>` → `Definition.fields: BTreeMap<String, FieldDef>` (new universal meaning)
- Edit-fields recipe JSON: move `"fields": { "values": {...}, "keepOnlySet": true }` into `"parameters": { "values": {...}, "keepOnlySet": true }`
- Edit-fields processor: read from `parameters` instead of `fields`

No backwards compatibility needed — no customers yet.

**Blast radius (small):**

- Rust: `definition.rs` (struct + field), processor code — 2-3 files
- TypeScript: generated via `ts-rs` codegen, auto-updates
- Manual TS: `definition.ts`, `index.ts` re-exports, registry types — ~8 files
- Recipe JSON: edit-fields recipes only — move `"fields"` content into `"parameters"`

---

## Schema

### `fields` on a Recipe

The recipe root declares user-facing controls that template into child node parameters:

```json
{
  "id": "download-video",
  "name": "Download Video",
  "fields": {
    "format": {
      "label": "Output Format",
      "description": "Video container format for the downloaded file",
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
    },
    "audioCodec": {
      "label": "Audio Codec",
      "type": "enum",
      "options": [
        { "value": "m4a", "label": "M4A (AAC)" },
        { "value": "opus", "label": "Opus" },
        { "value": "mp3", "label": "MP3" }
      ],
      "default": "m4a",
      "order": 3
    }
  },
  "nodes": [
    {
      "id": "download",
      "type": "shell-command",
      "parameters": {
        "command": "yt-dlp",
        "args": [
          "--no-playlist",
          "--no-warnings",
          "--merge-output-format",
          "{fields.format}",
          "-S",
          "vcodec:{fields.videoCodec},acodec:{fields.audioCodec}",
          "-o",
          "{ctx.output_dir}/%(title)s.%(ext)s"
        ],
        "outputMode": "file"
      }
    }
  ]
}
```

### `fields` on a Node

A node can also declare its own fields directly. This is useful when a single node wants to expose a curated surface without the recipe author having to hoist fields to the root:

```json
{
  "id": "download",
  "type": "shell-command",
  "fields": {
    "format": {
      "label": "Output Format",
      "type": "enum",
      "options": [
        { "value": "mp4", "label": "MP4" },
        { "value": "webm", "label": "WebM" }
      ],
      "default": "mp4"
    }
  },
  "parameters": {
    "command": "yt-dlp",
    "args": ["--merge-output-format", "{fields.format}"]
  }
}
```

Node-level fields template into that same node's `parameters`. Recipe-level fields template into child node `parameters`.

### Why `fields`

`fields` is the natural name — it's what users think of ("form fields"), what recipe authors write, and what templates reference (`{fields.format}`). The JSON key and the template namespace match perfectly: declare `fields`, reference as `{fields.*}`. No mental translation needed.

### Field Definition Schema

```rust
/// A single user-facing field declaration.
///
/// Fields are the interface layer — they declare controls that hook up
/// to node parameters via `{fields.*}` template substitution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct FieldDef {
    /// Display label shown in TUI/editor
    pub label: String,

    /// Help text shown below the control
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Default value (used when user doesn't override)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Display order (lower = first). Fields without order sort alphabetically.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,

    /// Visibility condition — show this field only when another field has a specific value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_when: Option<serde_json::Value>,
}
```

With type-specific variants via `#[serde(tag = "type")]`:

| Type        | Additional Fields              | Control                |
| ----------- | ------------------------------ | ---------------------- |
| `"string"`  | `placeholder`, `pattern`       | Text input             |
| `"number"`  | `min`, `max`, `step`, `suffix` | Slider or number input |
| `"boolean"` | —                              | Switch/checkbox        |
| `"enum"`    | `options: [{ value, label }]`  | Select/dropdown        |

### Rust Type

```rust
/// User-facing field declarations.
///
/// Maps field names to their type, label, default, and constraints.
/// Values are resolved at execution time via `{fields.<name>}`
/// template substitution into node parameters.
pub type FieldDefs = BTreeMap<String, FieldDef>;
```

On `Definition`:

```rust
/// User-facing field declarations — named, typed controls that
/// hook up to node parameters via `{fields.*}` template substitution.
/// Optional on any Definition — recipe root or individual node.
#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
pub fields: BTreeMap<String, FieldDef>,
```

---

## Template Variable Namespaces

All template variables use dot notation with an explicit namespace prefix. This makes templates self-documenting — when reading `{fields.format}`, you know exactly where the value comes from.

### Active Namespaces

| Namespace          | Source                                     | Example                     | Resolved By                   |
| ------------------ | ------------------------------------------ | --------------------------- | ----------------------------- |
| `{fields.*}`       | Field values (user-configured or defaults) | `{fields.format}`           | Engine, before node execution |
| `{ctx.output_dir}` | Engine execution context                   | `{ctx.output_dir}/file.mp4` | Engine, temp dir creation     |
| `{ctx.url}`        | Input node URL value                       | `{ctx.url}`                 | Engine, input injection       |
| `{ctx.input}`      | Input node text value                      | `{ctx.input}`               | Engine, input injection       |

### Future Namespaces (Not Implemented)

| Namespace       | Source                | Example             | Use Case              |
| --------------- | --------------------- | ------------------- | --------------------- |
| `{env.*}`       | Environment variables | `{env.HOME}`        | User-specific paths   |
| `{meta.*}`      | Recipe metadata       | `{meta.name}`       | Dynamic output naming |
| `{node.<id>.*}` | Cross-node output     | `{node.step1.path}` | Multi-step pipelines  |

### Migration: Current Placeholders -> Namespaced

The existing placeholders (`{output_dir}`, `{url}`, `{input}`) migrate to the `{ctx.*}` namespace. No backwards compatibility needed — no customers yet, so we update all recipes and the engine in one pass. Clean break, no dual-support complexity.

---

## Engine Resolution

### When Resolution Happens

Template variables are resolved **at pipeline execution time**, after the input node provides its value but before the processing node runs. The resolution order:

```
1. Parse recipe JSON -> extract `fields` declarations (recipe root + nodes)
2. Collect user overrides (from TUI form, editor, or CLI flags)
3. Merge: user overrides + defaults -> resolved field values
4. Walk all node `parameters` strings and substitute `{fields.*}` templates
5. Substitute `{ctx.*}` templates (output_dir, url, input)
6. Execute node with resolved parameters
```

### Resolution Function

```rust
/// Resolve `{fields.*}` placeholders in a string using field values.
fn resolve_fields(
    template: &str,
    field_values: &BTreeMap<String, serde_json::Value>,
) -> String {
    let mut result = template.to_string();
    for (name, value) in field_values {
        let placeholder = format!("{{fields.{name}}}");
        let replacement = match value {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        result = result.replace(&placeholder, &replacement);
    }
    result
}
```

### Where Resolution Lives

Resolution happens in the `PipelineExecutor` (engine-level), not in individual node processors. This keeps processors simple — they receive fully resolved parameters and don't know about templates. The shell-command processor sees `"--merge-output-format", "mp4"`, not `"--merge-output-format", "{fields.format}"`.

This means **any node type** benefits from fields, not just shell-command. An `image-compress` node in a recipe could use `{fields.quality}` in its `parameters.quality` value if the recipe author wants to surface quality as a user control.

---

## TUI Integration

### Detail Screen Changes

The TUI detail screen (`detail_loader.rs`) currently:

1. Walks recipe nodes
2. Skips input/output nodes
3. Collects surfaceable params from processor metadata

With fields, the loading logic becomes:

```
1. Check root definition for `fields` -> if present, these ARE the user controls
2. If no root `fields`, check individual nodes for `fields` -> aggregate those
3. If no `fields` anywhere, fall back to current behavior (walk nodes, collect processor params)
4. For input node: always surface the primary input field (URL, file, text) regardless
```

When `fields` is present, the shell-command's raw params (command, args, outputMode) are **not shown**. The author has explicitly declared what users should see.

### Input Node Visibility

The detail screen should always show the input node's primary field:

| Input Mode    | Surfaced As                                            | Control         |
| ------------- | ------------------------------------------------------ | --------------- |
| `url`         | Text field with URL placeholder from input node params | Text input      |
| `text`        | Text area with label from input node params            | Text area       |
| `file-upload` | File picker (already works via `PickerModel`)          | Embedded picker |

For `url` and `text` modes, this field appears **first** in the params list — before any fields. It's the "what are you processing?" question.

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

## Web Editor Integration

The web editor's config panel would discover fields the same way:

1. Check root definition for `fields`
2. Render `@bnto/form` controls for each field (schema-driven, already supports enum/string/number/boolean)
3. Store user values in the editor state
4. On export, the `.bnto.json` includes `fields` with current values as defaults

The visual editor could also allow recipe authors to **define** fields — a panel where you add named fields with types, which generates the `fields` block in the recipe JSON.

---

## TypeScript Types

```typescript
/** A single user-facing field declaration. */
export type FieldDef = {
  label: string;
  description?: string;
  default?: unknown;
  order?: number;
  visibleWhen?: VisibleWhen;
} & (
  | { type: "string"; placeholder?: string; pattern?: string }
  | { type: "number"; min?: number; max?: number; step?: number; suffix?: string }
  | { type: "boolean" }
  | { type: "enum"; options: Array<{ value: string; label: string }> }
);

/** Field declarations — maps field names to their definitions. */
export type FieldDefs = Record<string, FieldDef>;
```

On `Definition`:

```typescript
export type Definition = {
  // ... existing properties ...
  fields?: FieldDefs;
};
```

---

## Validation

### Recipe Load Time

When a recipe with `fields` is loaded:

1. Every `{fields.*}` reference in node parameters must have a matching key in `fields`
2. Every field in `fields` should be referenced by at least one `{fields.*}` template (warn if unused — not an error)
3. Default values must pass type validation (enum default must be one of the options, number default must be within min/max)

### Execution Time

Before resolving templates:

1. User-provided values must pass type validation
2. Missing values fall back to defaults
3. Fields without defaults and without user values produce a clear error: "Required field 'format' has no value"

---

## Examples

### Download Video (shell-command + recipe fields)

See [Schema](#fields-on-a-recipe) above for the full example.

### Compress Images (existing processor params -> recipe fields)

```json
{
  "id": "compress-images",
  "fields": {
    "quality": {
      "label": "Quality",
      "type": "number",
      "min": 1,
      "max": 100,
      "default": 80,
      "suffix": "%",
      "order": 1
    }
  },
  "nodes": [
    {
      "type": "image-compress",
      "parameters": { "quality": "{fields.quality}" }
    }
  ]
}
```

The recipe author decides what's surfaced. The processor declares what's **possible** (via `parameters`); the recipe's `fields` declare what's **visible** to the user.

### Future: AI Caption Images

```json
{
  "id": "caption-images",
  "fields": {
    "style": {
      "label": "Caption Style",
      "type": "enum",
      "options": [
        { "value": "descriptive", "label": "Descriptive" },
        { "value": "alt-text", "label": "Alt Text (Accessibility)" },
        { "value": "social", "label": "Social Media Caption" }
      ],
      "default": "descriptive"
    },
    "maxLength": {
      "label": "Max Length",
      "type": "number",
      "min": 10,
      "max": 500,
      "default": 150,
      "suffix": "chars"
    }
  }
}
```

---

## Implementation Order

### Phase 0: Prerequisite Rename

- Move edit-fields `{ values, keepOnlySet }` from `fields` into `parameters`
- Delete `FieldsConfig` type
- Update edit-fields processor to read from `parameters`
- Update edit-fields recipe JSON files
- Run codegen -> TypeScript types

### Phase 1: Schema + Engine Resolution

- Add `fields: BTreeMap<String, FieldDef>` to Rust `Definition`
- Add `FieldDef` enum with string/number/boolean/enum variants
- Implement `resolve_fields()` in `PipelineExecutor`
- Migrate all existing placeholders to `{ctx.*}` namespace
- Run codegen -> TypeScript types
- Unit tests for resolution, validation, type checking

### Phase 2: TUI Integration

- Update `detail_loader.rs` to check for `fields` before walking nodes
- Surface input node's primary field (URL/text) for non-file recipes
- Render fields as `bnto-form` controls (enum -> select, number -> slider, etc.)
- Pass resolved field values through to execution

### Phase 3: First Recipe Migration

- Update `download-video.bnto.json` with `fields` block
- Remove raw shell-command params from TUI surface
- Verify TUI renders: URL input + format/codec selects

### Phase 4: Web Editor

- Render `fields` in the editor's config panel via `@bnto/form`
- Allow recipe authors to define/edit fields in the visual editor
- Store field values in editor state, export in `.bnto.json`

---

## Design Decisions

| Decision                             | Choice                                  | Rationale                                                                                                                                                             |
| ------------------------------------ | --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JSON key name                        | `fields`                                | Natural name — matches template namespace `{fields.*}`, matches user mental model ("form fields"). Edit-fields' old `fields` was really `parameters` and moves there. |
| Fields location                      | Any `Definition`                        | Universal — works on recipe root (curated surface) or individual nodes (building blocks). Recipe-level fields take precedence over node-level.                        |
| `parameters` vs `fields`             | Distinct roles                          | `parameters` = internal config the processor consumes. `fields` = optional user-facing controls that hook up to parameters via templates.                             |
| Template namespace                   | `{fields.*}`                            | Matches JSON key. Self-documenting — reader knows exactly where the value comes from.                                                                                 |
| Resolution layer                     | PipelineExecutor                        | All node types benefit. Processors receive fully resolved params.                                                                                                     |
| Legacy placeholder support           | Clean break                             | No customers yet — migrate all recipes to `{ctx.*}` in one pass.                                                                                                      |
| Field type system                    | Tagged union (`#[serde(tag = "type")]`) | Clean JSON, exhaustive matching in Rust, easy to extend.                                                                                                              |
| Processor params when fields present | Hidden from TUI                         | Author opted into a curated surface. Raw params are implementation details.                                                                                           |
| Edit-fields migration                | Move to `parameters`                    | Its `{ values, keepOnlySet }` is operational config, not user-facing field declarations.                                                                              |

---

## Non-Goals (This Design)

- **Expression evaluation** — `{fields.format}` is literal substitution, not an expression language. No `{fields.width * 2}` or conditionals.
- **Cross-node references** — `{node.step1.output}` is a future namespace, not part of this design.
- **Runtime field discovery** — fields are statically declared in recipe JSON, not computed at runtime.
- **Nested fields** — no `{fields.video.codec}`. Flat namespace only.
