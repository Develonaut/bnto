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
import { resolveNodePresentation } from "./resolveNodePresentation";

/**
 * CompartmentNode — a processing node on the bento grid.
 *
 * Full-size card, higher elevation, delete button when selected.
 * During execution, status drives Pressable props (pressed/hovered/active)
 * and progress fills the node left-to-right.
 */

export type CompartmentStatus = "idle" | "pending" | "active" | "completed" | "failed";

export const CompartmentNode = memo(function CompartmentNode({
  id,
  data,
  selected,
}: NodeProps<BentoNode>) {
  const status = (data.status ?? "idle") as CompartmentStatus;
  const presentation = resolveNodePresentation(status, selected ?? false);

  return (
    <NodeRoot
      width={data.width}
      height={data.height}
      elevation={presentation.elevation}
      muted={presentation.muted}
      selected={selected}
      pressed={presentation.pressed}
      hovered={presentation.hovered}
      active={presentation.active}
      failed={presentation.failed}
      progress={data.progress}
      status={status}
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
