"use client";

/**
 * DevNodeControls — per-node dev controls for forcing execution status
 * and progress on individual nodes.
 *
 * Lists all non-I/O nodes from the store. Each node shows:
 * - Label and current status badge
 * - Status stepper buttons (pending/active/completed)
 * - Progress slider (0–100)
 */

import { useCallback } from "react";
import { Badge, Button, Row, Slider, Stack, Text } from "@bnto/ui";
import { isIoNodeType } from "@bnto/nodes";
import { useEditorStore } from "../../hooks/useEditorStore";
import { getEditorStore } from "../../store/instance";
import type { NodeExecutionStatus } from "../../store/types";

const STATUS_STEPS: NodeExecutionStatus[] = ["idle", "pending", "active", "completed", "failed"];

const STATUS_COLORS: Record<NodeExecutionStatus, "outline" | "secondary" | "destructive"> = {
  idle: "outline",
  pending: "secondary",
  active: "secondary",
  completed: "secondary",
  failed: "destructive",
};

function DevNodeControls() {
  const nodes = useEditorStore((s) => s.nodes);
  const configs = useEditorStore((s) => s.configs);
  const executionState = useEditorStore((s) => s.executionState);
  const nodeProgress = useEditorStore((s) => s.nodeProgress);

  const processingNodes = nodes.filter((n) => {
    const config = configs[n.id];
    return config && !isIoNodeType(config.nodeType);
  });

  const setNodeStatus = useCallback((nodeId: string, status: NodeExecutionStatus) => {
    const store = getEditorStore();
    store.setState((s) => ({
      executionState: { ...s.executionState, [nodeId]: status },
    }));
  }, []);

  const setNodeProgress = useCallback((nodeId: string, percent: number) => {
    const store = getEditorStore();
    store.setState((s) => ({
      nodeProgress: { ...s.nodeProgress, [nodeId]: percent },
    }));
  }, []);

  if (processingNodes.length === 0) {
    return (
      <Text size="xs" color="muted">
        No processing nodes
      </Text>
    );
  }

  return (
    <Stack className="gap-3">
      <Text size="xs" color="muted" weight="medium">
        Per-Node Controls
      </Text>
      {processingNodes.map((node) => {
        const config = configs[node.id];
        const status = executionState[node.id] ?? "idle";
        const progress = nodeProgress[node.id] ?? 0;

        return (
          <Stack key={node.id} className="gap-1.5 rounded-md border border-border p-2">
            <Row className="items-center justify-between">
              <Text size="xs" weight="medium" className="truncate">
                {config?.displayName ?? config?.name ?? node.id}
              </Text>
              <Badge variant={STATUS_COLORS[status as NodeExecutionStatus]}>{status}</Badge>
            </Row>
            <Row gap="xs" className="flex-wrap">
              {STATUS_STEPS.map((s) => (
                <Button
                  key={s}
                  variant={status === s ? "primary" : "outline"}
                  size="sm"
                  className="h-6 px-1.5 text-[10px]"
                  onClick={() => setNodeStatus(node.id, s)}
                >
                  {s}
                </Button>
              ))}
            </Row>
            <Row className="items-center gap-2">
              <Text size="xs" color="muted" className="shrink-0 font-mono w-8 text-right">
                {progress}%
              </Text>
              <Slider
                value={[progress]}
                onValueChange={([v]) => setNodeProgress(node.id, v)}
                min={0}
                max={100}
                step={1}
              />
            </Row>
          </Stack>
        );
      })}
    </Stack>
  );
}

export { DevNodeControls };
