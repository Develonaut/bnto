"use client";

import { useMemo } from "react";
import {
  ReactFlow,
  ReactFlowProvider,
  Background,
  BackgroundVariant,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";

import { StationNode } from "./StationNode";
import { ConveyorEdge } from "./ConveyorEdge";
import type { StationNodeType, ConveyorEdgeType } from "./types";
import { SALMON_CLIP } from "./BeltPiece";

/** Module-level constants — stable references prevent RF from
 * unmounting/remounting all nodes on every render. */
const NODE_TYPES = { station: StationNode } as const;
const EDGE_TYPES = { conveyor: ConveyorEdge } as const;

/**
 * A React Flow canvas that renders a Mini Motorways-style "level".
 *
 * Accepts nodes and edges as props so the scene is defined declaratively
 * by the consumer — the canvas is the renderer, not the data owner.
 * This keeps scenes composable and readable as blueprints.
 *
 * Piece layering: edges with pieces are split into TWO React Flow edges —
 * a "body" edge (zIndex 0) for the belt track/surface and a "pieces"
 * edge (zIndex 1) for the sushi. This guarantees all pieces paint on
 * top of all belt bodies regardless of edge order.
 */

type ConveyorCanvasProps = {
  nodes: StationNodeType[];
  edges: ConveyorEdgeType[];
  /** Canvas height in pixels. Default: 480 */
  height?: number;
  /** Show subtle dot background. Default: true */
  showGrid?: boolean;
};

export function ConveyorCanvas({
  nodes,
  edges,
  height = 480,
  showGrid = true,
}: ConveyorCanvasProps) {
  /* Ensure nodes always render above all edge layers (body + pieces). */
  const elevatedNodes = useMemo(
    () => nodes.map((n) => ({ ...n, zIndex: 2 })),
    [nodes],
  );

  /* Split edges with pieces into two layers:
   *   - body edges (zIndex 0) — belt track + surface texture
   *   - pieces edges (zIndex 1) — sushi pieces only
   * This guarantees pieces always paint on top of all belt bodies. */
  const layeredEdges = useMemo(() => {
    const result: ConveyorEdgeType[] = [];
    for (const edge of edges) {
      const hasPieces = (edge.data?.pieces ?? 0) > 0;
      // Body layer — always present
      result.push({
        ...edge,
        data: { ...edge.data, layer: "body" },
        zIndex: 0,
      });
      // Pieces layer — only for edges with pieces
      if (hasPieces) {
        result.push({
          ...edge,
          id: `${edge.id}--pieces`,
          data: { ...edge.data, layer: "pieces" },
          zIndex: 1,
        });
      }
    }
    return result;
  }, [edges]);

  return (
    <div
      className="w-full overflow-hidden rounded-xl border border-border bg-background"
      style={{ height }}
    >
      <ReactFlowProvider>
        <ReactFlow<StationNodeType, ConveyorEdgeType>
          nodes={elevatedNodes}
          edges={layeredEdges}
          nodeTypes={NODE_TYPES}
          edgeTypes={EDGE_TYPES}
          defaultEdgeOptions={{ type: "conveyor" }}
          fitView
          fitViewOptions={{ padding: 0.2 }}
          /* Read-only — no interaction for showcase */
          nodesDraggable={false}
          nodesConnectable={false}
          elementsSelectable={false}
          panOnDrag={false}
          zoomOnScroll={false}
          zoomOnPinch={false}
          zoomOnDoubleClick={false}
          preventScrolling={false}
          proOptions={{ hideAttribution: true }}
        >
          {showGrid && (
            <Background
              variant={BackgroundVariant.Dots}
              gap={20}
              size={1}
              color="var(--border)"
            />
          )}

          {/* SVG <defs> for shared filters and clip paths — rendered inside
           * the React Flow SVG layer so edge/piece elements can reference
           * them via url(). Defined once here to avoid duplicate IDs when
           * multiple pieces of the same type render simultaneously. */}
          <svg
            className="absolute inset-0 pointer-events-none"
            style={{ width: 0, height: 0 }}
          >
            <defs>
              {/* Warm drop shadow matching .surface elevation-sm language */}
              <filter
                id="piece-shadow"
                x="-50%"
                y="-50%"
                width="200%"
                height="200%"
              >
                <feDropShadow
                  dx={1}
                  dy={2}
                  stdDeviation={1}
                  floodColor="#3d2b1f"
                  floodOpacity={0.25}
                />
              </filter>
              {/* Salmon marbling clip — shared by all nigiri pieces */}
              <clipPath id="salmon-clip">
                <rect
                  x={SALMON_CLIP.x}
                  y={SALMON_CLIP.y}
                  width={SALMON_CLIP.width}
                  height={SALMON_CLIP.height}
                  rx={SALMON_CLIP.rx}
                />
              </clipPath>
            </defs>
          </svg>
        </ReactFlow>
      </ReactFlowProvider>
    </div>
  );
}
