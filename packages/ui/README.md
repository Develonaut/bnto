# @bnto/ui

Motorway — the bnto design system.

## Overview

`@bnto/ui` provides all shared UI components for bnto applications. Built on Radix UI primitives with Tailwind CSS, themed with the Motorway design language (warm palette, generous radius, Mini Motorways-inspired motion). Components are organized by function, not by abstraction level.

Consumed by `apps/web/` and `@bnto/editor`.

## Directory Structure

```
src/
├── animation/        # Motion components (ScaleIn, FadeIn, SlideUp, Stagger, Presence)
├── blocks/           # Composite business components (RecipeCard)
├── feedback/         # Status display (Skeleton, LinearProgress, EmptyState, StatusBanner)
├── hooks/            # UI hooks (useDialog, useKeyDown, usePrevious)
├── icons/            # Lucide icon re-exports
├── interaction/      # Controls (Button, Input, Select, Slider, Checkbox, Tabs, FileUpload, etc.)
├── layout/           # Structure (AppShell, Container, Grid, Row, Stack, List, BentoGrid)
├── overlay/          # Floating UI (Dialog, Popover, Sheet)
├── surface/          # Containers (Card, Panel, Surface, Toolbar, Divider, Pressable)
├── typography/       # Text (Heading, Text, Label, Badge, Kbd, IconBadge)
└── utils/
    ├── cn.ts         # Tailwind class merge (clsx + tailwind-merge)
    ├── createCn.ts   # Variant resolver (wraps tailwind-variants)
    └── ...           # formatFileSize, responsive helpers, layout types
```

## Key Concepts

- **`createCn()`** — variant-to-class resolver wrapping `tailwind-variants`. Use instead of raw `tv()` so the underlying library can be swapped
- **`cn()`** — static class merge (clsx + tailwind-merge). For non-variant class composition
- **Flat named exports** — all compound components use prefixed exports (`CardHeader`, `DialogContent`), never `Object.assign` dot-notation
- **Animation components** — `ScaleIn`, `FadeIn`, `SlideUp`, `Stagger` etc. are the public API for motion. Never apply `motion-safe:animate-*` classes directly

## Component Categories

| Category    | Count | Examples                                                                           |
| ----------- | ----- | ---------------------------------------------------------------------------------- |
| Interaction | 20+   | Button, Input, Select, Slider, Checkbox, Combobox, FileUpload, Tabs                |
| Layout      | 10    | AppShell, Container, Grid, Row, Stack, List, BentoGrid, Center, Inset              |
| Typography  | 6     | Heading, Text, Label, Badge, Kbd, IconBadge                                        |
| Surface     | 7     | Card, Panel, Surface, Toolbar, Divider, Pressable                                  |
| Feedback    | 6     | Skeleton, LinearProgress, ComparisonBar, EmptyState, ResultFileCard, StatusBanner  |
| Overlay     | 3     | Dialog, Popover, Sheet                                                             |
| Animation   | 13    | ScaleIn, FadeIn, SlideUp, SlideDown, Stagger, PulseSoft, Breathe, Presence, InView |
| Blocks      | 1     | RecipeCard (compound)                                                              |

## Development

```bash
task ui:build       # TypeScript compilation
task ui:test        # Run tests (Vitest)
task ui:lint        # Lint
```

## Usage

```tsx
import { Button, Card, CardHeader, Heading, Stack, Row } from "@bnto/ui";
import { ScaleIn, Stagger } from "@bnto/ui";
import { cn, createCn } from "@bnto/ui";

<Stagger className="grid grid-cols-3 gap-4">
  {items.map((item, i) => (
    <ScaleIn key={item.id} index={i}>
      <Card>
        <CardHeader>
          <Heading level={3}>{item.name}</Heading>
        </CardHeader>
      </Card>
    </ScaleIn>
  ))}
</Stagger>;
```
