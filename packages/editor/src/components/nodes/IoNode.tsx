import { memo } from "react";
import type { NodeProps } from "@xyflow/react";
import type { BentoNode } from "../../adapters/types";
import { NodeRoot, NodeBody, NodeIcon, NodeLabel } from "./Node";

/**
 * IoNode — Input/Output node on the bento grid.
 *
 * Smaller card, centered in the slot, muted color.
 * No header actions — I/O nodes are structural.
 * NodeRoot owns interaction state (selected → pressed, status → elevation).
 */

export const IoNode = memo(function IoNode({ id, data, selected }: NodeProps<BentoNode>) {
  return (
    <NodeRoot
      width={data.width}
      height={data.height}
      variant="muted"
      align={id === "input" ? "end" : "start"}
      selected={selected}
      status={data.status}
      aria-label={`${data.label} node`}
    >
      <NodeBody>
        <NodeIcon icon={data.icon} variant={data.variant} />
        <NodeLabel>{data.label}</NodeLabel>
      </NodeBody>
    </NodeRoot>
  );
});
