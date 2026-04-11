# Decision: Custom SVG Optimizer (April 2026)

## Context

We need an SVG optimization processor (`vector-optimize`) for the `/optimize-svg` recipe page. SVG optimization is a high-value SEO target — "optimize svg online free" has strong search demand.

**What happened with oxvg:** PR #375 shipped a working `vector-optimize` processor using oxvg (Rust SVGO port, v0.0.5). It worked correctly — all tests passed, SVGs were optimized. But oxvg added ~5MB to the WASM binary (3.4MB → 8.4MB raw, 0.9MB → 1.7MB brotli). The root cause is `lightningcss` — Parcel's full CSS compiler — which is a **non-optional hard dependency** in 4 oxvg sub-crates (`oxvg_ast`, `oxvg_optimiser`, `oxvg_collections`, `oxvg_serialize`). No feature flags can exclude it. `default-features = false` saves only 13KB. PR #375 was reverted via PR #376.

## Decision

Build a custom lightweight SVG optimizer using dependencies already in our WASM binary. Zero additional binary size cost.

## Key Finding: We Already Have Everything We Need

These libraries are already compiled into our 3.6MB WASM binary via the `resvg`/`usvg` transitive dependency chain:

| Library     | Version | Role                                                  | Already in binary? |
| ----------- | ------- | ----------------------------------------------------- | ------------------ |
| `roxmltree` | 0.21    | Read-only XML tree parser — fast, zero-allocation     | Yes (via usvg)     |
| `xmlwriter` | 0.1     | XML output writer — produces well-formed XML          | Yes (via usvg)     |
| `svgtypes`  | 0.16    | SVG type parsing — paths, colors, transforms, lengths | Yes (via resvg)    |
| `simplecss` | 0.2     | Lightweight CSS selector parser                       | Yes (via usvg)     |

**Verified post-revert (April 2026):** `cargo tree -i <dep>` confirms all four libraries flow through `bnto-vector → resvg/usvg`, not from oxvg. The revert of PR #375 did not remove any of these. Zero new crate dependencies needed.

**Our optimizer code (2,000-4,000 lines of Rust) adds only our own logic to the binary. No new crate dependencies.**

## SVGO Is Open Source (MIT)

SVGO is fully open source (MIT license, github.com/svg/svgo, 22k+ stars). All 33 default optimization plugins are available for study. Each plugin is a self-contained module in `plugins/` with clear input/output contracts — straightforward to port algorithms to Rust.

## Reference Implementations

| Project        | Language   | License | Status            | Value                                                                                                                                                                                         |
| -------------- | ---------- | ------- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **SVGO**       | JavaScript | MIT     | Active, 22k stars | Definitive reference — study plugin algorithms, especially `convertPathData`, `mergePaths`, `cleanupNumericValues`                                                                            |
| **svgcleaner** | Rust       | MIT     | Archived          | Same author as resvg/roxmltree (RazrFalcon). 40+ optimization passes. Best Rust-specific reference. Uses its own `svgdom` parser (not roxmltree) — study algorithms, reimplement on our stack |
| **oxvg**       | Rust       | MIT     | v0.0.5, alpha     | Full SVGO port. Too heavy for WASM (lightningcss). Algorithm reference only                                                                                                                   |

## Two-Tier Implementation Plan

### Tier 1: XML-Level Optimizations (Easy Wins)

Work at the raw XML tree level. Read with `roxmltree`, write with `xmlwriter`, skip/transform nodes.

| Optimization                      | Impact | Complexity | How it works                                                                     |
| --------------------------------- | ------ | ---------- | -------------------------------------------------------------------------------- |
| **Remove metadata**               | 15-28% | Very low   | Skip `<metadata>` elements (Illustrator/Figma/Inkscape cruft)                    |
| **Remove comments**               | 1-5%   | Very low   | Skip XML comment nodes                                                           |
| **Remove doctype/PI**             | 1-2%   | Very low   | Skip `<!DOCTYPE>` and `<?xml?>` processing instructions                          |
| **Remove editor namespaces**      | 5-15%  | Low        | Skip elements/attributes in `sodipodi:`, `inkscape:`, `sketch:`, `i:` namespaces |
| **Remove empty containers**       | 2-5%   | Low        | Skip `<g>`, `<defs>`, `<pattern>` with no children                               |
| **Remove empty attributes**       | 1-3%   | Very low   | Skip attributes with empty string values                                         |
| **Remove unused NS declarations** | 1-2%   | Low        | Track referenced namespaces, omit `xmlns:` for unreferenced ones                 |
| **Collapse redundant groups**     | 3-8%   | Medium     | If `<g>` has one child and no meaningful attributes, hoist child                 |
| **Minify whitespace**             | 2-5%   | Low        | Strip indentation, collapse whitespace between elements                          |

**Expected combined reduction: 25-50%** (Illustrator/Figma exports benefit most — they have the most cruft).

### Tier 2: SVG-Aware Optimizations

Parse SVG-specific attribute values using `svgtypes` (already in our binary).

