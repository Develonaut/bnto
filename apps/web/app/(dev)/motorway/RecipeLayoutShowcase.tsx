"use client";

import { useState, type ReactNode } from "react";

import { Button, Card, Grid, Heading, RadialSlider, Row, Stack, Switch, Text } from "@bnto/ui";
import {
  CheckCircle2Icon,
  DownloadIcon,
  FileIcon,
  ImageIcon,
  PlayIcon,
  TrashIcon,
  UploadIcon,
} from "@bnto/ui";

import { PhaseIndicator } from "@/app/(app)/[bnto]/_components/PhaseIndicator";

import { ShowcaseSection } from "./ShowcaseSection";

/* ── Mock content ────────────────────────────────────────────── */

const MOCK_H1 = "Compress Images Online Free";
const MOCK_DESCRIPTION =
  "Compress PNG, JPEG, and WebP images in your browser. Files never leave your machine.";

/** Simulated file card for the mockups. */
function MockFileCard({ name, size }: { name: string; size: string }) {
  return (
    <Card
      className="flex items-center gap-3 rounded-lg px-4 py-3"
      elevation="sm"
    >
      <div className="flex size-9 shrink-0 items-center justify-center rounded-md bg-muted">
        <ImageIcon className="size-4 text-muted-foreground" />
      </div>
      <div className="min-w-0 flex-1 text-left">
        <p className="truncate text-sm font-medium">{name}</p>
        <p className="text-xs text-muted-foreground">{size}</p>
      </div>
    </Card>
  );
}

/** Simulated result file card with download button. */
function MockResultCard({
  name,
  original,
  compressed,
  savings,
}: {
  name: string;
  original: string;
  compressed: string;
  savings: string;
}) {
  return (
    <Card
      className="flex items-center gap-3 rounded-lg px-4 py-3"
      elevation="sm"
    >
      <div className="flex size-9 shrink-0 items-center justify-center rounded-md bg-muted">
        <CheckCircle2Icon className="size-4 text-primary" />
      </div>
      <div className="min-w-0 flex-1 text-left">
        <p className="truncate text-sm font-medium">{name}</p>
        <p className="text-xs text-muted-foreground">
          {original} → {compressed}{" "}
          <span className="font-medium text-primary">({savings})</span>
        </p>
      </div>
      <Button variant="outline" size="icon" elevation="sm">
        <DownloadIcon className="size-4" />
      </Button>
    </Card>
  );
}

/* ── Shared grid skeleton ────────────────────────────────────── */

/**
 * Shared 9×5 grid layout for all recipe phases.
 * Heading, phase indicator are always the same.
 * Slots let each phase inject its own controls, stats, and content.
 */
function PhaseShell({
  phase,
  controlPanel,
  actionBar,
  statsCard,
  leftBottomCard,
  mainContent,
}: {
  phase: 1 | 2 | 3;
  controlPanel: ReactNode;
  actionBar: ReactNode;
  statsCard: ReactNode;
  leftBottomCard: ReactNode;
  mainContent: ReactNode;
}) {
  return (
    <Grid cols={9} rows={5} gap="md">
      {/* Heading/Description — always cols 1-3, rows 1-2 */}
      <Grid.Item colSpan={3} rowSpan={2} colStart={1} rowStart={1}>
        <div className="flex h-full flex-col justify-start">
          <Heading level={2} size="lg">
            {MOCK_H1}
          </Heading>
          <Text color="muted" className="mt-3 leading-snug text-balance">
            {MOCK_DESCRIPTION}
          </Text>
        </div>
      </Grid.Item>

      {/* Control panels — cols 4-7, rows 1-2 (phase-specific) */}
      {controlPanel}

      {/* Phase indicator — cols 7-9, row 1 */}
      <Grid.Item colSpan={3} rowSpan={1} colStart={7} rowStart={1}>
        <Card
          className="flex h-full items-center justify-center p-4"
          elevation="sm"
        >
          <PhaseIndicator activePhase={phase} />
        </Card>
      </Grid.Item>

      {/* Action bar — cols 8-9, row 2 */}
      <Grid.Item colSpan={2} rowSpan={1} colStart={8} rowStart={2}>
        {actionBar}
      </Grid.Item>

      {/* Stats card — cols 1-3, row 3 */}
      <Grid.Item colSpan={3} rowSpan={1} colStart={1} rowStart={3}>
        {statsCard}
      </Grid.Item>

      {/* Left bottom card — cols 1-3, row 4 */}
      <Grid.Item colSpan={3} rowSpan={1} colStart={1} rowStart={4}>
        {leftBottomCard}
      </Grid.Item>

      {/* Main content — cols 4-9, rows 3-5 */}
      <Grid.Item colSpan={6} rowSpan={3} colStart={4} rowStart={3}>
        {mainContent}
      </Grid.Item>
    </Grid>
  );
}

