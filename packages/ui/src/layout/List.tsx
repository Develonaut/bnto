import type { ComponentProps, CSSProperties, PropsWithChildren } from "react";

import { cn } from "../utils/cn";
import { Surface } from "../surface/Surface";
import type { SurfaceBorder, SurfaceVariant } from "../surface/Surface";

/* ── List ──────────────────────────────────────────────────────── */

type ListProps = {
  /** Render as a different element. Default `"div"`. */
  as?: "div" | "ul" | "ol";
  className?: string;
  children?: React.ReactNode;
};

export function List({ as: Tag = "div", className, children }: ListProps) {
  return <Tag className={cn("flex flex-col gap-0.5", className)}>{children}</Tag>;
}

/* ── ListItem ─────────────────────────────────────────────────── */

/** Shared row classes for list-like items (ListItem, SelectItem, etc.). */
export const ITEM_CN =
  "flex w-full items-center gap-2 px-4 py-2.5 text-sm transition-colors select-none outline-hidden [&_svg:not([class*='size-'])]:size-4 disabled:pointer-events-none disabled:opacity-50";

type ListItemProps = ComponentProps<"div"> & {
  /** Merge onto child element instead of wrapping in a `<div>`. */
  asChild?: boolean;
  /** Surface color variant. Default `"ghost"`. Overridden to `"muted"` when `selected`. */
  variant?: SurfaceVariant;
  /** Border style. Default `"none"`. Overridden to `"solid"` when `selected`. */
  border?: SurfaceBorder;
  /** Enable hover decoration and cursor pointer. Implied by `selected`. */
  selectable?: boolean;
  /** Selected state — shows muted background with solid border. */
  selected?: boolean;
};

export function ListItem({
  asChild,
  variant,
  border,
  selectable,
  selected,
  className,
  style,
  ...props
}: ListItemProps) {
  const isSelectable = selectable || selected;
  return (
    <Surface
      variant={variant ?? (selected ? "muted" : "ghost")}
      elevation="none"
      border={border ?? (selected ? "solid" : "none")}
      rounded="lg"
      asChild={asChild}
      className={cn(ITEM_CN, className)}
      data-selectable={isSelectable || undefined}
      style={selected ? ({ "--face-fg": "var(--foreground)", ...style } as CSSProperties) : style}
      {...props}
    />
  );
}

/* ── ListSeparator ─────────────────────────────────────────────── */

export function ListSeparator({ className, ...props }: ComponentProps<"div">) {
  return <div role="separator" className={cn("my-1 h-px bg-border", className)} {...props} />;
}

/* ── ListSubheader ─────────────────────────────────────────────── */

export function ListSubheader({ className, children }: PropsWithChildren<{ className?: string }>) {
  return (
    <div
      className={cn(
        "px-3 pt-3 pb-1 text-xs font-medium uppercase tracking-wider text-muted-foreground",
        className,
      )}
    >
      {children}
    </div>
  );
}

/* ── Slot sub-components ──────────────────────────────────────── */

export function ListItemContent({
  className,
  children,
}: PropsWithChildren<{ className?: string }>) {
  return <div className={cn("min-w-0 flex-1", className)}>{children}</div>;
}

export function ListItemActions({
  className,
  children,
}: PropsWithChildren<{ className?: string }>) {
  return <div className={cn("flex shrink-0 items-center gap-1", className)}>{children}</div>;
}
