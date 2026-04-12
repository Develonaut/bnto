# File Node Ecosystem — BRU-Style Composable File Operations

**Last Updated:** April 12, 2026
**Status:** Phases 3-4 complete (SVG → Raster, SVG Optimize). Phase 5 (EPS) and file nodes remain backlog
**Inspiration:** Bulk Rename Utility, real-world design asset workflows

---

## Problem

Bnto's file category is thin — one processor (`file-rename`) and one recipe (`rename-files`). Meanwhile, the most common real-world pain points involve filesystem orchestration: finding files scattered across directories, reorganizing them, converting formats, and renaming in bulk. Tools like Bulk Rename Utility solve this with monolithic UIs. Bnto can solve it better with composable nodes.

---

## Vision

A toolkit of small, focused file operation nodes that compose into powerful recipes. Instead of one tool with 20 panels (BRU), users chain 3-4 nodes to build exactly the workflow they need. Each node does one thing; the recipe composition gives BRU-level power with bnto's readability and reusability.

**Target outcome:** The `file` category grows from 1 recipe to 6-8, with building blocks that unlock unlimited custom compositions via the editor.

---

## Current State

### What `file-rename` Does Today

6 parameters, applied in fixed order: `find/replace → case → prefix → suffix → pattern`

| Param     | Type   | Description                                                       |
| --------- | ------ | ----------------------------------------------------------------- |
| `find`    | string | Regex or literal search pattern                                   |
| `replace` | string | Replacement (supports capture groups)                             |
| `case`    | enum   | `lower`, `upper`, `title` (stem only)                             |
| `prefix`  | string | Prepend to stem                                                   |
| `suffix`  | string | Append before extension                                           |
| `pattern` | string | Template override: `{{name}}`, `{{ext}}`, `{{index}}`, `{{date}}` |

**Platform:** Browser + CLI. Accepts any file type. Per-file processing.

### Gaps vs. BRU

| Capability                         | BRU | Bnto Today                 | Proposed Fix                           |
| ---------------------------------- | --- | -------------------------- | -------------------------------------- |
| Sequential numbering (001, 002...) | Yes | Static `{{index}}` only    | Add `counter` params to `file-rename`  |
| Extension manipulation             | Yes | Locked to original         | Add `extension` param to `file-rename` |
| Character sanitization             | Yes | One find/replace at a time | New `file-sanitize` node               |
| File property-based naming         | Yes | Filename only              | New `file-metadata` node               |
| Directory traversal                | Yes | Explicit file selection    | New `file-collect` node                |
| File placement/output              | Yes | Download only              | New `file-copy` node                   |
| Conditional branching              | Yes | None                       | New `file-filter` node                 |
| Vector format conversion           | N/A | Image formats only         | Extend `image-convert`                 |

---

## Proposed Node Processors

### Phase 1: Enhance `file-rename` (Low Effort, High Leverage)

Enrich the existing processor — no new crates needed.

**New parameters:**

| Param           | Type    | Description                                              |
| --------------- | ------- | -------------------------------------------------------- |
| `counter_start` | integer | Starting number for sequential naming (default: 1)       |
| `counter_pad`   | integer | Zero-pad width (default: 0, e.g. 3 → `001`, `002`)       |
| `extension`     | string  | Replace extension (e.g. `"svg"` changes `.eps` → `.svg`) |

**New template variables:** `{{counter}}` (auto-incrementing, respects `counter_start` and `counter_pad`)

**Platform impact:** Browser + CLI (no new dependencies).

**Recipes unlocked:**

- Sequential batch rename: `photo-001.jpg`, `photo-002.jpg`
- Extension normalization: `.JPEG` → `.jpg`

### Phase 2: New File Operation Nodes

#### `file-collect` — Directory Traversal + Glob Matching

The "point at a folder" primitive. Accepts a directory path and glob pattern, outputs matched files into the pipeline.

| Param       | Type    | Description                                                 |
| ----------- | ------- | ----------------------------------------------------------- |
| `pattern`   | string  | Glob pattern: `*.svg`, `**/*.eps`, `*.{svg,eps}`            |
| `recursive` | boolean | Traverse subdirectories (default: true)                     |
| `flatten`   | boolean | Strip directory structure from output names (default: true) |

**Platform:** Native-only (CLI/desktop) — requires filesystem traversal. Browser variant could use `webkitdirectory` folder upload for drag-and-drop directory selection.

