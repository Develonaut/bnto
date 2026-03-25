import type { HTMLAttributes, PropsWithChildren } from "react";

import { cn } from "../utils/cn";
import { Card } from "../surface/Card";
import type { SurfaceElevation } from "../surface/Surface";

/* ── FileList ─────────────────────────────────────────────────── */

type FileListProps = Omit<HTMLAttributes<HTMLElement>, "color"> & {
  /** Render as a different element. Default `"div"`. */
  as?: "div" | "ul" | "ol";
};

export function FileList({ as: Tag = "div", className, children, ...props }: FileListProps) {
  return (
    <Tag className={cn("grid grid-cols-1 gap-2", className)} {...props}>
      {children}
    </Tag>
  );
}

/* ── FileListItem ─────────────────────────────────────────────── */

type FileListItemProps = Omit<HTMLAttributes<HTMLDivElement>, "color"> & {
  /** Card elevation. Default `"sm"`. Use `"none"` for flat appearance. */
  elevation?: SurfaceElevation;
};

export function FileListItem({
  elevation = "sm",
  className,
  children,
  ...props
}: FileListItemProps) {
  return (
    <Card elevation={elevation} className={cn("bg-card", className)} {...props}>
      <div className="flex items-center gap-3 px-4 py-3">{children}</div>
    </Card>
  );
}

/* ── FileListIcon ─────────────────────────────────────────────── */

export function FileListIcon({ className, children }: PropsWithChildren<{ className?: string }>) {
  return (
    <div className={cn("flex shrink-0 items-center justify-center", className)}>{children}</div>
  );
}

/* ── FileListContent ──────────────────────────────────────────── */

export function FileListContent({
  className,
  children,
}: PropsWithChildren<{ className?: string }>) {
  return <div className={cn("flex min-w-0 flex-1 flex-col", className)}>{children}</div>;
}

/* ── FileListName ─────────────────────────────────────────────── */

export function FileListName({ className, children }: PropsWithChildren<{ className?: string }>) {
  return <span className={cn("truncate text-sm font-semibold", className)}>{children}</span>;
}

/* ── FileListMeta ─────────────────────────────────────────────── */

export function FileListMeta({ className, children }: PropsWithChildren<{ className?: string }>) {
  return <span className={cn("text-xs text-muted-foreground", className)}>{children}</span>;
}

/* ── FileListActions ──────────────────────────────────────────── */

export function FileListActions({
  className,
  children,
}: PropsWithChildren<{ className?: string }>) {
  return <div className={cn("flex shrink-0 items-center gap-1", className)}>{children}</div>;
}
