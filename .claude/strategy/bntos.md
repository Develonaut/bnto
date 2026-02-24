# Bnto Directory

The technical registry of predefined Bntos — slugs, fixtures, node requirements, and implementation status. This is what agents read when building Sprint 2 SEO routing and the template library.

**Strategy layer** (search volume data, prioritization rationale, launch philosophy) lives in Notion:
> **Notion:** Search the bnto workspace for "Bnto Directory & Launch Plan" using the Notion MCP.
> Agents: fetch that page when you need target query strings, search volume data, or tier prioritization rationale.

---

## Bnto Classification (from ROADMAP.md)

Every bnto falls into one of three execution categories:

| Category | Execution | Cost to Us | User Tier |
|----------|-----------|-----------|-----------|
| **Browser-only** | 100% client-side (Rust WASM or JS) | $0 | Free, unlimited |
| **Hybrid** | Browser primary, cloud optional | $0 base | Free (limited), Pro (full) |
| **Server-only** | Railway + R2 | Compute cost | Pro tier, usage-based |

---

## Tier 1: Launch Bntos (Sprint 2B — Browser Execution)

All 6 run 100% client-side. Fixtures exist, SEO URLs live, cloud pipeline verified (M4 ready).

| Bnto | Slug | Persona | Classification | Browser Engine | Fixture |
|------|------|---------|---------------|----------------|---------|
| Compress Images | `/compress-images` | Casual | Browser-only | Rust `image`+`mozjpeg-sys`+`oxipng` | ✅ exists |
| Resize Images | `/resize-images` | Casual | Browser-only | Rust `image` (resize module) | ✅ exists |
| Convert Image Format | `/convert-image-format` | Casual | Browser-only | Rust `image` (decode→encode) | ✅ exists |
| Rename Files | `/rename-files` | Both | Browser-only | Pure JS (no Rust needed) | ✅ exists |
| Clean CSV | `/clean-csv` | Both | Browser-only | Rust `csv`+`serde` | ✅ exists |
| Rename CSV Columns | `/rename-csv-columns` | Developer | Browser-only | Rust `csv`+`serde` | ✅ exists |

---

## Tier 2: Near-Term

| Bnto | Slug | Persona | Classification | Blocker |
|------|------|---------|---------------|---------|
| PDF to Images | `/pdf-to-images` | Casual | Browser-only | pdf.js + Canvas (JS) |
| Batch Watermark Images | `/watermark-images` | Casual | Browser-only | Rust `image` composite |
| Convert CSV to JSON | `/csv-to-json` | Developer | Browser-only | Rust `csv`+`serde_json` |
| Strip EXIF Data | `/strip-exif` | Both | Browser-only | Rust `image` metadata strip |
| Merge CSVs | `/merge-csv` | Both | Browser-only | Rust `csv` concat+dedupe |
| Fetch & Save URL | `/fetch-url` | Developer | Hybrid | CORS limits browser reach |

---

## Tier 3: Backlog

| Bnto | Slug | Classification | Notes |
|------|------|---------------|-------|
| Extract video thumbnail | `/extract-thumbnail` | **Server-only** | ffmpeg — impractical in browser WASM |
| Zip files | `/zip-files` | Browser-only | JS zip libraries (JSZip) |
| Unzip archive | `/unzip-files` | Browser-only | JS unzip libraries |
| Generate image grid | `/image-grid` | Browser-only | Rust `image` composite or Canvas API |
| Validate JSON | `/validate-json` | Browser-only | Pure JS (JSON.parse) |
| Format JSON | `/format-json` | Browser-only | Pure JS (JSON.stringify) |
| Sort CSV by column | `/sort-csv` | Browser-only | Rust `csv` or PapaParse |
| Filter CSV rows | `/filter-csv` | Browser-only | Rust `csv` or PapaParse |
| Fetch API to CSV | `/api-to-csv` | Hybrid | CORS limits browser; server proxy for Pro |

---

## Tier 4: AI-Powered Nodes (Backlog — M4, Server-Only, Pro Tier)

**Classification: Server-only.** AI nodes require server-side execution (API keys shouldn't be exposed client-side). These are premium Pro tier features with usage-based pricing — a natural conversion hook because they have real compute cost.

AI nodes bring non-deterministic processing into workflows — classification, summarization, extraction, generation. BYOK (Bring Your Own Key) on desktop; Bnto-proxied on cloud (Pro).

**Prerequisite:** The execution engine must support long-running nodes (2-30s) with progress reporting, per-node timeouts, and cancellation. See [architecture.md](../rules/architecture.md#execution-model-async-support).

**Desktop model:** Free forever. Users set `ANTHROPIC_API_KEY` or `OPENAI_API_KEY` as secrets — they pay their provider directly, not Bnto.
**Cloud model:** Pro tier. Bnto proxies the request. Usage-based (counts against server-side run quota).

| Bnto | Slug | Node Type | Notes |
|------|------|-----------|-------|
| AI Classify Files | `/ai-classify` | `ai` | Classify files by content (images, documents) |
| AI Summarize Text | `/ai-summarize` | `ai` | Summarize text files, CSV columns, logs |
| AI Extract Data | `/ai-extract` | `ai` | Extract structured data from unstructured text |
| AI Transform | `/ai-transform` | `ai` | General-purpose LLM transform (rename by content, tag, label) |

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

| Node Type | Needed For | Priority | Notes |
|-----------|-----------|----------|-------|
| `pdf` | PDF to Images, PDF to Text | High | Wrap `pdfcpu` or similar. Check shell-command + ghostscript as interim. |
| `archive` | Zip/Unzip | Medium | filesystem node extension or new node |

Before building a new node type: verify the task isn't achievable with `shell-command` + a pre-installed binary in the Railway container. Ship that first if so.

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

## Adding a New Bnto

See the full checklist in [rules/seo.md](../rules/seo.md#checklist-shipping-a-new-bnto). Summary:

1. Create or verify the fixture in `engine/examples/`
2. Add to Notion, this file, and `lib/bntoRegistry.ts` (with features array)
3. Verify slug doesn't collide with reserved paths
4. Page has plain-language description, JSON-LD features, and entry in `llms.txt`
5. Build passes, h1 matches target query, run counter increments
