# Power Recipes — Gap Analysis & Node Roadmap

**Created:** April 22, 2026
**Status:** Research — informing engine expansion priorities
**Related:** [engine-expansion.md](engine-expansion.md), [smart-iteration.md](smart-iteration.md), [expression-input-ux.md](expression-input-ux.md), [tui-strategy.md](tui-strategy.md)

---

## Context

Bnto's 18 predefined recipes prove the engine works for simple, file-centric transforms: compress images, clean CSVs, rename files. But the original motivation for bnto was a real production workflow — generating Etsy product images from 3D models. That workflow ran on the old Go-based engine and required capabilities the Rust engine doesn't have yet.

This document maps the target workflow against the current engine, identifies every gap, and proposes a priority order for closing them. The goal: a user can author a custom recipe in their library that orchestrates CSV data, API calls, CLI tools, and filesystem operations into an end-to-end pipeline.

This is the proving ground. If a user can build this recipe, bnto works.

---

## Node Maps: Visual Breakdown

### The Old Pipeline (5 Recipes, Run Sequentially)

The Heavy Handed workflow was 5 recipes run in order via `bento_order.json`. Each recipe is a separate `.bento.json` file. Here's every node, marked with what the Rust engine has today:

```
RECIPE 1: Flatten Source Folder
═══════════════════════════════
  ┌─────────────────────┐
  │ file-system (exists) │  ✗ MISSING — no file-system node
  │ Check folder exists  │
  └──────────┬──────────┘
             │
  ┌──────────▼──────────┐
  │ shell-command        │  ✗ MISSING — no shell-command node
  │ python3 Flatten.py   │
  └──────────┬──────────┘
             │
  ┌──────────▼──────────┐
  │ shell-command        │  ✗ MISSING
  │ bash -c "rename"     │
  └─────────────────────┘


RECIPE 2: Copy Product Overlays
═══════════════════════════════
  ┌─────────────────────┐
  │ spreadsheet (read)   │  ✗ MISSING — current CSV nodes transform bytes,
  │ Read products.csv    │              don't read from path or emit rows
  │   → outputs: rows    │
  └──────────┬──────────┘
             │ edges connect to loop
  ┌──────────▼──────────┐
  │ loop (forEach)       │  ✗ MISSING — current loop is per-file only,
  │ items: rows          │              not data-driven
  │ context: .item, .idx │
  │ ┌───────────────────┐│
  │ │ file-system (mkdir)││  ✗ MISSING
  │ │ Create folder:     ││
  │ │ {{.item.name}}/    ││
  │ └────────┬──────────┘│
  │          │            │
  │ ┌────────▼──────────┐│
  │ │ file-system (copy) ││  ✗ MISSING
  │ │ overlays/{{.idx}}  ││
  │ │  → {{.item.name}}/ ││
  │ │    overlay.png     ││
  │ └───────────────────┘│
  └─────────────────────┘


RECIPE 3: Merge Product STLs
════════════════════════════
  ┌─────────────────────┐
  │ file-system (exists) │  ✗ MISSING
  │ Check folder exists  │
  └──────────┬──────────┘
             │
  ┌──────────▼──────────┐
  │ shell-command        │  ✗ MISSING
  │ python3 Merge_STL.py │
  │ (calls Blender)      │
  └─────────────────────┘


RECIPE 4: Generate Product Renders
══════════════════════════════════
  ┌─────────────────────┐
  │ file-system (exists) │  ✗ MISSING
  │ Check folder exists  │
  └──────────┬──────────┘
             │
  ┌──────────▼──────────┐
  │ shell-command        │  ✗ MISSING
  │ python3 Render.py    │
  │ --theme {{THEME}}    │
  │ --zoom  {{ZOOM}}     │
  │ timeout: 1800s       │
  │ stream: true         │
  └─────────────────────┘


RECIPE 5: Generate Etsy Listings
════════════════════════════════
  ┌─────────────────────┐
  │ spreadsheet (read)   │  ✗ MISSING
  │ Read products.csv    │
  │   → outputs: rows    │
  └──────────┬──────────┘
             │
  ┌──────────▼──────────┐
  │ loop (forEach)       │  ✗ MISSING
  │ items: rows          │
  │ ┌───────────────────┐│
  │ │ shell-command      ││  ✗ MISSING
  │ │ python3 Etsy_CSV   ││
  │ │ --name {{.item.   ││
  │ │         name}}     ││
  │ │ --tags {{.item.   ││
  │ │         tags}}     ││
  │ │ env: PRICE, etc   ││
  │ └───────────────────┘│
  └─────────────────────┘
```

### Score: 0 of 5 Recipes Can Run Today

Every recipe uses at least one missing node type. The three critical gaps:

| Missing                               | Used In            | Blocking       |
| ------------------------------------- | ------------------ | -------------- |
| `shell-command`                       | Recipes 1, 3, 4, 5 | 4 of 5 recipes |
| `file-system`                         | Recipes 1, 2, 3, 4 | 4 of 5 recipes |
| `spreadsheet-read` + `forEach (data)` | Recipes 2, 5       | 2 of 5 recipes |

Plus the infrastructure that makes them work:

- **Recipe variables** — every recipe uses `{{.TARGET_PATH}}`, `{{.RENDER_THEME}}`, etc.
- **Template expressions** — every parameter reference
- **Inter-node data passing** — recipes 2 and 5 pass CSV rows through edges

---

### The New Vision: Unified Figma-Powered Pipeline

The old pipeline had 5 separate recipes because the Go engine lacked composability. The new vision is one recipe (or 2-3 composable ones) that replaces manual overlay generation with Figma API calls:

