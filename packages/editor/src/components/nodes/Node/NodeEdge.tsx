import type { ReactNode } from "react";
import { cn } from "@bnto/ui";
import { GAP_Y } from "../../../adapters/bentoSlots";

/**
 * NodeEdge — action zone positioned in the gap below the card.
 *
 * Rendered inside the slot div (outside the card's 3D transform context)
 * so edge content stays stationary when the card presses/hovers.
 *
 * Centered vertically within GAP_Y below the card using
 * `top-full` + half-GAP_Y offset.
 *
 * Visibility cascade (CSS-first, no JS hover state):
 * - default: opacity-0, pointer-events-none (hidden)
 * - group-hover: opacity-100 (visible but children stay disabled)
 * - visible prop (selected): opacity-100, pointer-events-auto (interactive)
 *
 * This gives a peek on hover → full interaction on select.
 */

interface NodeEdgeProps {
  /** When true, edge is fully interactive (pointer-events enabled). */
  visible?: boolean;
  children?: ReactNode;
  className?: string;
}

function NodeEdge({ visible = false, children, className }: NodeEdgeProps) {
  return (
    <div
      className={cn(
        "absolute left-1/2 z-10 flex items-center transition-opacity duration-fast",
        "nopan nodrag nowheel",
        visible
          ? "opacity-100 pointer-events-auto"
          : "opacity-0 pointer-events-none group-hover:opacity-100 group-focus-within:opacity-100",
        className,
      )}
      style={{ top: `calc(100% + ${GAP_Y / 2}px)`, translate: "-50% -50%" }}
    >
      {children}
    </div>
  );
}

export { NodeEdge };
export type { NodeEdgeProps };
