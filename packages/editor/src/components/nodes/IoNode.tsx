import { memo } from "react";
import type { NodeProps } from "@xyflow/react";
import type { BentoNode } from "../../adapters/types";
import { NodeRoot, NodeBody, NodeIcon, NodeLabel, NodeSublabel } from "./Node";

/**
 * IoNode — Input/Output node on the bento grid.
 *
 * Smaller card, centered in the slot, muted color.
 * No header actions — I/O nodes are structural.
 */

export const IoNode = memo(function IoNode({ id, data, selected }: NodeProps<BentoNode>) {
  const status = data.status ?? "idle";
  const isCompleted = status === "completed";

  return (
    <NodeRoot
      width={data.width}
      height={data.height}
      elevation={isCompleted ? "md" : selected ? "md" : "sm"}
      color={isCompleted ? "success" : "muted"}
      align={id === "input" ? "end" : "start"}
      selected={selected}
    >
      <NodeBody>
        <NodeIcon icon={data.icon} />
        <NodeLabel>{data.label}</NodeLabel>
        <NodeSublabel>{data.sublabel}</NodeSublabel>
      </NodeBody>
    </NodeRoot>
  );
});
