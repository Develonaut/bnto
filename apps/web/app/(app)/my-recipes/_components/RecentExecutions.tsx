"use client";

import { core } from "@bnto/core";

import { useDelayedLoading } from "../_hooks/useDelayedLoading";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { EmptyState } from "@/components/ui/EmptyState";
import { Row } from "@/components/ui/Row";
import { Skeleton } from "@/components/ui/Skeleton";
import { Stack } from "@/components/ui/Stack";
import { StatusBadge } from "@/components/ui/StatusBadge";
import { Text } from "@/components/ui/Text";
import { HistoryIcon } from "@/components/ui/icons";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/**
 * Recent executions list — self-fetching via useExecutionHistory().
 * Shows up to 5 most recent runs with status, timing, and recipe info.
 *
 * The outer min-h-[280px] wrapper keeps the tab panel stable across
 * skeleton → loaded → empty transitions. All three states render
 * inside the same container so nothing jumps.
 */
export function RecentExecutions() {
  const { items, isLoading } = core.executions.useExecutionHistory({
    pageSize: 5,
  });
  const showSkeleton = useDelayedLoading(isLoading);

  if (showSkeleton) return <RecentExecutionsSkeleton />;
  if (isLoading) return null;

  if (items.length === 0) {
    return (
      <div className="min-h-[240px]">
        <EmptyState size="sm">
          <EmptyState.Icon size="sm">
            <HistoryIcon />
          </EmptyState.Icon>
          <EmptyState.Title>No runs yet</EmptyState.Title>
          <EmptyState.Description>
            Your recent recipe runs will appear here.
          </EmptyState.Description>
        </EmptyState>
      </div>
    );
  }

  return (
    <Stack className="gap-2">
      {items.map((execution) => (
        <Card key={execution.id} elevation="sm" className="px-4 py-3">
          <Row justify="between" align="center" className="gap-3">
            <Stack className="min-w-0 flex-1 gap-0.5">
              <Text size="sm" weight="medium" className="truncate">
                {execution.workflowId ? "Saved workflow" : "Quick run"}
              </Text>
              <Text size="xs" color="muted">
                {formatTimeAgo(execution.startedAt)}
                {execution.completedAt &&
                  ` \u00B7 ${formatDuration(execution.startedAt, execution.completedAt)}`}
              </Text>
            </Stack>
            <StatusBadge status={execution.status} />
          </Row>
        </Card>
      ))}

      <Button variant="ghost" href="/executions" className="self-start">
        View all runs
      </Button>
    </Stack>
  );
}

/**
 * Skeleton — 5 rows matching the loaded card layout exactly.
 * Same Card, same padding, same Row layout, same inner stack gap.
 * The 5-row count matches pageSize so the skeleton ≈ loaded height.
 */
function RecentExecutionsSkeleton() {
  return (
    <Stack className="gap-2">
      {Array.from({ length: 5 }).map((_, i) => (
        <Card key={i} elevation="sm" className="px-4 py-3">
          <Row justify="between" align="center" className="gap-3">
            <Stack className="min-w-0 flex-1 gap-0.5">
              <Skeleton className="h-5 w-28" />
              <Skeleton className="h-3.5 w-20" />
            </Stack>
            <Skeleton className="h-5 w-16 rounded-full" />
          </Row>
        </Card>
      ))}
    </Stack>
  );
}

/** Format the duration between two timestamps as a human-readable string. */
function formatDuration(start: number, end: number): string {
  const ms = end - start;
  if (ms < 1000) return `${ms}ms`;
  if (ms < 60_000) return `${(ms / 1000).toFixed(1)}s`;
  const mins = Math.floor(ms / 60_000);
  const secs = Math.round((ms % 60_000) / 1000);
  return secs > 0 ? `${mins}m ${secs}s` : `${mins}m`;
}
