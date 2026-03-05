"use client";

import { Card, Text } from "@bnto/ui";

interface Stat {
  value: string;
  label: string;
}

interface StatsCardProps {
  title: string;
  stats: Stat[];
  /** When true, stat values use muted color (e.g., placeholder dashes). */
  muted?: boolean;
  /** Highlight color class for a specific stat value (e.g., "text-primary"). */
  highlightIndex?: number;
}

export function StatsCard({ title, stats, muted, highlightIndex }: StatsCardProps) {
  return (
    <Card className="h-full space-y-3 p-4" elevation="sm">
      <Text size="sm" weight="semibold">
        {title}
      </Text>
      <div className="grid grid-cols-3 gap-3 text-center">
        {stats.map((stat, i) => (
          <div key={stat.label}>
            <p
              className={`text-lg font-bold tabular-nums ${
                muted
                  ? "text-muted-foreground"
                  : highlightIndex === i
                    ? "text-primary"
                    : ""
              }`}
            >
              {stat.value}
            </p>
            <p className="text-xs text-muted-foreground">{stat.label}</p>
          </div>
        ))}
      </div>
    </Card>
  );
}
