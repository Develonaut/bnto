import { memo } from "react";
import type { NodeProps } from "@xyflow/react";
import type { BentoNode } from "../../adapters/types";
import { NodeRoot, NodeHeader, NodeBody, NodeIcon, NodeLabel, NodeDeleteButton } from "./Node";

/**
 * CompartmentNode — a processing node on the bento grid.
 *
 * Full-size card, delete button when selected.
 * NodeRoot owns interaction state (selected → pressed, status → elevation).
 */

export const CompartmentNode = memo(function CompartmentNode({
  id,
  data,
  selected,
}: NodeProps<BentoNode>) {
  const status = data.status ?? "idle";
  const isFailed = status === "failed";

  return (
    <NodeRoot width={data.width} height={data.height} selected={selected} status={status}>
      <NodeHeader>
        <NodeDeleteButton nodeId={id} selected={selected} />
      </NodeHeader>
      <NodeBody>
        <NodeIcon icon={data.icon} variant={data.variant} onSurface={isFailed} />
        <NodeLabel onSurface={isFailed}>{data.label}</NodeLabel>
      </NodeBody>
    </NodeRoot>
  );
});
