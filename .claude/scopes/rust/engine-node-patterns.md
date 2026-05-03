# Engine Node Processor Patterns

Correct patterns for creating and extending Rust node processors. Companion to [node-responsibilities.md](node-responsibilities.md).

---

## Checklist: Adding a New Node Processor

Adding a node processor is a foundational change that ripples through the entire stack. Every step below is required — not optional. The pipeline is sequential: each phase produces artifacts consumed by the next.

### Phase 1: Rust Implementation (TDD-first)

Write failing tests BEFORE implementing the processor.

- [ ] **Crate name = category** — the crate directory and `Cargo.toml` package name must match the node category (e.g., `bnto-image` for `image-*` processors, `bnto-spreadsheet` for `spreadsheet-*` processors)
- [ ] **File name = operation** — each processor file is named after the operation part of the registry key (e.g., `compress.rs` for `image-compress`, `clean.rs` for `spreadsheet-clean`)
- [ ] **Create processor file** — `engine/crates/bnto-{category}/src/{operation}.rs`
- [ ] **Implement `NodeProcessor` trait** — `name()`, `metadata()`, `process()`, `validate()`
- [ ] **`metadata()` complete** — all parameters have types, defaults, constraints, descriptions
- [ ] **`metadata().input_cardinality`** — declare `PerFile` (default) or `Batch`. See [smart-iteration.md](../../strategy/smart-iteration.md)
- [ ] **`process()` parameter contract** — every param in `metadata()` is read and used in ALL code paths
- [ ] **`validate()`** — validate param combinations that metadata constraints can't express
- [ ] **`FileData` selection** — processors that don't modify file content MUST emit `FileData::Path` (zero-copy), not `FileData::Bytes`. See [FileData Selection](#filedata-selection) below
- [ ] **Image processors use `encode::encode_image()`** — never custom encode functions
- [ ] **Shared params use `common.rs`** — e.g., `quality_param_def()`, `image_accepts()`
- [ ] **`name()` matches registry key** — `fn name()` returns the same string used in `registry.register()`. Convention: `category-operation` kebab-case (e.g., `"image-compress"`)
- [ ] **Re-export from crate `lib.rs`** — add `pub mod {name};` and `pub use {name}::ProcessorName;`
- [ ] **Unit tests pass** — `cargo test -p bnto-{crate}`

#### Required Unit Tests

| Test category        | What to verify                                                            |
| -------------------- | ------------------------------------------------------------------------- |
| **Trait basics**     | `name()` returns correct string, `validate()` passes with no params       |
| **Happy path**       | Valid input → valid output (correct format, non-empty, correct MIME type) |
| **Parameterized**    | Different param values produce measurably different outputs               |
| **Output filename**  | Correct suffix/prefix applied (e.g., `-compressed`, `-stripped`)          |
| **Output metadata**  | `NodeOutput.metadata` contains expected keys (sizes, format, etc.)        |
| **Error handling**   | Unsupported format, corrupt input, empty data, truncated files            |
| **Edge cases**       | 1x1 pixel images, single-byte files, boundary param values (min/max)      |
| **EXIF orientation** | (image processors) Orientation correction preserved through pipeline      |

### Phase 2: Engine Registration

- [ ] **Register in `bnto-engine`** — add `registry.register("type-name", Box::new(...))` in `create_browser_registry()` (`engine/crates/bnto-engine/src/lib.rs`). If the processor is native-only, add it in `create_registry()` behind `#[cfg(feature = "native")]` instead
- [ ] **Update registry test count** — `test_browser_registry_has_all_processors()` asserts exact count (e.g., 10 → 11)
- [ ] **Add to expected list** — the `expected` array in the same test must include the new type key
- [ ] **Add `NodeTypeInfo`** — add entry to the correct category function in `engine/crates/bnto-core/src/metadata.rs` (e.g., `image_node_types()`)
- [ ] **Update node type count** — `test_all_node_types_returns_N_entries()` asserts exact count (e.g., 15 → 16)
- [ ] **Update unique names count** — `test_all_node_types_unique_names()` also asserts the count
- [ ] **Update WASM catalog tests** — `test_catalog_has_all_N_processors()` and `test_catalog_serializes_to_valid_json()` in `engine/crates/bnto-wasm/src/catalog.rs` assert exact processor and node type counts
- [ ] **Update WASM expected types** — `test_catalog_contains_expected_node_types()` must include the new type key in its expected list
- [ ] **All engine tests pass** — `task wasm:test`

