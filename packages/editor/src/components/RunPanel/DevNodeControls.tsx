"use client";

/**
 * DevNodeControls — per-node dev controls for forcing execution status
 * and progress on individual nodes.
 *
 * Lists all non-I/O nodes from the store. Each node shows:
 * - Label and current status badge
 * - Status stepper buttons (pending/active/completed)
 * - Progress slider (0-100)
 */

import { useCallback, useRef, useState } from "react";
import { Badge, Button, Row, Slider, Stack, Text } from "@bnto/ui";
import { isIoNodeType } from "@bnto/nodes";
import { useEditor } from "../../context";
import type { NodeExecutionStatus } from "../../store/types";

/** Delay helper for simulation timing. */
function wait(ms: number) {
  return new Promise<void>((resolve) => setTimeout(resolve, ms));
}

const STATUS_STEPS: NodeExecutionStatus[] = ["idle", "pending", "active", "completed", "failed"];

const STATUS_COLORS: Record<NodeExecutionStatus, "outline" | "secondary" | "destructive"> = {
  idle: "outline",
  pending: "secondary",
  active: "secondary",
  completed: "secondary",
  failed: "destructive",
};

function DevNodeControls() {
  const editor = useEditor();
  const { nodes, configs } = editor.nodes.useNodes();
  const { executionState, nodeProgress } = editor.execution.useExecution();

  const processingNodes = nodes.filter((n) => {
    const config = configs[n.id];
    return config && !isIoNodeType(config.nodeType);
  });

  const setNodeStatus = useCallback(
    (nodeId: string, status: NodeExecutionStatus) => {
      editor.execution.setNodeStatus(nodeId, status);
    },
    [editor],
  );

  const setNodeProgress = useCallback(
    (nodeId: string, percent: number) => {
      editor.execution.setNodeProgress(nodeId, percent);
    },
    [editor],
  );

  const [simulating, setSimulating] = useState(false);
  const cancelRef = useRef(false);

  const simulate = useCallback(async () => {
    if (processingNodes.length === 0) return;
    cancelRef.current = false;
    setSimulating(true);

    // Reset all nodes to pending
    const allIds = processingNodes.map((n) => n.id);
    const pendingState: Record<string, NodeExecutionStatus> = {};
    const zeroProgress: Record<string, number> = {};
    for (const id of allIds) {
      pendingState[id] = "pending";
      zeroProgress[id] = 0;
    }
    editor.execution.forceExecutionState({
      executionState: pendingState,
      nodeProgress: zeroProgress,
    });
    await wait(400);

    // Walk each node through active -> progress -> completed
    for (const id of allIds) {
      if (cancelRef.current) break;

      editor.execution.setNodeStatus(id, "active");

      // Animate progress 0 -> 100 in steps
      const steps = 10;
      for (let i = 1; i <= steps; i++) {
        if (cancelRef.current) break;
        await wait(80);
        editor.execution.setNodeProgress(id, Math.round((i / steps) * 100));
      }

      if (cancelRef.current) break;
      editor.execution.setNodeStatus(id, "completed");
      await wait(300);
    }

    setSimulating(false);
  }, [processingNodes, editor]);

  const cancelSimulation = useCallback(() => {
    cancelRef.current = true;
    setSimulating(false);
  }, []);

  if (processingNodes.length === 0) {
    return (
      <Text size="xs" color="muted">
        No processing nodes
      </Text>
    );
  }

  return (
    <Stack className="gap-3" data-testid="per-node-controls">
      <Row className="items-center justify-between">
        <Text size="xs" color="muted" weight="medium">
          Per-Node Controls
        </Text>
        <Button
          variant={simulating ? "destructive" : "outline"}
          size="sm"
          className="h-6 px-2 text-[10px]"
          onClick={simulating ? cancelSimulation : simulate}
        >
          {simulating ? "Stop" : "Simulate"}
        </Button>
      </Row>
      {processingNodes.map((node) => {
        const config = configs[node.id];
        const status = executionState[node.id] ?? "idle";
        const progress = nodeProgress[node.id] ?? 0;

        return (
          <Stack
            key={node.id}
            className="gap-1.5 rounded-md border border-border p-2"
            data-testid={`node-control-${node.id}`}
          >
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
