import { memo } from "react";
import type { NodeProps } from "@xyflow/react";
import type { BentoNode } from "../../adapters/types";
import {
  NodeRoot,
  NodeHeader,
  NodeBody,
  NodeIcon,
  NodeLabel,
  NodeSublabel,
  NodeDeleteButton,
} from "./Node";

/**
 * CompartmentNode — a processing node on the bento grid.
 *
 * Full-size card with action buttons in overlaid zones.
 * NodeRoot owns interaction state (selected → pressed, status → elevation).
 *
 * Add-node actions are handled by divider nodes between nodes on the canvas.
 * Containers are always expanded — no collapse toggle needed.
 */

export const CompartmentNode = memo(function CompartmentNode({
  id,
  data,
  selected,
}: NodeProps<BentoNode>) {
  const status = data.status ?? "idle";
  const isFailed = status === "failed";

  return (
    <NodeRoot
      width={data.width}
      height={data.height}
      selected={selected}
      status={status}
      aria-label={`${data.label} node`}
    >
      <NodeHeader visible={selected}>
        <NodeDeleteButton nodeId={id} selected={selected} />
      </NodeHeader>
      <NodeBody>
        <NodeIcon icon={data.icon} variant={data.variant} onSurface={isFailed} />
        <NodeLabel onSurface={isFailed}>{data.label}</NodeLabel>
        {data.sublabel && <NodeSublabel onSurface={isFailed}>{data.sublabel}</NodeSublabel>}
      </NodeBody>
    </NodeRoot>
  );
});
