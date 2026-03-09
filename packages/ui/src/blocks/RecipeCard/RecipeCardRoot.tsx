import type { PropsWithChildren } from "react";

import type { LucideIcon } from "../../icons";
import { BlocksIcon } from "../../icons";
import { Row } from "../../layout/Row";
import { Stack } from "../../layout/Stack";
import { Card } from "../../surface/Card";
import { Pressable } from "../../surface/Pressable";
import { Badge } from "../../typography/Badge";
import { Heading } from "../../typography/Heading";
import { IconBadge } from "../../typography/IconBadge";
import { Text } from "../../typography/Text";
import { cn } from "../../utils/cn";

/* ── Root ────────────────────────────────────────────────────── */

type RecipeCardRootProps = PropsWithChildren<{
  onClick?: () => void;
  className?: string;
  /** Grounded loading state — card springs up when loading clears. */
  loading?: boolean;
  /** Compact horizontal row layout for lists/dialogs. */
  compact?: boolean;
}>;

export function RecipeCardRoot({
  onClick,
  className,
  loading,
  compact,
  children,
}: RecipeCardRootProps) {
  const baseCn = compact
    ? "flex flex-row items-center gap-3 p-3"
    : "flex h-full flex-col justify-between p-5";

  const card = (
    <Card loading={loading} className={cn(baseCn, className)}>
      {children}
    </Card>
  );

  if (onClick) {
    return (
      <Pressable asChild className={cn(compact ? "" : "h-full", "text-left")} onClick={onClick}>
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

/* ── Exports ─────────────────────────────────────────────────── */

export {
  RecipeCardHeader,
  RecipeCardContent,
  RecipeCardFooter,
  RecipeCardIcon,
  RecipeCardCategory,
  RecipeCardTitle,
  RecipeCardDescription,
  RecipeCardTags,
};
