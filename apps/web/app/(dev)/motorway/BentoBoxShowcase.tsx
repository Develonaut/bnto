"use client";

import { useState, useCallback, useMemo } from "react";
import dynamic from "next/dynamic";
import { Plus, Minus, RotateCcw } from "lucide-react";
import { Button } from "@/components/ui/Button";
import { Row } from "@/components/ui/Row";
import { Text } from "@/components/ui/Text";
import type { CompartmentNodeType, CompartmentData } from "./CompartmentNode";

/**
 * Interactive "Bento Box" showcase — add/remove surface compartments
 * on a grid canvas and watch it grow like a Mini Motorways city.
 *
 * Each compartment is a .surface Card of varying size, placed in a
 * predefined bento-box layout. As you add compartments, the viewport
 * smoothly zooms out to accommodate the growing box — the signature
 * Mini Motorways "camera pulling back" effect.
 *
 * Layout (3 rows, horizontal flow):
 * ┌────────┬────────────┬────────┬──────────┐
 * │   1    │     2      │   3    │    4     │
 * ├────────┴────────────┼────────┴──────────┤
 * │         5           │        6          │
 * ├──────────┬──────────┼────────┬──────────┤
 * │    7     │    8     │   9    │   10     │
 * └──────────┴──────────┴────────┴──────────┘
 */

const BentoCanvas = dynamic(
  () => import("./BentoCanvas").then((m) => m.BentoCanvas),
  { ssr: false },
);

/* ── Compartment palette ─────────────────────────────────────── */

const PALETTE: CompartmentData[] = [
  { label: "Drop Files", sublabel: "Input", variant: "muted" },
  { label: "Compress", sublabel: "Image", variant: "primary" },
  { label: "Resize", sublabel: "Transform", variant: "secondary" },
  { label: "Convert", sublabel: "Format", variant: "accent" },
  { label: "Filter", sublabel: "Clean", variant: "success" },
  { label: "Download", sublabel: "Output", variant: "warning" },
  { label: "Rename", sublabel: "File", variant: "primary" },
  { label: "Validate", sublabel: "Check", variant: "secondary" },
  { label: "Merge", sublabel: "Combine", variant: "accent" },
  { label: "Split", sublabel: "Divide", variant: "success" },
];

/* ── Bento layout slots ──────────────────────────────────────── */

const CELL = 120;
const GAP = 20;

/** Predefined positions that tile like a bento box.
 *  Horizontal flow (left-to-right), 3 rows max. Grows wider as
 *  compartments are added — matching pipeline reading direction.
 *
 *  Total width: 660px (4 cells: 120 + 20 + 200 + 20 + 120 + 20 + 160)
 *
 *  ┌────────┬────────────┬────────┬──────────┐
 *  │   1    │     2      │   3    │    4     │
 *  ├────────┴────────────┼────────┴──────────┤
 *  │         5           │        6          │
 *  ├──────────┬──────────┼────────┬──────────┤
 *  │    7     │    8     │   9    │   10     │
 *  └──────────┴──────────┴────────┴──────────┘
 */
const S = CELL + GAP; // stride (one cell + gap)

/* Column boundaries: 0, 140, 340, 480, 660
 * Left half:  0–340  (340px)   Right half: 340–660 (320px)
 * Row 0: 120 | 20 | 200 | 20 | 120 | 20 | 160 = 660
 * Row 1: 340          | 20 |          300       = 660
 * Row 2: 160 | 20 | 160 | 20 | 140 | 20 | 140 = 660
 */
const SLOTS: { x: number; y: number; w: number; h: number }[] = [
  /* Row 0 — alternating small + wide, left to right */
  { x: 0, y: 0, w: 120, h: CELL },
  { x: 140, y: 0, w: 200, h: CELL },
  { x: 360, y: 0, w: 120, h: CELL },
  { x: 500, y: 0, w: 160, h: CELL },
  /* Row 1 — two wide panels spanning each half */
  { x: 0, y: S, w: 340, h: CELL },
  { x: 360, y: S, w: 300, h: CELL },
  /* Row 2 — four cells filling the bottom */
  { x: 0, y: S * 2, w: 160, h: CELL },
  { x: 180, y: S * 2, w: 160, h: CELL },
  { x: 360, y: S * 2, w: 140, h: CELL },
  { x: 520, y: S * 2, w: 140, h: CELL },
];

/* ── Node builder ────────────────────────────────────────────── */

function buildNodes(count: number): CompartmentNodeType[] {
  return Array.from({ length: count }, (_, i) => {
    const slot = SLOTS[i]!;
    const data = PALETTE[i % PALETTE.length]!;
    return {
      id: `compartment-${i}`,
      type: "compartment" as const,
      position: { x: slot.x, y: slot.y },
      data: { ...data, width: slot.w, height: slot.h },
    };
  });
}

/* ── Showcase ─────────────────────────────────────────────────── */

export function BentoBoxShowcase() {
  const [count, setCount] = useState(1);
  const nodes = useMemo(() => buildNodes(count), [count]);

  const add = useCallback(() => {
    setCount((c) => Math.min(c + 1, SLOTS.length));
  }, []);

  const remove = useCallback(() => {
    setCount((c) => Math.max(c - 1, 0));
  }, []);

  const reset = useCallback(() => setCount(1), []);

  return (
    <div className="flex flex-col gap-4">
      {/* Controls */}
      <Row className="items-center gap-3">
        <Button
          size="md"
          variant="secondary"
          onClick={add}
          disabled={count >= SLOTS.length}
        >
          <Plus className="size-3.5" />
          Add Compartment
        </Button>
        <Button
          size="md"
          variant="ghost"
          onClick={remove}
          disabled={count <= 0}
        >
          <Minus className="size-3.5" />
          Remove
        </Button>
        <Button
          size="md"
          variant="ghost"
          onClick={reset}
          disabled={count <= 1}
        >
          <RotateCcw className="size-3.5" />
          Reset
        </Button>
        <Text size="sm" color="muted" className="ml-auto font-mono">
          {count} / {SLOTS.length}
        </Text>
      </Row>

      {/* Canvas — the bento box */}
      <BentoCanvas nodes={nodes} />
    </div>
  );
}