```
RECIPE: Generate Product Images (Unified)
══════════════════════════════════════════

  ┌─────────────────────────────┐
  │ spreadsheet-read             │  ✗ MISSING
  │ path: {{.PRODUCTS_DIR}}/     │
  │       products.csv           │
  │ → outputs: rows[]            │
  │   (name, stl_path, title,    │
  │    description, tags, theme) │
  └──────────────┬──────────────┘
                 │ data edge → rows
  ┌──────────────▼──────────────┐
  │ loop (forEach)               │  ✗ MISSING (data-driven)
  │ items: rows from upstream    │
  │ context: .item, .index       │
  │                              │
  │  ┌─────────────────────────┐ │
  │  │ file-system (mkdir)      │ │  ✗ MISSING
  │  │ {{.PRODUCTS_DIR}}/       │ │
  │  │ {{.item.name}}/          │ │
  │  └────────────┬────────────┘ │
  │               │               │
  │  ┌────────────▼────────────┐ │
  │  │ http-request             │ │  ✗ MISSING
  │  │ POST figma.com/api/...   │ │
  │  │ headers:                 │ │
  │  │   X-Figma-Token:         │ │
  │  │   {{.FIGMA_TOKEN}}       │ │
  │  │ body:                    │ │
  │  │   modelName:             │ │
  │  │   {{.item.name}}         │ │
  │  │   modelTitle:            │ │
  │  │   {{.item.title}}        │ │
  │  │ → outputs: overlay.png   │ │
  │  └────────────┬────────────┘ │
  │               │               │
  │  ┌────────────▼────────────┐ │
  │  │ shell-command            │ │  ✗ MISSING
  │  │ python3 render.py        │ │
  │  │ --model                  │ │
  │  │   {{.PRODUCTS_DIR}}/     │ │
  │  │   {{.item.stl_path}}     │ │
  │  │ --overlay                │ │
  │  │   {{output_of:           │ │
  │  │     http-request}}       │ │
  │  │ --theme                  │ │
  │  │   {{.item.theme}}        │ │
  │  │ --output-dir             │ │
  │  │   {{.PRODUCTS_DIR}}/     │ │
  │  │   {{.item.name}}/renders │ │
  │  │ timeout: 1800            │ │
  │  │ stream: true             │ │
  │  └────────────┬────────────┘ │
  │               │               │
  │  ┌────────────▼────────────┐ │
  │  │ shell-command            │ │  ✗ MISSING
  │  │ python3 etsy_csv.py      │ │
  │  │ --name {{.item.name}}    │ │
  │  │ --title {{.item.title}}  │ │
  │  │ --tags {{.item.tags}}    │ │
  │  │ --output-dir             │ │
  │  │   {{.PRODUCTS_DIR}}/     │ │
  │  │   {{.item.name}}         │ │
  │  └────────────┬────────────┘ │
  │               │               │
  │  ┌────────────▼────────────┐ │
  │  │ image-overlay            │ │  ✓ EXISTS (bnto-image)
  │  │ Composite overlay onto   │ │  (may need enhancement for
  │  │ each render              │ │   non-watermark compositing)
  │  └─────────────────────────┘ │
  │                              │
  └──────────────────────────────┘

  Result per product:
  Dark Souls Miniatures/
  ├── Paladin/
  │   ├── overlay.png         ← from Figma API
  │   ├── renders/
  │   │   ├── render_0.png    ← from Blender
  │   │   ├── render_45.png
  │   │   ├── ...
  │   │   └── render_315.png
  │   ├── listing/
  │   │   ├── listing_1.png   ← overlay composited on render
  │   │   ├── listing_2.png
  │   │   └── ...
  │   └── etsy.csv            ← listing metadata
  ├── Warlock/
  │   ├── ...
  └── ...
```

### Node Inventory for Unified Recipe

| #   | Node                  | Type                  | Status       | Notes                              |
| --- | --------------------- | --------------------- | ------------ | ---------------------------------- |
| 1   | Read products.csv     | `spreadsheet-read`    | ✗ Missing    | Read from path, emit rows as data  |
| 2   | forEach row           | `loop` (data-driven)  | ✗ Missing    | Iterate structured data, not files |
| 3   | Create product folder | `file-system` (mkdir) | ✗ Missing    | Need fs operations                 |
| 4   | Generate overlay      | `http-request`        | ✗ Missing    | Figma Variables API                |
| 5   | Render in Blender     | `shell-command`       | ✗ Missing    | ProcessContext infra exists        |
| 6   | Generate Etsy CSV     | `shell-command`       | ✗ Missing    | Same node type as #5               |
| 7   | Composite overlay     | `image-overlay`       | **✓ Exists** | May need position/sizing tweaks    |

**1 of 7 nodes exists today.** The missing 6 depend on 4 infrastructure pieces (variables, expressions, data forEach, inter-node data).

### Alternative: Decomposed Into Smaller Recipes

If building the unified recipe feels too monolithic, it decomposes cleanly into 3 recipes that run in sequence (like the old `bento_order.json`):

```
RECIPE A: Setup Product Folders
────────────────────────────────
  spreadsheet-read  →  forEach  →  file-system (mkdir)
       ✗                  ✗              ✗
  Reads CSV, creates a folder per product


RECIPE B: Generate Overlays + Renders
──────────────────────────────────────
  spreadsheet-read  →  forEach  →  http-request  →  shell-command
       ✗                  ✗            ✗                  ✗
  For each product: Figma API → Blender render


RECIPE C: Generate Listings
───────────────────────────
  spreadsheet-read  →  forEach  →  shell-command  →  image-overlay
       ✗                  ✗            ✗                  ✓
  For each product: generate etsy.csv, composite overlays
```

The decomposed version needs the same infrastructure — it just has smaller blast radius per recipe. Whether to ship unified or decomposed is a recipe authoring choice, not an engine constraint.

### Recipe D: The Full Monty (A + B + C Combined)

This is the showcase. One recipe, one CSV, one `bnto run`. STLs on disk become Etsy-ready product folders with renders, branded overlays, composite listing images, and metadata CSVs. This is the "look what bnto can do" demo.

