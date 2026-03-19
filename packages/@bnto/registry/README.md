# @bnto/registry

Public facade for the entire node system — re-exports all types, constants, helpers, and validation from `@bnto/nodes`, plus curation functions for recipe/node type discovery.

## What This Package Does

- **Node system facade:** Re-exports everything from `@bnto/nodes` that consumers need. `@bnto/nodes` is internal — only this package imports from it
- **Curation functions:** Stateless lookups over the engine-generated catalog (recipes, node types, categories, processors)
- No React, no Zustand, no state — purely stateless functions and re-exports

## Dependency Chain

```
@bnto/editor → @bnto/core → @bnto/registry → @bnto/nodes
```

`@bnto/core` re-exports from registry for runtime consumers. Build-time SSG code can import from registry directly.

## Directory Structure

```
src/
├── index.ts          # Barrel — all re-exports + curation functions
├── types.ts          # RegistryData interface + type re-exports
├── catalog.ts        # Catalog constants re-exports (NODE_TYPES, PROCESSORS, etc.)
├── definition.ts     # Definition CRUD re-exports (createBlankDefinition, addNode, etc.)
├── helpers.ts        # Classification + I/O helper re-exports
├── schemas.ts        # Schema introspection re-exports
├── validation.ts     # Validation re-exports
├── recipes.ts        # getAllRecipes(), getRecipeBySlug(), getRecipesByCategory()
├── nodeTypes.ts      # getAllNodeTypes(), getBrowserNodeTypes()
├── categories.ts     # getAllCategories()
└── processors.ts     # getAllProcessors()
```

## API

```typescript
// Curation functions
import { getAllRecipes, getRecipeBySlug, getRecipesByCategory } from "@bnto/registry";
import { getAllNodeTypes, getBrowserNodeTypes } from "@bnto/registry";
import { getAllCategories } from "@bnto/registry";
import { getAllProcessors } from "@bnto/registry";

// Types (re-exported from @bnto/nodes)
import type { Definition, Recipe, NodeTypeName, NodeTypeInfo } from "@bnto/registry";

// Constants (re-exported from @bnto/nodes)
import { NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/registry";

// Helpers (re-exported from @bnto/nodes)
import { isIoNodeType, isContainerNodeType, validateDefinition } from "@bnto/registry";
```

## Consumers

| Consumer         | Usage                                                              |
| ---------------- | ------------------------------------------------------------------ |
| `@bnto/core`     | Re-exports for runtime consumers + wraps in Zustand store          |
| `apps/web` (SSG) | Build-time routes, metadata, llms.txt (no React context available) |

## Development

```bash
pnpm test          # Run tests
pnpm build         # Type-check (tsc --noEmit)
```
