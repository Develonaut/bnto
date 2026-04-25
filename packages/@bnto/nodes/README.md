# @bnto/nodes

Engine-agnostic node definitions. The TypeScript mirror of what the Rust engine knows.

## Overview

`@bnto/nodes` bridges the Rust engine and TypeScript consumers. Most of its code is **generated** from the engine's self-describing catalog. Hand-written code is limited to type interfaces, predefined recipe compositions, and schema helpers.

Consumed by `@bnto/core`, `@bnto/editor`, and `apps/web/`. Never imports from those packages. It's a leaf dependency.

## Directory Structure

```
src/
├── generated/                  # Auto-generated from engine catalog (DO NOT EDIT)
│   ├── catalog.ts              # NODE_TYPES, NODE_TYPE_INFO, PROCESSORS, PROCESSOR_MAP
│   ├── schemas.ts              # Zod schemas + NodeSchema per node type
│   └── definitionSchema.ts     # JSON Schema for .bnto.json validation
├── recipes/                    # Predefined recipe compositions (hand-written)
│   ├── compressImages.ts
│   ├── resizeImages.ts
│   ├── convertImageFormat.ts
│   ├── generateThumbnails.ts
│   ├── optimizeImagesForWeb.ts
│   ├── cleanCsv.ts
│   ├── renameCsvColumns.ts
│   ├── renameFiles.ts
│   ├── defaultInputNode.ts
│   └── defaultOutputNode.ts
├── schemas/                    # Schema registry + per-operation configs (hand-written)
│   ├── registry.ts             # NODE_SCHEMAS + NODE_PARAM_FIELDS lookup
│   ├── imageCompress.ts        # Image compress field configs
│   ├── imageConvert.ts         # Image convert field configs
│   ├── imageResize.ts          # Image resize field configs
│   ├── fileRename.ts           # File rename field configs
│   ├── spreadsheetClean.ts     # Spreadsheet clean field configs
│   ├── spreadsheetRename.ts    # Spreadsheet rename field configs
│   ├── getVisibleParams.ts
│   └── inferFieldType.ts
├── definition.ts               # Definition, Edge, Port, Metadata interfaces
├── recipe.ts                   # Recipe, AcceptSpec interfaces
├── validate.ts                 # Structural validation
└── index.ts                    # Public exports
scripts/
└── generate-from-catalog.ts    # Codegen script
```

## Generated vs Hand-Written

| Source                   | Files                                                                           | How It Gets There                                   |
| ------------------------ | ------------------------------------------------------------------------------- | --------------------------------------------------- |
| **Generated**            | `generated/catalog.ts`, `generated/schemas.ts`, `generated/definitionSchema.ts` | Codegen from `engine/catalog.snapshot.json`         |
| **Hand-written types**   | `definition.ts`, `recipe.ts`, `execution.ts`                                    | Core interfaces shared across TS consumers          |
| **Hand-written recipes** | `recipes/*.ts`                                                                  | Predefined compositions referencing generated types |
| **Hand-written helpers** | `schemas/*.ts`, `validate*.ts`, node CRUD ops                                   | Logic that reads from generated data                |

## Codegen Pipeline

```
Rust engine (NodeProcessor::metadata())
    ↓ (registry self-describes at build time)
engine/catalog.snapshot.json
    ↓ (scripts/generate-from-catalog.ts)
src/generated/*.ts
```

The engine's `NodeRegistry` iterates all registered processors, serializes their metadata, and writes `catalog.snapshot.json`. The codegen script transforms that into TypeScript constants, Zod schemas, and JSON Schema.

## Key Types

- **`Definition`** - recursive tree structure representing a `.bnto.json` recipe. Nodes contain children (for containers), edges, and metadata
- **`Recipe`** - a `Definition` plus display metadata (name, slug, description, accept spec, SEO)
- **`NodeTypeName`** - union of all engine-backed node type strings (e.g. `"image-compress"`, `"spreadsheet-clean"`, `"file-rename"`)
- **`NODE_TYPE_INFO`** - per-type metadata: label, category, isContainer, icon
- **`PROCESSORS`** - flat list of all processor metadata (type + parameters)

## Predefined Recipes

18 recipes ship with bnto, each composing engine nodes into a ready-to-use pipeline. The authoritative recipe definitions live in the engine (`engine/recipes/*.bnto.json`). TypeScript recipe metadata is generated from the engine's catalog snapshot.

Access via `RECIPES` array or `getRecipeBySlug(slug)`. See the root README for the full recipe table.

## Development

```bash
task ui:build       # TypeScript compilation
task ui:test        # Run tests (Vitest)
```

To regenerate from the engine catalog:

```bash
npx tsx packages/@bnto/nodes/scripts/generate-from-catalog.ts
```

## Usage

```tsx
import { RECIPES, getRecipeBySlug, NODE_TYPE_INFO, isContainerNodeType } from "@bnto/nodes";

// Look up a predefined recipe
const recipe = getRecipeBySlug("compress-images");

// Check node type metadata
const info = NODE_TYPE_INFO["image-compress"]; // { label, category, isContainer, icon }

// Type guards
isContainerNodeType("loop"); // true - reads from generated NODE_TYPE_INFO
```
