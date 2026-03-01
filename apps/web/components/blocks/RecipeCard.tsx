"use client";

import type { Execution, WorkflowListItem } from "@bnto/core";
import { BlocksIcon } from "@/components/ui/icons";
import { Badge } from "@/components/ui/Badge";
import { Card } from "@/components/ui/Card";
import { Heading } from "@/components/ui/Heading";
import { IconBadge } from "@/components/ui/IconBadge";
import { Pressable } from "@/components/ui/Pressable";
import { Row } from "@/components/ui/Row";
import { Skeleton } from "@/components/ui/Skeleton";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/* ── Status helpers ──────────────────────────────────────────── */

type ExecutionStatus = Execution["status"];

const STATUS_VARIANT: Record<ExecutionStatus, "default" | "primary" | "secondary" | "destructive"> = {
  pending: "default",
  running: "secondary",
  completed: "primary",
  failed: "destructive",
};

const STATUS_LABEL: Record<ExecutionStatus, string> = {
  pending: "Pending",
  running: "Running",
  completed: "Completed",
  failed: "Failed",
};

/* ── Props ───────────────────────────────────────────────────── */

interface RecipeCardProps {
  recipe: WorkflowListItem;
  /** Most recent execution status, if available. */
  lastStatus?: ExecutionStatus;
  /** Click handler — parent controls navigation. */
  onClick?: () => void;
}

/* ── Skeleton ────────────────────────────────────────────────── */

export function RecipeCardSkeleton() {
  return (
    <Card elevation="md" className="flex flex-col justify-between p-5">
      <Row align="start" justify="between">
        <Skeleton className="size-10 rounded-lg" />
        <Skeleton className="h-5 w-16 rounded-full" />
      </Row>
      <Stack className="mt-auto gap-1.5 pt-4">
        <Skeleton className="h-5 w-3/4" />
        <Skeleton className="h-4 w-1/2" />
      </Stack>
    </Card>
  );
}

/* ── Component ───────────────────────────────────────────────── */

export function RecipeCard({ recipe, lastStatus, onClick }: RecipeCardProps) {
  const nodeLabel = recipe.nodeCount === 1 ? "1 node" : `${recipe.nodeCount} nodes`;

  return (
    <Pressable asChild className="h-full text-left" onClick={onClick}>
      <Card elevation="md" className="flex h-full flex-col justify-between p-5">
        <Row align="start" justify="between">
          <IconBadge variant="primary" size="lg" shape="square">
            <BlocksIcon className="size-5" />
          </IconBadge>
          {lastStatus && (
            <Badge variant={STATUS_VARIANT[lastStatus]} size="sm">
              {STATUS_LABEL[lastStatus]}
            </Badge>
          )}
        </Row>

        <Stack className="mt-auto gap-1.5 pt-4">
          <Heading level={3} size="xs" className="text-left">
            {recipe.name}
          </Heading>
          <Row className="gap-2">
            <Text as="span" size="xs" color="muted">
              {nodeLabel}
            </Text>
            <Text as="span" size="xs" color="muted">
              &middot;
            </Text>
            <Text as="span" size="xs" color="muted">
              {formatTimeAgo(recipe.updatedAt)}
            </Text>
          </Row>
        </Stack>
      </Card>
    </Pressable>
  );
}
