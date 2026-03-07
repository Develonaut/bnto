import type { ReactNode } from "react";

/**
 * NodeBody — center zone of the node card.
 *
 * Vertically and horizontally centered, fills remaining space.
 * Use for NodeIcon, NodeLabel, NodeSublabel.
 */

function NodeBody({ children }: { children: ReactNode }) {
  return <div className="flex flex-1 flex-col items-center justify-center gap-1">{children}</div>;
}

export { NodeBody };
