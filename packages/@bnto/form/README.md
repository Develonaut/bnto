# @bnto/form

Schema-driven form system. Auto-generates UI controls from node schemas.

## What This Package Does

- **SchemaForm:** Renders a complete form from a `NodeSchema`, with field grouping, visibility rules, and control inference from Zod types
- **SchemaField:** Renders a single field with the correct control via registry lookup
- **FieldGroup:** Renders related parameters in a compact layout (inline switches/selects + 2-column grid)
- **Control registry:** Maps `NodeParamControl` types to React components (select, switch, slider, number, text, textarea, tagPicker, keyValue)

## Dependency Chain

```
@bnto/editor -> @bnto/form -> @bnto/core, @bnto/ui
```

`@bnto/form` is a leaf package with no editor dependencies. The editor re-exports `SchemaForm` and `SchemaField` for backwards compatibility.

## Directory Structure

```
src/
├── index.ts                  # Barrel exports
├── SchemaForm.tsx            # Top-level form component
├── SchemaField.tsx           # Single field renderer
├── FieldGroup.tsx            # Grouped field layout
├── buildFormEntries.ts       # Pure: schema + params -> form entries
├── fieldLayout.ts            # Pure: control type -> layout mode
├── partitionGroupFields.ts   # Pure: split fields into inline vs grid
├── controlCategories.ts      # Shared control type sets
└── controls/
    ├── index.ts              # CONTROL_REGISTRY mapping
    ├── types.ts              # ControlProps interface
    ├── SelectControl.tsx
    ├── SwitchControl.tsx
    ├── SliderControl.tsx
    ├── NumberControl.tsx
    ├── TextControl.tsx
    ├── TextareaControl.tsx
    ├── TagPickerControl.tsx
    └── KeyValueEditorControl.tsx
```

## API

```typescript
// Components
import { SchemaForm, SchemaField, FieldGroup } from "@bnto/form";

// Control registry
import { CONTROL_REGISTRY } from "@bnto/form";
import type { ControlProps } from "@bnto/form";

// Pure functions
import { buildFormEntries, getFieldLayout, partitionGroupFields } from "@bnto/form";
```

## Consumers

| Consumer       | Usage                                              |
| -------------- | -------------------------------------------------- |
| `@bnto/editor` | ConfigPanel renders SchemaForm for node parameters |
| `apps/web`     | Motorway playground renders SchemaForm standalone  |

## Development

```bash
pnpm test          # Run tests
pnpm build         # Type-check (tsc --noEmit)
```
