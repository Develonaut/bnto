import type { PropsWithChildren } from "react";
import Link from "next/link";

import type { Execution } from "@bnto/core";
import { RecipeCard as RecipeCardBase, Pressable, Row, Text } from "@bnto/ui";
import { StatusBadge } from "@/components/blocks/StatusBadge";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/* ── Root — extends @bnto/ui RecipeCard with Next.js Link support ── */

type RecipeCardRootProps = PropsWithChildren<{
  onClick?: () => void;
  href?: string;
  className?: string;
  loading?: boolean;
}>;

export function RecipeCardRoot({
  onClick,
  href,
  className,
  loading,
  children,
}: RecipeCardRootProps) {
  if (href) {
    return (
      <Link href={href} className="group h-full">
        <Pressable
          asChild
          className="flex h-full items-stretch justify-start whitespace-normal text-left"
        >
          <RecipeCardBase loading={loading} className={className}>
            {children}
          </RecipeCardBase>
        </Pressable>
      </Link>
    );
  }

  return (
    <RecipeCardBase onClick={onClick} loading={loading} className={className}>
      {children}
    </RecipeCardBase>
  );
}

/* ── Domain sub-components (not in @bnto/ui) ─────────────────── */

function RecipeCardStatus({ status }: { status: Execution["status"] }) {
  return <StatusBadge status={status} />;
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

export { RecipeCardStatus, RecipeCardMeta };