```
RECIPE D: Etsy Product Pipeline
════════════════════════════════
Variables prompted before execution:
  PRODUCTS_DIR    = /path/to/Dark Souls Miniatures
  FIGMA_TOKEN     = (from global config)
  FIGMA_FILE_KEY  = abc123
  RENDER_THEME    = fae_glow        (select: 19 themes)
  ZOOM_MULTIPLIER = 1.0
  PRICE           = 9.99
  MATERIALS       = Resin,ABS Like Resin

  ┌──────────────────────────────────┐
  │ 1. spreadsheet-read               │
  │    path: {{.PRODUCTS_DIR}}/       │
  │          products.csv             │
  │    → rows[]                       │
  └───────────────┬──────────────────┘
                  │
  ┌───────────────▼──────────────────┐
  │ 2. loop (forEach)                 │
  │    items: rows                    │
  │    continueOnError: true          │
  │                                   │
  │  ┌──────────────────────────────┐ │
  │  │ 2a. file-system (mkdir)      │ │
  │  │     {{.PRODUCTS_DIR}}/       │ │
  │  │     {{.item.name}}/          │ │
  │  │     {{.PRODUCTS_DIR}}/       │ │
  │  │     {{.item.name}}/renders/  │ │
  │  │     {{.PRODUCTS_DIR}}/       │ │
  │  │     {{.item.name}}/listing/  │ │
  │  └─────────────┬────────────────┘ │
  │                │                   │
  │  ┌─────────────▼────────────────┐ │
  │  │ 2b. http-request              │ │
  │  │     POST figma.com/v1/...     │ │
  │  │     headers:                  │ │
  │  │       X-Figma-Token:          │ │
  │  │       {{.FIGMA_TOKEN}}        │ │
  │  │     body: {                   │ │
  │  │       modelName:              │ │
  │  │         {{.item.name}},       │ │
  │  │       modelTitle:             │ │
  │  │         {{.item.title}},      │ │
  │  │       collectionName:         │ │
  │  │         {{.item.collection}}  │ │
  │  │     }                         │ │
  │  │     saveTo:                   │ │
  │  │       {{.PRODUCTS_DIR}}/      │ │
  │  │       {{.item.name}}/         │ │
  │  │       overlay.png             │ │
  │  │     → data: { path: "..." }   │ │
  │  └─────────────┬────────────────┘ │
  │                │                   │
  │  ┌─────────────▼────────────────┐ │
  │  │ 2c. shell-command             │ │
  │  │     python3 render.py         │ │
  │  │     --model                   │ │
  │  │       {{.PRODUCTS_DIR}}/      │ │
  │  │       {{.item.stl_path}}      │ │
  │  │     --overlay                 │ │
  │  │       {{.PRODUCTS_DIR}}/      │ │
  │  │       {{.item.name}}/         │ │
  │  │       overlay.png             │ │
  │  │     --theme                   │ │
  │  │       {{.RENDER_THEME}}       │ │
  │  │     --zoom                    │ │
  │  │       {{.ZOOM_MULTIPLIER}}    │ │
  │  │     --output-dir              │ │
  │  │       {{.PRODUCTS_DIR}}/      │ │
  │  │       {{.item.name}}/renders  │ │
  │  │     timeout: 1800             │ │
  │  │     stream: true              │ │
  │  └─────────────┬────────────────┘ │
  │                │                   │
  │  ┌─────────────▼────────────────┐ │
  │  │ 2d. file-system (list)        │ │
  │  │     {{.PRODUCTS_DIR}}/        │ │
  │  │     {{.item.name}}/           │ │
  │  │     renders/*.png             │ │
  │  │     → data: render_files[]    │ │
  │  └─────────────┬────────────────┘ │
  │                │                   │
  │  ┌─────────────▼────────────────┐ │
  │  │ 2e. loop (forEach) — nested   │ │
  │  │     items: render_files       │ │
  │  │  ┌────────────────────────┐   │ │
  │  │  │ image-overlay           │  │ │  ✓ EXISTS
  │  │  │ base: {{.item}}         │  │ │
  │  │  │ overlay:                │  │ │
  │  │  │   {{.PRODUCTS_DIR}}/    │  │ │
  │  │  │   {{$.item.name}}/      │  │ │
  │  │  │   overlay.png           │  │ │
  │  │  │ output:                 │  │ │
  │  │  │   {{.PRODUCTS_DIR}}/    │  │ │
  │  │  │   {{$.item.name}}/      │  │ │
  │  │  │   listing/              │  │ │
  │  │  │   {{basename .item}}    │  │ │
  │  │  └────────────────────────┘   │ │
  │  └───────────────────────────────┘ │
  │                │                   │
  │  ┌─────────────▼────────────────┐ │
  │  │ 2f. shell-command             │ │
  │  │     python3 etsy_csv.py       │ │
  │  │     --name {{.item.name}}     │ │
  │  │     --title {{.item.title}}   │ │
  │  │     --desc {{.item.desc}}     │ │
  │  │     --tags {{.item.tags}}     │ │
  │  │     --images-dir              │ │
  │  │       {{.PRODUCTS_DIR}}/      │ │
  │  │       {{.item.name}}/listing  │ │
  │  │     --output                  │ │
  │  │       {{.PRODUCTS_DIR}}/      │ │
  │  │       {{.item.name}}/         │ │
  │  │       etsy.csv                │ │
  │  │     env:                      │ │
  │  │       PRICE={{.PRICE}}        │ │
  │  │       MATERIALS=              │ │
  │  │         {{.MATERIALS}}        │ │
  │  └──────────────────────────────┘ │
  │                                   │
  └───────────────────────────────────┘

  Output:
  ═══════
  Dark Souls Miniatures/
  ├── products.csv                    ← input (unchanged)
  ├── source/                         ← STL files (unchanged)
  ├── Paladin/                        ← created by recipe
  │   ├── overlay.png                 ← Figma API
  │   ├── renders/
  │   │   ├── render_000.png          ← Blender (8 angles)
  │   │   ├── render_045.png
  │   │   ├── render_090.png
  │   │   ├── render_135.png
  │   │   ├── render_180.png
  │   │   ├── render_225.png
  │   │   ├── render_270.png
  │   │   └── render_315.png
  │   ├── listing/
  │   │   ├── render_000.png          ← overlay composited on render
  │   │   ├── render_045.png
  │   │   └── ...                     (Etsy-ready product photos)
  │   └── etsy.csv                    ← listing metadata
  ├── Warlock/
  │   ├── overlay.png
  │   ├── renders/ ...
  │   ├── listing/ ...
  │   └── etsy.csv
  ├── Necromancer/
  │   └── ...
  └── ... (one folder per CSV row)
```