### Phase 3: Codegen Pipeline (Engine → TypeScript)

The engine is the single source of truth. TypeScript types are generated, not hand-written.

- [ ] **Generate catalog snapshot** — `task wasm:snapshot` writes `engine/catalog.snapshot.json`
- [ ] **Generate TypeScript** — `task nodes:generate` reads the snapshot and produces:
  - `packages/@bnto/nodes/src/generated/catalog.ts` — `NODE_TYPES`, `PROCESSORS`, `NODE_TYPE_INFO`, `PROCESSOR_MAP`
  - `packages/@bnto/nodes/src/generated/schemas.ts` — Zod schemas per processor
  - `packages/@bnto/nodes/src/generated/definitionSchema.ts` — JSON Schema
  - `packages/@bnto/nodes/docs/{node-type}.md` — auto-generated documentation
  - `packages/@bnto/backend/convex/_helpers/nodeTypeLabels.ts` — label map for Convex
  - `packages/@bnto/i18n/src/generated/nodes.json` — i18n strings
- [ ] **Or run the full pipeline** — `task wasm:codegen` (build → copy → snapshot → generate)

#### Verify: Node Appears in Generated Output

After codegen, verify the new node landed in every generated artifact:

- [ ] **`NODE_TYPES`** — new camelCase key exists in `packages/@bnto/nodes/src/generated/catalog.ts`
- [ ] **`NODE_TYPE_INFO`** — new entry with label, description, category, icon, platforms
- [ ] **`PROCESSORS`** — new processor entry with parameters and accepts
- [ ] **`PROCESSOR_MAP`** — `PROCESSOR_MAP.has("your-type-key")` is true
- [ ] **Zod schema** — new schema file in `packages/@bnto/nodes/src/schemas/` (auto-generated)
- [ ] **i18n** — node label and param labels in `packages/@bnto/i18n/src/generated/nodes.json`
- [ ] **Convex labels** — new entry in `packages/@bnto/backend/convex/_helpers/nodeTypeLabels.ts`
- [ ] **TypeScript compiles** — `task ui:build`

#### Update TypeScript Test Counts

The generated catalog changes will break exact-count assertions in TypeScript tests. Update these:

- [ ] **`packages/@bnto/nodes/src/nodeTypes.test.ts`** — `NODE_TYPES` count (e.g., 15 → 16), `NODE_TYPE_NAMES` count, and add the new type to the `NODE_TYPES` key-value mapping test
- [ ] **`packages/@bnto/nodes/src/catalogValidation.test.ts`** — `PROCESSORS` count (e.g., 6 → 7), add new type to the "all expected per-operation node types" test, add engine-default-to-Zod-schema verification for the new processor's params
- [ ] **`packages/@bnto/registry/src/nodeTypes.test.ts`** — `getAllNodeTypes()` count (e.g., 15 → 16)
- [ ] **`packages/@bnto/nodes/src/schemas/registry`** — add new schema to `NODE_SCHEMAS` and `NODE_PARAM_FIELDS` registries (if not auto-generated)
- [ ] **TypeScript tests pass** — `task ui:test`

### Phase 4: Recipe & Fixtures