**Crate:** `bnto-file` (extend existing crate).

**Input cardinality:** Special — this is an _input source_ node, not a per-file processor. It produces files rather than consuming them. Architecturally similar to the `input` I/O node but with filesystem access.

**Engine consideration:** Today's pipeline model is "files in → process → files out." `file-collect` inverts the input side: "directory path in → files discovered → pipeline continues." This may need the input node to accept a directory path parameter rather than explicit files. Needs design spike.

#### `file-copy` — File Placement

Place output files in a specific destination directory.

| Param                | Type    | Description                                            |
| -------------------- | ------- | ------------------------------------------------------ |
| `destination`        | string  | Output directory path                                  |
| `create_dirs`        | boolean | Create destination if missing (default: true)          |
| `conflict`           | enum    | `skip`, `overwrite`, `rename` (default: `skip`)        |
| `preserve_structure` | boolean | Maintain relative directory structure (default: false) |

**Platform:** Native-only (CLI/desktop) — writes to filesystem.

**Crate:** `bnto-file` (extend existing crate).

#### `file-filter` — Conditional Pipeline Split

Filter files by extension, name pattern, or size. Files that don't match are dropped from the pipeline.

| Param          | Type    | Description                         |
| -------------- | ------- | ----------------------------------- |
| `extensions`   | string  | Comma-separated: `svg,eps,ai`       |
| `name_pattern` | string  | Glob or regex for filename matching |
| `min_size`     | integer | Minimum file size in bytes          |
| `max_size`     | integer | Maximum file size in bytes          |

**Platform:** Browser + CLI (operates on in-memory file metadata).

**Crate:** `bnto-file` (extend existing crate).

#### `file-sanitize` — Filename Cleanup

Strip special characters, normalize unicode, slugify filenames.

| Param        | Type    | Description                                                                              |
| ------------ | ------- | ---------------------------------------------------------------------------------------- |
| `mode`       | enum    | `slugify` (lowercase+hyphens), `strip` (remove special chars), `normalize` (unicode NFC) |
| `separator`  | string  | Replacement character for spaces/special chars (default: `-`)                            |
| `max_length` | integer | Truncate stem to N characters (default: 0 = no limit)                                    |

**Platform:** Browser + CLI (pure string manipulation).

**Crate:** `bnto-file` (extend existing crate).

#### `file-metadata` — Property Extraction

Extract file properties and inject them as template variables for downstream `file-rename` nodes.

| Param     | Type  | Description                                                                          |
| --------- | ----- | ------------------------------------------------------------------------------------ |
| `extract` | array | Properties to extract: `size`, `created`, `modified`, `width`, `height`, `exif_date` |

**Output:** Enriches the file's metadata map with extracted properties. Downstream `file-rename` pattern templates can reference them: `{{created_year}}`, `{{width}}x{{height}}`, `{{size_kb}}`.

**Platform:** Browser (limited — no created/modified dates) + CLI (full access).

**Crate:** `bnto-file` (extend existing crate). Image dimension extraction may delegate to `bnto-image`.

### Phase 3: SVG → Raster Conversion (extend `image-convert`) — DONE

**Shipped:** PRs #364, #369, #370, #372 (April 2026)

**Decision (April 9, 2026):** Vector operations live in a new `vector` node category — the counterpart to `image` (raster). Users think "I'm working with vector graphics" regardless of whether they're optimizing SVGs, converting EPS, or rasterizing logos. The category scales to future formats (AI, PDF→SVG) without renaming.

**Crate:** New `bnto-vector` crate. Houses all vector format operations.

The existing `image-convert` processor handles raster formats (JPEG, PNG, WebP). SVG→raster conversion extends it to accept SVG input, rasterize via `resvg`, then encode to the target raster format through the existing pipeline.

| Conversion          | Method                           | Platform      |
| ------------------- | -------------------------------- | ------------- |
| SVG → PNG/JPEG/WebP | `resvg` (pure Rust SVG renderer) | Browser + CLI |

**New parameter:** `dpi` (default: 96, range: 72–300) controls rasterization resolution.

**Why extend `image-convert`:** Users think of format conversion as one concept. "Convert SVG to PNG" and "Convert PNG to WebP" are the same mental operation. The processor detects SVG input internally and routes through `resvg` before the existing encode pipeline.

