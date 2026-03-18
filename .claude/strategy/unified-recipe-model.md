# Unified Recipe Model

**Status:** Design approved
**Date:** March 18, 2026
**Sprint:** 7 (Explore & Discovery Infrastructure)

---

## Decision

**A recipe IS a Definition.** The `Definition` type is the universal unit — what you build in the editor, what the engine executes, what gets exported as `.bnto.json`. Everything else is context about where it lives.

The two existing `Recipe` types (`@bnto/nodes` predefined wrapper and `@bnto/core` persistence wrapper) are eliminated. The `RecipeDefinition` structural duplicate in `@bnto/core` is deleted.

---

## The Model

```
Definition                    — the recipe itself
  ├── id, name                — identity
  ├── metadata.description    — what it does
  ├── metadata.tags           — user-defined tags
  ├── nodes[], edges[]        — the pipeline
  └── input node parameters   — what files it accepts

Publishing (web registry)     — how we present it on bnto.io
  └── slug, seo, features

Persistence (store envelope)  — how we track saves and cloud sync
  └── savedAt, syncedAt, cloudId
```

### What lives where

| Data                  | Owner                                                      | Rationale                                                          |
| --------------------- | ---------------------------------------------------------- | ------------------------------------------------------------------ |
| `name`                | `Definition.name`                                          | Identity — travels with the recipe                                 |
| `description`         | `Definition.metadata.description`                          | Already there. Useful for all recipes                              |
| `tags`                | `Definition.metadata.tags`                                 | User-defined organization. Travels with the recipe                 |
| `accept` (file types) | **Derived** from input node `parameters.accept/extensions` | Already declared on the input node — don't duplicate               |
| `category`            | **Derived** from processing node types                     | An image recipe uses image nodes. Don't store what you can compute |
| `slug`                | Web registry                                               | Publishing concern — the engine doesn't know about URLs            |
| `seo.title`, `seo.h1` | Web registry                                               | Marketing copy for the website                                     |
| `features`            | Web registry                                               | Marketing labels ("Browser-based", "No upload")                    |
| `savedAt`             | Store envelope                                             | When the user saved it                                             |
| `syncedAt`            | Store envelope                                             | When it last synced to Convex                                      |
| `cloudId`             | Store envelope                                             | Convex document ID for cloud sync                                  |

### The line

> If you exported the Definition as a `.bnto.json` and gave it to someone else, would they need that field?
>
> `name` and `description` — yes. `seo.h1` — no. That's ours.

---

## Type Changes

### `@bnto/nodes`

**Before:**

```typescript
// recipe.ts — predefined wrapper
interface Recipe {
  slug: string;
  name: string;
  description: string;
  category: string;
  accept: AcceptSpec;
  features: string[];
  seo: SEOSpec;
  definition: Definition;
}
```

**After:**

```typescript
// Deleted. Predefined recipes are just Definition objects.
// RECIPES array becomes Definition[].
// AcceptSpec, SEOSpec deleted from this package.
```

The `RECIPES` export becomes `Definition[]`:

```typescript
export const RECIPES: readonly Definition[] = [
  compressImages,
  resizeImages,
  // ...
];
```

Individual recipe files export `Definition` directly:

```typescript
// recipes/compressImages.ts
export const compressImages: Definition = {
  id: "compress-images",
  type: "group",
  name: "Compress Images",
  metadata: { description: "Compress PNG, JPEG, and WebP images." },
  // ... nodes, edges
};
```

### `@bnto/core`

**Before:**

```typescript
// types/recipe.ts — persistence wrapper
interface Recipe {
  id: string;
  name: string;
  definition: RecipeDefinition;  // structural duplicate of Definition
  type: string;
  version: string;
  cloudId?: string;
  savedAt: number;
  syncedAt: number | null;
}

// RecipeDefinition — full duplicate of @bnto/nodes Definition
interface RecipeDefinition { ... }
interface Position { ... }
interface Metadata { ... }
// etc.
```

**After:**

```typescript
// types/recipe.ts — thin persistence envelope
import type { Definition } from "@bnto/nodes";

/** Persistence metadata for a saved recipe. */
interface SavedRecipe {
  definition: Definition;
  savedAt: number;
  syncedAt: number | null;
  cloudId?: string;
}

// RecipeDefinition, Position, Metadata, Port, Edge, FieldsConfig — all deleted.
// Import Definition from @bnto/nodes instead.
```

The store becomes `Record<string, SavedRecipe>`.

`RecipeListItem` stays as a projection but derives from `Definition`:

```typescript
interface RecipeListItem {
  id: string; // from definition.id
  name: string; // from definition.name
  nodeCount: number; // from definition.nodes.length
  nodeTypes: string[]; // from processing node type labels
  updatedAt: number; // from savedAt
  syncedAt?: number | null;
}
```

