"use client";

import { core } from "@bnto/core";

import { useDelayedLoading } from "../_hooks/useDelayedLoading";
import { Button, Card, EmptyState, HistoryIcon, Skeleton, Stack, Text } from "@bnto/ui";
import { getBntoBySlug } from "@/lib/bntoRegistry";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/** Resolve a slug to a human-readable recipe name. */
function getRecipeName(slug?: string): string {
  if (!slug) return "Quick run";
  return getBntoBySlug(slug)?.h1 ?? slugToTitle(slug);
}

/** Fallback: convert a hyphenated slug to title case. */
function slugToTitle(slug: string): string {
  return slug
    .split("-")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");
}

/**
 * Paginated execution history — self-fetching via useExecutionHistory().
 *
 * Routes transparently: authenticated users see Convex-backed history,
 * unauthenticated users see browser-local (IndexedDB) history.
 *
 * Read-only view — recipe name + timestamp. Observable, not actionable.
 */
export function ExecutionHistory() {
  const { items, isLoading, status, loadMore } =
    core.executions.useExecutionHistory({ pageSize: 20 });
  const showSkeleton = useDelayedLoading(isLoading);

  if (showSkeleton) return <ExecutionHistorySkeleton />;
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
            Run a recipe to see your execution history here.
          </EmptyState.Description>
        </EmptyState>
      </div>
    );
  }

  return (
    <Stack className="gap-2">
      {items.map((execution) => {
        const recipeName = getRecipeName(execution.slug);

        return (
          <Card key={execution.id} elevation="sm" className="px-4 py-3">
            <Stack className="min-w-0 gap-0.5">
              <Text size="sm" weight="medium" className="truncate">
                {recipeName}
              </Text>
              <Text size="xs" color="muted">
                {formatTimeAgo(execution.startedAt)}
                {execution.completedAt &&
                  ` \u00B7 ${formatDuration(execution.startedAt, execution.completedAt)}`}
              </Text>
            </Stack>
          </Card>
        );
      })}

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

/**
 * Skeleton — 6 rows matching the loaded card layout.
 * Same Card, same padding, same Row layout, same inner stack gap.
 */
function ExecutionHistorySkeleton() {
  return (
    <Stack className="gap-2">
      {Array.from({ length: 6 }).map((_, i) => (
        <Card key={i} loading elevation="sm" className="px-4 py-3">
          <Stack className="min-w-0 gap-0.5">
            <Skeleton className="h-5 w-28" />
            <Skeleton className="h-3.5 w-20" />
          </Stack>
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
