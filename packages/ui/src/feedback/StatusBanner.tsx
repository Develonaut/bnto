"use client";

import { forwardRef, type ReactNode } from "react";
import { createCn } from "../utils/createCn";
import { cn } from "../utils/cn";

/* ── Variant types ────────────────────────────────────────────── */

type StatusBannerVariant = "success" | "warning" | "error" | "processing";

/* ── Root ─────────────────────────────────────────────────────── */

interface StatusBannerProps {
  /** Visual variant controlling border/bg tint. */
  variant?: StatusBannerVariant;
  children: ReactNode;
  className?: string;
}

const rootCn = createCn({
  base: "flex w-full flex-col gap-2 rounded-lg border p-3",
  variants: {
    variant: {
      success: "border-success/30 bg-success/5",
      warning: "border-warning/30 bg-warning/10",
      error: "border-destructive/50 bg-destructive/5",
      processing: "border-border bg-card",
    },
  },
  defaultVariants: { variant: "processing" },
});

function StatusBanner({ variant = "processing", children, className }: StatusBannerProps) {
  return (
    <div className={rootCn({ variant }, className)} role="status" data-testid="status-banner">
      {children}
    </div>
  );
}

/* ── Row — horizontal layout container ────────────────────────── */

const StatusBannerRow = forwardRef<HTMLDivElement, { children: ReactNode; className?: string }>(
  ({ children, className }, ref) => (
    <div ref={ref} className={cn("flex items-center gap-2", className)}>
      {children}
    </div>
  ),
);
StatusBannerRow.displayName = "StatusBannerRow";

/* ── Icon — leading icon slot ─────────────────────────────────── */

interface StatusBannerIconProps {
  children: ReactNode;
  className?: string;
}

function StatusBannerIcon({ children, className }: StatusBannerIconProps) {
  return <span className={cn("shrink-0 [&>svg]:size-4", className)}>{children}</span>;
}

/* ── Label — text label (title or detail) ─────────────────────── */

interface StatusBannerLabelProps {
  children: ReactNode;
  muted?: boolean;
  mono?: boolean;
  className?: string;
}

function StatusBannerLabel({ children, muted, mono, className }: StatusBannerLabelProps) {
  return (
    <span
      className={cn(
        "truncate text-sm",
        muted ? "text-muted-foreground" : "font-medium text-foreground",
        mono && "font-mono tabular-nums",
        className,
      )}
    >
      {children}
    </span>
  );
}

/* ── Spacer — flex spacer between left/right content ──────────── */

function StatusBannerSpacer() {
  return <span className="flex-1" />;
}

/* ── Progress — horizontal bar ────────────────────────────────── */

interface StatusBannerProgressProps {
  /** Current value (0–100). */
  value: number;
  /** Bar fill color variant. Defaults to primary. */
  variant?: StatusBannerVariant;
  className?: string;
}

const barFillCn = createCn({
  base: "h-full rounded-full motion-safe:transition-[width] motion-safe:duration-fast",
  variants: {
    variant: {
      success: "bg-success",
      warning: "bg-warning",
      error: "bg-destructive",
      processing: "bg-primary",
    },
  },
  defaultVariants: { variant: "processing" },
});

function StatusBannerProgress({
  value,
  variant = "processing",
  className,
}: StatusBannerProgressProps) {
  const clamped = Math.min(100, Math.max(0, value));

  return (
    <div
      role="progressbar"
      aria-valuenow={clamped}
      aria-valuemin={0}
      aria-valuemax={100}
      className={cn(
        "h-4 w-full overflow-hidden rounded-full border border-border bg-input",
        className,
      )}
    >
      <div className={barFillCn({ variant })} style={{ width: `${clamped}%` }} />
    </div>
  );
}

/* ── Exports ──────────────────────────────────────────────────── */

export {
  StatusBanner,
  StatusBannerRow,
  StatusBannerIcon,
  StatusBannerLabel,
  StatusBannerSpacer,
  StatusBannerProgress,
};
export type { StatusBannerProps, StatusBannerVariant };
