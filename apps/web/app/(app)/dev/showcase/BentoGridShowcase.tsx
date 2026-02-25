"use client";

import { useState } from "react";
import {
  ArrowRightLeft,
  Columns3,
  type LucideIcon,
  Minimize2,
  PenLine,
  Scaling,
  Sparkles,
} from "lucide-react";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/Card";
import { Grid } from "@/components/ui/Grid";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Tabs } from "@/components/ui/tabs";
import { Text } from "@/components/ui/Text";
import { BNTO_REGISTRY, type BntoEntry } from "@/lib/bntoRegistry";

/* ── Icon map ────────────────────────────────────────────────── */

const BNTO_ICONS: Record<string, LucideIcon> = {
  "compress-images": Minimize2,
  "resize-images": Scaling,
  "convert-image-format": ArrowRightLeft,
  "rename-files": PenLine,
  "clean-csv": Sparkles,
  "rename-csv-columns": Columns3,
};

/* ── Cell-budget algorithm ───────────────────────────────────── */

interface CellLayout {
  colSpan: 1 | 2 | 3;
  rowSpan: 1 | 2;
  featured: boolean;
}

/**
 * Sidebar-aware layout algorithm. The grid has two zones:
 *
 *  1. **Top zone** — featured card (2×2) + sidebar column(s) to its right
 *  2. **Bottom zone** — full-width rows below the featured card
 *
 * Items in the sidebar must be 1-col wide (they only have 1 column).
 * Items in the bottom zone are widened evenly to fill complete rows.
 */
function assignCellLayouts(
  itemCount: number,
  cols: number,
): { layouts: CellLayout[]; rows: number } {
  if (itemCount === 0) return { layouts: [], rows: 0 };

  // Single item → full-width hero
  if (itemCount === 1) {
    const c = Math.min(cols, 3) as 1 | 2 | 3;
    return { layouts: [{ colSpan: c, rowSpan: 2, featured: true }], rows: 2 };
  }

  // For 1-col grid, everything stacks
  if (cols === 1) {
    return {
      rows: itemCount,
      layouts: Array.from({ length: itemCount }, (_, i) => ({
        colSpan: 1 as const,
        rowSpan: 1 as const,
        featured: i === 0,
      })),
    };
  }

  // Featured card: span min(2, cols) columns × 2 rows
  const featColSpan = Math.min(2, cols) as 1 | 2 | 3;
  const featRowSpan = 2;

  // Sidebar: columns to the right of featured (0 for 2-col, 1 for 3-col)
  const sidebarWidth = cols - featColSpan;
  const sidebarSlots = sidebarWidth * featRowSpan;
  const rest = itemCount - 1;
  const sidebarCount = Math.min(rest, sidebarSlots);
  const bottomCount = rest - sidebarCount;

  const layouts: CellLayout[] = [
    { colSpan: featColSpan, rowSpan: featRowSpan, featured: true },
  ];

  // Sidebar items: 1-col wide. If only 1 item with 2 vertical
  // slots, stretch it tall (1×2) so the sidebar isn't half-empty.
  if (sidebarCount === 1 && sidebarSlots >= 2) {
    layouts.push({ colSpan: 1, rowSpan: 2, featured: false });
  } else {
    for (let i = 0; i < sidebarCount; i++) {
      layouts.push({ colSpan: 1, rowSpan: 1, featured: false });
    }
  }

  // Bottom items: fill complete rows with even span distribution
  if (bottomCount > 0) {
    const bottomRows = Math.ceil(bottomCount / cols);
    const totalCells = bottomRows * cols;
    const baseSpan = Math.floor(totalCells / bottomCount);
    const wider = totalCells % bottomCount;

    for (let i = 0; i < bottomCount; i++) {
      const span = Math.min(i < wider ? baseSpan + 1 : baseSpan, 3) as 1 | 2 | 3;
      layouts.push({ colSpan: span, rowSpan: 1, featured: false });
    }
  }

  const rows = featRowSpan + (bottomCount > 0 ? Math.ceil(bottomCount / cols) : 0);
  return { layouts, rows };
}

/* ── Bento cell ──────────────────────────────────────────────── */

