"use client";

import { memo } from "react";
import type { NodeProps } from "@xyflow/react";
import type { BentoNode } from "../../adapters/types";

/**
 * ContainerGroupNode — dashed boundary overlay for expanded containers.
 *
 * Renders a rounded dashed rectangle that encompasses the parent +
 * its children. Non-interactive — purely visual indicator of grouping.
 *
 * The node's data.width and data.height are set by useLayoutNodes
 * to cover the bounding box of parent + children.
 */

export const ContainerGroupNode = memo(function ContainerGroupNode({ data }: NodeProps<BentoNode>) {
  const w = data.width ?? 0;
  const h = data.height ?? 0;

  return (
    <div
      className="relative pointer-events-none motion-safe:animate-fade-in"
      style={{ width: w, height: h }}
      role="group"
      aria-label={data.label ? `${data.label} group` : "Node group"}
    >
      <div className="absolute inset-0 rounded-xl bg-muted/40" />
      <svg className="absolute inset-0 text-muted-foreground/40" width={w} height={h}>
        <rect
          x="0.5"
          y="0.5"
          width={w - 1}
          height={h - 1}
          rx={24}
          fill="none"
          stroke="currentColor"
          strokeWidth={1}
          strokeDasharray="6 4"
        />
      </svg>
    </div>
  );
});
