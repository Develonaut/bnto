"use client";

import { Card, Grid, Text } from "@bnto/ui";
import { PhaseShell } from "./PhaseShell";
import { MockResultCard } from "./MockCards";
import { ActionBar } from "./SharedPanels";
import { StatsCard } from "./StatsCard";

/* -- Phase 3 chart panels -- */

function SizeComparisonPanel() {
  return (
    <Grid.Item colSpan={3} rowSpan={2} colStart={4} rowStart={1}>
      <Card className="flex h-full flex-col p-4" elevation="sm">
        <Text size="xs" color="muted" weight="semibold" className="mb-3 uppercase tracking-wider">
          Size Comparison
        </Text>
        <div className="flex flex-1 items-end gap-3">
          <div className="flex flex-1 flex-col gap-2">
            {[
              { name: "hero-banner.jpg", originalW: "100%", compressedW: "26%" },
              { name: "product-shot.png", originalW: "75%", compressedW: "20%" },
            ].map(({ name, originalW, compressedW }) => (
              <div key={name}>
                <Text size="xs" color="muted">{name}</Text>
                <div className="mt-1 h-4 rounded-sm bg-muted-foreground/20" style={{ width: originalW }} />
                <div className="mt-0.5 h-4 rounded-sm bg-primary" style={{ width: compressedW }} />
              </div>
            ))}
          </div>
        </div>
        <div className="mt-2 flex gap-4">
          <div className="flex items-center gap-1.5">
            <div className="size-2 rounded-full bg-muted-foreground/20" />
            <Text size="xs" color="muted">Original</Text>
          </div>
          <div className="flex items-center gap-1.5">
            <div className="size-2 rounded-full bg-primary" />
            <Text size="xs" color="muted">Compressed</Text>
          </div>
        </div>
      </Card>
    </Grid.Item>
  );
}

function SavingsDonutTile() {
  return (
    <Grid.Item colSpan={1} rowSpan={1} colStart={7} rowStart={2}>
      <Card className="flex h-full flex-col items-center justify-center p-4" elevation="sm">
        <div className="relative flex size-16 items-center justify-center">
          <svg viewBox="0 0 36 36" className="size-16 -rotate-90">
            <circle cx="18" cy="18" r="14" fill="none" strokeWidth="4" className="stroke-muted" />
            <circle
              cx="18" cy="18" r="14" fill="none" strokeWidth="4"
              strokeDasharray="88 88" strokeDashoffset="23"
              strokeLinecap="round" className="stroke-primary"
            />
          </svg>
          <span className="absolute text-sm font-bold tabular-nums">74%</span>
        </div>
        <Text size="xs" color="muted" className="mt-2 text-center leading-tight">
          saved
        </Text>
      </Card>
    </Grid.Item>
  );
}

/* -- Phase 3 -- */

export function Phase3() {
  return (
    <PhaseShell
      phase={3}
      controlPanel={
        <>
          <SizeComparisonPanel />
          <SavingsDonutTile />
        </>
      }
      actionBar={<ActionBar trashDisabled runDisabled />}
      statsCard={
        <StatsCard
          title="Compression complete"
          stats={[
            { value: "2", label: "files" },
            { value: "4.2", label: "MB in" },
            { value: "1.1", label: "MB out" },
          ]}
          highlightIndex={2}
        />
      }
      leftBottomCard={
        <Card className="h-full space-y-2 p-4" elevation="sm">
          <Text size="sm" weight="semibold">
            Savings
          </Text>
          <div className="grid grid-cols-2 gap-3 text-center">
            <div>
              <p className="text-lg font-bold tabular-nums text-primary">74%</p>
              <p className="text-xs text-muted-foreground">reduced</p>
            </div>
            <div>
              <p className="text-lg font-bold tabular-nums">3.1</p>
              <p className="text-xs text-muted-foreground">MB saved</p>
            </div>
          </div>
        </Card>
      }
      mainContent={
        <div className="flex h-full flex-col gap-2">
          <MockResultCard
            name="hero-banner.jpg"
            original="2.4 MB"
            compressed="620 KB"
            savings="-74%"
          />
          <MockResultCard
            name="product-shot.png"
            original="1.8 MB"
            compressed="480 KB"
            savings="-73%"
          />
        </div>
      }
    />
  );
}
