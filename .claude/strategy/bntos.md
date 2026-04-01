# Recipe & Node Directory

The technical registry of predefined recipes, node types, and implementation status. This is what agents read when building SEO routing, the recipe editor, and the template library.

**Pricing model** (free vs premium, three-layer framework, terminology): See [pricing-model.md](pricing-model.md) — the single source of truth.

**Strategy layer** (search volume data, prioritization rationale, launch philosophy): See `seo-monetization.md` and `mvp-roadmap.md` in private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`).

---

## Node Classification

> **The dividing line:** Nodes that can run in your browser are free. Nodes that need a server cost money. Node _definitions_ are always available to everyone (`@bnto/nodes`, MIT licensed). The _execution_ of server nodes is what costs money.

### Browser Nodes (free, unlimited)

These execute 100% client-side via Rust WASM or JS. Cost to bnto: $0. No account needed.

| Node Type   | Crate / Library                       | What It Does                                                |
| ----------- | ------------------------------------- | ----------------------------------------------------------- |
| `image`     | Rust `image`, `mozjpeg-sys`, `oxipng` | Compress, resize, convert, strip EXIF, watermark            |
| `csv`       | Rust `csv` + `serde`                  | Clean, rename columns, merge, sort, filter, convert to JSON |
| `file`      | Rust `bnto-file`                      | Rename (pattern/regex), zip, unzip                          |
| `transform` | Rust / JS                             | Expression evaluation, field mapping, data transforms       |
| `pdf`       | JS `pdf.js` + Canvas                  | PDF to images, PDF to text                                  |
| `archive`   | JS (JSZip) or Rust                    | Zip/unzip operations                                        |

### Server Nodes (Pro tier, usage-based)

These require server-side execution on Railway. Real CPU cost per execution. On desktop, these are free (BYOK for AI, local binaries for shell-command).

| Node Type                     | Why Server-Only                                               | Pro Gate                          |
| ----------------------------- | ------------------------------------------------------------- | --------------------------------- |
| `ai`                          | API keys shouldn't be exposed client-side; needs server proxy | Usage-based (real inference cost) |
| `shell-command`               | Impossible in browser (ffmpeg, imagemagick, etc.)             | Usage-based (Railway CPU)         |
| `video`                       | ffmpeg WASM impractically large (~25MB)                       | Usage-based (heavy CPU)           |
| `http-request` (unrestricted) | CORS limits browser reach; server bypasses                    | Usage-based                       |

### Hybrid Nodes

Work in browser with limitations. Cloud unlocks the full experience.

| Node Type                  | Browser Limitation                | Cloud Unlock                    |
| -------------------------- | --------------------------------- | ------------------------------- |
| `http-request` (CORS-safe) | Only CORS-friendly APIs reachable | Server-side fetch bypasses CORS |
| Large file operations      | Browser memory ~2GB practical max | Server handles larger files     |

---

## Recipe Classification

Every predefined recipe falls into one of three execution categories:

| Category         | Execution                          | Cost to Us   | User Access                        |
| ---------------- | ---------------------------------- | ------------ | ---------------------------------- |
| **Browser-only** | 100% client-side (Rust WASM or JS) | $0           | Free, unlimited                    |
| **Hybrid**       | Browser primary, cloud optional    | $0 base      | Free (browser), Pro (cloud unlock) |
| **Server-only**  | Railway + R2                       | Compute cost | Pro tier, usage-based              |

---

## Tier 1: Launch Recipes (Sprint 2B — Browser Execution)

All 6 run 100% client-side. All use browser nodes only. Free, unlimited, no account needed. Fixtures exist, SEO URLs live.

| Recipe               | Slug                    | Persona   | Node Types | Browser Engine                      | Fixture   |
| -------------------- | ----------------------- | --------- | ---------- | ----------------------------------- | --------- |
| Compress Images      | `/compress-images`      | Casual    | `image`    | Rust `image`+`mozjpeg-sys`+`oxipng` | ✅ exists |
| Resize Images        | `/resize-images`        | Casual    | `image`    | Rust `image` (resize module)        | ✅ exists |
| Convert Image Format | `/convert-image-format` | Casual    | `image`    | Rust `image` (decode→encode)        | ✅ exists |
| Rename Files         | `/rename-files`         | Both      | `file`     | Rust `bnto-file` (regex)            | ✅ exists |
| Clean CSV            | `/clean-csv`            | Both      | `csv`      | Rust `csv`+`serde`                  | ✅ exists |
| Rename CSV Columns   | `/rename-csv-columns`   | Developer | `csv`      | Rust `csv`+`serde`                  | ✅ exists |

---

## Tier 1B: Multi-Node Compositions (Browser Execution)

First multi-node predefined recipes. Each runs a pipeline of 3 operations inside a forEach loop. All browser-only, free, unlimited.

| Recipe                  | Slug                       | Persona | Node Types             | Pipeline                                   | Fixture          |
| ----------------------- | -------------------------- | ------- | ---------------------- | ------------------------------------------ | ---------------- |
| Optimize Images for Web | `/optimize-images-for-web` | Casual  | `image`                | Resize → Convert (WebP) → Compress         | N/A (multi-node) |
| Generate Thumbnails     | `/generate-thumbnails`     | Casual  | `image`, `file-system` | Resize → Convert (WebP) → Rename (thumb\_) | N/A (multi-node) |

**Key insight:** `definitionToPipeline` merges flat `configOverrides` into ALL leaf processing nodes. For multi-node recipes this works because each processor ignores unknown keys — `width` (resize), `format` (convert), `quality`/`prefix` (compress/rename) don't conflict.

---

## Tier 2: Explore & Discovery Infrastructure

**Prerequisite for recipe expansion.** Before adding more recipes, unify how recipes and nodes are listed across all surfaces. Currently, the home page (8 recipes), navbar Explore menu (6 recipes), editor palette (12 node types), and editor open dialog (all predefined recipes) all use different data transforms and show different subsets.

**What this tier delivers:**

- **Unified listing mechanism** — Single source of truth for what recipes/nodes are available, used by all surfaces (home, Explore, editor, sitemap). Currently `@bnto/nodes` is the source but each surface applies its own adapter/transform.
- **Dedicated Explore page** — The Explore navbar dropdown becomes a link to `/explore` (or similar), a full-page searchable/filterable recipe & node repository. Think npm registry or Figma community — browse by category, search by keyword, see metadata.
- **Registry unification** — `bntoRegistry.ts` (web SEO adapter), `navData.ts` (nav categories), `useNodePalette` (editor node types), and `RecipePickerGrid` (editor recipes) all derive from the same unified query. Add a recipe = it appears everywhere automatically.

**Tasks (to be detailed in Sprint 6):**

- Audit all listing surfaces and their data sources
- Design unified recipe/node query API in `@bnto/core`
- Build `/explore` page with search, filtering, and category browsing
- Migrate Explore dropdown to link to `/explore` page
- Ensure adding a recipe to `@bnto/nodes` automatically propagates to all surfaces

---

## Tier 3: Near-Term Recipes

All browser-only (free, unlimited) except Fetch & Save URL which is hybrid. Tier 2 (Explore & Discovery) is complete — new recipes automatically appear across all surfaces.

| Recipe                 | Slug                | Persona   | Node Types             | Status                        |
| ---------------------- | ------------------- | --------- | ---------------------- | ----------------------------- |
| Strip EXIF Data        | `/strip-exif`       | Both      | `image`                | Delivered (PR #292)           |
| Convert CSV to JSON    | `/csv-to-json`      | Developer | `csv`, `transform`     | Delivered (PR #294)           |
| Merge CSVs             | `/merge-csv`        | Both      | `csv`                  | Delivered (PRs #295, #296)    |
| Batch Watermark Images | `/watermark-images` | Casual    | `image`                | Delivered (PRs #308, #309)    |
| PDF to Images          | `/pdf-to-images`    | Casual    | `pdf`                  | Blocked: pdf.js + Canvas (JS) |
| Fetch & Save URL       | `/fetch-url`        | Developer | `http-request`, `file` | Blocked: Hybrid — CORS limits |

---

## Tier 4: Backlog Recipes

| Recipe                  | Slug                 | Classification        | Node Types            | Notes                                     |
| ----------------------- | -------------------- | --------------------- | --------------------- | ----------------------------------------- |
| Extract video thumbnail | `/extract-thumbnail` | **Server-only (Pro)** | `shell-command`       | ffmpeg — impractical in browser WASM      |
| Zip files               | `/zip-files`         | Browser-only          | `archive`             | JS zip libraries (JSZip)                  |
| Unzip archive           | `/unzip-files`       | Browser-only          | `archive`             | JS unzip libraries                        |
| Generate image grid     | `/image-grid`        | Browser-only          | `image`               | Rust `image` composite or Canvas API      |
| Validate JSON           | `/validate-json`     | Browser-only          | `transform`           | Pure JS (JSON.parse)                      |
| Format JSON             | `/format-json`       | Browser-only          | `transform`           | Pure JS (JSON.stringify)                  |
| Sort CSV by column      | `/sort-csv`          | Browser-only          | `csv`                 | Rust `csv` or PapaParse                   |
| Filter CSV rows         | `/filter-csv`        | Browser-only          | `csv`                 | Rust `csv` or PapaParse                   |
| Fetch API to CSV        | `/api-to-csv`        | Hybrid                | `http-request`, `csv` | CORS limits browser; server proxy for Pro |

---

## Tier 5: AI-Powered Recipes (Backlog — M4, Server-Only, Pro Tier)

**Uses server nodes.** The `ai` node type requires server-side execution (API keys shouldn't be exposed client-side). These are Pro tier recipes with usage-based pricing — a natural conversion hook because they have real compute cost.

AI nodes bring non-deterministic processing into recipes — classification, summarization, extraction, generation. BYOK (Bring Your Own Key) on desktop; bnto-proxied on cloud (Pro).

**Prerequisite:** The execution engine must support long-running nodes (2-30s) with progress reporting, per-node timeouts, and cancellation. See [architecture.md](../rules/architecture.md#execution-model-async-support).

**Desktop model:** Free forever. Users set `ANTHROPIC_API_KEY` or `OPENAI_API_KEY` as secrets — they pay their provider directly, not bnto.
**Cloud model:** Pro tier. Bnto proxies the request. Usage-based (counts against server-side execution quota).

| Recipe            | Slug            | Node Types | Notes                                                         |
| ----------------- | --------------- | ---------- | ------------------------------------------------------------- |
| AI Classify Files | `/ai-classify`  | `ai`       | Classify files by content (images, documents)                 |
| AI Summarize Text | `/ai-summarize` | `ai`       | Summarize text files, CSV columns, logs                       |
| AI Extract Data   | `/ai-extract`   | `ai`       | Extract structured data from unstructured text                |
| AI Transform      | `/ai-transform` | `ai`       | General-purpose LLM transform (rename by content, tag, label) |

**Node config shape (planned):**

```json
{
  "type": "ai",
  "config": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514",
    "prompt": "Classify this image into one of: landscape, portrait, product, document",
    "outputFormat": "json"
  }
}
```

**Not in scope yet:** AI-assisted workflow authoring (natural language to `.bnto.json`) and AI-assisted node scaffolding. These are product surface features, not engine nodes. The `.bnto.json` format being human-readable and LLM-friendly is an architectural asset that enables both — but they live in the editor/CLI layer, not the node layer.

---

## Node Types Needed

| Node Type | Needed For                 | Priority | Classification | Notes                                             |
| --------- | -------------------------- | -------- | -------------- | ------------------------------------------------- |
| `pdf`     | PDF to Images, PDF to Text | High     | Browser node   | pdf.js (JS) for browser; `pdfcpu` (Go) for server |
| `archive` | Zip/Unzip                  | Medium   | Browser node   | JSZip (JS) for browser; Go stdlib for server      |

Before building a new node type: verify the task isn't achievable with existing browser nodes. For server-only tasks, check if `shell-command` + a pre-installed binary in the Railway container works first.

---

## SEO URL Conventions

See [rules/seo.md](../rules/seo.md) for the full SEO strategy -- slug registry, naming conventions, metadata format, static generation, sitemap, middleware integration, and canonical URL handling.

**Quick reference (details in seo.md):**

- Lowercase, hyphen-separated: `/compress-images` not `/CompressImages`
- Verb-first: `/compress-images`, `/rename-files`, `/clean-csv`
- Action-oriented: `/compress-images` not `/image-compressor`
- No internal bnto names: `/compress-images` not `/run-image-compress`
- All metadata and slug validation driven by `lib/bntoRegistry.ts`

---

## Fixture File Conventions

Fixtures live in `engine/examples/`. Named `kebab-case.bnto.json` matching the slug.

```
engine/examples/
  compress-images.bnto.json
  resize-images.bnto.json
  convert-image-format.bnto.json
  rename-files.bnto.json
  clean-csv.bnto.json
  rename-csv-columns.bnto.json
```

Every fixture must:

- Execute clean via `bnto run` with sample input files
- Be registered as an integration test in the engine test suite
- Have a `description` field in plain language

---

## Adding a New Recipe

See the full checklist in [rules/seo.md](../rules/seo.md#checklist-shipping-a-new-bnto). Summary:

1. **Classify the node types** — Does it use browser nodes only (free) or server nodes (Pro)? See [pricing-model.md](pricing-model.md).
2. Create or verify the fixture in `engine/examples/`
3. Add to this file and `lib/bntoRegistry.ts` (with features array)
4. Verify slug doesn't collide with reserved paths
5. Page has plain-language description, JSON-LD features, and entry in `llms.txt`
6. Build passes, h1 matches target query, execution counter increments