### What Recipe D Demonstrates

This is bnto's **"one recipe to rule them all"** moment. A single `.bnto.json` file that:

| Capability                         | Node                   | Demo Value                                          |
| ---------------------------------- | ---------------------- | --------------------------------------------------- |
| **Data as input** (not just files) | spreadsheet-read       | CSV drives the entire pipeline                      |
| **Data-driven iteration**          | forEach (data)         | Each CSV row becomes a product                      |
| **Filesystem orchestration**       | file-system            | Creates organized folder trees                      |
| **External API integration**       | http-request           | Figma generates branded overlays                    |
| **CLI tool orchestration**         | shell-command          | Blender renders 3D models                           |
| **Nested loops**                   | forEach inside forEach | Per-render compositing inside per-product loop      |
| **Built-in image processing**      | image-overlay          | Composites overlay onto renders                     |
| **Variable system**                | recipe variables       | Theme, zoom, price — prompted once, used everywhere |
| **Template expressions**           | `{{.item.name}}`       | Dynamic paths, args, API bodies                     |
| **Long-running resilience**        | timeout + stream       | 30-min Blender renders with live output             |
| **Error tolerance**                | continueOnError        | One failed product doesn't kill the batch           |

**Node count: 8 nodes (4 unique types) in 1 recipe.**
**Input: 1 CSV file. Output: complete Etsy product catalog.**

This is the recipe you put on the landing page. Not "compress 3 images" — "turn a spreadsheet and 3D models into a ready-to-upload Etsy store." That's bnto.

### The Recursive Insight: Recipe D = Recipe of Recipes

Recipe D isn't "a big recipe." It's **A, B, and C composed together.** A recipe whose nodes are themselves recipes. That's the recursive power of bnto — a recipe is a function. Recipe D calls three functions.

```
RECIPE D (composed):
═══════════════════

  ┌─────────────┐       ┌─────────────┐       ┌─────────────┐
  │  Recipe A    │  ──→  │  Recipe B    │  ──→  │  Recipe C    │
  │  Setup       │       │  Generate    │       │  Listings    │
  │  Folders     │       │  Overlays +  │       │  + Composite │
  │              │       │  Renders     │       │              │
  └─────────────┘       └─────────────┘       └─────────────┘
        ↑                      ↑                      ↑
   3 nodes each           4 nodes each           3 nodes each
   (read, loop,           (read, loop,           (read, loop,
    mkdir)                 http, shell)           shell, overlay)
```

The old Go engine approximated this with `bento_order.json` — a flat sequence file that ran 5 recipes in order. That was external orchestration. The real power is a **`recipe` node type** — a node whose implementation is another `.bnto.json` file:

```json
{
  "id": "etsy-pipeline",
  "type": "group",
  "nodes": [
    { "id": "setup", "type": "recipe", "parameters": { "recipe": "setup-folders.bnto.json" } },
    {
      "id": "generate",
      "type": "recipe",
      "parameters": { "recipe": "generate-renders.bnto.json" }
    },
    {
      "id": "listings",
      "type": "recipe",
      "parameters": { "recipe": "generate-listings.bnto.json" }
    }
  ],
  "edges": [
    { "source": "setup", "target": "generate" },
    { "source": "generate", "target": "listings" }
  ]
}
```

This means:

- **Each sub-recipe is independently useful.** `bnto run setup-folders.bnto.json` works alone
- **Composition is explicit.** Recipe D's `.bnto.json` is 15 lines — it just says "run these three, in order"
- **Variables cascade.** Recipe D prompts for `PRODUCTS_DIR` and `RENDER_THEME` once; sub-recipes inherit them
- **Errors are scoped.** If Recipe B fails on product #5, Recipe A's folders still exist, Recipe C can run on the ones that succeeded
- **You can swap parts.** Replace Recipe B's Blender rendering with a different renderer? Just point at a different `.bnto.json`

This is the bnto thesis in one example: **small composable parts that chain into powerful workflows.** The bento box principle applied to automation itself.

### Tier 3 Addition: `recipe` Node Type

This adds one more item to the gap analysis:

| #   | Gap                    | Priority | Why                                                                                                                                                                                    |
| --- | ---------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 16  | **`recipe` node type** | Tier 3   | A node that executes another `.bnto.json` file. Variables from parent recipe flow into child. Child's output flows back. The engine already has the executor — this is just recursion. |

The `recipe` node type isn't strictly necessary (you can inline everything into one big recipe, or use `bnto run --chain`). But it's the cleanest expression of composability and it makes Recipe D trivially simple to author.

### Recipe D as the Acceptance Test

When Recipe D runs end-to-end on the Rust engine, the power recipe infrastructure is proven. It exercises every Tier 0 infrastructure piece and every Tier 1 node type. If this works, everything simpler works too.

The test matrix:

| Test                     | What it proves                                                    |
| ------------------------ | ----------------------------------------------------------------- |
| CSV with 1 row           | Happy path — single product, full pipeline                        |
| CSV with 20 rows         | Batch — folder creation, iteration, no cross-contamination        |
| CSV with missing column  | Error handling — expression fails on `{{.item.missing}}`          |
| Figma API timeout        | http-request timeout + continueOnError                            |
| Blender crash on 1 model | shell-command failure + continueOnError — other products complete |
| Nested loop (8 renders)  | Inner forEach over file-system list output                        |
| Resume after interrupt   | Idempotent — mkdir skips existing, Blender skips existing renders |

---

## The Target Workflow: Etsy Product Image Pipeline

A collection release (e.g., "Dark Souls Miniatures") ships multiple products. Each product needs renders, overlays, and listing images. Today this is a manual process spanning Blender, Figma, and spreadsheet tools. The recipe automates it.

### Inputs

- **Source folder** — contains STL files (3D models) for each product
- **products.csv** — one row per product with columns: name, stl_path, description, tags, theme, zoom, etc.

