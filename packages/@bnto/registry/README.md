# @bnto/registry

Public facade for the entire node system — re-exports all types, constants, helpers, and validation from `@bnto/nodes`, plus recipe definitions, curation functions, and recipe validation.

## What This Package Does

- **Node system facade:** Re-exports everything from `@bnto/nodes` that consumers need. `@bnto/nodes` is internal — only this package imports from it
- **Recipe system:** Owns all predefined recipe definitions, the `Recipe` type, and recipe-level validation (`validateRecipe`)
- **Recipe utilities:** `definitionToRecipe()` (wraps definitions into recipes), `deriveAcceptSpec()` (extracts file acceptance from input nodes)
- **Curation functions:** Stateless lookups over the recipe catalog (by slug, by category) and engine-generated node metadata
- **Codegen:** Script to generate `.bnto.json` fixture files from TypeScript recipe definitions (consumed by Rust engine tests, CLI tests, and E2E drift checks)
- No React, no Zustand, no state — purely stateless functions and re-exports

## Dependency Chain

```
@bnto/editor → @bnto/core → @bnto/registry → @bnto/nodes
```

`@bnto/core` re-exports from registry for runtime consumers. Build-time SSG code can import from registry directly.

## Directory Structure

```
src/
├── index.ts                # Barrel — all re-exports + curation functions
├── types.ts                # RegistryData interface + type re-exports
├── recipe.ts               # Recipe + AcceptSpec type definitions
├── recipesCatalog.ts       # RECIPES constant array + getRecipeBySlug()
├── recipes.ts              # getAllRecipes(), getRecipeBySlug(), getRecipesByCategory()
├── definitionToRecipe.ts   # Wraps a Definition into a Recipe with metadata
├── deriveAcceptSpec.ts     # Derives AcceptSpec from a Definition's input node
├── validateRecipe.ts       # Recipe-level validation (I/O, structure, connectivity)
├── catalog.ts              # Catalog constants re-exports (NODE_TYPES, PROCESSORS, etc.)
├── definition.ts           # Definition CRUD re-exports (createBlankDefinition, addNode, etc.)
├── helpers.ts              # Classification + I/O helper re-exports
├── schemas.ts              # Schema introspection re-exports
├── validation.ts           # Definition validation re-exports
├── nodeTypes.ts            # getAllNodeTypes(), getBrowserNodeTypes()
├── categories.ts           # getAllCategories()
├── processors.ts           # getAllProcessors()
└── recipes/                # Predefined recipe definitions
    ├── index.ts            # Barrel export for all recipes
    ├── compressImages.ts   # Tier 1 single-op recipes
    ├── resizeImages.ts
    ├── convertImageFormat.ts
    ├── renameFiles.ts
    ├── cleanCsv.ts
    ├── renameCsvColumns.ts
    ├── optimizeImagesForWeb.ts
    ├── generateThumbnails.ts
    ├── compressAndRename.ts      # Tier 2 multi-node compositions
    ├── standardizeCsv.ts
    ├── defaultInputNode.ts       # Shared recipe building blocks
    ├── defaultOutputNode.ts
    └── generated/                # .bnto.json fixtures (codegen output)
        ├── compress-images.bnto.json
        └── ...
scripts/
└── generate-recipe-fixtures.ts   # TS recipes → JSON fixtures (task recipes:generate)
```

## API

```typescript
// Recipe types
import type { Recipe, AcceptSpec } from "@bnto/registry";

// Curation functions
import { getAllRecipes, getRecipeBySlug, getRecipesByCategory } from "@bnto/registry";
import { getAllNodeTypes, getBrowserNodeTypes } from "@bnto/registry";
import { getAllCategories } from "@bnto/registry";
import { getAllProcessors } from "@bnto/registry";

// Recipe utilities
import { definitionToRecipe } from "@bnto/registry";
import { deriveAcceptSpec } from "@bnto/registry";
import { validateRecipe } from "@bnto/registry";

// Types (re-exported from @bnto/nodes)
import type { Definition, NodeTypeName, NodeTypeInfo } from "@bnto/registry";

// Constants (re-exported from @bnto/nodes)
import { NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/registry";

// Helpers (re-exported from @bnto/nodes)
import { isIoNodeType, isContainerNodeType, validateDefinition } from "@bnto/registry";
```

## Consumers

| Consumer         | Usage                                                                  |
| ---------------- | ---------------------------------------------------------------------- |
| `@bnto/core`     | Re-exports for runtime consumers + wraps in Zustand store              |
| `apps/web` (SSG) | Build-time routes, metadata, llms.txt (no React context available)     |
| `bnto-engine`    | Rust crate consumes generated `.bnto.json` fixtures via `include_str!` |
| `bnto-cli`       | CLI integration tests load fixtures from `recipes/generated/`          |

## Development

```bash
pnpm test          # Run tests
pnpm build         # Type-check (tsc --noEmit)
task recipes:generate  # Regenerate .bnto.json fixtures from TS definitions
```
