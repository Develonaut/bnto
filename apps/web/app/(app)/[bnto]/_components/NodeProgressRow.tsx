"use client";

import { cn } from "@bnto/ui";
import type { NodeProgress } from "@bnto/core";

/** A single node's progress row within the execution progress list. */
export function NodeProgressRow({ node }: { node: NodeProgress }) {
  const isComplete = node.status === "completed";
  const isFailed = node.status === "failed";

  return (
    <li
      className="flex items-center gap-2 text-sm"
      data-testid="node-progress"
      data-node-id={node.nodeId}
      data-node-status={node.status}
    >
      <span
        className={cn(
          "size-2 rounded-full",
          isComplete && "bg-success",
          isFailed && "bg-destructive",
          !isComplete && !isFailed && "bg-primary motion-safe:animate-pulse",
        )}
      />
      <span className="text-muted-foreground">{node.nodeId}</span>
      <span className="ml-auto text-xs text-muted-foreground">{node.status}</span>
    </li>
  );
}