### Pipeline (per CSV row)

```
1. Read CSV           → structured rows (name, stl_path, description, ...)
2. For each row:
   a. Create folder   → mkdir "Dark Souls Miniatures/{row.name}/"
   b. Figma API call  → POST row data as variables → receive overlay PNG
   c. Blender render  → python3 render_script.py --model {row.stl_path}
                         --overlay {overlay.png} --theme {row.theme}
                         → 8 PNG renders land in the product folder
   d. (Future) Composite overlay onto renders
3. Output             → organized folder tree ready for Etsy upload
```

### What makes this hard

- **Data-driven iteration** — the loop is over CSV rows (structured data), not files
- **Variable passing** — downstream nodes need column values from the current row
- **Mixed node types** — filesystem ops, HTTP API calls, CLI execution, image processing
- **Stateful context** — the overlay PNG from step (b) feeds into step (c)
- **Long-running commands** — Blender renders take 5-30 minutes per product

---

## The Old Go Engine: What It Had

The Heavy Handed `.bento/` folder contains 11 production recipes that ran on the Go engine. Key capabilities used:

### Recipe-Level Variables

```json
{
  "variables": [
    { "name": "TARGET_PATH", "description": "...", "defaultValue": "" },
    { "name": "RENDER_THEME", "type": "select", "options": ["default|Default", "fae_glow|Fae Glow", ...] },
    { "name": "ZOOM_MULTIPLIER", "defaultValue": "1.0" }
  ]
}
```

Variables had types (`string`, `path`, `select`), default values, descriptions, and select options. The TUI prompted for each variable before execution. Global variables lived in `variables.json` and could be referenced as `{{GDRIVE}}`.

### Template Expression Engine

Go template syntax with functions:

| Pattern           | Example                         | Purpose                                              |
| ----------------- | ------------------------------- | ---------------------------------------------------- |
| Recipe variable   | `{{.TARGET_PATH}}`              | Access recipe-level variable                         |
| Loop item         | `{{.item}}`                     | Current iteration value (string or object)           |
| Loop item field   | `{{.item.name}}`                | Field access on structured data                      |
| Loop index        | `{{.index}}`                    | Current iteration index (0-based)                    |
| Upstream output   | `{{index . "node-id" "field"}}` | Access another node's output port                    |
| Global variable   | `{{GDRIVE}}`                    | From variables.json                                  |
| Parent scope      | `{{$.OVERLAY}}`                 | Access recipe variable from inside loop              |
| Built-in function | `{{basename .item}}`            | String manipulation (basename, basenameNoExt, split) |

### Node Types

| Type                      | Operations                                        | Status in Rust Engine                                                                            |
| ------------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `file-system`             | exists, list, mkdir, copy, move                   | **Missing entirely**                                                                             |
| `shell-command`           | Run CLI with args, env, timeout, streaming, retry | **Missing entirely**                                                                             |
| `spreadsheet` (read mode) | Read CSV from path, emit rows as structured data  | **Missing** — current spreadsheet nodes transform uploaded CSV bytes, they don't read from paths |
| `image` (composite mode)  | Layer overlay on base image with position control | **Partial** — `image-overlay` exists but designed for watermarking, not arbitrary compositing    |
| `loop` (forEach mode)     | Iterate over structured data (arrays, CSV rows)   | **Missing** — current loop iterates over files only                                              |

### Data Flow Between Nodes

Nodes declared output ports. Edges connected output ports to input ports. Inside a forEach loop, `{{.item}}` resolved to the current element. For CSV rows, `{{.item.name}}` accessed columns by header name.

```json
{
  "edges": [
    {
      "source": "read-products-csv",
      "target": "setup-product-folders",
      "sourcePort": "out-1",
      "targetPort": "in-1"
    }
  ]
}
```

### Recipe Ordering

`bento_order.json` defined a sequence of recipes to run in order:

```json
{
  "order": [
    "flatten-source-folder",
    "copy-product-overlays",
    "merge-product-stls",
    "generate-product-renders",
    "generate-etsy-listings"
  ]
}
```

### Other Features Used

- **`continueOnError`** on loops — skip failed items, keep going
- **`timeout`** per node (up to 4 hours for Blender renders)
- **`stream: true`** — relay stdout/stderr in real time
- **`retry`/`retryDelay`/`stallTimeout`** — resilience for flaky downloads
- **`env`** on shell-command — inject environment variables
- **`concurrency`** — parallel processing within a loop

---

## Current Rust Engine: What It Has

### Processors (13 total)

| Domain      | Processors                                     | Data Model                        |
| ----------- | ---------------------------------------------- | --------------------------------- |
| Image       | compress, resize, convert, strip-exif, overlay | File bytes in, file bytes out     |
| Spreadsheet | clean, rename, convert, merge                  | CSV bytes in, CSV bytes out       |
| File        | rename                                         | Filename transform only           |
| Vector      | rasterize, optimize                            | SVG bytes in, image/SVG bytes out |
| Video       | download (CLI-only)                            | URL in, file out                  |

### Structural Features

- **Smart iteration** — auto-wraps per-file processor sequences (files only, not data)
- **Loop container** — exists in executor but only iterates over files
- **ProcessContext** — `run_command()`, `temp_file()`, `env_var()`, `work_dir()` trait
- **NativeContext** — full system access in CLI mode
- **Dependency checker** — `bnto doctor` verifies external tools
- **Progress events** — structured PipelineStarted/NodeStarted/FileProgress/NodeCompleted stream

### What Works for the Target Workflow

- `ProcessContext::run_command()` — the infrastructure for running Blender/Python exists
- `NativeContext` — CLI has full system access
- Dependency system — can declare Blender, Python as requirements
- Progress events — can relay Blender's stdout as progress
- `image-overlay` — close to compositing (needs position/sizing enhancements)

---

## Gap Analysis

### Tier 0: Foundational Infrastructure (Unlocks Everything Else)

These are prerequisites. Without them, none of the node types below can work in recipes.

#### 1. Recipe-Level Variables

