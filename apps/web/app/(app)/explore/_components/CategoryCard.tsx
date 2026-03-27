/**
 * Category filter card for the explore page.
 *
 * Shorter, wide card in a horizontal scrolling list.
 * Clicking a card filters recipes by that category.
 */

import type { LucideIcon } from "@bnto/ui";
import { Card, Text, cn } from "@bnto/ui";

interface CategoryCardProps {
  label: string;
  icon?: LucideIcon;
  count?: number;
  active: boolean;
  muted?: boolean;
  onClick: () => void;
}

export function CategoryCard({
  label,
  icon: Icon,
  count,
  active,
  muted,
  onClick,
}: CategoryCardProps) {
  const dimmed = Boolean(muted && !active);

  return (
    <Card asChild>
      <button
        type="button"
        onClick={onClick}
        data-testid={`explore-category-${label.toLowerCase().replace(/\s+/g, "-")}`}
        className="pressable flex w-[180px] shrink-0 items-center gap-3 px-4 py-3"
      >
        {Icon && <CategoryIcon Icon={Icon} active={active} dimmed={dimmed} />}
        <CategoryLabel label={label} count={count} dimmed={dimmed} />
      </button>
    </Card>
  );
}

function CategoryIcon({
  Icon,
  active,
  dimmed,
}: {
  Icon: LucideIcon;
  active: boolean;
  dimmed: boolean;
}) {
  return (
    <div
      className={cn(
        "flex size-10 shrink-0 items-center justify-center rounded-lg",
        active ? "bg-primary/10" : "bg-muted",
      )}
    >
      <Icon
        className={cn(
          "size-5",
          active ? "text-primary" : "text-muted-foreground",
          dimmed && "text-muted-foreground/50",
        )}
      />
    </div>
  );
}

function CategoryLabel({
  label,
  count,
  dimmed,
}: {
  label: string;
  count?: number;
  dimmed: boolean;
}) {
  return (
    <div className="flex min-w-0 flex-col items-start gap-0.5">
      <Text
        size="sm"
        weight="medium"
        className={cn("truncate text-left", dimmed && "text-muted-foreground")}
      >
        {label}
      </Text>
      {count !== undefined && (
        <Text
          size="xs"
          color="muted"
          className={cn("text-left", dimmed && "text-muted-foreground/50")}
        >
          {count} {count === 1 ? "recipe" : "recipes"}
        </Text>
      )}
    </div>
  );
}