/* ── Reusable panels ─────────────────────────────────────────── */

function QualitySliderPanel({
  value,
  onChange,
}: {
  value: number;
  onChange: (v: number) => void;
}) {
  return (
    <Grid.Item colSpan={3} rowSpan={2} colStart={4} rowStart={1}>
      <Card
        className="flex h-full items-center justify-center p-4"
        elevation="sm"
      >
        <RadialSlider
          min={1}
          max={100}
          value={value}
          onChange={onChange}
          startAngle={135}
          endAngle={405}
          size={120}
          aria-label="Quality"
        >
          <div className="flex flex-col items-center">
            <span className="text-2xl font-bold tabular-nums">{value}</span>
            <span className="text-xs text-muted-foreground">Quality</span>
          </div>
        </RadialSlider>
      </Card>
    </Grid.Item>
  );
}

function AutoDownloadTile({ defaultChecked = false }: { defaultChecked?: boolean }) {
  return (
    <Grid.Item colSpan={1} rowSpan={1} colStart={7} rowStart={2}>
      <Card
        className="flex h-full flex-col items-center justify-center gap-2 p-4"
        elevation="sm"
      >
        <Switch defaultChecked={defaultChecked} />
        <Text size="xs" color="muted" className="text-center leading-tight">
          Auto
          <br />
          download
        </Text>
      </Card>
    </Grid.Item>
  );
}

function AcceptsCard() {
  return (
    <Card className="h-full space-y-2 p-4" elevation="sm">
      <Text size="sm" weight="semibold">
        Accepts
      </Text>
      <Row gap="sm" wrap>
        {["JPEG", "PNG", "WebP"].map((fmt) => (
          <span
            key={fmt}
            className="inline-flex items-center gap-1 rounded-full bg-muted px-2.5 py-0.5 text-xs font-medium text-muted-foreground"
          >
            <FileIcon className="size-3" />
            {fmt}
          </span>
        ))}
      </Row>
    </Card>
  );
}

function ActionBar({
  trashDisabled = false,
  runDisabled = false,
  downloadDisabled = false,
}: {
  trashDisabled?: boolean;
  runDisabled?: boolean;
  downloadDisabled?: boolean;
}) {
  return (
    <Row gap="sm" justify="end" align="end" className="h-full">
      <Button variant="outline" size="icon" elevation="md" disabled={trashDisabled}>
        <TrashIcon className="size-4" />
      </Button>
      <Button variant="primary" disabled={runDisabled}>
        <PlayIcon className="size-4" />
        Run
      </Button>
      <Button variant="outline" size="icon" elevation="md" disabled={downloadDisabled}>
        <DownloadIcon className="size-4" />
      </Button>
    </Row>
  );
}

/* ── Phase 1: Files (dropzone) ───────────────────────────────── */

