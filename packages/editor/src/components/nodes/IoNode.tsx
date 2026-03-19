import { memo } from "react";
import type { NodeProps } from "@xyflow/react";
import type { BentoNode } from "../../adapters/types";
import { NodeRoot, NodeBody, NodeIcon, NodeLabel, NodeSublabel, NodeHandles } from "./Node";

/**
 * IoNode — Input/Output node on the bento grid.
 *
 * Smaller card, centered in the slot, muted color.
 * No header actions — I/O nodes are structural.
 * NodeRoot owns interaction state (selected → pressed, status → elevation).
 */

export const IoNode = memo(function IoNode({ id, data, selected }: NodeProps<BentoNode>) {
  const isInput = id === "input";
  return (
    <NodeRoot
      width={data.width}
      height={data.height}
      variant="muted"
      align={isInput ? "end" : "start"}
      selected={selected}
      status={data.status}
      dormant={data.dormant}
      aria-label={`${data.label} node`}
    >
      <NodeHandles sourceOnly={isInput} targetOnly={!isInput} />
      <NodeBody>
        <NodeIcon icon={data.icon} variant={data.variant} />
        <NodeLabel>{data.label}</NodeLabel>
        {data.sublabel && <NodeSublabel>{data.sublabel}</NodeSublabel>}
      </NodeBody>
    </NodeRoot>
  );
});