### `apps/web`

**Before:**

```typescript
// bntoRegistry.ts — SEO projection
interface BntoEntry {
  slug: string;
  title: string;
  description: string;
  h1: string;
  fixture: string;
  features: string[];
}
```

**After:**

```typescript
// bntoRegistry.ts — publishing metadata for predefined recipes
interface PublishedRecipe {
  definition: Definition; // the recipe itself
  slug: string; // URL path (e.g., "compress-images")
  seo: {
    title: string; // page title for SERP
    h1: string; // h1 heading for on-page SEO
  };
  features: string[]; // marketing tags for JSON-LD
}
```

`accept` is derived at the point of use by reading the input node's parameters — a pure function:

```typescript
function deriveAcceptSpec(definition: Definition): AcceptSpec { ... }
```

`category` is derived from the processing node types:

```typescript
function deriveCategory(definition: Definition): string { ... }
```

### `navData.ts`

Currently groups `RECIPES` by `recipe.category`. After: derives category from each Definition and groups by that. The hardcoded `NAV_DESCRIPTIONS` map stays (shorter copy for nav is a legitimate UX choice).

---

## Derivation Functions

### `deriveAcceptSpec(definition: Definition): AcceptSpec`

Finds the input node in `definition.nodes`, reads its `parameters.accept`, `parameters.extensions`, and `parameters.label`. Returns the `AcceptSpec`. If no input node found, returns a permissive default.

### `deriveCategory(definition: Definition): string`

Walks `definition.nodes` (recursively into containers), finds the first processing node type (not I/O, not container), returns the category from `NODE_TYPE_INFO[type].category`. Falls back to `"general"`.

Both live in `@bnto/nodes` as pure functions next to the existing `isContainerNodeType`/`isIoNodeType` helpers.

---

## Migration Path

### Wave 2 tasks (build the unified layer)

1. **`@bnto/nodes` — Refactor predefined recipes to export `Definition` directly.** Delete `Recipe`, `AcceptSpec`, `SEOSpec` types. Update `RECIPES` to `Definition[]`. Add `deriveAcceptSpec()` and `deriveCategory()` pure functions. Update `getRecipeBySlug()` → `getDefinitionBySlug()`. Update all imports across the codebase.

2. **`@bnto/core` — Delete `RecipeDefinition` and simplify persistence.** Replace `Recipe` with `SavedRecipe` (thin envelope). Import `Definition` from `@bnto/nodes`. Update `recipesStore`, `recipeClient`, transforms. Keep `RecipeListItem` as a projection.

3. **`apps/web` — Replace `BntoEntry` with `PublishedRecipe`.** Move publishing metadata to `bntoRegistry.ts`. Derive `accept` and `category` from Definitions. Update all consumers (RecipeMarquee, RecipeGrid, tool pages, sitemap, navData).

### Wave 3 tasks (migrate surfaces)

4. **Build `core.catalog` client** — unified access to predefined Definitions and node type info for all surfaces.

5. **Migrate all surfaces** to use `core.catalog` instead of direct `@bnto/nodes` imports.

### Wave 4 tasks (verify + auto-generate)

6. **Auto-generate README** recipe list from the catalog.
7. **E2E verification** — all surfaces render correctly with the new data flow.

---

## What Stays

- **`Definition` type** — the universal recipe format. Unchanged.
- **`RecipeListItem`** — legitimate list projection. Derived from Definition instead of Recipe.
- **`CloudRecipeDetail`** — internal Convex transform. Renamed/simplified but still needed.
- **`RawRecipeDoc` / `RawRecipeListProjection`** — adapter-level Convex shapes. Still needed.
- **`PublishedRecipe`** (new) — replaces `BntoEntry`. Pairs Definition with publishing metadata.
- **`SavedRecipe`** (new) — replaces `@bnto/core` `Recipe`. Thin persistence envelope.

---

## FAQ

**Q: Why not put `category` and `features` in `Definition.metadata`?**
A: Category is derivable from node types. Features are marketing labels ("Browser-based", "No upload") — they don't describe the recipe, they describe our positioning. Neither needs to travel with the `.bnto.json` file.

**Q: Why keep `SavedRecipe` as a wrapper instead of putting persistence fields in `Definition.metadata.customData`?**
A: Persistence concerns (cloud sync, timestamps) are orthogonal to the recipe content. Putting them in `customData` couples storage with the engine format. The thin envelope keeps them separated.

**Q: What about the editor URL? Currently `?from={slug}` vs `?recipe={id}`.**
A: Both are identifying a Definition. Sprint 7 W1 task "Unify editor URL slug pattern" handles this — single `?recipe={identifier}` param that resolves to a Definition by slug or by store ID.