function Phase1() {
  const [quality, setQuality] = useState(80);

  return (
    <PhaseShell
      phase={1}
      controlPanel={
        <>
          <QualitySliderPanel value={quality} onChange={setQuality} />
          <AutoDownloadTile />
        </>
      }
      actionBar={<ActionBar trashDisabled runDisabled downloadDisabled />}
      statsCard={
        <Card className="h-full space-y-3 p-4" elevation="sm">
          <Text size="sm" weight="semibold">
            Waiting for files
          </Text>
          <div className="grid grid-cols-3 gap-3 text-center">
            <div>
              <p className="text-lg font-bold tabular-nums text-muted-foreground">—</p>
              <p className="text-xs text-muted-foreground">files</p>
            </div>
            <div>
              <p className="text-lg font-bold tabular-nums text-muted-foreground">—</p>
              <p className="text-xs text-muted-foreground">MB in</p>
            </div>
            <div>
              <p className="text-lg font-bold tabular-nums text-muted-foreground">—</p>
              <p className="text-xs text-muted-foreground">MB est.</p>
            </div>
          </div>
        </Card>
      }
      leftBottomCard={<AcceptsCard />}
      mainContent={
        <Card
          className="flex h-full flex-col items-center justify-center gap-3 border-2 border-dashed border-border p-8"
          elevation="sm"
        >
          <div className="rounded-full bg-muted p-3 text-muted-foreground">
            <UploadIcon className="size-6" />
          </div>
          <div className="text-center">
            <p className="text-sm font-medium text-foreground">
              Drag &amp; drop files here
            </p>
            <p className="mt-1 text-xs text-muted-foreground">
              or click to browse &middot; accepts JPEG, PNG, WebP
            </p>
          </div>
        </Card>
      }
    />
  );
}

/* ── Phase 2: Configure ──────────────────────────────────────── */

function Phase2() {
  const [quality, setQuality] = useState(80);

  return (
    <PhaseShell
      phase={2}
      controlPanel={
        <>
          <QualitySliderPanel value={quality} onChange={setQuality} />
          <AutoDownloadTile />
        </>
      }
      actionBar={<ActionBar downloadDisabled />}
      statsCard={
        <Card className="h-full space-y-3 p-4" elevation="sm">
          <Text size="sm" weight="semibold">
            Ready to compress
          </Text>
          <div className="grid grid-cols-3 gap-3 text-center">
            <div>
              <p className="text-lg font-bold tabular-nums">2</p>
              <p className="text-xs text-muted-foreground">files</p>
            </div>
            <div>
              <p className="text-lg font-bold tabular-nums">4.2</p>
              <p className="text-xs text-muted-foreground">MB in</p>
            </div>
            <div>
              <p className="text-lg font-bold tabular-nums">~1.1</p>
              <p className="text-xs text-muted-foreground">MB est.</p>
            </div>
          </div>
        </Card>
      }
      leftBottomCard={<AcceptsCard />}
      mainContent={
        <div className="flex h-full flex-col gap-2">
          <MockFileCard name="hero-banner.jpg" size="2.4 MB" />
          <MockFileCard name="product-shot.png" size="1.8 MB" />
        </div>
      }
    />
  );
}

/* ── Phase 3: Results chart panels ────────────────────────────── */

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

function Phase3() {
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
        <Card className="h-full space-y-3 p-4" elevation="sm">
          <Text size="sm" weight="semibold">
            Compression complete
          </Text>
          <div className="grid grid-cols-3 gap-3 text-center">
            <div>
              <p className="text-lg font-bold tabular-nums">2</p>
              <p className="text-xs text-muted-foreground">files</p>
            </div>
            <div>
              <p className="text-lg font-bold tabular-nums">4.2</p>
              <p className="text-xs text-muted-foreground">MB in</p>
            </div>
            <div>
              <p className="text-lg font-bold tabular-nums text-primary">1.1</p>
              <p className="text-xs text-muted-foreground">MB out</p>
            </div>
          </div>
        </Card>
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

/* ── Showcase ─────────────────────────────────────────────────── */

export function RecipeLayoutShowcase() {
  return (
    <Stack gap="xl" className="gap-16">
      <ShowcaseSection
        id="phase-1"
        title="Phase 1: Files"
        description="Upload state. Dropzone dominates the right side. Left column shows title, description, and accepted formats."
      >
        <Phase1 />
      </ShowcaseSection>

      <ShowcaseSection
        id="phase-2"
        title="Phase 2: Configure"
        description="Files selected. Controls appear in the mosaic — quality dial, phase stepper, action bar. Stats and file list fill the bottom."
      >
        <Phase2 />
      </ShowcaseSection>

      <ShowcaseSection
        id="phase-3"
        title="Phase 3: Results"
        description="Compression complete. Stats update with actual output sizes and savings. Each result card shows before/after with a download button."
      >
        <Phase3 />
      </ShowcaseSection>
    </Stack>
  );
}
