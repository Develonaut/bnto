/**
 * Conveyor belt shared types — used by StationNode, ConveyorEdge,
 * and ConveyorCanvas.
 */

import type { Node, Edge } from "@xyflow/react";
import type { PieceType } from "./BeltPiece";

// ---------------------------------------------------------------------------
// Station node
// ---------------------------------------------------------------------------

type StationData = {
  label: string;
  sublabel?: string;
  variant?:
    | "primary"
    | "secondary"
    | "accent"
    | "muted"
    | "success"
    | "warning";
  /** Hide the target (left) handle — for input-only nodes */
  hideTarget?: boolean;
  /** Hide the source (right) handle — for output-only nodes */
  hideSource?: boolean;
  /** Position in the stagger cascade (0-based). Controls entrance delay. */
  staggerIndex?: number;
};

type StationNodeType = Node<StationData, "station">;

// ---------------------------------------------------------------------------
// Conveyor edge
// ---------------------------------------------------------------------------

type ConveyorEdgeData = {
  /** Color variant — matches source station variant. Default: "muted" */
  variant?: "primary" | "secondary" | "accent" | "muted" | "success";
  /** Speed variant — "fast" for batch processing, "paused" for idle */
  speed?: "normal" | "fast" | "paused";
  /** Show border outline — useful in dark themes where belts need
   * contrast against the background (see Tokyo Night reference). */
  bordered?: boolean;
  /** Position in the entrance stagger. Belts fade in AFTER stations
   * pop in — delay = 400ms base + staggerIndex * 80ms. */
  staggerIndex?: number;
  /** Number of sushi pieces traveling this belt (0 = none). */
  pieces?: number;
  /** Traversal time per piece in seconds. Default: 2.5 */
  pieceSpeed?: number;
  /** Shape of the sushi pieces. Default: "tekka" */
  pieceType?: PieceType;
  /** Render layer — set internally by ConveyorCanvas.
   * "body" = belt track only, "pieces" = sushi only. */
  layer?: "body" | "pieces";
};

type ConveyorEdgeType = Edge<ConveyorEdgeData, "conveyor">;

export type {
  StationData,
  StationNodeType,
  ConveyorEdgeData,
  ConveyorEdgeType,
};
