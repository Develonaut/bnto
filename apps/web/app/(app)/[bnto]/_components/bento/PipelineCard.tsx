"use client";

import { Card, CardContent, Text, Stack } from "@bnto/ui";
import type { ProcessingNode } from "../../_utils/extractProcessingNodes";
import { PipelineNodeRow } from "./PipelineNodeRow";

interface PipelineCardProps {
  nodes: ProcessingNode[];
  nodeProgress: Record<string, "pending" | "running" | "completed" | "failed">;
}

/** Ordered node list with execution status indicators. */
export function PipelineCard({ nodes, nodeProgress }: PipelineCardProps) {
  if (nodes.length === 0) {
    return (
      <Card elevation="sm" className="flex items-center justify-center p-5">
        <Text size="sm" color="muted">
          No processing nodes
        </Text>
      </Card>
    );
  }

  return (
    <Card elevation="sm" className="p-5">
      <CardContent className="p-0">
        <Text size="xs" color="muted" className="mb-3 font-medium uppercase tracking-wider">
          Pipeline
        </Text>
        <Stack gap="sm">
          {nodes.map((node, i) => (
            <PipelineNodeRow
              key={node.id}
              node={node}
              index={i}
              status={nodeProgress[node.id] ?? "pending"}
            />
          ))}
        </Stack>
      </CardContent>
    </Card>
  );
}
