# Bnto Directory

The technical registry of predefined Bntos — slugs, fixtures, node requirements, and implementation status. This is what agents read when building Sprint 2 SEO routing and the template library.

**Strategy layer** (search volume data, prioritization rationale, launch philosophy) lives in Notion:
> **Notion:** Search the bnto workspace for "Bnto Directory & Launch Plan" using the Notion MCP.
> Agents: fetch that page when you need target query strings, search volume data, or tier prioritization rationale.

---

## Tier 1: Launch Bntos (Sprint 2)

All 6 must have fixtures, verified execution, and live SEO URLs before Sprint 2 closes.

| Bnto | Slug | Persona | Fixture |
|------|------|---------|---------|
| Compress Images | `/compress-images` | Casual | ✅ exists |
| Resize Images | `/resize-images` | Casual | ✅ exists |
| Convert Image Format | `/convert-image-format` | Casual | ⬜ needs fixture |
| Rename Files | `/rename-files` | Both | ⬜ needs fixture |
| Clean CSV | `/clean-csv` | Both | ⬜ needs fixture |
| Rename CSV Columns | `/rename-csv-columns` | Developer | ⬜ needs fixture |

---

## Tier 2: Near-Term (First 60 Days)

| Bnto | Slug | Persona | Blocker |
|------|------|---------|---------|
| PDF to Images | `/pdf-to-images` | Casual | needs `pdf` node type |
| Batch Watermark Images | `/watermark-images` | Casual | image node (composite op) |
| Convert CSV to JSON | `/csv-to-json` | Developer | spreadsheet + transform |
| Strip EXIF Data | `/strip-exif` | Both | image node (metadata strip) |
| Merge CSVs | `/merge-csv` | Both | spreadsheet node |
| Fetch & Save URL | `/fetch-url` | Developer | http-request + filesystem |

---

## Tier 3: Backlog

| Bnto | Slug | Notes |
|------|------|-------|
| Extract video thumbnail | `/extract-thumbnail` | ffmpeg via shell-command, cloud only |
| Zip files | `/zip-files` | filesystem node (needs zip op) |
| Unzip archive | `/unzip-files` | filesystem node (needs unzip op) |
| Generate image grid | `/image-grid` | image node composite |
| Validate JSON | `/validate-json` | transform node |
| Format JSON | `/format-json` | transform node |
| Sort CSV by column | `/sort-csv` | spreadsheet node |
| Filter CSV rows | `/filter-csv` | spreadsheet node |
| Fetch API to CSV | `/api-to-csv` | http-request + spreadsheet |

---

## Tier 4: AI-Powered Nodes (Backlog — Requires Async Execution)

AI nodes bring non-deterministic processing into workflows — classification, summarization, extraction, generation. The key differentiator: BYOK (Bring Your Own Key). Users supply their own API keys via the existing secrets system (`engine/pkg/secrets/`). No inference costs for Bnto, no data privacy concerns for users.

**Prerequisite:** The execution engine must support long-running nodes (2-30s) with progress reporting, per-node timeouts, and cancellation before any AI node ships. See [architecture.md](../rules/architecture.md#execution-model-async-support).

**Model:** Desktop-first. Users set `ANTHROPIC_API_KEY` or `OPENAI_API_KEY` as secrets. The `ai` node reads the key at execution time and calls the provider API. Cloud support requires either user-supplied keys via settings UI or Bnto-proxied inference (cost/privacy implications — separate product decision).

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
- All metadata and slug validation driven by `lib/bnto-registry.ts`

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
2. Add to Notion, this file, and `lib/bnto-registry.ts` (with features array)
3. Verify slug doesn't collide with reserved paths
4. Page has plain-language description, JSON-LD features, and entry in `llms.txt`
5. Build passes, h1 matches target query, run counter increments
