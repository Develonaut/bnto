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
import { BentoGrid, useBentoItem, assignCellLayouts } from "@/components/ui/BentoGrid";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/Card";
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

/* ── Bento cell ──────────────────────────────────────────────── */

function BentoCell({ entry }: { entry: BntoEntry }) {
  const { featured } = useBentoItem();
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
          {featured && (
            <Text size="sm" color="muted" leading="snug" className="text-left">
              {entry.description}
            </Text>
          )}
          <div className="flex flex-wrap gap-1.5 pt-1">
            {entry.features.slice(0, featured ? 5 : 3).map((f) => (
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

function BentoGridDemo({ count, cols }: { count: number; cols: 1 | 2 | 3 }) {
  const items = ALL_ITEMS.slice(0, count);
  const { rows } = assignCellLayouts(items.length, cols);

  return (
    <>
      <BentoGrid key={`${count}-${cols}`} cols={cols}>
        {items.map((entry, i) => (
          <Animate.ScaleIn
            key={entry.slug}
            index={i}
            from={0.6}
            easing="spring-bouncier"
            className="h-full"
            style={{ animationDelay: `${i * 80}ms` }}
          >
            <BentoCell entry={entry} />
          </Animate.ScaleIn>
        ))}
      </BentoGrid>

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

      <BentoGridDemo count={Number(count)} cols={Number(cols) as 1 | 2 | 3} />
    </Stack>
  );
}