| Optimization                | Impact | Complexity | How it works                                                                                                                                                                                    |
| --------------------------- | ------ | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Round numeric values**    | ~12%   | Medium     | Parse numbers, round to configurable precision, remove trailing zeros. `svgtypes` handles parsing                                                                                               |
| **Shorten colors**          | 2-5%   | Medium     | `rgb(255,0,0)` → `red`, `#ff0000` → `#f00`. `svgtypes::Color` parses colors. Lookup table for named colors shorter than hex                                                                     |
| **Optimize path data**      | 15-30% | High       | Parse path `d` attribute (`svgtypes::PathParser`), choose shorter of absolute vs relative per segment, use shorthand commands (H/V/S/T), round coordinates to precision. **Biggest single win** |
| **Convert shapes to paths** | 1-3%   | Medium     | `<rect>`, `<line>`, `<polygon>` → `<path>` when the path representation is shorter                                                                                                              |
| **Simplify transforms**     | 3-8%   | Medium     | Parse transform matrices (`svgtypes`), simplify `translate(0,0)` → remove, merge consecutive transforms                                                                                         |

**Expected additional reduction: 15-35%** on top of Tier 1.

### Tier 3: CSS-Dependent (Deferred)

These require a CSS parser/compiler. This is exactly what caused oxvg's 5MB bloat — `lightningcss` handles these.

| Optimization               | Why defer                                     |
| -------------------------- | --------------------------------------------- |
| Minify `<style>` blocks    | Requires CSS minifier                         |
| Inline styles → attributes | Requires CSS selector matching against DOM    |
| Remove unused CSS rules    | Requires CSS parsing + DOM reference tracking |

**These can be added later** if `simplecss` (already in our deps) proves sufficient, or if a lightweight CSS minifier emerges. Not needed for a competitive product — Tier 1 + 2 matches ~80% of SVGO's default output.

## Architecture

```
engine/crates/bnto-vector/src/
├── optimize.rs          # OptimizeSvg processor (NodeProcessor trait impl)
├── optimize/
│   ├── mod.rs           # Orchestrator — runs passes in order
│   ├── xml_passes.rs    # Tier 1: remove metadata, comments, empty containers, etc.
│   ├── numeric.rs       # Tier 2: round numeric values
│   ├── colors.rs        # Tier 2: shorten color representations
│   ├── paths.rs         # Tier 2: optimize path d attribute
│   └── transforms.rs    # Tier 2: simplify transform matrices
```

Each pass is a pure function: `fn pass(tree: &roxmltree::Document) -> PassResult`. The orchestrator collects pass results and writes the final SVG with `xmlwriter`.

## Predecessor Code

PR #375 has everything except the optimization core:

- Processor struct with `NodeProcessor` trait impl (metadata, validate, process shell)
- 5 params: precision, removeComments, removeMetadata, collapseGroups, minify
- 15 unit tests covering trait basics, happy path, error handling, metadata
- Recipe definition (`optimize-svg.bnto.json`)
- Golden test + explicit equivalence test
- All registration (engine, WASM catalog, codegen, web overlay, nav)

**Strategy:** Bring back the PR #375 shell, rip out the `run_optimization()` oxvg call, replace with our own pass pipeline.

## WASM Binary Size Comparison

| Approach                       | Binary delta              | Transfer delta (brotli) |
| ------------------------------ | ------------------------- | ----------------------- |
| **oxvg** (what we tried)       | +5.0 MB (3.4→8.4)         | +0.8 MB (0.9→1.7)       |
| **Custom using existing deps** | ~0 KB                     | ~0 KB                   |
| **svgcleaner as dep**          | +200-400 KB (adds svgdom) | +50-100 KB              |

## Expected Competitive Position

| Tool                     | Approach                          | Reduction       |
| ------------------------ | --------------------------------- | --------------- |
| SVGO (JS, gold standard) | 33 plugins, full CSS awareness    | ~48% average    |
| **Our Tier 1 only**      | 9 XML-level passes                | 25-50%          |
| **Our Tier 1 + 2**       | 14 passes incl. path optimization | 40-70%          |
| svgcleaner (archived)    | 40+ passes                        | Similar to SVGO |

Tier 1 alone is a competitive product. Tier 2 (especially path optimization) makes it comparable to SVGO for most real-world SVGs. We can always iterate — ship Tier 1 fast, add Tier 2 passes incrementally.

## Open Questions

1. **Minify mode:** Should minified output omit all whitespace (single-line SVG) or preserve minimal formatting? SVGO defaults to single-line. Consider a `pretty` param for readable output.
2. **Pass ordering:** Some passes enable others (removing groups may create new empty containers). Run passes in a fixed order or iterate until stable? SVGO does fixed order. svgcleaner iterates. Start with fixed order for determinism.
3. **Path optimization depth:** `convertPathData` is the highest-impact single optimization but also the most complex. Ship Tier 1 first, add path optimization as a follow-up PR? Or bundle them?
