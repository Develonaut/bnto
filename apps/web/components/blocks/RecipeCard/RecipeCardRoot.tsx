import type { PropsWithChildren } from "react";
import Link from "next/link";

import type { Execution } from "@bnto/core";
import type { LucideIcon } from "@/components/ui/icons";
import { BlocksIcon } from "@/components/ui/icons";
import { Badge } from "@/components/ui/Badge";
import { Card } from "@/components/ui/Card";
import { StatusBadge } from "@/components/ui/StatusBadge";
import { Heading } from "@/components/ui/Heading";
import { IconBadge } from "@/components/ui/IconBadge";
import { Pressable } from "@/components/ui/Pressable";
import { Row } from "@/components/ui/Row";
import { Skeleton } from "@/components/ui/Skeleton";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { cn } from "@/lib/cn";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/* ── Root ────────────────────────────────────────────────────── */

type RecipeCardRootProps = PropsWithChildren<{
  onClick?: () => void;
  href?: string;
  className?: string;
}>;

export function RecipeCardRoot({ onClick, href, className, children }: RecipeCardRootProps) {
  const card = (
    <Card elevation="md" className={cn("flex h-full flex-col justify-between p-5", className)}>
      {children}
    </Card>
  );

  if (href) {
    return (
      <Link href={href} className="group h-full">
        <Pressable
          asChild
          className="flex h-full items-stretch justify-start whitespace-normal text-left"
        >
          {card}
        </Pressable>
      </Link>
    );
  }

  if (onClick) {
    return (
      <Pressable asChild className="h-full text-left" onClick={onClick}>
        {card}
      </Pressable>
    );
  }

  return card;
}

/* ── Structural sub-components ───────────────────────────────── */

function RecipeCardHeader({ children }: PropsWithChildren) {
  return <Row align="start" justify="between">{children}</Row>;
}

function RecipeCardContent({ children }: PropsWithChildren) {
  return <Stack className="mt-auto gap-1.5 pt-4">{children}</Stack>;
}

function RecipeCardFooter({ children, className }: PropsWithChildren<{ className?: string }>) {
  return <div className={className}>{children}</div>;
}

/* ── Styled slot sub-components ──────────────────────────────── */

function RecipeCardIcon({ icon: Icon }: { icon?: LucideIcon }) {
  const Glyph = Icon ?? BlocksIcon;
  return (
    <IconBadge variant="primary" size="lg" shape="square">
      <Glyph className="size-5" />
    </IconBadge>
  );
}

function RecipeCardCategory({ children }: PropsWithChildren) {
  return (
    <Text as="span" size="xs" mono color="muted" className="uppercase tracking-wider">
      {children}
    </Text>
  );
}

function RecipeCardStatus({ status }: { status: Execution["status"] }) {
  return <StatusBadge status={status} />;
}

function RecipeCardTitle({ children }: PropsWithChildren) {
  return (
    <Heading level={3} size="xs" className="text-left">
      {children}
    </Heading>
  );
}

function RecipeCardDescription({ children }: PropsWithChildren) {
  return (
    <Text size="sm" color="muted" leading="snug" className="text-left">
      {children}
    </Text>
  );
}

function RecipeCardTags({ tags, limit = 3 }: { tags: string[]; limit?: number }) {
  return (
    <Row wrap className="gap-1.5 pt-1">
      {tags.slice(0, limit).map((tag) => (
        <Badge key={tag} size="sm">{tag}</Badge>
      ))}
    </Row>
  );
}

function RecipeCardMeta({ nodeCount, updatedAt }: { nodeCount: number; updatedAt: number }) {
  const nodeLabel = nodeCount === 1 ? "1 node" : `${nodeCount} nodes`;
  return (
    <Row className="gap-2">
      <Text as="span" size="xs" color="muted">{nodeLabel}</Text>
      <Text as="span" size="xs" color="muted">&middot;</Text>
      <Text as="span" size="xs" color="muted">{formatTimeAgo(updatedAt)}</Text>
    </Row>
  );
}

/* ── Skeleton ────────────────────────────────────────────────── */

function RecipeCardSkeleton() {
  return (
    <Card elevation="md" className="flex h-full flex-col justify-between p-5">
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

/* ── Exports (used by index.ts Object.assign) ────────────────── */

export {
  RecipeCardHeader,
  RecipeCardContent,
  RecipeCardFooter,
  RecipeCardIcon,
  RecipeCardCategory,
  RecipeCardStatus,
  RecipeCardTitle,
  RecipeCardDescription,
  RecipeCardTags,
  RecipeCardMeta,
  RecipeCardSkeleton,
};
