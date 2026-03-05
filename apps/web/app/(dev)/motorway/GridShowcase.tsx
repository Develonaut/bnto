"use client";

import { useState } from "react";

import { Card, Grid, GridItem, Stack, Tabs, TabsList, TabsTrigger, Text } from "@bnto/ui";

/* ── Cell helper ─────────────────────────────────────────────── */

const TINTS = [
  "bg-chart-1/10",
  "bg-chart-2/10",
  "bg-chart-3/10",
  "bg-chart-4/10",
  "bg-chart-5/10",
] as const;

function GridCell({ n, label }: { n: number; label?: string }) {
  return (
    <Card
      elevation="sm"
      className={`${TINTS[(n - 1) % TINTS.length]} flex h-full flex-col items-center justify-center gap-1 p-4`}
    >
      <Text size="lg" className="font-display font-bold tabular-nums">
        {n}
      </Text>
      {label && (
        <Text size="xs" color="muted" className="text-center">
          {label}
        </Text>
      )}
    </Card>
  );
}

/* ── Layouts ─────────────────────────────────────────────────── */

type LayoutKey = "hero" | "dashboard" | "mosaic" | "magazine";

const LAYOUTS: { key: LayoutKey; label: string; description: string }[] = [
  { key: "hero", label: "Hero", description: "6\u00d74 \u00b7 featured card + sidebar + footer" },
  {
    key: "dashboard",
    label: "Dashboard",
    description: "6\u00d75 \u00b7 stat cards + main + sidebar",
  },
  {
    key: "mosaic",
    label: "Mosaic",
    description: "6\u00d76 \u00b7 varied sizes, explicit placement",
  },
  {
    key: "magazine",
    label: "Magazine",
    description: "6\u00d76 \u00b7 hero image + headline + articles",
  },
];

function HeroLayout() {
  return (
    <Grid cols={6} rows={4} gap="md" animated>
      <GridItem colSpan={4} rowSpan={3}>
        <GridCell n={1} label="Hero" />
      </GridItem>
      <GridItem colSpan={2} rowSpan={2} colStart={5}>
        <GridCell n={2} label="Sidebar Top" />
      </GridItem>
      <GridItem colSpan={2} colStart={5} rowStart={3}>
        <GridCell n={3} label="Sidebar Bottom" />
      </GridItem>
      <GridItem colSpan={3} rowStart={4}>
        <GridCell n={4} label="Footer Left" />
      </GridItem>
      <GridItem colSpan={3} colStart={4} rowStart={4}>
        <GridCell n={5} label="Footer Right" />
      </GridItem>
    </Grid>
  );
}

function DashboardLayout() {
  return (
    <Grid cols={6} rows={5} gap="md" animated>
      <GridItem colSpan={2}>
        <GridCell n={1} label="Stat A" />
      </GridItem>
      <GridItem colSpan={2}>
        <GridCell n={2} label="Stat B" />
      </GridItem>
      <GridItem colSpan={2}>
        <GridCell n={3} label="Stat C" />
      </GridItem>
      <GridItem colSpan={4} rowSpan={3} rowStart={2}>
        <GridCell n={4} label="Main Content" />
      </GridItem>
      <GridItem colSpan={2} rowSpan={3} colStart={5} rowStart={2}>
        <GridCell n={5} label="Sidebar" />
      </GridItem>
      <GridItem colSpan={6} rowStart={5}>
        <GridCell n={6} label="Footer" />
      </GridItem>
    </Grid>
  );
}

function MosaicLayout() {
  return (
    <Grid cols={6} rows={6} gap="md" animated>
      <GridItem colSpan={3} rowSpan={3}>
        <GridCell n={1} label="Large" />
      </GridItem>
      <GridItem colSpan={3} rowSpan={2} colStart={4}>
        <GridCell n={2} label="Wide" />
      </GridItem>
      <GridItem colSpan={1} rowSpan={4} colStart={4} rowStart={3}>
        <GridCell n={3} label="Tall" />
      </GridItem>
      <GridItem colSpan={2} rowSpan={2} colStart={5} rowStart={3}>
        <GridCell n={4} label="Medium" />
      </GridItem>
      <GridItem colSpan={3} rowSpan={3} rowStart={4}>
        <GridCell n={5} label="Large" />
      </GridItem>
      <GridItem colSpan={2} rowSpan={2} colStart={5} rowStart={5}>
        <GridCell n={6} label="Medium" />
      </GridItem>
    </Grid>
  );
}

function MagazineLayout() {
  return (
    <Grid cols={6} rows={6} gap="md" animated>
      <GridItem colSpan={3} rowSpan={4}>
        <GridCell n={1} label="Hero Image" />
      </GridItem>
      <GridItem colSpan={3} rowSpan={2} colStart={4}>
        <GridCell n={2} label="Headline" />
      </GridItem>
      <GridItem colSpan={3} rowSpan={2} colStart={4} rowStart={3}>
        <GridCell n={3} label="Pull Quote" />
      </GridItem>
      <GridItem colSpan={2} rowSpan={2} rowStart={5}>
        <GridCell n={4} label="Article 1" />
      </GridItem>
      <GridItem colSpan={2} rowSpan={2} colStart={3} rowStart={5}>
        <GridCell n={5} label="Article 2" />
      </GridItem>
      <GridItem colSpan={2} rowSpan={2} colStart={5} rowStart={5}>
        <GridCell n={6} label="Article 3" />
      </GridItem>
    </Grid>
  );
}

const LAYOUT_COMPONENTS = {
  hero: HeroLayout,
  dashboard: DashboardLayout,
  mosaic: MosaicLayout,
  magazine: MagazineLayout,
};

/* ── Showcase ────────────────────────────────────────────────── */

export function GridShowcase() {
  const [layout, setLayout] = useState<string>("hero");
  const current = LAYOUTS.find((l) => l.key === layout) ?? LAYOUTS[0];
  const LayoutComponent = LAYOUT_COMPONENTS[current.key];

  return (
    <Stack gap="md">
      <Tabs value={layout} onValueChange={setLayout}>
        <TabsList>
          {LAYOUTS.map((l) => (
            <TabsTrigger key={l.key} value={l.key}>
              {l.label}
            </TabsTrigger>
          ))}
        </TabsList>
      </Tabs>

      <div key={layout}>
        <LayoutComponent />
      </div>

      <Text size="xs" color="muted" mono className="text-center uppercase tracking-wider">
        {current.description}
      </Text>
    </Stack>
  );
}