**Dependencies:** `resvg`, `usvg`, `tiny-skia` — all pure Rust, WASM-compatible. Added to `bnto-vector/Cargo.toml`, consumed by `bnto-image` for the rasterization step.

**Delivered:** `/svg-to-png` and `/svg-to-jpeg` recipe pages (browser + CLI).

### Phase 4: `svg-optimize` — SVG Cleanup/Minification — DONE

**Shipped:** PR #379 (April 2026). Custom lightweight optimizer using roxmltree/xmlwriter/svgtypes — zero new binary cost.

Dedicated SVG optimization processor in the `vector` category. Fundamentally different from `compress-images` (raster re-encoding) — this is lossless XML structural cleanup.

| Param             | Type    | Description                                                   |
| ----------------- | ------- | ------------------------------------------------------------- |
| `precision`       | integer | Decimal precision for coordinates (default: 3)                |
| `remove_comments` | boolean | Strip XML comments (default: true)                            |
| `remove_metadata` | boolean | Strip editor metadata (Illustrator, Inkscape) (default: true) |
| `collapse_groups` | boolean | Flatten unnecessary `<g>` wrappers (default: true)            |
| `minify`          | boolean | Remove whitespace/indentation (default: true)                 |

**Why NOT in `compress-images`:** Compress operates on pixel grids (lossy re-encoding with quality slider). SVG optimize operates on XML text (lossless structural cleanup). Different input types, different parameters, different libraries, different mental models. One thing per processor.

**Implementation: Custom lightweight optimizer using existing deps (zero binary cost).**

