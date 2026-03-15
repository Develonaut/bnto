import type { PropsWithChildren } from "react";

import Link from "next/link";

import type { LucideIcon } from "../../icons";
import { BlocksIcon } from "../../icons";
import { Button } from "../../interaction/Button";
import { Row } from "../../layout/Row";
import { Stack } from "../../layout/Stack";
import { Card } from "../../surface/Card";
import type { SurfaceVariant } from "../../surface/Surface";
import { Badge } from "../../typography/Badge";
import { Heading } from "../../typography/Heading";
import { IconBadge } from "../../typography/IconBadge";
import { Text } from "../../typography/Text";
import { cn } from "../../utils/cn";

/* ── Root ────────────────────────────────────────────────────── */

type RecipeCardRootProps = PropsWithChildren<{
  onClick?: () => void;
  /** Internal link — renders the card as a Next.js Link. */
  href?: string;
  className?: string;
  /** Color variant forwarded to Card surface. */
  color?: SurfaceVariant;
  /** Grounded loading state — card springs up when loading clears. */
  loading?: boolean;
  /** Compact horizontal row layout for lists/dialogs. */
  compact?: boolean;
}>;

export function RecipeCardRoot({
  onClick,
  href,
  className,
  color,
  loading,
  compact,
  children,
}: RecipeCardRootProps) {
  const baseCn = compact
    ? "flex flex-row items-center gap-3 p-3"
    : "flex h-full flex-col justify-between p-5";

  const card = (
    <Card loading={loading} color={color} className={cn(baseCn, className)}>
      {children}
    </Card>
  );

  if (!href && !onClick) return card;

  const interactive = (
    <Button asChild className={cn(compact ? "" : "h-full", "text-left")} onClick={onClick}>
      {card}
    </Button>
  );

  if (href) {
    return (
      <Link href={href} className={cn(compact ? "" : "h-full", "group")}>
        {interactive}
      </Link>
    );
  }

  return interactive;
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

function RecipeCardIcon({
  icon: Icon,
  onSurface,
}: {
  icon?: LucideIcon;
  /** Use translucent white badge for colored card surfaces. */
  onSurface?: boolean;
}) {
  const Glyph = Icon ?? BlocksIcon;
  return (
    <IconBadge variant={onSurface ? "onSurface" : "primary"} size="lg" shape="square">
      <Glyph className="size-5" />
    </IconBadge>
  );
}

function RecipeCardCategory({ children, onSurface }: PropsWithChildren<{ onSurface?: boolean }>) {
  return (
    <Text
      as="span"
      size="xs"
      mono
      color={onSurface ? "inherit" : "muted"}
      className={cn("uppercase tracking-wider", onSurface && "opacity-80")}
    >
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

function RecipeCardDescription({
  children,
  onSurface,
}: PropsWithChildren<{ onSurface?: boolean }>) {
  return (
    <Text
      size="sm"
      color={onSurface ? "inherit" : "muted"}
      leading="snug"
      className={cn("text-left", onSurface && "opacity-85")}
    >
      {children}
    </Text>
  );
}

function RecipeCardTags({
  tags,
  limit = 3,
  onSurface,
}: {
  tags: string[];
  limit?: number;
  /** Use translucent white badges for colored card surfaces. */
  onSurface?: boolean;
}) {
  return (
    <Row wrap className="gap-1.5 pt-1">
      {tags.slice(0, limit).map((tag) => (
        <Badge key={tag} size="sm" className={onSurface ? "bg-white/20 text-inherit" : undefined}>
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