**What:** A `variables` array on the recipe definition with typed declarations, defaults, and UI hints. The TUI/CLI prompts for values before execution. Variables are resolved in template expressions throughout the recipe.

**Why first:** Every node parameter in every old recipe references variables. Without this, shell-command args are hardcoded, loop items are hardcoded, paths are hardcoded. Variables are the parameterization layer that makes recipes reusable.

**Scope:**

- `variables` field on `PipelineDefinition` (Rust)
- Variable types: `string`, `path`, `select` (with options), `number`, `boolean`
- Default values, required flag, description
- TUI prompts for variable values before pipeline execution
- CLI accepts `--var NAME=VALUE` flags
- Variables available in template expressions as `{{.NAME}}`

**Old recipe example:**

```json
"variables": [
  { "name": "RENDER_THEME", "type": "select", "defaultValue": "default",
    "options": ["default|Default", "fae_glow|Fae Glow", ...] }
]
```

#### 2. Template Expression Engine

**What:** Resolve `{{.variable}}` patterns in node parameters before execution. Start simple (variable substitution), extend later (functions, conditionals).

**Why:** Node parameters reference variables (`{{.TARGET_PATH}}`), loop context (`{{.item.name}}`), and upstream outputs. Without expression resolution, nodes can't be dynamic.

**Scope (Phase 1 — variable substitution only):**

- Resolve `{{.NAME}}` → recipe variable value
- Resolve `{{.item}}` → current loop iteration value
- Resolve `{{.item.field}}` → field access on structured iteration data
- Resolve `{{.index}}` → loop iteration index
- Resolve `{{$.NAME}}` → parent scope variable (from inside loops)
- Walk all string values in `parameters` before passing to processor
- Error on unresolved variables (fail fast, not silent empty string)

**Scope (Phase 2 — functions, deferred):**

- `{{basename .item}}`, `{{basenameNoExt .item}}`, `{{split .VAR ","}}`
- `{{index . "node-id" "field"}}` for upstream node output references
- Only add functions when a recipe actually needs them

#### 3. Data-Driven forEach

**What:** Loop container that iterates over structured data (arrays, objects), not just files. Each iteration exposes `{{.item}}` (current element) and `{{.index}}` (iteration index) to child nodes.

**Why:** The Etsy workflow loops over CSV rows. The image optimizer loops over file lists. The batch downloader loops over URL lists. Per-file iteration (current) only works when the data source is uploaded files.

**Scope:**

- `loop` container with `mode: "forEach"` and `items` expression
- `items` resolves to an array (from upstream node output or variable)
- Each iteration sets `{{.item}}` and `{{.index}}` in the expression context
- Child nodes execute sequentially per iteration (parallel deferred)
- `continueOnError: true` option to skip failed items

