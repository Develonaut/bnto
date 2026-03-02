"use client";

import { core } from "@bnto/core";

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
 * Paginated execution history list. Self-fetching via useExecutionHistory().
 * Shows all executions with status, timing, and a "Load more" button.
 */
export function ExecutionList() {
  const { items, isLoading, status, loadMore } =
    core.executions.useExecutionHistory({ pageSize: 20 });

  if (isLoading) return <ExecutionListSkeleton />;

  if (items.length === 0) {
    return (
      <EmptyState size="md">
        <EmptyState.Icon>
          <HistoryIcon />
        </EmptyState.Icon>
        <EmptyState.Title>No executions yet</EmptyState.Title>
        <EmptyState.Description>
          Run a recipe to see your execution history here.
        </EmptyState.Description>
        <EmptyState.Action>
          <Button href="/" variant="primary">
            Browse recipes
          </Button>
        </EmptyState.Action>
      </EmptyState>
    );
  }

  return (
    <Stack className="gap-3">
      {items.map((execution) => (
        <Card key={execution.id} elevation="sm" className="px-5 py-4">
          <Row justify="between" align="center" className="gap-4">
            <Stack className="min-w-0 flex-1 gap-1">
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

      {status !== "Exhausted" && (
        <Button
          variant="ghost"
          className="self-start"
          onClick={() => loadMore(20)}
        >
          Load more
        </Button>
      )}
    </Stack>
  );
}

function ExecutionListSkeleton() {
  return (
    <Stack className="gap-3">
      {Array.from({ length: 6 }).map((_, i) => (
        <Card key={i} elevation="sm" className="px-5 py-4">
          <Row justify="between" align="center">
            <Stack className="gap-1.5">
              <Skeleton className="h-4 w-32" />
              <Skeleton className="h-3 w-20" />
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
