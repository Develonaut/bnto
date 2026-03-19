# Unified Recipe Model — Stacked PRs

**Status:** PR 1 in progress
**Created:** 2026-03-18
**Updated:** 2026-03-18

---

## Core Decision

**Layered types following the layered architecture.** Each layer adds only its own concerns:

```
Definition (@bnto/nodes)  →  Recipe (@bnto/nodes)  →  UserRecipe (@bnto/core)
    engine blueprint          + display metadata        + persistence state
```

```typescript
// @bnto/nodes/src/recipe.ts
interface Recipe {
  id: string; // UUID — all recipes, including predefined
  slug: string; // URL-safe, always present
  name: string;
  description: string;
  category: string;
  definition: Definition;
  accept: AcceptSpec;
  features: string[];
}

// @bnto/core/src/types/recipe.ts
interface UserRecipe extends Recipe {
  cloudId: string | null; // Convex _id. null = never synced
  savedAt: number | null; // null if never saved
  syncedAt: number | null; // null = never synced
}
```

### Why layered types (not one unified type)

Persistence is a core concern, not a node concern. A Recipe describes _what_ to run — a Definition + display metadata. A UserRecipe describes _who saved it and where_ — persistence state that only matters when a user owns the recipe.

This follows the existing layered architecture: `@bnto/nodes` owns engine-agnostic definitions, `@bnto/core` owns persistence and transport. The type chain mirrors the package chain.

**The open source boundary test:** Someone contributing a predefined recipe to `@bnto/registry` only touches `Recipe`. They never see `cloudId`, `savedAt`, or `syncedAt`. Clean.

### What we YAGNI'd

- **No SEOSpec** — title = `"${name} Online Free -- bnto"`, h1 = `name`. Derived in web layer.
- **No version on Recipe** — `definition.version` already exists.
- **No subtitle** — if needed later, add it then.

### Key rules

- **All recipe IDs are UUIDs** — predefined have hardcoded UUIDs, user-created get `crypto.randomUUID()`
- **Slug is required on ALL recipes** — derived from name for user-created
- **Persistence fields are nullable (not optional)** — `cloudId: string | null`, not `cloudId?: string`
- **Persistence lives on UserRecipe only** — `Recipe` has no knowledge of cloud sync
- **Web layer derives SEO** — `bntoRegistry.ts` computes title/h1 from `recipe.name`

---

## What lives where

| Data                  | Owner                                | Rationale                                                         |
| --------------------- | ------------------------------------ | ----------------------------------------------------------------- |
| `id` (UUID)           | `Recipe.id`                          | Identity — same shape everywhere                                  |
| `slug`                | `Recipe.slug`                        | Required on all recipes. User profiles, sharing, URL paths        |
| `name`                | `Recipe.name`                        | Display name — travels with the recipe                            |
| `description`         | `Recipe.description`                 | User-facing description                                           |
| `category`            | `Recipe.category`                    | Grouping. Predefined: hand-set. User-created: derived             |
| `accept` (file types) | `Recipe.accept`                      | File drop zone configuration                                      |
| `features`            | `Recipe.features`                    | Tags for JSON-LD and display                                      |
| Page title            | **Derived** in web layer from `name` | `"${name} Online Free -- bnto"` — marketing copy, not recipe data |
| h1 heading            | **Derived** in web layer from `name` | On-page SEO, not recipe data                                      |
| `cloudId`             | `UserRecipe.cloudId`                 | Convex document ID. Core persistence concern                      |
| `savedAt`             | `UserRecipe.savedAt`                 | When the user saved it. Core persistence concern                  |
| `syncedAt`            | `UserRecipe.syncedAt`                | When it last synced to Convex. Core persistence concern           |

### The two lines

> **Recipe vs Definition:** If you exported the recipe and gave it to someone else, would they need that field?
> `name`, `slug`, `description`, `accept` — yes. Page title, h1 — no.

> **Recipe vs UserRecipe:** Does this field describe the recipe itself, or describe a user's relationship to it?
> `name`, `category`, `features` — the recipe. `cloudId`, `savedAt`, `syncedAt` — the user's copy.

---

## Derivation Functions

### `deriveAcceptSpec(definition: Definition): AcceptSpec | null`

Finds the input node, reads `parameters.accept`, `.extensions`, `.label`. Already existed.

### `deriveCategory(definition: Definition): string`

Walks `definition.nodes` recursively, finds first processing node type, returns `NODE_TYPE_INFO[type].category`. Falls back to `"custom"`. New in this PR.

Both live in `@bnto/nodes` as pure functions.

---

## PR Stack

### PR 1: Type Unification (current — in progress)

**Branch:** `feat/1-type-unification`

- Layered types: `Recipe` (no persistence) in `@bnto/nodes`, `UserRecipe extends Recipe` in `@bnto/core`
- Removed SEOSpec — web layer derives title/h1 from `recipe.name`
- All 8 predefined recipes: UUID ids, no persistence fields, no seo
- `definitionToRecipe()` returns `Recipe` (no persistence)
- `deriveCategory()` added
- Core: deleted `RecipeDefinition` + 5 duplicate types, `UserRecipe` extends `Recipe`
- Core: `recipesStore`, `recipeClient`, `recipeToListItem` all use `UserRecipe`
- Editor: `RecipeMetadata` = `{ id, name, slug, cloudId }` (editor's own type)
- Web: `bntoRegistry.ts` derives title/h1 from name
- All tests updated — build + tests green

### PR 2: core.registry + @bnto/registry package

- `@bnto/registry` package houses predefined recipes (open source friendly)
- `core.registry` Zustand store with localStorage persistence
- Client API: `core.registry.getRecipes()`, `.getNodeTypes()`, etc.
- React hooks for reactive access

### PR 3: URL Unification

- Eliminate `?from={slug}` — "Open in Editor" clones template to personal store, navigates by ID
- `createFromDefinition(definition)` in recipe client
- Simplify `useEditorRecipe` — only `?recipe={id}` or blank canvas

### PR 4: Surface Migration

- Runtime surfaces read from `core.registry` instead of direct `@bnto/nodes` imports
- Build-time surfaces (sitemap, llms.txt) keep direct imports

### PR 5: Documentation + Test Audit

- Update journey matrices, PLAN.md, CLAUDE.md
- Document core.registry as 6th domain
- Final test coverage sweep
