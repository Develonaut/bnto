"use client";

import { useState, useCallback, useEffect, useMemo } from "react";
import dynamic from "next/dynamic";
import { useReactFlow } from "@xyflow/react";
import {
  PlusIcon,
  MinusIcon,
  RotateCcwIcon,
  PlayIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  SquareIcon,
} from "@bnto/ui";
import { Button, Row, Text } from "@bnto/ui";
import type { CompartmentStatus } from "@/editor/components/canvas/CompartmentNode";
import type { BentoNode } from "@/editor/adapters/types";
import type { CompartmentNodeData } from "@/editor/adapters/types";
import { SLOTS } from "@/editor/adapters/bentoSlots";

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
  () => import("@/editor/components/canvas/BentoCanvas").then((m) => m.BentoCanvas),
  { ssr: false },
);

/* ── Compartment palette ─────────────────────────────────────── */

type PaletteEntry = Pick<CompartmentNodeData, "label" | "sublabel" | "variant">;

const PALETTE: PaletteEntry[] = [
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

/* ── Node builder ────────────────────────────────────────────── */

function statusForIndex(
  i: number,
  activeIndex: number | null,
): CompartmentStatus {
  if (activeIndex === null) return "idle";
  if (i < activeIndex) return "completed";
  if (i === activeIndex) return "active";
  return "pending";
}

function buildNodes(
  count: number,
  activeIndex: number | null,
): BentoNode[] {
  return Array.from({ length: count }, (_, i) => {
    const slot = SLOTS[i]!;
    const data = PALETTE[i % PALETTE.length]!;
    const nodeId = `compartment-${i}`;
    return {
      id: nodeId,
      type: "compartment" as const,
      position: { x: slot.x, y: slot.y },
      data: {
        ...data,
        width: slot.w,
        height: slot.h,
        status: statusForIndex(i, activeIndex),
        // Domain fields (showcase uses demo values)
        nodeType: "image",
        name: data.label,
        parameters: {},
      },
    };
  });
}

/* ── Headless node sync — rendered as a child inside <ReactFlow> ── */

function NodeSync({
  count,
  activeIndex,
}: {
  count: number;
  activeIndex: number | null;
}) {
  const { setNodes } = useReactFlow<BentoNode>();

  useEffect(() => {
    setNodes(buildNodes(count, activeIndex));
  }, [count, activeIndex, setNodes]);

  return null;
}

/* ── Showcase ─────────────────────────────────────────────────── */

export function BentoBoxShowcase() {
  const [count, setCount] = useState(1);
  /* activeIndex: null = idle (no simulation), 0..count-1 = running */
  const [activeIndex, setActiveIndex] = useState<number | null>(null);
  const running = activeIndex !== null;

  const defaultNodes = useMemo(() => buildNodes(1, null), []);

  const add = useCallback(() => {
    setCount((c) => Math.min(c + 1, SLOTS.length));
  }, []);

  const remove = useCallback(() => {
    setCount((c) => Math.max(c - 1, 0));
  }, []);

  const reset = useCallback(() => {
    setCount(1);
    setActiveIndex(null);
  }, []);

  const run = useCallback(() => setActiveIndex(0), []);
  const stop = useCallback(() => setActiveIndex(null), []);

  const next = useCallback(() => {
    setActiveIndex((i) => (i !== null && i < count - 1 ? i + 1 : i));
  }, [count]);

  const prev = useCallback(() => {
    setActiveIndex((i) => (i !== null && i > 0 ? i - 1 : i));
  }, []);

  return (
    <div className="flex flex-col gap-4">
      {/* Build controls */}
      <Row className="items-center gap-3">
        <Button
          size="md"
          variant="secondary"
          onClick={add}
          disabled={count >= SLOTS.length || running}
        >
          <PlusIcon className="size-3.5" />
          Add
        </Button>
        <Button
          size="md"
          variant="ghost"
          onClick={remove}
          disabled={count <= 0 || running}
        >
          <MinusIcon className="size-3.5" />
          Remove
        </Button>
        <Button
          size="md"
          variant="ghost"
          onClick={reset}
          disabled={count <= 1 && !running}
        >
          <RotateCcwIcon className="size-3.5" />
          Reset
        </Button>
        <Text size="sm" color="muted" className="ml-auto font-mono">
          {count} / {SLOTS.length}
        </Text>
      </Row>

      {/* Simulation controls */}
      <Row className="items-center gap-3">
        {!running ? (
          <Button
            size="md"
            variant="primary"
            onClick={run}
            disabled={count === 0}
          >
            <PlayIcon className="size-3.5" />
            Run
          </Button>
        ) : (
          <>
            <Button size="md" variant="ghost" onClick={stop}>
              <SquareIcon className="size-3.5" />
              Stop
            </Button>
            <Button
              size="md"
              variant="ghost"
              onClick={prev}
              disabled={activeIndex === 0}
            >
              <ChevronLeftIcon className="size-3.5" />
              Prev
            </Button>
            <Button
              size="md"
              variant="ghost"
              onClick={next}
              disabled={activeIndex === count - 1}
            >
              <ChevronRightIcon className="size-3.5" />
              Next
            </Button>
            <Text size="sm" color="muted" className="ml-auto font-mono">
              Step {activeIndex! + 1} / {count}
            </Text>
          </>
        )}
      </Row>

      {/* Canvas — the bento box. NodeSync lives inside <ReactFlow>
          and calls setNodes() when count/activeIndex changes. */}
      <BentoCanvas defaultNodes={defaultNodes}>
        <NodeSync count={count} activeIndex={activeIndex} />
      </BentoCanvas>
    </div>
  );
}
