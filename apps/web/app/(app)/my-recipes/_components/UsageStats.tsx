"use client";

import { core } from "@bnto/core";

import { useDelayedLoading } from "../_hooks/useDelayedLoading";
import { Card, Row, Skeleton, Stack, Text } from "@bnto/ui";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/**
 * Usage analytics summary — total runs, plan tier, last activity.
 *
 * Authenticated users see server-backed stats via useUsageAnalytics().
 * Unauthenticated users see local execution history stats from IndexedDB.
 *
 * Skeleton and loaded state use identical outer containers (Card with
 * the same padding, flex-1, elevation) so content paints in without shift.
 */
export function UsageStats() {
  const { isAuthenticated } = core.auth.useAuth();
  const { data: serverData, isLoading: serverLoading } = core.user.useUsageAnalytics();
  const { items: localItems, isLoading: localLoading } =
    core.executions.useExecutionHistory({ pageSize: 10 });

  const isLoading = isAuthenticated ? serverLoading : localLoading;
  const showSkeleton = useDelayedLoading(isLoading);

  if (showSkeleton) return <UsageStatsSkeleton />;
  if (isLoading) return null;

  const stats = isAuthenticated && serverData
    ? [
        { label: "Total runs", value: String(serverData.totalRuns) },
        { label: "Plan", value: serverData.plan === "pro" ? "Pro" : "Free" },
        {
          label: "Last activity",
          value: serverData.lastRunAt ? formatTimeAgo(serverData.lastRunAt) : "Never",
        },
      ]
    : [
        { label: "Total runs", value: String(localItems.length) },
        { label: "Plan", value: "Free" },
        {
          label: "Last activity",
          value: localItems.length > 0
            ? formatTimeAgo(localItems[0].startedAt)
            : "Never",
        },
      ];

  return (
    <Row wrap className="gap-3">
      {stats.map((stat) => (
        <Card key={stat.label} elevation="sm" className="flex-1 px-4 py-3">
          <Stack className="gap-0.5">
            <Text size="xs" color="muted" className="uppercase tracking-wider">
              {stat.label}
            </Text>
            <Text size="lg" weight="semibold">
              {stat.value}
            </Text>
          </Stack>
        </Card>
      ))}
    </Row>
  );
}

/**
 * Skeleton — matches the loaded layout exactly:
 * same Card, same padding, same gap, same inner stack gap (0.5).
 * Skeleton rectangles match the font sizes they replace.
 */
function UsageStatsSkeleton() {
  return (
    <Row wrap className="gap-3">
      {Array.from({ length: 3 }).map((_, i) => (
        <Card key={i} loading elevation="sm" className="flex-1 px-4 py-3">
          <Stack className="gap-0.5">
            <Skeleton className="h-3.5 w-16" />
            <Skeleton className="h-7 w-12" />
          </Stack>
        </Card>
      ))}
    </Row>
  );
}
