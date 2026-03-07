import { memo } from "react";
import type { NodeProps } from "@xyflow/react";
import type { BentoNode } from "../../adapters/types";
import { NodeRoot, NodeBody, NodeIcon, NodeLabel, NodeSublabel } from "./Node";

/**
 * IoNode — Input/Output node on the bento grid.
 *
 * Smaller card, centered in the slot, muted color.
 * No header actions — I/O nodes are structural.
 *
 * During execution, the output node sublabel updates to show
 * file count when completed.
 */

export const IoNode = memo(function IoNode({ id, data, selected }: NodeProps<BentoNode>) {
  const status = data.status ?? "idle";
  const isCompleted = status === "completed";
  const isOutput = id === "output";

  const sublabel = isOutput && isCompleted ? "Ready" : data.sublabel;

  return (
    <NodeRoot
      width={data.width}
      height={data.height}
      elevation={selected ? "md" : "sm"}
      color="muted"
      align={id === "input" ? "end" : "start"}
      selected={selected}
    >
      <NodeBody>
        <NodeIcon icon={isOutput && isCompleted ? "check-circle" : data.icon} />
        <NodeLabel>{data.label}</NodeLabel>
        <NodeSublabel>{sublabel}</NodeSublabel>
      </NodeBody>
    </NodeRoot>
  );
});
