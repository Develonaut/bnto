# @bnto/registry

Stateless curation and discovery layer for predefined recipes and node type metadata. Sits between `@bnto/nodes` (raw engine catalog) and `@bnto/core` (reactive store + hooks).

## What This Package Does

- Provides lookup functions over the engine-generated catalog
- Curates what's available to consumers (recipes, node types, categories, processors)
- No React, no Zustand, no state — purely stateless functions

## Directory Structure

```
src/
├── index.ts          # Barrel export
├── types.ts          # RegistryData interface
├── recipes.ts        # getAllRecipes(), getRecipeBySlug(), getRecipesByCategory()
├── nodeTypes.ts      # getAllNodeTypes(), getBrowserNodeTypes()
├── categories.ts     # getAllCategories()
├── processors.ts     # getAllProcessors()
├── recipes.test.ts
├── nodeTypes.test.ts
└── categories.test.ts
```

## API

```typescript
import { getAllRecipes, getRecipeBySlug, getRecipesByCategory } from "@bnto/registry";
import { getAllNodeTypes, getBrowserNodeTypes } from "@bnto/registry";
import { getAllCategories } from "@bnto/registry";
import { getAllProcessors } from "@bnto/registry";
import type { RegistryData } from "@bnto/registry";
```

## Consumers

| Consumer         | Usage                                                              |
| ---------------- | ------------------------------------------------------------------ |
| `@bnto/core`     | Wraps in Zustand store for reactive hooks (`core.registry.*`)      |
| `apps/web` (SSG) | Build-time routes, metadata, llms.txt (no React context available) |

## Development

```bash
pnpm test          # Run tests
pnpm build         # Type-check (tsc --noEmit)
```
