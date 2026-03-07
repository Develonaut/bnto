# Input & Output Nodes — Architecture Reference

**Created:** March 4, 2026
**Status:** Proposed — Sprint 4C
**Related:** [editor-architecture.md](editor-architecture.md), [bntos.md](bntos.md), [visual-editor.md](visual-editor.md), [node-architecture.md](node-architecture.md)

---

## Problem

Recipes are not self-describing. The `.bnto.json` definition says _what to do_ but not _what it needs_ or _what it produces_. Input and output are implicit — hardcoded into the UI per recipe slug.

### Today's I/O model

```
UI layer (hardcoded per slug)          Recipe definition (.bnto.json)
─────────────────────────────          ────────────────────────────────
FileUpload.Dropzone                    file-system "list" → {{.INPUT_DIR}}/*
  accept={recipe.accept}               ↓
  ↓                                    loop → process node
RecipeConfigSection                     ↓
  slug → switch → CompressImagesConfig  {{.OUTPUT_DIR}}/{{basename .item}}
  ↓
BrowserExecutionResults
  downloadResult()
```

**What's wrong:**

1. **Per-slug UI hardcoding.** 6 config components (`CompressImagesConfig`, `ResizeImagesConfig`, etc.) + `BntoConfigMap` type + defaults map + switch router. Every new recipe needs a new component.
2. **Recipe portability.** A shared `.bnto.json` loses all I/O context. Community recipes, CLI, desktop — none can derive what widget to show.
3. **Editor blindness.** The editor has no way to know what a recipe expects. Custom recipes created in the editor can't render appropriate I/O controls.
4. **Dual I/O contracts.** Go engine uses `{{.INPUT_DIR}}`/`{{.OUTPUT_DIR}}` template variables. Rust WASM bypasses templates entirely — files are `ArrayBuffer` blobs. The recipe doesn't describe either.
5. **AcceptSpec duplication.** `Recipe.accept` (MIME types, extensions) lives in recipe metadata, disconnected from the node graph. It's a workaround for the missing input node.

---

## Proposal: Two New Node Types

Add `input` and `output` as first-class node types in `@bnto/nodes`. They are **declaration nodes** — they don't transform data, they describe the contract between the recipe and its environment.

### `input` node

Declares how data enters the recipe. The execution environment (browser UI, CLI, desktop, API) reads this node to know what to present to the user.

```typescript
// Node type: "input"
// Category: "io"
// browserCapable: true
// isContainer: false

parameters: {
  /** How data is provided. */
  mode: "file-upload" | "text" | "url";

  /** MIME types accepted (file-upload mode). */
  accept?: string[];       // e.g., ["image/jpeg", "image/png", "image/webp"]

  /** File extensions accepted (file-upload mode). */
  extensions?: string[];   // e.g., [".jpg", ".jpeg", ".png", ".webp"]

  /** Human-readable label for the input control. */
  label?: string;          // e.g., "JPEG, PNG, or WebP images"

  /** Whether multiple files/items are accepted. */
  multiple?: boolean;      // default: true for file-upload, false for text/url

  /** Maximum file size in bytes (file-upload mode). 0 = no limit. */
  maxFileSize?: number;

  /** Maximum number of files (file-upload mode). 0 = no limit. */
  maxFiles?: number;

  /** Placeholder text (text/url modes). */
  placeholder?: string;    // e.g., "Paste your CSV data here"
}
```

**Modes:**
| Mode | What the UI renders | Use case |
|------|-------------------|----------|
| `file-upload` | Drag-and-drop file zone | Image compression, CSV cleaning, file rename — most recipes |
| `text` | Textarea / text input | Future: paste CSV data, enter a prompt, provide JSON |
| `url` | URL input field | Future: fetch from URL, scrape, download-and-process |

**Phase 1 (this sprint):** Only `file-upload` mode. `text` and `url` are defined in the schema but not implemented in the UI — they're there for forward compatibility and the code editor's schema validation.

### `output` node

Declares how results are delivered. The execution environment reads this node to know how to present results.

```typescript
// Node type: "output"
// Category: "io"
// browserCapable: true
// isContainer: false

parameters: {
  /** How results are delivered. */
  mode: "download" | "display" | "preview";

  /** Filename template for output files (download mode). */
  filename?: string;       // e.g., "compressed-{{name}}"

  /** Whether to auto-zip multiple output files. */
  zip?: boolean;           // default: true when multiple outputs

  /** Label for the download button or display section. */
  label?: string;          // e.g., "Compressed Images"

  /** Whether to auto-download on completion. */
  autoDownload?: boolean;  // default: false
}
```

