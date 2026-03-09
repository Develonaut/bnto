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
 * to animate the card through elevation states.
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
      color={presentation.color}
      muted={presentation.muted}
      pressed={presentation.pressed}
      hovered={presentation.hovered}
      active={presentation.active}
      status={status}
    >
      <NodeHeader>
        <NodeDeleteButton nodeId={id} selected={selected} />
      </NodeHeader>
      <NodeBody>
        <NodeIcon icon={data.icon} variant={data.variant} />
        <NodeLabel>{data.label}</NodeLabel>
        <NodeSublabel>{data.sublabel}</NodeSublabel>
      </NodeBody>
    </NodeRoot>
  );
});
