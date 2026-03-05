"use client";

import { useCallback, useState } from "react";
import dynamic from "next/dynamic";
import { RotateCcw } from "lucide-react";
import { Button } from "@/components/ui/Button";
import { Row } from "@/components/ui/Row";
import { Text } from "@/components/ui/Text";
import type { StationNodeType } from "@/components/editor/archive/conveyor/StationNode";
import type { ConveyorEdgeType } from "@/components/editor/archive/conveyor/ConveyorEdge";
import { PieceShape } from "@/components/editor/archive/conveyor/PieceShape";
import type { PieceType } from "@/components/editor/archive/conveyor/BeltPiece";

/**
 * Conveyor Belt showcase — a Mini Motorways-style "level" showing
 * a recipe pipeline with surface-card stations and belt connections.
 *
 * The scene is declared right here as a blueprint. Reading this file
 * tells you exactly what the level contains — which stations, where
 * they sit, and how they're connected. The canvas just renders it.
 *
 * Scene: An image processing pipeline with a branch.
 *
 *   ┌─────────┐    ┌──────────┐    ┌────────┐
 *   │  Input   │───▶│ Compress │──┬▶│ Resize │──┐
 *   └─────────┘    └──────────┘  │ └────────┘  │  ┌──────────┐
 *                                │              ├─▶│  Output   │
 *                                │ ┌─────────┐  │  └──────────┘
 *                                └▶│ Convert │──┘
 *                                  └─────────┘
 */

const ConveyorCanvas = dynamic(
  () => import("@/components/editor/archive/conveyor/ConveyorCanvas").then((m) => m.ConveyorCanvas),
  { ssr: false },
);

/* ── Stations (nodes) ──────────────────────────────────────────── */
/* staggerIndex controls the entrance animation order — stations
 * pop in left-to-right with a springy bounce delay. */

const stations: StationNodeType[] = [
  {
    id: "input",
    type: "station",
    position: { x: 0, y: 140 },
    data: {
      label: "Drop Files",
      sublabel: "Input",
      variant: "muted",
      hideTarget: true,
      staggerIndex: 0,
    },
  },
  {
    id: "compress",
    type: "station",
    position: { x: 260, y: 140 },
    data: {
      label: "Compress",
      sublabel: "WASM",
      variant: "primary",
      staggerIndex: 1,
    },
  },
  {
    id: "resize",
    type: "station",
    position: { x: 520, y: 20 },
    data: {
      label: "Resize",
      sublabel: "WASM",
      variant: "secondary",
      staggerIndex: 2,
    },
  },
  {
    id: "convert",
    type: "station",
    position: { x: 520, y: 260 },
    data: {
      label: "Convert",
      sublabel: "WebP",
      variant: "accent",
      staggerIndex: 3,
    },
  },
  {
    id: "output",
    type: "station",
    position: { x: 800, y: 140 },
    data: {
      label: "Download",
      sublabel: "Output",
      variant: "success",
      hideSource: true,
      staggerIndex: 4,
    },
  },
];

/* ── Belts (edges) ─────────────────────────────────────────────── */
/* Each belt inherits its color variant from the source station,
 * like Mini Motorways roads matching their building color.
 *
 * Entrance sequence:
 *   Stations pop in first (0–400ms, staggered 80ms each)
 *   → Belts fade in after (400ms base + staggered 80ms each)
 * This creates a "buildings appear, then roads connect them" effect. */

const belts: ConveyorEdgeType[] = [
  { id: "input-compress", source: "input", target: "compress", type: "conveyor", data: { variant: "muted", staggerIndex: 0, pieces: 2 } },
  { id: "compress-resize", source: "compress", target: "resize", type: "conveyor", data: { variant: "primary", staggerIndex: 1, pieces: 2 } },
  { id: "compress-convert", source: "compress", target: "convert", type: "conveyor", data: { variant: "primary", staggerIndex: 1, pieces: 2 } },
  { id: "resize-output", source: "resize", target: "output", type: "conveyor", data: { variant: "secondary", staggerIndex: 2, pieces: 1 } },
  { id: "convert-output", source: "convert", target: "output", type: "conveyor", data: { variant: "accent", staggerIndex: 2, pieces: 1 } },
];

/* ── All piece types for the preview row ─────────────────────── */

const ALL_PIECES: PieceType[] = [
  "tekka", "sake", "ebi", "hotate", "tamago", "kappa",
  "nigiri", "tamago-nigiri", "ebi-nigiri", "onigiri",
];

/* ── Showcase ──────────────────────────────────────────────────── */

export function ConveyorShowcase() {
  /* Changing the key forces React to unmount + remount the canvas,
   * replaying all entrance animations (stagger bounce on stations). */
  const [reloadKey, setReloadKey] = useState(0);
  const handleReload = useCallback(() => setReloadKey((k) => k + 1), []);

  return (
    <div>
      <Row className="mb-3 justify-end">
        <Button
          variant="outline"
          size="md"
          onClick={handleReload}
          aria-label="Replay entrance animations"
        >
          <RotateCcw className="size-3.5" />
          Replay
        </Button>
      </Row>
      {/* Piece shape preview — static display of each sushi type.
       * Uses the shared PieceShape component from BeltPiece so
       * the preview always matches what rides on the belts. */}
      <Row className="mb-4 gap-5 flex-wrap">
        {ALL_PIECES.map((type) => (
          <Row key={type} className="items-center gap-2">
            <svg width={36} height={36} viewBox="-18 -18 36 36">
              <defs>
                <filter id={`shadow-${type}`} x="-50%" y="-50%" width="200%" height="200%">
                  <feDropShadow dx={1} dy={2} stdDeviation={1} floodColor="#3d2b1f" floodOpacity={0.25} />
                </filter>
                {/* Salmon clip for nigiri preview */}
                {type === "nigiri" && (
                  <clipPath id="salmon-clip-preview">
                    <rect x={-8 * 1.8 * 0.38 * -1} y={-8 * 1.5 * 0.33} width={8 * 1.8 * 0.76} height={8 * 1.5 * 0.66} rx={2.5} />
                  </clipPath>
                )}
              </defs>
              <g filter={`url(#shadow-${type})`}>
                <PieceShape type={type} />
              </g>
            </svg>
            <Text size="xs" color="muted" className="font-mono">{type}</Text>
          </Row>
        ))}
      </Row>
      <ConveyorCanvas key={reloadKey} nodes={stations} edges={belts} />
    </div>
  );
}
