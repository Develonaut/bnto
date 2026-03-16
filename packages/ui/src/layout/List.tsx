import type { ComponentProps, PropsWithChildren } from "react";

import { cn } from "../utils/cn";
import { Card } from "../surface/Card";

/* ── List ──────────────────────────────────────────────────────── */

type ListProps = {
  /** Render as a different element. Default `"div"`. */
  as?: "div" | "ul" | "ol";
  className?: string;
  children?: React.ReactNode;
};

export function List({ as: Tag = "div", className, children }: ListProps) {
  return <Tag className={cn("flex flex-col gap-3", className)}>{children}</Tag>;
}

/* ── ListItem ─────────────────────────────────────────────────── */

type ListItemProps = ComponentProps<typeof Card>;

export function ListItem({ className, elevation = "sm", ...props }: ListItemProps) {
  return (
    <Card
      elevation={elevation}
      className={cn("flex items-center gap-4 px-5 py-4", className)}
      {...props}
    />
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
