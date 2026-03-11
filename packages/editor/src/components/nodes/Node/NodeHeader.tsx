import type { ReactNode } from "react";
import { cn } from "@bnto/ui";

/**
 * NodeHeader — top overlay zone inside the node card grid.
 *
 * Overlays the single grid cell aligned to the top (self-start).
 * Houses action buttons that appear on hover/select.
 * Does NOT displace body content — body stays centered.
 *
 * Visibility cascade (CSS-first, no JS hover state):
 * - default: opacity-0, pointer-events-none (hidden)
 * - group-hover / group-focus-within: opacity-100 (peek)
 * - visible prop (selected): opacity-100, pointer-events-auto (interactive)
 */

function NodeHeader({ visible = false, children }: { visible?: boolean; children?: ReactNode }) {
  return (
    <div
      className={cn(
        "col-start-1 row-start-1 self-start",
        "z-10 flex items-center justify-end gap-1 p-2.5",
        "transition-opacity duration-fast",
        "nopan nodrag nowheel",
        visible
          ? "opacity-100 pointer-events-auto"
          : "opacity-0 pointer-events-none group-hover:opacity-100 group-focus-within:opacity-100",
      )}
    >
      {children}
    </div>
  );
}

export { NodeHeader };
