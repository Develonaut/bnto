import { memo } from "react";
import { Handle, Position, type NodeProps } from "@xyflow/react";
import { Animate, Card, Text } from "@bnto/ui";
import type { StationData, StationNodeType } from "./types";

/**
 * Custom React Flow node that renders as a .surface Card — the "building"
 * on our Mini Motorways map.
 *
 * Each station has:
 * - A surface Card with color variant (primary, secondary, accent, etc.)
 * - Left handle (target — where incoming belts connect)
 * - Right handle (source — where outgoing belts connect)
 * - Label and sublabel text
 * - Springy entrance animation (staggered ScaleIn, like buildings
 *   materializing on the map in Mini Motorways)
 *
 * Handles are invisible — belts slide under card edges seamlessly.
 */

const SURFACE_CLASS: Record<NonNullable<StationData["variant"]>, string> = {
  primary: "surface-primary",
  secondary: "surface-secondary",
  accent: "surface-accent",
  muted: "surface-muted",
  success: "surface-success",
  warning: "surface-warning",
};

export const StationNode = memo(function StationNode({
  data,
}: NodeProps<StationNodeType>) {
  const variant = data.variant ?? "primary";

  return (
    <div className="relative">
      {/* Target handle (left side) — invisible connection point.
       * The belt slides under the card edge seamlessly. No visible
       * roller or dot — the building covers the junction, just like
       * Mini Motorways buildings sitting on top of roads. */}
      {!data.hideTarget && (
        <Handle
          type="target"
          position={Position.Left}
          className="!bg-transparent !border-0 !h-0 !w-0 !min-h-0 !min-w-0"
        />
      )}

      {/* Springy entrance — wrapper div isolates the opacity/scale
       * animation from the surface Card's preserve-3d. Each station
       * pops in with a staggered delay, like buildings materializing.
       *
       * We set animation-delay directly because React Flow's DOM
       * structure prevents stagger-cascade from reaching child nodes
       * (nodes are nested inside RF's own wrappers, not direct children). */}
      <Animate.ScaleIn
        from={0.85}
        easing="spring-bouncy"
        style={{ animationDelay: `${(data.staggerIndex ?? 0) * 80}ms` }}
      >
        {/* The building — surface Card.
         * Sized to comfortably cover belt endpoints where they meet
         * the station, hiding junction seams (like MM buildings on roads). */}
        <Card
          elevation="lg"
          className={`${SURFACE_CLASS[variant]} flex h-[88px] w-[124px] flex-col items-center justify-center rounded-xl`}
        >
          <Text size="sm" className="font-display font-semibold leading-tight">
            {data.label}
          </Text>
          {data.sublabel && (
            <Text size="xs" color="muted" className="mt-0.5">
              {data.sublabel}
            </Text>
          )}
        </Card>
      </Animate.ScaleIn>

      {/* Source handle (right side) — invisible, same as target */}
      {!data.hideSource && (
        <Handle
          type="source"
          position={Position.Right}
          className="!bg-transparent !border-0 !h-0 !w-0 !min-h-0 !min-w-0"
        />
      )}
    </div>
  );
});
