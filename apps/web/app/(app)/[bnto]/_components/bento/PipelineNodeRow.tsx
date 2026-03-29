"use client";

import { Text, Row } from "@bnto/ui";
import { NODE_TYPE_INFO } from "@bnto/core";
import type { ProcessingNode } from "../../_utils/extractProcessingNodes";
import { NodeStatusIndicator } from "./NodeStatusIndicator";

interface PipelineNodeRowProps {
  node: ProcessingNode;
  index: number;
  status: "pending" | "running" | "completed" | "failed";
}

/** Single row in the pipeline node list. */
export function PipelineNodeRow({ node, index, status }: PipelineNodeRowProps) {
  const info = NODE_TYPE_INFO[node.type as keyof typeof NODE_TYPE_INFO];
  const label = info?.label ?? node.type;

  return (
    <Row className="items-center gap-2 rounded-lg bg-muted/50 px-3 py-2">
      <Text size="xs" color="muted" className="w-5 text-center font-mono">
        {index + 1}
      </Text>
      <Text size="sm" className="flex-1 truncate">
        {label}
      </Text>
      <NodeStatusIndicator status={status} />
    </Row>
  );
}
