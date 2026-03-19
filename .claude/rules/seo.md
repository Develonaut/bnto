# SEO & URL Strategy

Every predefined Bnto has its own dedicated, indexable URL at the root level. The URL IS the app — users land ready to drop files.

---

## Routing: Root-Level Slugs

Slugs live at the root: `/compress-images`, `/clean-csv`, `/rename-files`. Route: `app/[bnto]/page.tsx`.

**Trade-off:** Root-level dynamic segments match every unknown path. Every new top-level app route must be static (wins over `[bnto]`). Check `lib/bntoRegistry.ts` for collisions.

---

## Slug Registry

Single source of truth: `apps/web/lib/bntoRegistry.ts`. Drives `generateStaticParams`, `generateMetadata`, middleware pass-through, and sitemap from one place. See the file for the `BntoEntry` type (derived from `Recipe` in `@bnto/nodes`) and helper functions.

When adding a bnto to `strategy/bntos.md`, add it to the registry too.

---

## Static Generation & 404s

- All SEO pages use `generateStaticParams` + `generateMetadata` for build-time rendering
- Unknown slugs call `notFound()` — real 404 status, not soft 404
- `app/not-found.tsx` renders a branded 404 page

---

## Canonical URLs

Slugs are lowercase-hyphen only. Non-canonical variants 301 redirect (handled in middleware before auth check):

- `/Compress-Images` → `/compress-images`
- `/compress_images` → `/compress-images`
- `/compress-images/` → `/compress-images` (no trailing slash)

---

## Metadata Strategy

```
title:       "[Action] Online Free -- bnto"  (flexible, must end with " -- bnto")
description: One sentence. Plain language. Includes the action and "free".
h1:          Exact target search query from `seo-monetization.md` (private business docs, see `BNTO_PRIVATE_DOCS_PATH`)
```

Titles are for browser tabs and SERP; h1 is for on-page SEO. They may differ.

---

## Structured Data (JSON-LD)

Each tool page includes `WebApplication` JSON-LD with `price: "0"` and `featureList`. See `BntoJsonLd` in `app/[bnto]/page.tsx`.

---

## LLM & AI Search Discovery

Tool pages include a factual description section (what/accepts/outputs/costs) using semantic HTML (`<dl>`, `<dt>`, `<dd>`). `/llms.txt` and `/llms-full.txt` are auto-generated route handlers (`app/llms.txt/route.ts`, `app/llms-full.txt/route.ts`) that read from the `RECIPES` registry in `@bnto/nodes` — no manual editing needed when adding new recipes. `robots.txt` explicitly allows all AI crawlers (GPTBot, ClaudeBot, PerplexityBot, etc.).

---

## Slug Naming Conventions

- **Lowercase, hyphen-separated, verb-first:** `/compress-images` not `/image-compressor`
- **No reserved words:** `signin`, `signup`, `workflows`, `settings`, `pricing`, `about`, `blog`, `docs`, `api`, `admin`, `dashboard`
- **User workflows (future):** Will live under `/w/[id]`, never at root level

---

## Checklist: Shipping a New Bnto

### Engine

1. Create/verify fixture in `engine/examples/`
2. Verify fixture runs clean via `bnto run`

### Registry

3. Add to [strategy/bntos.md](../strategy/bntos.md)
4. Add to `strategy/bntos.md`
5. Add to `lib/bntoRegistry.ts` with full metadata (title, description, h1, fixture, features)
6. Verify slug doesn't collide with reserved paths

### SEO & LLM

7. h1 matches exact target query from `seo-monetization.md` (private business docs, see `BNTO_PRIVATE_DOCS_PATH`)
8. Page has plain-language description section (what/accepts/outputs/costs)
9. JSON-LD `featureList` populated
10. Verify `pnpm build` succeeds — `llms.txt` and `llms-full.txt` auto-generate from `RECIPES`

### Runtime

12. Verify execution increments run counter in Convex
