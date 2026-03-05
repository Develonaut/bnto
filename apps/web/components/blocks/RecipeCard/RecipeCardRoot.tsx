import type { PropsWithChildren } from "react";
import Link from "next/link";

import type { Execution } from "@bnto/core";
import {
  Badge,
  BlocksIcon,
  Card,
  Heading,
  IconBadge,
  Pressable,
  Row,
  Stack,
  Text,
  cn,
  type LucideIcon,
} from "@bnto/ui";
import { StatusBadge } from "@/components/blocks/StatusBadge";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/* ── Root ────────────────────────────────────────────────────── */

type RecipeCardRootProps = PropsWithChildren<{
  onClick?: () => void;
  href?: string;
  className?: string;
  /** Grounded loading state — card springs up when loading clears. */
  loading?: boolean;
}>;

export function RecipeCardRoot({
  onClick,
  href,
  className,
  loading,
  children,
}: RecipeCardRootProps) {
  const card = (
    <Card loading={loading} className={cn("flex h-full flex-col justify-between p-5", className)}>
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
  return (
    <Row align="start" justify="between">
      {children}
    </Row>
  );
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
    <Heading level={3} as="p" size="xs" className="text-left">
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
        <Badge key={tag} size="sm">
          {tag}
        </Badge>
      ))}
    </Row>
  );
}

function RecipeCardMeta({ nodeCount, updatedAt }: { nodeCount: number; updatedAt: number }) {
  const nodeLabel = nodeCount === 1 ? "1 node" : `${nodeCount} nodes`;
  return (
    <Row className="gap-2">
      <Text as="span" size="xs" color="muted">
        {nodeLabel}
      </Text>
      <Text as="span" size="xs" color="muted">
        &middot;
      </Text>
      <Text as="span" size="xs" color="muted">
        {formatTimeAgo(updatedAt)}
      </Text>
    </Row>
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
};
