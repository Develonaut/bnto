import type { ReactNode } from "react";

/**
 * NodeHeader — top-right action zone inside the node card.
 *
 * Absolutely positioned so it never shifts the card layout.
 */

function NodeHeader({ children }: { children?: ReactNode }) {
  return (
    <div
      className="absolute right-2 top-2 z-10 flex items-center gap-1
        transition-opacity duration-fast"
      style={{ opacity: children ? 1 : 0, pointerEvents: children ? "auto" : "none" }}
    >
      {children}
    </div>
  );
}

export { NodeHeader };
