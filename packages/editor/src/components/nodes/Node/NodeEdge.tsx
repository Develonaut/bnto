import type { ReactNode } from "react";
import { cn } from "@bnto/ui";

/**
 * NodeEdge — action zone positioned outside the card bounds.
 *
 * Rendered inside the slot div (outside the card's 3D transform context)
 * so edge content stays stationary when the card presses/hovers.
 *
 * Visibility cascade (CSS-first, no JS hover state):
 * - default: opacity-0, pointer-events-none (hidden)
 * - group-hover: opacity-100 (visible but children stay disabled)
 * - visible prop (selected): opacity-100, pointer-events-auto (interactive)
 *
 * This gives a peek on hover → full interaction on select.
 */

type NodeEdgePosition = "right" | "bottom";

interface NodeEdgeProps {
  position: NodeEdgePosition;
  /** When true, edge is fully interactive (pointer-events enabled). */
  visible?: boolean;
  children?: ReactNode;
  className?: string;
}

const POSITION_CLASSES: Record<NodeEdgePosition, string> = {
  right: "left-full top-1/2 -translate-y-1/2",
  bottom: "top-full left-1/2 -translate-x-1/2",
};

function NodeEdge({ position, visible = false, children, className }: NodeEdgeProps) {
  return (
    <div
      className={cn(
        "absolute z-10 flex items-center transition-opacity duration-fast",
        "nopan nodrag nowheel",
        visible
          ? "opacity-100 pointer-events-auto"
          : "opacity-0 pointer-events-none group-hover:opacity-100 group-focus-within:opacity-100",
        POSITION_CLASSES[position],
        className,
      )}
    >
      {children}
    </div>
  );
}

export { NodeEdge };
export type { NodeEdgeProps };
