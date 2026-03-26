"use client";

import { core } from "@bnto/core";
import { useElapsedTime } from "../_hooks/useElapsedTime";
import { ExecutionProgressLoading } from "./ExecutionProgressLoading";
import { ExecutionProgressHeader } from "./ExecutionProgressHeader";
import { NodeProgressRow } from "./NodeProgressRow";
import { ExecutionErrorPanel } from "./ExecutionErrorPanel";

interface ExecutionProgressProps {
  executionId: string;
}

/**
 * Real-time execution progress display.
 *
 * Subscribes to execution status via Convex and shows:
 * - Per-node progress with status indicators
 * - Elapsed time counter
 * - Error message on failure
 */
export function ExecutionProgress({ executionId }: ExecutionProgressProps) {
  const { data: execution, isLoading } = core.executions.useExecution(executionId);
  const isActive = execution?.status === "pending" || execution?.status === "running";
  const elapsed = useElapsedTime(execution?.startedAt, isActive);

  if (isLoading || !execution) {
    return <ExecutionProgressLoading />;
  }

  return (
    <div
      className="space-y-3 rounded-lg border border-border bg-card p-4"
      data-testid="execution-progress"
      data-status={execution.status}
    >
      <ExecutionProgressHeader status={execution.status} elapsed={elapsed} />

      {execution.progress.length > 0 && (
        <ul className="space-y-1.5">
          {execution.progress.map((node, index) => (
            <NodeProgressRow key={`${node.nodeId}-${index}`} node={node} />
          ))}
        </ul>
      )}

      {execution.status === "failed" && execution.error && (
        <ExecutionErrorPanel error={execution.error} />
      )}
    </div>
  );
}
