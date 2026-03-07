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
 * Full-size card, higher elevation, delete button when selected.
 */

export type CompartmentStatus = "idle" | "pending" | "active" | "completed";

export const CompartmentNode = memo(function CompartmentNode({
  id,
  data,
  selected,
}: NodeProps<BentoNode>) {
  const status = (data.status ?? "idle") as CompartmentStatus;

  return (
    <NodeRoot
      width={data.width}
      height={data.height}
      elevation={selected ? "lg" : "md"}
      muted={status === "pending"}
      selected={selected}
    >
      <NodeHeader>
        <NodeDeleteButton nodeId={id} selected={selected} />
      </NodeHeader>
      <NodeBody>
        <NodeIcon icon={data.icon} />
        <NodeLabel>{data.label}</NodeLabel>
        <NodeSublabel>{data.sublabel}</NodeSublabel>
      </NodeBody>
    </NodeRoot>
  );
});
