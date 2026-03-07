import type { ReactNode } from "react";

/**
 * NodeFooter — bottom zone of the node card.
 *
 * Absolutely positioned so it never shifts the card layout.
 * Use for status indicators, progress bars, or secondary actions.
 */

function NodeFooter({ children }: { children?: ReactNode }) {
  return (
    <div
      className="absolute bottom-2 left-2 right-2 z-10 flex items-center justify-center
        transition-opacity duration-fast"
      style={{ opacity: children ? 1 : 0, pointerEvents: children ? "auto" : "none" }}
    >
      {children}
    </div>
  );
}

export { NodeFooter };
