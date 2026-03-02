"use client";

import { core } from "@bnto/core";

import { Card } from "@/components/ui/Card";
import { Row } from "@/components/ui/Row";
import { Skeleton } from "@/components/ui/Skeleton";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/**
 * Usage analytics summary — total runs, plan tier, last activity.
 * Self-fetching via useUsageAnalytics().
 */
export function UsageStats() {
  const { data, isLoading } = core.analytics.useUsageAnalytics();

  if (isLoading || !data) return <UsageStatsSkeleton />;

  const stats = [
    { label: "Total runs", value: String(data.totalRuns) },
    { label: "Plan", value: data.plan === "pro" ? "Pro" : "Free" },
    {
      label: "Last activity",
      value: data.lastRunAt ? formatTimeAgo(data.lastRunAt) : "Never",
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

function UsageStatsSkeleton() {
  return (
    <Row wrap className="gap-3">
      {Array.from({ length: 3 }).map((_, i) => (
        <Card key={i} elevation="sm" className="flex-1 px-4 py-3">
          <Stack className="gap-1.5">
            <Skeleton className="h-3 w-16" />
            <Skeleton className="h-6 w-12" />
          </Stack>
        </Card>
      ))}
    </Row>
  );
}