Every processor needs at least one recipe that exercises it. See [Checklist: Adding a New Recipe](#checklist-adding-a-new-recipe) below for the full recipe pipeline.

- [ ] **Create recipe definition** — `engine/recipes/{slug}.bnto.json` (engine is the source of truth for recipes)
- [ ] **Add to `builtin_recipes()`** — add `include_str!()` in `engine/crates/bnto-engine/src/recipes.rs`
- [ ] **Register web overlay** — add to `RECIPE_IDS`, `WEB_DESCRIPTIONS`, `WEB_FEATURES`, `DISPLAY_ORDER` in `packages/@bnto/registry/src/recipesCatalog.ts`
- [ ] **Regenerate TypeScript** — `task wasm:codegen` (snapshot → generate TS from engine catalog)
- [ ] **Add engine integration test** — `test_generated_{slug}_recipe()` in `bnto-engine/src/lib.rs`
- [ ] **Add to recipe parse test** — `test_all_generated_recipes_parse()` must include the new recipe's `include_str!()`

### Phase 5: Golden Tests (byte-exact output verification)

Golden tests prove deterministic output and catch silent regressions.

- [ ] **Add golden test** — `engine/crates/bnto/tests/golden_tests.rs`:
  ```rust
  #[test]
  fn golden_{slug_underscored}() {
      let (out, _) = run_recipe_ok("{slug}", &fixture_image("small.jpg"));
      assert_golden("{slug}", &out);
  }
  ```
- [ ] **Add explicit (loop-container) equivalence test** — same file, proves auto and explicit iteration produce byte-identical output:
  ```rust
  #[test]
  fn golden_{slug_underscored}_explicit() {
      let (out, _) = run_explicit_recipe_ok("{slug}", &fixture_image("small.jpg"));
      assert_golden("{slug}", &out);  // same golden dir as auto version
  }
  ```
- [ ] **Create explicit fixture** — `engine/crates/bnto/tests/fixtures/explicit/{slug}.bnto.json` (recipe with explicit loop containers instead of auto iteration)
- [ ] **Bless golden files** — `BLESS=1 cargo test -p bnto -- golden` (or `task cli:golden:bless`)
- [ ] **Verify golden files** — `task cli:golden` (subsequent runs verify byte-exact match)
- [ ] **Review golden diff** — `git diff engine/crates/bnto/tests/golden/` before committing
- [ ] **Commit golden files** — these are the source of truth for output correctness

### Phase 6: Quality Gate

- [ ] **Clippy clean** — `task wasm:lint`
- [ ] **Rust formatted** — `task wasm:fmt`
- [ ] **All Rust tests** — `task wasm:test`
- [ ] **CLI tests + golden** — `task cli:test`
- [ ] **TypeScript builds** — `task ui:build`
- [ ] **TypeScript tests** — `task ui:test`
- [ ] **All generated files committed** — snapshot, TS catalog, Zod schemas, recipe fixtures, golden files, i18n strings, Convex labels

### Phase 7: SEO & Product (if the processor enables a new recipe page)

Only when the new processor creates a user-facing recipe at a new URL. See [Checklist: Adding a New Recipe](#checklist-adding-a-new-recipe) below — the recipe checklist covers the full end-to-end verification including SEO surfaces.

---

## Checklist: Adding a New Recipe

A recipe is a predefined pipeline composition that maps to a public URL. Adding a recipe touches TypeScript, codegen, engine tests, SEO, sitemap, and LLM discovery. Every surface is tested — but those tests have exact-count assertions or explicit lists that must include your new addition.

### Step 1: Define the Recipe (Engine-Owned)

Recipes are defined as `.bnto.json` files in `engine/recipes/`. The engine is the source of truth.

- [ ] **Create recipe definition** — `engine/recipes/{slug}.bnto.json`
  - Must include: `id` (slug), `type: "group"`, `version`, `name`, `metadata` (with `description` and `category`), `nodes[]`, `edges[]`, `settings` with iteration mode
  - The `definition` contains the full node graph: input node, processor nodes, output node, edges connecting them
- [ ] **Add to `builtin_recipes()`** — add `include_str!()` in `engine/crates/bnto-engine/src/recipes.rs`
- [ ] **Register web overlay** — add to `RECIPE_IDS`, `WEB_DESCRIPTIONS`, `WEB_FEATURES`, `DISPLAY_ORDER` in `packages/@bnto/registry/src/recipesCatalog.ts`

### Step 2: Codegen Pipeline

- [ ] **Regenerate TypeScript** — `task wasm:codegen` (build → snapshot → generate TS from engine catalog including recipes)
- [ ] **Verify generated recipe** — check `GENERATED_RECIPES` in `packages/@bnto/nodes/src/generated/recipes.ts` includes the new recipe
- [ ] **TypeScript builds** — `task ui:build`

### Step 3: Engine Integration Tests

- [ ] **Add integration test** — `test_generated_{slug}_recipe()` in `engine/crates/bnto-engine/src/lib.rs` that runs the fixture through `run_pipeline()`
- [ ] **Add to recipe parse test** — `test_all_generated_recipes_parse()` must include `include_str!("../../../recipes/{slug}.bnto.json")`
- [ ] **Engine tests pass** — `task wasm:test`

### Step 4: Golden Tests

- [ ] **Add golden test** — `golden_{slug_underscored}()` in `engine/crates/bnto/tests/golden_tests.rs`
- [ ] **Add explicit equivalence test** — `golden_{slug_underscored}_explicit()` in the same file
- [ ] **Create explicit fixture** — `engine/crates/bnto/tests/fixtures/explicit/{slug}.bnto.json`
- [ ] **Bless golden files** — `task cli:golden:bless`
- [ ] **Verify golden files** — `task cli:golden`
- [ ] **Commit golden files** — `engine/crates/bnto/tests/golden/{slug}/`

### Step 5: Verify Recipe Appears in All Surfaces

These surfaces auto-derive from the `RECIPES` array, but tests have assertions that must include your addition:

| Surface            | How it picks up the recipe                                                                  | Test that catches a miss                                                |
| ------------------ | ------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| **RECIPES array**  | You added it in Step 1                                                                      | `recipesCatalog.test.ts` — "every recipe export is included in RECIPES" |
| **Home page grid** | `ExploreRecipeGrid.tsx` reads `getAllRecipes()`                                             | Derived — shows automatically if in RECIPES                             |
| **Nav dropdown**   | `recipeLinks.ts` reads `getAllRecipes()`, groups by category                                | `recipeLinks.test.ts` — categories contain expected links               |
| **SEO pages**      | `apps/web/app/(app)/[bnto]/page.tsx` reads `BNTO_REGISTRY` (derived from `getAllRecipes()`) | `bntoRegistry.test.ts` — "BNTO_REGISTRY count matches getAllRecipes()"  |
| **Sitemap**        | `buildBntoSitemapEntries.ts` reads `BNTO_REGISTRY`                                          | Derived — entry auto-generated if in BNTO_REGISTRY                      |
| **llms.txt**       | `app/llms.txt/route.ts` reads `getAllRecipes()`                                             | Derived — auto-generated at build time                                  |
| **llms-full.txt**  | `app/llms-full.txt/route.ts` reads `getAllRecipes()`                                        | Derived — auto-generated at build time                                  |
| **README table**   | `task readme:generate` reads `RECIPES`                                                      | Manual — you must run the command                                       |
| **Static params**  | `generateStaticParams()` reads `BNTO_REGISTRY`                                              | Build will generate the page if in registry                             |
| **JSON-LD**        | `BntoJsonLd` renders from `BntoEntry`                                                       | Derived — auto-rendered per page                                        |

**Key insight:** Almost every surface derives from `RECIPES` in `recipesCatalog.ts`. If the recipe is in that array, it flows everywhere automatically. The tests below verify the chain isn't broken:

- [ ] **Recipe catalog tests pass** — `packages/@bnto/registry/src/recipesCatalog.test.ts` verifies every export is in RECIPES, unique slugs, valid definition structure
- [ ] **SEO registry tests pass** — `apps/web/lib/__tests__/bntoRegistry.test.ts` verifies BNTO_REGISTRY count matches getAllRecipes(), all entries have required fields, no slug collisions with reserved paths
- [ ] **Nav links tests pass** — `apps/web/components/blocks/nav/recipeLinks.test.ts` verifies categories and links

### Step 6: Nav Category (if new category)

If the recipe's `category` is new (not `image`, `spreadsheet`, or `file`):

- [ ] **Add category title** — `CATEGORY_TITLES` in `apps/web/components/blocks/nav/recipeLinks.ts`
- [ ] **Add to category order** — `CATEGORY_ORDER` in the same file
- [ ] **Update nav test** — `recipeLinks.test.ts` — "categories are ordered" assertion must include the new category

### Step 7: README Update

- [ ] **Regenerate README table** — `task readme:generate` updates the recipe table in `README.md`
- [ ] **Commit README** — the generated table between `<!-- BEGIN AUTO-GENERATED RECIPES TABLE -->` markers

### Step 8: Quality Gate

- [ ] **All Rust tests** — `task wasm:test`
- [ ] **CLI tests + golden** — `task cli:test`
- [ ] **TypeScript builds** — `task ui:build`
- [ ] **TypeScript tests** — `task ui:test`
- [ ] **All generated files committed** — recipe fixtures, golden files, README table

---

## Test Count Registry

When adding a processor or recipe, these exact-count assertions MUST be updated. Search for the current values before changing.

### Rust (engine)

| File                                       | Test function                                 | What it counts                              |
| ------------------------------------------ | --------------------------------------------- | ------------------------------------------- |
| `engine/crates/bnto-engine/src/lib.rs`     | `test_browser_registry_has_all_processors()`  | Browser processor count + expected key list |
| `engine/crates/bnto-engine/src/lib.rs`     | `test_all_generated_recipes_parse()`          | Recipe fixture `include_str!()` list        |
| `engine/crates/bnto-core/src/metadata.rs`  | `test_all_node_types_returns_N_entries()`     | Total node types (processors + planned)     |
| `engine/crates/bnto-core/src/metadata.rs`  | `test_all_node_types_unique_names()`          | Same count (uniqueness check)               |
| `engine/crates/bnto-wasm/src/catalog.rs`   | `test_catalog_has_all_N_processors()`         | Processor count in WASM catalog             |
| `engine/crates/bnto-wasm/src/catalog.rs`   | `test_catalog_serializes_to_valid_json()`     | Both processor and node type counts         |
| `engine/crates/bnto-wasm/src/catalog.rs`   | `test_catalog_contains_expected_node_types()` | Expected type key list                      |
| `engine/crates/bnto/tests/golden_tests.rs` | Individual golden test functions              | One per recipe (auto + explicit)            |

### TypeScript (packages)

| File                                                 | Test                                        | What it counts                                                    |
| ---------------------------------------------------- | ------------------------------------------- | ----------------------------------------------------------------- |
| `packages/@bnto/nodes/src/nodeTypes.test.ts`         | `NODE_TYPES` count, `NODE_TYPE_NAMES` count | Node type count (e.g., 15) + explicit key-value map               |
| `packages/@bnto/nodes/src/catalogValidation.test.ts` | `PROCESSORS` count + expected type list     | Processor count (e.g., 6) + `PROCESSOR_MAP.has()` checks          |
| `packages/@bnto/registry/src/nodeTypes.test.ts`      | `getAllNodeTypes()` count                   | Node type count (e.g., 15)                                        |
| `packages/@bnto/registry/src/recipesCatalog.test.ts` | Completeness checks                         | Every export in RECIPES, unique slugs, valid definitions          |
| `apps/web/lib/__tests__/bntoRegistry.test.ts`        | BNTO_REGISTRY parity                        | Count matches getAllRecipes(), metadata shape, no slug collisions |
| `apps/web/components/blocks/nav/recipeLinks.test.ts` | Category order, link presence               | Categories ordered, specific slugs present                        |

---

## Surface Propagation Map

This table shows how a node type flows from the engine through every consumer surface:

```
Engine (Rust)
  └─ metadata.rs: NodeTypeInfo
  └─ bnto-engine: registry.register()
  └─ bnto-wasm: catalog.rs → node_catalog()
        │
        ▼
  catalog.snapshot.json (task wasm:snapshot)
        │
        ▼
  generate-from-catalog.ts (task nodes:generate)
        │
        ├─► @bnto/nodes/generated/catalog.ts → NODE_TYPES, NODE_TYPE_INFO, PROCESSORS
        ├─► @bnto/nodes/generated/schemas.ts → Zod schemas
        ├─► @bnto/nodes/generated/definitionSchema.ts → JSON Schema
        ├─► @bnto/backend/convex/_helpers/nodeTypeLabels.ts → Convex labels
        └─► @bnto/i18n/src/generated/nodes.json → i18n strings
              │
              ▼
        @bnto/registry → re-exports all
              │
              ▼
        @bnto/core → re-exports via registryClient
              │
              ├─► Editor node palette (useNodePalette)
              ├─► Recipe pages (via BNTO_REGISTRY)
              └─► Explore grid (via getAllRecipes)
```

And how a recipe flows from the engine through every consumer surface:

```
Recipe Definition (Engine-Owned)
  └─ engine/recipes/{slug}.bnto.json              ← Source of truth
  └─ engine/crates/bnto-engine/src/recipes.rs     ← builtin_recipes() via include_str!()
        │
        ├─► engine integration tests (include_str! in lib.rs)
        ├─► CLI golden tests (recipe_path() → engine/recipes/)
        ├─► explicit fixtures (hand-maintained in bnto/tests/fixtures/explicit/)
        │
        ├─► task wasm:codegen → catalog.snapshot.json → generate TS
        │     └─► @bnto/nodes/src/generated/recipes.ts → GENERATED_RECIPES
        │
        └─► recipesCatalog.ts overlays web metadata (descriptions, features, IDs)
              └─► RECIPES array (web-facing)
                    │
                    ├─► getAllRecipes() → runtime consumers
                    │     ├─► apps/web/lib/bntoRegistry.ts → BNTO_REGISTRY
                    │     │     ├─► [bnto]/page.tsx → generateStaticParams + generateMetadata
                    │     │     ├─► buildBntoSitemapEntries.ts → sitemap.xml
                    │     │     └─► BntoJsonLd → structured data
                    │     ├─► llms.txt route → AI discovery
                    │     ├─► llms-full.txt route → detailed AI discovery
                    │     ├─► recipeLinks.ts → nav dropdown
                    │     ├─► ExploreRecipeGrid.tsx → home/explore grid
                    │     └─► RecipeMarquee.tsx → landing page marquee
                    │
                    └─► task readme:generate → README.md recipe table
```

---

## Command Sequence (quick reference)

```bash
# 1. Implement + test processor
cargo test -p bnto-{crate}                   # Unit tests

# 2. Register + codegen
task wasm:codegen                            # build → snapshot → generate TS

# 3. Recipe definition (engine-owned)
# ... create engine/recipes/{slug}.bnto.json
# ... add include_str!() to engine/crates/bnto-engine/src/recipes.rs
# ... add web overlay to packages/@bnto/registry/src/recipesCatalog.ts
task wasm:codegen                            # Regenerate TS from engine catalog

# 4. Golden tests
task cli:golden:bless                        # Generate golden files (first time)
task cli:golden                              # Verify byte-exact (subsequent)

# 5. README
task readme:generate                         # Update recipe table in README.md

# 6. Quality gate
task check                                   # Full lint + test + build
```

---

## FileData Selection

**Processors that pass through files without modifying their content MUST use `FileData::Path`, not `FileData::Bytes`.** This is a performance-critical decision — `FileData::Path` enables zero-copy file moves via `rename()` (O(1)), while `FileData::Bytes` reads the entire file into RAM.

```rust
// BAD — reads a 2 GB video file into memory just to pass it downstream
let data = std::fs::read(&full_path)?;
output_files.push(OutputFile {
    data: FileData::Bytes(data),  // 2 GB heap allocation
    ..
});

// GOOD — path reference, zero memory overhead
output_files.push(OutputFile {
    data: FileData::Path(full_path),  // O(1), zero-copy on write
    ..
});
```

**When to use which:**

| Variant           | Use when                                                                    | Examples                                                  |
| ----------------- | --------------------------------------------------------------------------- | --------------------------------------------------------- |
| `FileData::Path`  | Processor doesn't modify file content (collect, filter, rename, move, copy) | `file-collect`, `file-rename`, `file-move`, `file-filter` |
| `FileData::Bytes` | Processor transforms content and produces new bytes                         | `image-compress`, `image-resize`, `spreadsheet-clean`     |

**How it works downstream:** `FileData::Path` defers reading until needed. `write_to()` uses `rename()` for same-device moves (O(1)) and falls back to copy + delete for cross-device. `into_bytes()` reads from disk only when a downstream processor actually needs the content.

**The rule:** If your processor's `process()` function doesn't call `input.data.into_bytes()` or otherwise consume the file content, its output should use `FileData::Path` to avoid unnecessary memory allocation.

---

## Parameter Contract

**Every parameter defined in `metadata()` MUST be read and used in ALL code paths of `process()`.** If a param only applies to some formats, document that in the param description and validate it in `validate()`.

```rust
// BAD — quality param defined but only used for JPEG
fn process(&self, ...) -> Result<...> {
    let quality = get_param("quality");  // read
    match format {
        Jpeg => encode_jpeg(img, quality),  // used
        Png => encode_png(img),             // quality ignored!
        WebP => encode_webp(img),           // quality ignored!
    }
}

// GOOD — quality param flows to all format paths via shared encode
fn process(&self, ...) -> Result<...> {
    let quality = get_param("quality");
    encode::encode_image(&img, format, quality)  // all formats respect quality
}
```

**Test requirement:** Every parameterized node MUST have a test that verifies different parameter values produce measurably different outputs.

---

## Shared Encoding

All image processors MUST use `encode::encode_image()` for final encoding. Never write format-specific encode functions inside individual processors.

```rust
// encode.rs — the single encoding entrypoint
pub fn encode_image(img: &DynamicImage, format: ImageFormat, quality: u8) -> Result<Vec<u8>>
```

Format-specific behavior:

- **JPEG**: `quality` maps to encoder quality (1-100)
- **PNG**: `quality` maps to `CompressionType` — <33 Fast, <66 Default, >=66 Best (lossless, affects speed/size)
- **WebP**: Lossless only (`image` crate limitation). Quality param accepted but documented as no-op

---

## Shared Parameter Definitions

Reusable params live in `common.rs`, not duplicated per-processor:

| Param           | Location                      | Used by                   |
| --------------- | ----------------------------- | ------------------------- |
| `quality`       | `common::quality_param_def()` | compress, resize, convert |
| `image_accepts` | `common::image_accepts()`     | all image processors      |

When adding a new shared param, put it in `common.rs` and reference it from each processor's `metadata()`.

---

## Parameter Extraction Pattern

Standard pattern for reading params from the JSON config:

```rust
let quality = params
    .get("quality")
    .and_then(|v| v.as_u64())
    .unwrap_or(DEFAULT_JPEG_QUALITY as u64) as u8;
let quality = quality.clamp(1, 100);
```

Steps: get -> and_then (type coerce) -> unwrap_or (default) -> clamp (bounds).

---

## Common Violations

| Violation                                                               | Fix                                                                                |
| ----------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| Param defined in `metadata()` but not read in some `process()` branches | Wire the param through all branches, or use shared encode                          |
| Duplicated encode functions across processors                           | Delete them, use `encode::encode_image()`                                          |
| Default value in code differs from `metadata()` default                 | Use the constant from `bnto-core` (e.g., `DEFAULT_JPEG_QUALITY`)                   |
| Test only checks output validity, not param sensitivity                 | Add a test comparing outputs at two different param values                         |
| Missing golden test for new recipe                                      | Every recipe MUST have golden + explicit equivalence tests                         |
| Test count not updated after adding processor                           | See Test Count Registry table above — update every assertion                       |
| Generated files not committed                                           | Snapshot, TS catalog, recipe fixtures, golden files all committed                  |
| NodeTypeInfo not added                                                  | Add to correct category function in `metadata.rs`                                  |
| Recipe not in RECIPES array                                             | Add to `recipesCatalog.ts` — ALL surfaces derive from this                         |
| Recipe not in `builtin_recipes()`                                       | Add `include_str!()` in `engine/crates/bnto-engine/src/recipes.rs`                 |
| README table stale                                                      | Run `task readme:generate` after adding/changing recipes                           |
| Nav category missing for new category                                   | Add to `CATEGORY_TITLES` and `CATEGORY_ORDER` in `recipeLinks.ts`                  |
| `name()` doesn't match registry key                                     | Align to return the registry key (category-first: `"image-compress"`)              |
| Crate README missing processors                                         | Update the Processors table when adding/removing processors                        |
| Crate name doesn't match category                                       | Rename crate to match (e.g., `bnto-csv` → `bnto-spreadsheet`)                      |
| Processor file name doesn't match operation                             | Rename file to match (e.g., `csv_to_json.rs` → `convert.rs`)                       |
| Pass-through processor uses `FileData::Bytes` instead of `Path`         | Use `FileData::Path` for zero-copy — see [FileData Selection](#filedata-selection) |
