import type { CSSProperties } from "react";
import { getSmoothStepPath, Position, type EdgeProps } from "@xyflow/react";
import { BeltPiece, VARIANT_PIECE_MAP } from "./BeltPiece";
import type { ConveyorEdgeType } from "./types";

/**
 * Custom React Flow edge — a thick conveyor belt connection between stations.
 *
 * Two-layer SVG approach — belts are FLAT (no shadow, no surface elevation):
 *
 *   1. Belt body — solid thick stroke in the variant color
 *   2. Belt surface — animated dashes (scrolling ridges = conveyor texture)
 *
 * Piece layering: ConveyorCanvas splits each edge with pieces into TWO
 * React Flow edges — a "body" edge (low zIndex) and a "pieces" edge
 * (high zIndex). This guarantees all sushi pieces paint on top of all
 * belt bodies, regardless of edge order. The `layer` field controls
 * which part this instance renders.
 *
 * Orthogonal routing via getSmoothStepPath with rounded corners.
 *
 * Variant colors are resolved via CSS classes (.belt-primary, .belt-muted, etc.)
 * defined in conveyor.css. The <g> wrapper carries the class so all child
 * paths inherit the CSS custom properties.
 */

/* ── Constants ──────────────────────────────────────────────── */

/** Belt stroke width in px — thick, like a Mini Motorways road */
const BELT_WIDTH = 22;

/** Rounded corners at orthogonal turns */
const BORDER_RADIUS = 12;

/**
 * Dash pattern for the belt surface texture.
 * 3px dash + 17px gap = 20px cycle (matches convey-svg keyframe).
 */
const DASH_PATTERN = "3 17";

/**
 * How far (in px) the piece motion path extends INTO each station node.
 * Pieces spawn hidden under the source node and vanish under the target,
 * so the loop reset is invisible behind the station card.
 */
const PIECE_INSET = 30;

/** Move a point INTO its node — opposite to the handle direction. */
function inset(x: number, y: number, position: Position, amount: number) {
  switch (position) {
    case Position.Right:
      return { x: x - amount, y };
    case Position.Left:
      return { x: x + amount, y };
    case Position.Bottom:
      return { x, y: y - amount };
    case Position.Top:
      return { x, y: y + amount };
  }
}

export function ConveyorEdge({
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  data,
}: EdgeProps<ConveyorEdgeType>) {
  const [path] = getSmoothStepPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
    borderRadius: BORDER_RADIUS,
  });

  const variant = data?.variant ?? "muted";
  const speed = data?.speed ?? "normal";
  const bordered = data?.bordered ?? false;
  const staggerIndex = data?.staggerIndex ?? 0;
  const layer = data?.layer;

  /* Belt entrance delay — belts fade in AFTER stations pop in.
   * 400ms base (lets first stations appear) + staggered per belt. */
  const enterDelay = 400 + staggerIndex * 80;

  /* ── Pieces-only layer ─────────────────────────────────────── */
  if (layer === "pieces") {
    const pieceCount = data?.pieces ?? 0;
    if (pieceCount === 0) return null;

    /* Extended path — starts inside the source node and ends inside the
     * target node. Pieces spawn hidden under the source card and vanish
     * under the target card, so the animation loop reset is invisible. */
    const src = inset(sourceX, sourceY, sourcePosition, PIECE_INSET);
    const tgt = inset(targetX, targetY, targetPosition, PIECE_INSET);
    const [piecePath] = getSmoothStepPath({
      sourceX: src.x,
      sourceY: src.y,
      targetX: tgt.x,
      targetY: tgt.y,
      sourcePosition,
      targetPosition,
      borderRadius: BORDER_RADIUS,
    });

    const duration = data?.pieceSpeed ?? 2.5;
    const pieceType = data?.pieceType ?? VARIANT_PIECE_MAP[variant] ?? "tekka";

    return (
      <g
        className="belt-pieces belt-enter"
        style={{ "--belt-enter-delay": `${enterDelay}ms` } as CSSProperties}
      >
        {Array.from({ length: pieceCount }, (_, i) => (
          <BeltPiece
            key={i}
            path={piecePath}
            duration={duration}
            begin={(i * duration) / pieceCount}
            type={pieceType}
          />
        ))}
      </g>
    );
  }

  /* ── Belt body layer (default) ─────────────────────────────── */

  /* Surface animation class — belt-surface + optional speed modifier */
  const surfaceClass = [
    "belt-surface",
    speed === "fast" && "belt-surface-fast",
    speed === "paused" && "belt-surface-paused",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <g
      className={`belt-${variant} belt-enter`}
      style={{ "--belt-enter-delay": `${enterDelay}ms` } as CSSProperties}
    >
      {/* Optional: Border outline for dark themes */}
      {bordered && (
        <path
          d={path}
          stroke="var(--belt-rail, var(--surface-muted-wall))"
          strokeWidth={BELT_WIDTH + 4}
          strokeLinecap="round"
          strokeLinejoin="round"
          fill="none"
          opacity={0.4}
        />
      )}

      {/* Layer 1: Belt body — thick solid stroke, flush to the ground */}
      <path
        d={path}
        stroke="var(--belt-track, var(--muted))"
        strokeWidth={BELT_WIDTH}
        strokeLinecap="round"
        strokeLinejoin="round"
        fill="none"
      />

      {/* Layer 2: Belt surface — animated dashed stroke (conveyor texture) */}
      <path
        d={path}
        stroke="var(--belt-ridge, var(--border))"
        strokeWidth={BELT_WIDTH}
        strokeLinecap="butt"
        strokeLinejoin="round"
        strokeDasharray={DASH_PATTERN}
        fill="none"
        className={surfaceClass}
        opacity={0.55}
      />
    </g>
  );
}
