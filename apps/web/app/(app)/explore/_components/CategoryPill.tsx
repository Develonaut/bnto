/**
 * Pill-shaped category filter button for the explore page.
 */

import type { LucideIcon } from "@bnto/ui";
import { Badge, cn } from "@bnto/ui";

interface CategoryPillProps {
  label: string;
  icon?: LucideIcon;
  count?: number;
  active: boolean;
  muted?: boolean;
  onClick: () => void;
}

const baseCn =
  "inline-flex items-center gap-1.5 rounded-full border px-3 py-1.5 text-sm font-medium transition-colors";
const activeCn = "border-primary bg-primary/5 text-primary ring-2 ring-primary/20";
const inactiveCn = "border-border bg-card text-foreground hover:bg-muted";

export function CategoryPill({
  label,
  icon: Icon,
  count,
  active,
  muted,
  onClick,
}: CategoryPillProps) {
  return (
    <button
      type="button"
      onClick={onClick}
      data-testid={`explore-category-${label.toLowerCase().replace(/\s+/g, "-")}`}
      className={cn(baseCn, active ? activeCn : inactiveCn, muted && !active && "opacity-50")}
    >
      {Icon && <Icon className="size-3.5" />}
      <span>{label}</span>
      {count !== undefined && (
        <Badge size="sm" className={cn("ml-0.5", active && "bg-primary/10 text-primary")}>
          {count}
        </Badge>
      )}
    </button>
  );
}
