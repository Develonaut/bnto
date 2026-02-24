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

import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/Card";
import { Grid } from "@/components/ui/Grid";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
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

const COLS = 3;

function assignCellLayouts(itemCount: number): { layouts: CellLayout[]; rows: number } {
  if (itemCount === 0) return { layouts: [], rows: 0 };

  // For small counts, use hand-tuned patterns
  if (itemCount <= 6) return buildFromTable(itemCount);

  // Fallback for larger counts: featured 2×2, rest 1×1, pad with 2×1 cards
  const featured = 4; // 2×2 costs 4 cells
  const rest = itemCount - 1;
  const minCells = featured + rest;
  const rows = Math.ceil(minCells / COLS);
  const totalCells = COLS * rows;
  let extra = totalCells - minCells;

  const layouts: CellLayout[] = [{ colSpan: 2, rowSpan: 2, featured: true }];
  for (let i = 1; i < itemCount; i++) {
    if (extra > 0) {
      layouts.push({ colSpan: 2, rowSpan: 1, featured: false });
      extra -= 1;
    } else {
      layouts.push({ colSpan: 1, rowSpan: 1, featured: false });
    }
  }
  return { layouts, rows };
}

function buildFromTable(n: number): { layouts: CellLayout[]; rows: number } {
  // Hand-tuned layouts that actually pack correctly in a 3-col grid.
  // Each pattern verified: sum(colSpan × rowSpan) === COLS × rows.
  const patterns: Record<number, { rows: number; cells: [1|2|3, 1|2, boolean][] }> = {
    //  1 item  → 3×2 hero
    1: { rows: 2, cells: [[3,2,true]] },
    //  2 items → 2×2 featured + 1×2 tall
    //  [FF][B]    cells: 4+2 = 6 = 3×2 ✓
    //  [FF][B]
    2: { rows: 2, cells: [[2,2,true],[1,2,false]] },
    //  3 items → 2×2 featured + 2 × 1×1
    //  [FF][B]    cells: 4+1+1 = 6 = 3×2 ✓
    //  [FF][C]
    3: { rows: 2, cells: [[2,2,true],[1,1,false],[1,1,false]] },
    //  4 items → 2×1 featured + 1×1 + 2 × 1×1
    //  [FF][B]    cells: 2+1+1+2 = 6 = 3×2 ✓
    //  [C][D D]
    4: { rows: 2, cells: [[2,1,true],[1,1,false],[1,1,false],[2,1,false]] },
    //  5 items → 2×2 featured + 1×1 + 1×1 + 2×1 wide
    //  [FF][B]    cells: 4+1+1+2+1 = 9 = 3×3 ✓
    //  [FF][C]
    //  [DD][E]
    5: { rows: 3, cells: [[2,2,true],[1,1,false],[1,1,false],[2,1,false],[1,1,false]] },
    //  6 items → 2×2 featured + 5 × 1×1
    //  [FF][B]    cells: 4+1+1+1+1+1 = 9 = 3×3 ✓
    //  [FF][C]
    //  [D][E][F]
    6: { rows: 3, cells: [[2,2,true],[1,1,false],[1,1,false],[1,1,false],[1,1,false],[1,1,false]] },
  };

  const p = patterns[n];
  return {
    rows: p.rows,
    layouts: p.cells.map(([colSpan, rowSpan, featured]) => ({ colSpan, rowSpan, featured })),
  };
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

export function BentoGridShowcase() {
  const [count, setCount] = useState(ALL_ITEMS.length);
  const items = ALL_ITEMS.slice(0, count);
  const { layouts, rows } = assignCellLayouts(items.length);

  return (
    <Stack gap="md">
      <div className="flex flex-wrap gap-2">
        {Array.from({ length: ALL_ITEMS.length }, (_, i) => i + 1).map((n) => (
          <Button
            key={n}
            variant={n === count ? "default" : "muted"}
            size="sm"
            onClick={() => setCount(n)}
          >
            {n}
          </Button>
        ))}
      </div>

      <Grid
        cols={{ mobile: 1, tablet: 2, desktop: 3 }}
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
            <BentoCell entry={entry} layout={layouts[i]} />
          </Grid.Item>
        ))}
      </Grid>

      <Text size="xs" color="muted" mono className="text-center uppercase tracking-wider">
        {items.length} recipes &middot; {COLS}&times;{rows} grid &middot; zero gaps &middot; dense
        auto-flow
      </Text>
    </Stack>
  );
}