**Modes:**
| Mode | What the UI renders | Use case |
|------|-------------------|----------|
| `download` | File cards with download buttons + optional ZIP | Image compression, CSV cleaning — most recipes |
| `display` | Text/data display area | Future: show extracted text, JSON result, stats |
| `preview` | Inline preview (image gallery, table view) | Future: image preview, CSV preview before download |

**Phase 1 (this sprint):** Only `download` mode. `display` and `preview` defined in schema for forward compatibility.

---

## Relationship to Node Architecture

I/O nodes sit at the **recipe boundary** in the [three-layer node model](node-architecture.md). At the recipe level, Input and Output nodes define the recipe's **external interface** — what data comes in and what comes out. They are the contract between the recipe and its execution environment.

For nested composition (compound nodes containing subgraphs), child groups inherit the parent's I/O. Nested I/O nodes are not needed today — the root recipe's Input and Output nodes are the only I/O boundary. Future extensions may allow nested I/O for compound node interfaces.

---

## Where They Sit in a Recipe

Input and output nodes are the **first and last** nodes in every recipe. They're visible in the editor as compartments and in the code editor as JSON blocks.

### Before (current compress-images definition)

```json
{
  "id": "compress-images",
  "type": "group",
  "nodes": [
    {
      "id": "list-images",
      "type": "file-system",
      "parameters": { "operation": "list", "path": "{{.INPUT_DIR}}/*" }
    },
    {
      "id": "compress-loop",
      "type": "loop",
      "nodes": [
        {
          "id": "compress-image",
          "type": "image",
          "parameters": { "operation": "optimize", "quality": 80, "output": "{{.OUTPUT_DIR}}/..." }
        }
      ]
    }
  ]
}
```

### After (with explicit I/O nodes)

```json
{
  "id": "compress-images",
  "type": "group",
  "nodes": [
    {
      "id": "input",
      "type": "input",
      "name": "Input Files",
      "parameters": {
        "mode": "file-upload",
        "accept": ["image/jpeg", "image/png", "image/webp"],
        "extensions": [".jpg", ".jpeg", ".png", ".webp"],
        "label": "JPEG, PNG, or WebP images",
        "multiple": true
      }
    },
    {
      "id": "compress-loop",
      "type": "loop",
      "nodes": [
        {
          "id": "compress-image",
          "type": "image",
          "parameters": { "operation": "optimize", "quality": 80 }
        }
      ]
    },
    {
      "id": "output",
      "type": "output",
      "name": "Compressed Images",
      "parameters": {
        "mode": "download",
        "label": "Compressed Images",
        "zip": true,
        "autoDownload": false
      }
    }
  ]
}
```

**Key change:** The `file-system "list"` node with `{{.INPUT_DIR}}/*` is absorbed into the input node. The `{{.OUTPUT_DIR}}` template in the process node is absorbed into the output node. The recipe becomes self-describing.

---

## Impact on `Recipe.accept`

The `AcceptSpec` on `Recipe` (`accept.mimeTypes`, `accept.extensions`, `accept.label`) becomes **derivable from the input node**. Two options:

**Option A: Derive accept from input node (recommended).** Add a pure function `deriveAcceptSpec(definition)` that finds the input node and extracts its `accept`, `extensions`, and `label` parameters. `Recipe.accept` becomes computed, not stored.

**Option B: Keep accept as metadata, validate against input node.** `Recipe.accept` stays as-is. A validation function ensures it matches the input node's parameters. Dual source of truth but backward compatible.

**Recommendation:** Option A. The input node IS the accept spec. Having it in two places invites drift. The `Recipe` type keeps its `accept` field for backward compatibility, but it's populated from the input node during `definitionToRecipe()`.

---

## Engine Impact

### Rust WASM (`engine/crates/`)

Input/output nodes are **pass-through** in the Rust engine. They don't process data — they're declarations read by the environment. The Rust `NodeProcessor` trait doesn't need `InputProcessor` or `OutputProcessor` implementations.

The browser adapter in `@bnto/core` reads the input node to configure the file drop zone and reads the output node to configure result presentation. The WASM engine processes the middle nodes (image, csv, file, etc.) as it does today.

**No Rust code changes needed.** The engine processes the same node types it always has. I/O nodes are metadata consumed by the environment, not by the engine.

### Go engine (`archive/engine-go/`)

