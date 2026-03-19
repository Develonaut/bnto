"use client";

import { useEffect, useRef, useState } from "react";
import { getSmoothStepPath, Position, type EdgeProps, type Edge } from "@xyflow/react";

/**
 * NodeEdge — thick pipeline edge connecting nodes in sequence.
 *
 * Three-layer SVG stack:
 *   1. Base track — faded road (always visible, shows full path)
 *   2. Progress fill — opaque road clipped by stroke-dashoffset
 *   3. Surface texture — subtle dash pattern for road feel
 *
 * Progress (0–1) drives the fill from source → target via
 * stroke-dasharray/dashoffset animation.
 */

const BELT_WIDTH = 22;
const BORDER_RADIUS = 12;
const DASH_PATTERN = "3 17";

type NodeEdgeData = { progress?: number };
type NodeEdgeType = Edge<NodeEdgeData, "pipeline">;

function NodeEdge({
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition = Position.Right,
  targetPosition = Position.Left,
  data,
}: EdgeProps<NodeEdgeType>) {
  const [path] = getSmoothStepPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
    borderRadius: BORDER_RADIUS,
  });

  const progress = data?.progress ?? 0;

  // Measure path length for stroke-dasharray progress animation
  const pathRef = useRef<SVGPathElement>(null);
  const [pathLength, setPathLength] = useState(0);

  useEffect(() => {
    if (pathRef.current) {
      setPathLength(pathRef.current.getTotalLength());
    }
  }, [path]);

  const fillOffset = pathLength * (1 - progress);

  return (
    <g>
      {/* Layer 1: Base track — faded, always visible */}
      <path
        d={path}
        stroke="var(--road-base, var(--muted))"
        strokeWidth={BELT_WIDTH}
        strokeLinecap="round"
        strokeLinejoin="round"
        fill="none"
        opacity={0.3}
      />

      {/* Layer 2: Progress fill — opaque, clipped by dashoffset */}
      {pathLength > 0 && (
        <path
          d={path}
          stroke="var(--road-fill, var(--border))"
          strokeWidth={BELT_WIDTH}
          strokeLinecap="round"
          strokeLinejoin="round"
          fill="none"
          strokeDasharray={pathLength}
          strokeDashoffset={fillOffset}
          className="motion-safe:transition-[stroke-dashoffset] motion-safe:duration-300 motion-safe:ease-out"
        />
      )}

      {/* Layer 3: Surface texture — dashed pattern on top */}
      <path
        d={path}
        stroke="var(--road-ridge, var(--border))"
        strokeWidth={BELT_WIDTH}
        strokeLinecap="butt"
        strokeLinejoin="round"
        strokeDasharray={DASH_PATTERN}
        fill="none"
        opacity={0.15}
      />

      {/* Hidden ref path for length measurement */}
      <path ref={pathRef} d={path} fill="none" stroke="none" />
    </g>
  );
}

export { NodeEdge };
export type { NodeEdgeData, NodeEdgeType };
