"use client";

import { useState } from "react";

import { Card } from "@/components/ui/Card";
import { Grid } from "@/components/ui/Grid";
import { Stack } from "@/components/ui/Stack";
import { Tabs } from "@/components/ui/Tabs";
import { Text } from "@/components/ui/Text";

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
    <Card elevation="sm" className={`${TINTS[(n - 1) % TINTS.length]} flex h-full flex-col items-center justify-center gap-1 p-4`}>
      <Text size="lg" className="font-display font-bold tabular-nums">{n}</Text>
      {label && <Text size="xs" color="muted" className="text-center">{label}</Text>}
    </Card>
  );
}

/* ── Layouts ─────────────────────────────────────────────────── */

type LayoutKey = "hero" | "dashboard" | "mosaic" | "magazine";

const LAYOUTS: { key: LayoutKey; label: string; description: string }[] = [
  { key: "hero", label: "Hero", description: "6\u00d74 \u00b7 featured card + sidebar + footer" },
  { key: "dashboard", label: "Dashboard", description: "6\u00d75 \u00b7 stat cards + main + sidebar" },
  { key: "mosaic", label: "Mosaic", description: "6\u00d76 \u00b7 varied sizes, explicit placement" },
  { key: "magazine", label: "Magazine", description: "6\u00d76 \u00b7 hero image + headline + articles" },
];

function HeroLayout() {
  return (
    <Grid cols={6} rows={4} gap="md" animated>
      <Grid.Item colSpan={4} rowSpan={3}><GridCell n={1} label="Hero" /></Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} colStart={5}><GridCell n={2} label="Sidebar Top" /></Grid.Item>
      <Grid.Item colSpan={2} colStart={5} rowStart={3}><GridCell n={3} label="Sidebar Bottom" /></Grid.Item>
      <Grid.Item colSpan={3} rowStart={4}><GridCell n={4} label="Footer Left" /></Grid.Item>
      <Grid.Item colSpan={3} colStart={4} rowStart={4}><GridCell n={5} label="Footer Right" /></Grid.Item>
    </Grid>
  );
}

function DashboardLayout() {
  return (
    <Grid cols={6} rows={5} gap="md" animated>
      <Grid.Item colSpan={2}><GridCell n={1} label="Stat A" /></Grid.Item>
      <Grid.Item colSpan={2}><GridCell n={2} label="Stat B" /></Grid.Item>
      <Grid.Item colSpan={2}><GridCell n={3} label="Stat C" /></Grid.Item>
      <Grid.Item colSpan={4} rowSpan={3} rowStart={2}><GridCell n={4} label="Main Content" /></Grid.Item>
      <Grid.Item colSpan={2} rowSpan={3} colStart={5} rowStart={2}><GridCell n={5} label="Sidebar" /></Grid.Item>
      <Grid.Item colSpan={6} rowStart={5}><GridCell n={6} label="Footer" /></Grid.Item>
    </Grid>
  );
}

function MosaicLayout() {
  return (
    <Grid cols={6} rows={6} gap="md" animated>
      <Grid.Item colSpan={3} rowSpan={3}><GridCell n={1} label="Large" /></Grid.Item>
      <Grid.Item colSpan={3} rowSpan={2} colStart={4}><GridCell n={2} label="Wide" /></Grid.Item>
      <Grid.Item colSpan={1} rowSpan={4} colStart={4} rowStart={3}><GridCell n={3} label="Tall" /></Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} colStart={5} rowStart={3}><GridCell n={4} label="Medium" /></Grid.Item>
      <Grid.Item colSpan={3} rowSpan={3} rowStart={4}><GridCell n={5} label="Large" /></Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} colStart={5} rowStart={5}><GridCell n={6} label="Medium" /></Grid.Item>
    </Grid>
  );
}

function MagazineLayout() {
  return (
    <Grid cols={6} rows={6} gap="md" animated>
      <Grid.Item colSpan={3} rowSpan={4}><GridCell n={1} label="Hero Image" /></Grid.Item>
      <Grid.Item colSpan={3} rowSpan={2} colStart={4}><GridCell n={2} label="Headline" /></Grid.Item>
      <Grid.Item colSpan={3} rowSpan={2} colStart={4} rowStart={3}><GridCell n={3} label="Pull Quote" /></Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} rowStart={5}><GridCell n={4} label="Article 1" /></Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} colStart={3} rowStart={5}><GridCell n={5} label="Article 2" /></Grid.Item>
      <Grid.Item colSpan={2} rowSpan={2} colStart={5} rowStart={5}><GridCell n={6} label="Article 3" /></Grid.Item>
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
        <Tabs.List>
          {LAYOUTS.map((l) => (
            <Tabs.Trigger key={l.key} value={l.key}>
              {l.label}
            </Tabs.Trigger>
          ))}
        </Tabs.List>
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