When cloud execution (M4) uses I/O nodes, the Go engine would read the input node to know what files to expect from R2 and the output node to know what to write back. This is future work — the Go engine is archived.

---

## UI Impact

### Generic `InputRenderer`

Replaces the hardcoded `FileUpload.Dropzone` + `RecipeConfigSection` switch. Reads the input node from the recipe definition and renders the appropriate widget.

```
Input node → InputRenderer
  mode: "file-upload" → FileUpload.Dropzone (accept from node params)
  mode: "text"        → Textarea (placeholder from node params) [future]
  mode: "url"         → URL input (placeholder from node params) [future]
```

### Generic `OutputRenderer`

Replaces the hardcoded `BrowserExecutionResults`. Reads the output node from the recipe definition and renders the appropriate result presentation.

```
Output node → OutputRenderer
  mode: "download" → FileCard grid + ZIP download (label from node params)
  mode: "display"  → Text display area [future]
  mode: "preview"  → Inline preview [future]
```

### Per-recipe config components

These DON'T go away immediately. The per-recipe configs (`CompressImagesConfig`, etc.) control **processing parameters** (quality, width, format), not I/O. They stay until the NodeConfigPanel (from Sprint 4 Wave 3) can render parameter forms generically from schemas. The migration path:

1. **This sprint:** I/O nodes handle input/output. Config components stay for processing params.
2. **Later:** NodeConfigPanel replaces per-recipe config components using `@bnto/nodes` schemas.

### Editor integration

I/O nodes appear as compartments in the bento grid and as JSON blocks in the code editor. In the visual editor:

- Input node = first compartment (top-left position, distinct variant color)
- Output node = last compartment (bottom-right position, distinct variant color)
- Both are always present — users can configure them but not delete them

---

## Migration Strategy

### Predefined recipes

All 6 predefined recipes get updated definitions with explicit input/output nodes:

| Recipe               | Input mode  | Input accept                      | Output mode |
| -------------------- | ----------- | --------------------------------- | ----------- |
| compress-images      | file-upload | image/jpeg, image/png, image/webp | download    |
| resize-images        | file-upload | image/jpeg, image/png, image/webp | download    |
| convert-image-format | file-upload | image/jpeg, image/png, image/webp | download    |
| clean-csv            | file-upload | text/csv                          | download    |
| rename-csv-columns   | file-upload | text/csv                          | download    |
| rename-files         | file-upload | \* (any)                          | download    |

### `file-system "list"` node removal

The `file-system` node with `operation: "list"` + `{{.INPUT_DIR}}/*` is no longer needed as the first node. The input node replaces it. The loop node iterates over files provided by the environment (via the input node's contract), not from a filesystem listing.

### `{{.OUTPUT_DIR}}` removal

Process nodes no longer write to `{{.OUTPUT_DIR}}`. Instead, they produce output that the environment (browser adapter) routes to the output node's configured delivery mode.

### Backward compatibility

Old `.bnto.json` files without I/O nodes still work. The browser adapter falls back to current behavior (accept from `Recipe.accept`, download from execution results). The migration is additive.

---

## What This Unlocks

1. **Community recipes work out of the box.** Share a `.bnto.json` and any consumer knows what to render.
2. **Editor creates runnable recipes.** Blank canvas + input node + process nodes + output node = a recipe anyone can run.
3. **Future input modes.** Text input for prompt-based recipes (AI nodes, M4). URL input for web scraping. No UI changes needed — the input node schema drives the widget.
4. **Future output modes.** Display mode for text extraction results. Preview mode for image galleries. Same principle.
5. **CLI/desktop portability.** CLI reads the input node and prompts for file paths. Desktop reads it and shows a native file picker. Same recipe, different environments.
6. **`AcceptSpec` simplification.** Recipe metadata derives from the definition, not duplicated alongside it.

---

## Open Questions

1. **Should every recipe require exactly one input and one output node?** Or can recipes have zero (pure computation) or multiple (multi-source merge)?
   - **Recommendation:** Exactly one of each for now. Recipes are linear pipelines. Multi-source is a future extension.

2. **Should the input node replace `file-system "list"` in the engine, or sit alongside it?**
   - **Recommendation:** Replace it for browser execution. The browser adapter reads the input node directly. Cloud/Go execution can still use `file-system "list"` until M4 migration.

3. **Should `Recipe.accept` be removed or kept as a derived cache?**
   - **Recommendation:** Keep the field, populate it from the input node via `deriveAcceptSpec()`. Backward compatible, single source of truth.
