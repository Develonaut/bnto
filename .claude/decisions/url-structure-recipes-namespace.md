# Decision: URL Structure — Root-Level Slugs vs /recipes/ Namespace

**Date:** 2026-02-23
**Status:** Decided — keep root-level slugs, revisit when community recipes ship
**Context:** Whether predefined recipes should move from `bnto.io/compress-images` to `bnto.io/recipes/compress-images` to align with engine terminology and prepare for community recipes

---

## The Question

The engine uses "menu" (catalog) and "recipes" (predefined workflows) terminology internally (`engine/pkg/menu/recipes/`). Should the URL structure reflect this by namespacing under `/recipes/`?

The motivation: if community-authored recipes are a future goal, establishing the namespace now avoids a migration later. The "menu" as a browsable registry of both curated and community recipes is a compelling product concept.

---

## Decision: Keep Root-Level Slugs

Curated recipes stay at root level. Community content gets its own namespace when it ships.

```
bnto.io/compress-images          # Curated (root-level, SEO-optimized)
bnto.io/resize-images            # Curated (root-level, SEO-optimized)

bnto.io/menu                     # Browsable catalog (future — curated + community)
bnto.io/menu/@ryan/my-pipeline   # Community recipe (future — user namespace)
```

---

## Rationale

### SEO authority is the primary acquisition channel

The entire growth strategy depends on root-level SEO pages targeting high-intent queries (100K-200K monthly searches for Tier 1 bntos). `bnto.io/compress-images` reads to Google as a top-level page about image compression. `bnto.io/recipes/compress-images` reads as a sub-section of a catalog. Shorter, flatter URLs carry more perceived authority.

Every competitor uses flat URLs: TinyPNG (`tinypng.com`), iLovePDF (`ilovepdf.com/compress_pdf`), Convertio (`convertio.co/png-to-jpg/`). None namespace their tools.

### "The URL IS the app"

From the SEO strategy: users land ready to run, not on a catalog page. `bnto.io/compress-images` feels like a dedicated tool. `bnto.io/recipes/compress-images` feels like an item in a collection. Persona 1 (casual user) doesn't think in terms of recipes — they think "I need to compress images." The root slug matches their mental model.

### Namespace collision is already solved

`RESERVED_PATHS` + the slug registry prevent collisions between bnto slugs and app routes. Adding `/recipes/` solves a problem that doesn't exist.

### Community namespace doesn't require moving curated recipes

Community recipes can live under `/menu/` (or `/r/`, `/community/`) without touching curated root-level slugs. A popular community recipe can be "promoted" to a root-level curated slug — similar to how npm features popular packages.

### Premature optimization

Pre-Sprint 2. Curated recipes haven't been indexed yet. Optimizing for community namespace before the first user runs their first bnto is premature. Let the SEO trees compound first.

---

## The Community Recipe Model (Future)

When community recipes ship (Sprint 7+ at earliest), the URL structure extends naturally:

| Content type | URL pattern | Example |
|---|---|---|
| Curated recipe | `/{slug}` | `/compress-images` |
| Browsable catalog | `/menu` | `/menu` |
| Community recipe (authored) | `/menu/@{user}/{slug}` | `/menu/@ryan/compress-and-email` |
| Community recipe (promoted) | `/menu/{slug}` | `/menu/pdf-to-docx` |

The "menu" is where you browse everything — curated and community. Root-level slugs are the "featured items on the front counter."

### Promotion path

A community recipe that proves itself (usage, ratings) can get promoted to a root-level curated slug. This creates a flywheel: community authors are incentivized to build quality recipes because promotion means more visibility.

---

## Terminology vs URL Structure

These serve different masters:

- **Terminology** (menu, recipes) serves brand, docs, developer experience, and UI copy
- **URL structure** serves SEO, user perception, and information architecture

Use "recipes" everywhere in the product (UI labels, gallery title, CLI commands, API responses). Keep the URL structure flat for SEO. The gallery page can be titled "Menu" and say "Browse recipes" — that's a UI concern, not a routing concern.

---

## When to Revisit

1. **Real community recipe demand** — users are asking to publish their own recipes
2. **Catalog exceeds ~30-40 curated recipes** — root-level namespace pressure becomes real
3. **SEO data available** — we can measure how root-level pages actually perform before changing them
4. **Migration cost is known** — 301 redirects preserve some SEO equity but not all; measure before committing

---

## Related

- [rules/seo.md](../rules/seo.md) — Full SEO strategy, slug conventions, static generation
- [strategy/bntos.md](../strategy/bntos.md) — Bnto directory with tiers and fixture status
- Notion: "SEO & Monetization Strategy" — pricing, search volume data, acquisition strategy
- Notion: "Bnto Directory & Launch Plan" — tier prioritization and launch philosophy
