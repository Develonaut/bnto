"use client";

import { useEffect, useState } from "react";
import { CheckCircle2Icon, LoaderIcon, XCircleIcon, ClockIcon } from "@/components/ui/icons";
import { useExecution } from "@bnto/core";
import type { Execution, NodeProgress } from "@bnto/core";
import { cn } from "@/lib/cn";

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
  const { data: execution, isLoading } = useExecution(executionId);
  const elapsed = useElapsedTime(execution);

  if (isLoading || !execution) {
    return (
      <div
        className="rounded-lg border border-border bg-card p-4"
        data-testid="execution-progress"
        data-status="loading"
      >
        <div className="flex items-center gap-3">
          <LoaderIcon className="size-5 shrink-0 text-primary motion-safe:animate-spin" />
          <p className="text-sm text-muted-foreground">
            Starting execution&hellip;
          </p>
        </div>
      </div>
    );
  }

  return (
    <div
      className="space-y-3 rounded-lg border border-border bg-card p-4"
      data-testid="execution-progress"
      data-status={execution.status}
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <StatusIcon status={execution.status} />
          <p className="text-sm font-medium text-foreground">
            {getStatusLabel(execution.status)}
          </p>
        </div>
        <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
          <ClockIcon className="size-3.5" />
          <span>{formatElapsed(elapsed)}</span>
        </div>
      </div>

      {execution.progress.length > 0 && (
        <ul className="space-y-1.5">
          {execution.progress.map((node, index) => (
            <NodeProgressRow key={`${node.nodeId}-${index}`} node={node} />
          ))}
        </ul>
      )}

      {execution.status === "failed" && execution.error && (
        <div
          className="rounded-md border border-destructive/30 bg-destructive/5 p-3"
          data-testid="execution-error"
        >
          <p className="text-sm text-destructive">{friendlyError(execution.error)}</p>
          {execution.error !== friendlyError(execution.error) && (
            <p className="mt-1 text-xs text-muted-foreground">
              {execution.error}
            </p>
          )}
        </div>
      )}
    </div>
  );
}

function NodeProgressRow({ node }: { node: NodeProgress }) {
  const isComplete = node.status === "completed";
  const isFailed = node.status === "failed";

  return (
    <li
      className="flex items-center gap-2 text-sm"
      data-testid="node-progress"
      data-node-id={node.nodeId}
      data-node-status={node.status}
    >
      <span
        className={cn(
          "size-2 rounded-full",
          isComplete && "bg-green-500",
          isFailed && "bg-destructive",
          !isComplete && !isFailed && "bg-primary motion-safe:animate-pulse",
        )}
      />
      <span className="text-muted-foreground">{node.nodeId}</span>
      <span className="ml-auto text-xs text-muted-foreground">
        {node.status}
      </span>
    </li>
  );
}

function StatusIcon({ status }: { status: Execution["status"] }) {
  switch (status) {
    case "pending":
    case "running":
      return (
        <LoaderIcon className="size-5 shrink-0 text-primary motion-safe:animate-spin" />
      );
    case "completed":
      return (
        <CheckCircle2Icon className="size-5 shrink-0 text-green-600 dark:text-green-400" />
      );
    case "failed":
      return <XCircleIcon className="size-5 shrink-0 text-destructive" />;
  }
}

function getStatusLabel(status: Execution["status"]): string {
  switch (status) {
    case "pending":
      return "Queued\u2026";
    case "running":
      return "Processing\u2026";
    case "completed":
      return "Completed";
    case "failed":
      return "Failed";
  }
}

/** Count seconds since execution started, ticking every second while active. */
function useElapsedTime(execution: Execution | null | undefined) {
  const [elapsed, setElapsed] = useState(0);
  const startedAt = execution?.startedAt;
  const completedAt = execution?.completedAt;
  const isActive =
    execution?.status === "pending" || execution?.status === "running";

  useEffect(() => {
    const compute = () => {
      if (!startedAt) return 0;
      return Math.floor(((completedAt ?? Date.now()) - startedAt) / 1000);
    };

    // Sync elapsed with external time source on each subscription change.
    // eslint-disable-next-line react-hooks/set-state-in-effect -- timer sync with Date.now()
    setElapsed(compute());

    if (!startedAt || !isActive) return;
    const interval = setInterval(() => setElapsed(compute()), 1000);
    return () => clearInterval(interval);
  }, [startedAt, completedAt, isActive]);

  return elapsed;
}

/** Translate backend error strings into user-friendly messages. */
function friendlyError(raw: string): string {
  if (raw.includes("file transit not configured"))
    return "The file processing server isn't fully configured. Please try again later.";
  if (raw.includes("GO_API_URL not configured"))
    return "The processing server isn't available right now. Please try again later.";
  if (raw.includes("timed out") || raw.includes("polling limit"))
    return "The execution took too long and was stopped. Try with fewer or smaller files.";
  if (raw.includes("Poll error"))
    return "Lost connection to the processing server. Your execution may still be running.";
  if (raw.match(/Go API returned [45]\d\d/))
    return "The processing server returned an error. Please try again.";
  return raw;
}

function formatElapsed(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}m ${s}s`;
}