**Depends on:** Template Expression Engine (#2)

#### 4. Inter-Node Data Passing

**What:** Nodes can emit structured data (not just file bytes) via output ports. Downstream nodes reference upstream outputs in expressions.

**Why:** The CSV reader emits rows. The Figma API emits a PNG path. The folder creator emits a path. Downstream nodes need to reference these outputs.

**Scope:**

- `NodeOutput` gains an optional `data: serde_json::Value` field alongside `files`
- Nodes declare output ports in metadata (name, data type hint)
- Expression engine supports `{{index . "node-id" "port-name"}}` to access upstream data
- Data flows through edges (source node → edge → target node)
- Within a loop, the loop container sets `.item` from the upstream data

**This is the biggest architectural change.** Current `NodeOutput` is `Vec<ProcessedFile>` — pure file bytes. Adding structured data output means processors can emit metadata, not just files. This unlocks CSV-as-data-source, API response parsing, filesystem listing, and every non-file-centric node.

---

### Tier 1: Core Node Types (Unlocks the Target Workflow)

These are the node processors that the Etsy pipeline specifically needs. Build them after Tier 0 infrastructure is in place.

#### 5. `shell-command` Node

**What:** Execute an arbitrary CLI command with arguments, environment variables, timeout, and streaming output.

**Platforms:** `["cli", "desktop"]` — impossible in browser sandbox

**Why:** The Etsy workflow runs `python3 render_script.py` with dynamic arguments from CSV data. The old Go engine's most-used node type across all Heavy Handed recipes.

**Scope:**

- Parameters: `command`, `args[]`, `env{}`, `timeout`, `stream`, `workDir`
- Template expressions resolved in `command`, `args`, and `env` values
- Uses `ProcessContext::run_command()` (already exists)
- stdout/stderr relayed as progress events when `stream: true`
- Non-zero exit code = node failure (with stderr in error message)
- Dependency: declares required binaries (e.g., `python3`, `blender`)

**Processor metadata:**

```rust
NodeMetadata {
    node_type: "shell-command",
    category: System,
    platforms: vec![Platform::Cli, Platform::Desktop],
    input_cardinality: InputCardinality::Source, // generates output, doesn't transform files
    requires: vec![], // dynamic — depends on user's command
}
```

**Old recipe usage (7 of 11 recipes):**

- Blender rendering, STL merging, folder flattening, Etsy CSV generation, video downloading

#### 6. `file-system` Node

**What:** Filesystem operations: exists, list (with glob), mkdir, copy, move, delete.

**Platforms:** `["cli", "desktop"]` — no filesystem access in browser

**Why:** The Etsy workflow creates folders per product, checks if paths exist, lists files. 6 of 11 old recipes used filesystem operations.

**Scope:**

- Operations exposed as separate processors or a single processor with an `operation` parameter
- `fs-mkdir` — create directory (recursive option)
- `fs-list` — list files matching glob pattern, emit as structured data (array of paths)
- `fs-exists` — check if path exists, emit boolean
- `fs-copy` — copy file or directory
- `fs-move` — move file or directory
- Uses `std::fs` operations via ProcessContext (or directly in CLI)

**Design decision: single processor with `operation` param vs. separate processors.**
The old Go engine used a single `file-system` type with an `operation` parameter. This is more ergonomic for recipe authors (one node type to learn) but breaks the "one thing per processor" principle. Recommend: **single processor** since the operations share the same platform constraints and parameter patterns.

#### 7. `spreadsheet-read` (or enhanced spreadsheet node)

**What:** Read a CSV file from a filesystem path and emit rows as structured data (array of objects, keyed by column headers).

**Why:** The Etsy workflow's data source is `products.csv`. The loop iterates over its rows. Current spreadsheet processors transform uploaded CSV bytes — they don't read from paths or emit structured data.

**Scope:**

- Read from filesystem path (not from uploaded file bytes)
- Parse CSV with headers → `Vec<HashMap<String, String>>`
- Emit rows as structured data via output port
- Parameters: `path`, `hasHeaders`, `delimiter`
- Template expressions resolved in `path`

**Key distinction from current CSV nodes:** Current nodes receive CSV bytes as input and output transformed CSV bytes. This node reads from a path and outputs structured data for downstream consumption. It's a data source, not a transform.

**Platforms:** `["cli", "desktop"]` for path-based reading. Browser could support this if the CSV is provided as uploaded bytes (hybrid mode).

#### 8. `http-request` Node

**What:** Make HTTP requests (GET, POST, PUT, DELETE) with headers, body, and authentication. Parse response as JSON, save response body as file, or both.

**Platforms:** `["cli", "desktop", "server"]` — browser limited by CORS

**Why:** The Etsy workflow calls the Figma API to generate overlay PNGs from template variables. Any API integration recipe needs this node.

**Scope:**

- Parameters: `method`, `url`, `headers{}`, `body`, `bodyType` (json/form/raw), `auth` (bearer/basic/api-key), `timeout`, `responseType` (json/file/text)
- Template expressions resolved in `url`, `headers`, `body`
- JSON response emitted as structured data via output port
- File response (binary) emitted as ProcessedFile
- Error on non-2xx status (configurable: `failOnError: true/false`)

**Figma API usage example:**

```json
{
  "type": "http-request",
  "parameters": {
    "method": "POST",
    "url": "https://api.figma.com/v1/images/{{.FIGMA_FILE_KEY}}/...",
    "headers": { "X-Figma-Token": "{{.FIGMA_TOKEN}}" },
    "body": { "variables": { "modelName": "{{.item.name}}" } },
    "responseType": "file",
    "timeout": 30
  }
}
```

---

### Tier 2: Quality of Life (Makes Power Recipes Practical)

These aren't strictly required for a minimal viable version of the target workflow but make it practical for real use.

#### 9. Global Variables / User Config

**What:** User-level variables stored in `~/.config/bnto/variables.toml` (or `~/.local/share/bnto/config.toml`). Available in all recipes as `{{GDRIVE}}`, `{{FIGMA_TOKEN}}`, etc.

**Why:** Secrets (API tokens), machine-specific paths (Google Drive location), and user preferences shouldn't be hardcoded in recipes. The old system had `variables.json` for this.

**Scope:**

- TOML file in XDG config directory
- Loaded before recipe execution, merged into expression context
- Recipe variables override globals of the same name
- TUI settings screen for managing global variables
- `bnto config set KEY VALUE` / `bnto config get KEY` CLI commands

#### 10. Streaming Output for Long-Running Commands

**What:** Relay stdout/stderr from shell-command nodes as progress events in real time.

**Why:** Blender renders take 5-30 minutes. Without streaming output, the user stares at a blank progress bar. The old engine had `stream: true` on shell-command nodes.

**Scope:**

- `ProcessContext::run_command_streaming()` — returns a reader, not a buffer
- Progress events carry stdout lines as they arrive
- TUI displays streaming output in a log pane
- Timeout measured from last output, not from start (stall detection)

#### 11. Error Resilience

**What:** Per-node retry, continue-on-error in loops, stall timeout.

**Why:** Network requests fail. Blender crashes on specific models. The old engine had `retry`, `retryDelay`, `stallTimeout`, and `continueOnError`.

**Scope:**

- `retry: N` — retry N times on failure
- `retryDelay: seconds` — wait between retries
- `stallTimeout: seconds` — kill if no output for N seconds
- `continueOnError: true` on loop — log error, skip item, continue

#### 12. Recipe Chaining

**What:** Run multiple recipes in sequence via a manifest file or `bnto run --chain recipe1 recipe2 recipe3`.

**Why:** The Etsy workflow is actually 5 recipes run in order (`bento_order.json`). Sometimes the full pipeline is too big for one recipe and should be decomposed.

**Scope:**

- `bnto run --chain` flag accepting multiple recipe paths
- Output of recipe N available as input to recipe N+1 (optional)
- Per-recipe variable overrides
- Alternative: a `chain` recipe type that references other recipes

---

### Tier 3: Enhancement (Makes Power Recipes Elegant)

#### 13. Expression Functions

**What:** Built-in functions in template expressions: `basename`, `basenameNoExt`, `split`, `join`, `lower`, `upper`, `trim`, `replace`.

**Why:** The old image-optimizer recipe used `{{basenameNoExt .item}}` and `{{split $.SIZE_PRESET "x"}}` for dynamic output filenames. Without functions, users resort to shell-command nodes for trivial string operations.

#### 14. Concurrency Control

**What:** `concurrency: N` on loop containers to process N items in parallel.

**Why:** Rendering 20 products sequentially takes hours. The old engine supported `concurrency: 4` to run 4 Blender instances simultaneously.

#### 15. Conditional Nodes

**What:** `if` container that evaluates an expression and executes child nodes only when true.

**Why:** `copy-product-overlays` needed "if NEW_NAME is provided, rename the folder." The old engine did this with inline bash conditionals as a workaround.

---

## Priority & Dependency Chain

```
Tier 0 (Foundation)
  ├── 1. Recipe Variables
  ├── 2. Template Expressions  ← depends on 1
  ├── 3. Data-Driven forEach   ← depends on 2
  └── 4. Inter-Node Data       ← depends on 2, 3

Tier 1 (Core Nodes)           ← all depend on Tier 0
  ├── 5. shell-command
  ├── 6. file-system
  ├── 7. spreadsheet-read      ← depends on 4 (emits data)
  └── 8. http-request          ← depends on 4 (emits data)

Tier 2 (Quality of Life)      ← independent of each other
  ├── 9.  Global variables
  ├── 10. Streaming output
  ├── 11. Error resilience
  └── 12. Recipe chaining

Tier 3 (Enhancement)
  ├── 13. Expression functions
  ├── 14. Concurrency control
  ├── 15. Conditional nodes
  └── 16. Recipe-as-node          ← recipes compose into recipes
```

### Minimum Viable "Etsy Pipeline"

To run the target workflow, the minimum set is:

1. Recipe variables (Tier 0.1)
2. Template expressions (Tier 0.2)
3. Data-driven forEach (Tier 0.3)
4. Inter-node data passing (Tier 0.4)
5. shell-command node (Tier 1.5)
6. file-system node with mkdir (Tier 1.6)
7. spreadsheet-read (Tier 1.7)
8. http-request node (Tier 1.8)

Items 1-4 are the infrastructure. Items 5-8 are the consumers. The infrastructure is the hard part — it changes how the engine works, not just what nodes it has.

---

## What This Does NOT Change

- **Predefined recipes** — All 18 continue working. They don't use variables or data-driven loops.
- **Browser execution** — Tier 1 nodes are CLI/desktop only. Browser recipes stay file-centric.
- **Smart iteration** — Auto mode still works for simple per-file recipes. Data-driven forEach is a separate code path.
- **Editor/web app** — Custom recipes authored in TUI or text editor first. Web editor support is future work.
- **Engine API surface** — `NodeProcessor::process()` signature doesn't change. Structured data output is additive.

---

## Architectural Considerations

### Expression Engine: Rust or Scripting Language?

The old Go engine used Go templates (`text/template`). Options for Rust:

| Approach               | Pros                                    | Cons                                            |
| ---------------------- | --------------------------------------- | ----------------------------------------------- |
| **Custom Rust**        | Fast, no dependencies, exact control    | Must build parser, evaluator, function registry |
| **Tera** (Jinja2-like) | Mature Rust crate, rich syntax, filters | May be too powerful (full Jinja2 is overkill)   |
| **Handlebars-rust**    | Mustache-compatible, simple             | Limited functions, no expressions               |
| **MiniJinja**          | Lightweight Jinja2, good ergonomics     | Another dependency                              |

**Recommendation:** Start with **simple custom resolver** (Phase 1 — just `{{.name}}` variable substitution and `.field` access). If expression needs grow, adopt **MiniJinja** for Phase 2. Avoid Go template syntax — it's unfamiliar outside Go. Jinja2/Mustache syntax is more universal.

### NodeOutput: Files + Data

Current `NodeOutput` is `Vec<ProcessedFile>`. Adding structured data:

```rust
pub struct NodeOutput {
    pub files: Vec<ProcessedFile>,
    pub data: Option<serde_json::Value>,  // NEW: structured output
}
```

- File-centric processors (image, vector) continue returning `files` only
- Data-centric processors (spreadsheet-read, http-request) return `data` only
- Some processors return both (http-request downloading a file + JSON metadata)
- The executor stores `data` in the pipeline context, accessible via expressions

### CLI-Only vs. Browser: Two Worlds

Power recipes are inherently CLI/desktop — they need filesystem access, shell commands, and unrestricted network. This is fine. The browser is for the 18 predefined recipes. The CLI/TUI is for power users building custom pipelines.

This aligns with the product strategy: **local execution is free, CLI is the primary consumer, browser is a polished showcase.** Power recipes strengthen the CLI story.

---

## Validation: Old Recipes as Test Cases

The 11 Heavy Handed recipes are the acceptance test suite. When the Rust engine can run these recipes (with syntax adaptations), the feature set is complete:

| Old Recipe                    | Nodes Required                                             | Tier |
| ----------------------------- | ---------------------------------------------------------- | ---- |
| flatten-source-folder         | file-system (exists), shell-command                        | 1    |
| copy-product-overlays         | spreadsheet-read, forEach, file-system (mkdir, copy)       | 1    |
| merge-product-stls            | file-system (exists), shell-command                        | 1    |
| generate-product-renders      | file-system (exists), shell-command (long-running)         | 1+2  |
| generate-etsy-listings        | spreadsheet-read, forEach, shell-command                   | 1    |
| composite-images-with-overlay | file-system (list, mkdir), forEach, image (composite)      | 1    |
| image-optimizer               | file-system (list, mkdir), forEach, image (resize+convert) | 1    |
| download-video                | shell-command                                              | 1    |
| download-video-authenticated  | shell-command                                              | 1    |
| batch-download-videos         | forEach (over split string), shell-command                 | 1    |
| batch-download-videos-auth    | forEach, shell-command (retry, stall timeout)              | 1+2  |

**All 11 recipes are achievable with Tier 0 + Tier 1.** Tier 2 adds resilience for production use (retry, streaming, stall detection).

---

## Open Questions

1. **Expression syntax** — Go templates (`{{.name}}`), Jinja2 (`{{ name }}`), or Mustache (`{{name}}`)? The old recipes used Go template syntax. Jinja2 is more familiar to Python/web users. Mustache is the simplest.

2. **Variable prompting UX** — TUI wizard (current direction) prompts variables before execution. Should variables also be settable via CLI flags (`--var THEME=fae_glow`) for scriptability? Almost certainly yes.

3. **Data typing** — Should the expression engine enforce types (string, number, boolean, array, object) or treat everything as stringly-typed like the old engine? Stringly-typed is simpler and matches shell conventions.

4. **File-system node granularity** — Single `file-system` processor with `operation` param, or separate processors (`fs-mkdir`, `fs-list`, `fs-copy`, etc.)? Single is more ergonomic for authors, separate is more "bento box." Both are valid.

5. **Figma API specifics** — The overlay generation via Figma needs investigation: which Figma API endpoints support variable injection and image export? This may need a specialized node or a generic http-request + JSON body construction.

6. **Recipe format migration** — The old `.bento.json` format is close to the current `.bnto.json` but not identical (variables, edge structure, node types). Should the engine support both formats, or require migration? Migration is simpler (one-time script).