function BentoCell({
  entry,
  layout,
}: {
  entry: BntoEntry;
  layout: CellLayout;
}) {
  const Icon = BNTO_ICONS[entry.slug] ?? Sparkles;

  return (
    <Button
      variant="outline"
      asChild
      className="flex h-full items-stretch justify-start whitespace-normal text-left"
    >
      <Card className="flex flex-col justify-between p-5">
        <div className="flex items-start justify-between">
          <div className="bg-primary/10 text-primary flex size-10 items-center justify-center rounded-lg">
            <Icon className="size-5" />
          </div>
          <Text
            as="span"
            size="xs"
            mono
            color="muted"
            className="uppercase tracking-wider"
          >
            {entry.features[0]}
          </Text>
        </div>

        <div className="mt-auto flex flex-col gap-1.5 pt-4">
          <Heading level={3} size="xs" className="text-left">
            {entry.h1.replace(/ Online Free$/, "")}
          </Heading>
          {layout.featured && (
            <Text size="sm" color="muted" leading="snug" className="text-left">
              {entry.description}
            </Text>
          )}
          <div className="flex flex-wrap gap-1.5 pt-1">
            {entry.features.slice(0, layout.featured ? 5 : 3).map((f) => (
              <span
                key={f}
                className="bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs font-medium"
              >
                {f}
              </span>
            ))}
          </div>
        </div>
      </Card>
    </Button>
  );
}

/* ── Main showcase ───────────────────────────────────────────── */

const ALL_ITEMS = [...BNTO_REGISTRY];
const COUNTS = Array.from({ length: ALL_ITEMS.length }, (_, i) => i + 1);

function BentoGrid({ count, cols }: { count: number; cols: 1 | 2 | 3 }) {
  const items = ALL_ITEMS.slice(0, count);
  const { layouts, rows } = assignCellLayouts(items.length, cols);

  const colsResponsive =
    cols === 1 ? 1 as const
    : cols === 2 ? { mobile: 1 as const, tablet: 2 as const }
    : { mobile: 1 as const, tablet: 2 as const, desktop: 3 as const };

  return (
    <>
      <Grid
        key={`${count}-${cols}-${rows}`}
        cols={colsResponsive}
        gap="md"
        flow="dense"
        rows={`repeat(${rows}, minmax(10rem, auto))`}
      >
        {items.map((entry, i) => (
          <Grid.Item
            key={entry.slug}
            colSpan={layouts[i].colSpan}
            rowSpan={layouts[i].rowSpan}
            className="min-h-0"
          >
            <Animate.ScaleIn
              index={i}
              from={0.6}
              easing="spring-bouncier"
              className="h-full"
              style={{ animationDelay: `${i * 80}ms` }}
            >
              <BentoCell entry={entry} layout={layouts[i]} />
            </Animate.ScaleIn>
          </Grid.Item>
        ))}
      </Grid>

      <Text size="xs" color="muted" mono className="mt-8 text-center uppercase tracking-wider">
        {items.length} recipes &middot; {cols}&times;{rows} grid &middot; zero gaps &middot; dense
        auto-flow
      </Text>
    </>
  );
}

export function BentoGridShowcase() {
  const [count, setCount] = useState(String(ALL_ITEMS.length));
  const [cols, setCols] = useState("3");

  return (
    <Stack gap="md">
      <div className="flex flex-wrap items-center gap-4">
        <div className="flex items-center gap-2">
          <Text size="xs" mono color="muted" className="uppercase tracking-wider">Items</Text>
          <Tabs value={count} onValueChange={setCount}>
            <Tabs.List>
              {COUNTS.map((n) => (
                <Tabs.Trigger key={n} value={String(n)}>
                  {n}
                </Tabs.Trigger>
              ))}
            </Tabs.List>
          </Tabs>
        </div>

        <div className="flex items-center gap-2">
          <Text size="xs" mono color="muted" className="uppercase tracking-wider">Cols</Text>
          <Tabs value={cols} onValueChange={setCols}>
            <Tabs.List>
              {[1, 2, 3].map((n) => (
                <Tabs.Trigger key={n} value={String(n)}>
                  {n}
                </Tabs.Trigger>
              ))}
            </Tabs.List>
          </Tabs>
        </div>

      </div>

      <BentoGrid count={Number(count)} cols={Number(cols) as 1 | 2 | 3} />
    </Stack>
  );
}