We tried oxvg (Rust SVGO port, v0.0.5) — it adds ~5MB to the WASM binary because `lightningcss` (Parcel's full CSS compiler) is a non-optional hard dependency in 4 oxvg sub-crates. No feature flags can exclude it. PR #375 merged, then reverted via PR #376.

The correct approach is a from-scratch optimizer using `roxmltree` + `xmlwriter` + `svgtypes` — all three are **already compiled into our WASM binary** via the resvg/usvg transitive dependency chain. Using them adds zero additional binary size.

**Reference implementations:**

- **SVGO** (JS, MIT) — 33 default plugins, 22k+ GitHub stars. The definitive SVG optimizer. Study plugin algorithms at `github.com/svg/svgo/tree/main/plugins/`
- **svgcleaner** (Rust, MIT, archived) — 40+ optimization passes, same author as resvg/roxmltree. Best Rust reference for algorithm porting. Uses its own `svgdom` parser (not roxmltree), so we study algorithms but reimplement on our own stack

**Two-tier implementation:**

| Tier                  | Optimizations                                                                                                                             | Dependencies                                  | WASM cost | Expected reduction |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- | --------- | ------------------ |
| **Tier 1: XML-level** | Remove metadata/comments/doctype, remove editor namespaces, remove empty containers/attrs, collapse groups, remove unused NS declarations | `roxmltree` + `xmlwriter` (already in binary) | ~0 KB     | 25-50%             |
| **Tier 2: SVG-aware** | Round numeric values, shorten colors (`#ff0000` → `#f00`), optimize path `d` data (relative/absolute, shorthand commands, precision)      | `svgtypes` (already in binary)                | ~0 KB     | additional 15-35%  |

Tier 3 (CSS-dependent: minify `<style>`, inline styles) is deferred — requires a CSS parser, which is what caused the oxvg bloat.

**Platform:** Browser (pure Rust) + CLI (same Rust code).

**Crate:** `bnto-vector` (same crate as Phase 3). See `.claude/decisions/svg-optimizer.md` for full research findings.

**Predecessor code:** PR #375 has the processor shell (metadata, params, tests, recipe, golden tests, registration) — everything except the optimization core. Bring it back and replace the `run_optimization()` function's oxvg call with our own passes.

### Phase 5: EPS → SVG Conversion (CLI-only)

Vector-to-vector conversion requiring external system binaries. First processor to shell out to native tools for format conversion.

| Conversion | Method                               | Platform          |
| ---------- | ------------------------------------ | ----------------- |
| EPS → SVG  | Shell out to Inkscape or Ghostscript | CLI-only (native) |
| AI → SVG   | Shell out to Inkscape                | CLI-only (native) |
| PDF → SVG  | Shell out to Inkscape or `pdf2svg`   | CLI-only (native) |

**Native dependency management:** Like `video-download` checks for `yt-dlp`, vector conversion checks for Inkscape/Ghostscript availability via `bnto doctor` and provides clear error messages when tools are missing. Browser shows "CLI only" for these conversions.

**Crate:** `bnto-vector` (same crate as Phases 3-4).

---

## Proposed Recipes

### Out-of-the-Box (ship with the nodes)

| Recipe                    | Slug                     | Nodes                                                             | Platform      | Category |
| ------------------------- | ------------------------ | ----------------------------------------------------------------- | ------------- | -------- |
| SVG to PNG                | `svg-to-png`             | `image-convert` (svg→png)                                         | Browser + CLI | vector   |
| SVG to JPEG               | `svg-to-jpeg`            | `image-convert` (svg→jpeg)                                        | Browser + CLI | vector   |
| Optimize SVG              | `optimize-svg`           | `svg-optimize`                                                    | Browser + CLI | vector   |
| Convert EPS to SVG        | `convert-eps-to-svg`     | `vector-convert` (eps→svg)                                        | CLI           | vector   |
| Batch Rename with Numbers | `number-files`           | `file-rename` (counter)                                           | Browser + CLI | file     |
| Flatten Folder            | `flatten-folder`         | `file-collect` → `file-copy`                                      | CLI           | file     |
| Collect and Rename        | `collect-and-rename`     | `file-collect` → `file-rename`                                    | CLI           | file     |
| Design Asset Pipeline     | `optimize-design-assets` | `file-collect` → `image-convert` → `svg-optimize` → `file-rename` | CLI           | file     |
| Sanitize Filenames        | `sanitize-filenames`     | `file-sanitize`                                                   | Browser + CLI | file     |

### Custom Compositions (enabled by building blocks)

These aren't shipped recipes but examples of what users can build in the editor:

- **Organize photos by date:** `file-collect` → `file-metadata` → `file-rename(pattern={{year}}/{{month}}/{{name}}.{{ext}})` → `file-copy`
- **Clean up downloads:** `file-collect(~/Downloads)` → `file-filter(extensions=png,jpg,svg)` → `file-sanitize(slugify)` → `file-copy(./sorted/)`
- **Prepare icons for web:** `file-collect(**/*.svg)` → `svg-optimize` → `file-rename(prefix=icon-, case=lower)` → `file-copy(./public/icons/)`
- **Extract and convert vectors:** `file-collect(**/*.eps)` → `image-convert(→svg)` → `svg-optimize` → `file-copy(./vectors/)`

---

## Implementation Order

**Priority is driven by:** leverage (how many recipes does it unlock), effort (how much new code), and SEO value (does it create a new recipe page).

| Priority | Work                                               | Effort                                               | Unlocks                                           | Status                  |
| -------- | -------------------------------------------------- | ---------------------------------------------------- | ------------------------------------------------- | ----------------------- |
| ~~1~~    | SVG → raster (extend `image-convert`)              | Medium — `resvg` in `bnto-vector`, browser+CLI       | `svg-to-png`, `svg-to-jpeg` recipes, 2 SEO pages  | **DONE** (PRs #364–372) |
| ~~2~~    | `svg-optimize` node + `optimize-svg` recipe        | Medium — custom optimizer in `bnto-vector`           | High-value standalone recipe, SEO page            | **DONE** (PR #379)      |
| 3        | EPS/AI → SVG (CLI-only shell-out)                  | Medium — native dependency (Inkscape), `bnto-vector` | `convert-eps-to-svg` recipe, CLI-only             | Backlog                 |
| 4        | Enhance `file-rename` (counter + extension)        | Small — extend existing processor                    | `number-files` recipe, better rename compositions | Backlog                 |
| 5        | `file-sanitize` node + `sanitize-filenames` recipe | Small — string manipulation in `bnto-file`           | Standalone recipe, useful building block          | Backlog                 |
| 6        | `file-filter` node                                 | Small — metadata checks in `bnto-file`               | Composition building block                        | Backlog                 |
| 7        | `file-collect` node                                | Medium — filesystem traversal, input model change    | Unlocks all CLI directory workflows               | Backlog                 |
| 8        | `file-copy` node                                   | Small — filesystem write in `bnto-file`              | Completes the collect→process→place pipeline      | Backlog                 |
| 9        | `file-metadata` node                               | Medium — property extraction, cross-crate            | Advanced rename compositions                      | Backlog                 |

---

## Architecture Notes

### `file-collect` Input Model

Today's execution model: user provides files → pipeline processes them. `file-collect` needs the inverse: pipeline discovers files from the filesystem. Options:

1. **Input node variant** — `file-collect` replaces the `input` I/O node at the start of a recipe. The engine's `PipelineExecutor` recognizes it as a source node and calls its `collect()` method instead of expecting user-provided files.
2. **Pre-pipeline step** — `file-collect` runs before the pipeline as a gather phase, feeding discovered files into the standard input. Simpler engine changes but less composable.
3. **Standard processor with directory input** — The CLI accepts a directory path as "input" and `file-collect` processes it like any other node, outputting individual files. Most composable, least engine change.

Option 3 is preferred — it keeps the execution model uniform and requires the least engine refactoring.

### Platform Matrix

| Node                     | Browser                   | CLI                        | Desktop (future) | Category |
| ------------------------ | ------------------------- | -------------------------- | ---------------- | -------- |
| `image-convert` (SVG in) | Yes (`resvg`)             | Yes (`resvg`)              | Yes              | vector   |
| `svg-optimize`           | Yes (Rust)                | Yes (Rust + optional SVGO) | Yes              | vector   |
| `vector-convert` (EPS+)  | No                        | Yes (Inkscape/Ghostscript) | Yes              | vector   |
| `file-rename` (enhanced) | Yes                       | Yes                        | Yes              | file     |
| `file-sanitize`          | Yes                       | Yes                        | Yes              | file     |
| `file-filter`            | Yes                       | Yes                        | Yes              | file     |
| `file-collect`           | Limited (webkitdirectory) | Yes                        | Yes              | file     |
| `file-copy`              | No (download only)        | Yes                        | Yes              | file     |
| `file-metadata`          | Limited                   | Yes                        | Yes              | file     |

### SEO Impact

Each new recipe with a dedicated page is an SEO surface. Estimated new pages:

- `/svg-to-png` — "convert svg to png free" (vector category, browser+CLI)
- `/svg-to-jpeg` — "convert svg to jpeg free" (vector category, browser+CLI)
- `/optimize-svg` — "optimize svg online free" (vector category, browser+CLI)
- `/convert-eps-to-svg` — "convert eps to svg free" (vector category, CLI-only)
- `/number-files` — "batch rename files with numbers"
- `/flatten-folder` — "flatten folder structure"
- `/sanitize-filenames` — "sanitize filenames batch"

All high-intent, tool-seeking queries with strong conversion potential.

---

## Decisions

1. **Vector category (April 9, 2026):** Vector operations use a `vector` node category — the counterpart to `image` (raster). Users think "I'm working with vector graphics" whether optimizing SVGs, converting EPS, or rasterizing logos. Scales to future formats without renaming.
2. **Crate naming (April 9, 2026):** `bnto-vector` — matches the category name. Consistent naming engine-to-UI.
3. **SVG optimize is NOT compress-images (April 9, 2026):** `compress-images` = lossy raster re-encoding (quality slider). `svg-optimize` = lossless XML structural cleanup. Different inputs, params, libraries, mental models. Separate processor.
4. **Priority reorder (April 9, 2026):** Vector work (Phases 3-5) prioritized ahead of file operation nodes. SVG→raster first (highest SEO demand), SVG optimize second (builds on same deps), EPS→SVG third (CLI-only, introduces shell-out pattern).
5. **Custom SVG optimizer shipped (April 2026, PR #379):** After reverting oxvg (5MB WASM bloat from `lightningcss`), built a from-scratch optimizer using `roxmltree` + `xmlwriter` + `svgtypes` — all already in the binary via resvg transitive chain. 9 XML-level passes, zero additional binary cost. Tier 2 SVG-aware optimizations (path `d` data, color shortening) deferred. See `.claude/decisions/svg-optimizer.md`.

## Open Questions

1. **`file-collect` execution model** — Which of the three options above? Needs a design spike with the engine architecture.
2. **`resvg` WASM size** — Need to benchmark the size impact of adding `resvg` to the WASM binary for SVG→raster in browser.
3. **SVGO integration** — Should the CLI prefer SVGO when available, or always use the Rust implementation for determinism? Golden tests need consistent output.
4. **`file-metadata` scope** — How much property extraction is useful? EXIF is well-defined but filesystem metadata varies by OS.
5. **`file-copy` in browser** — Browser can't write to filesystem. Should `file-copy` be hidden in browser recipes, or should it fall back to zip download with directory structure preserved?
