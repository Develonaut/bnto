import { forwardRef, type ReactNode } from "react";
import { createCn } from "../utils/createCn";
import { cn } from "../utils/cn";
import { clamp } from "../utils/clamp";

/* ── Variant types ────────────────────────────────────────────── */

type StatusBannerVariant = "success" | "warning" | "error" | "processing" | "secondary";

/* ── Root ─────────────────────────────────────────────────────── */

interface StatusBannerProps {
  /** Visual variant controlling border/bg tint. */
  variant?: StatusBannerVariant;
  /** Compact padding for tight spaces (px-2 py-1 instead of p-3). */
  dense?: boolean;
  children: ReactNode;
  className?: string;
}

const rootCn = createCn({
  base: "flex w-full flex-col gap-2 rounded-lg border",
  variants: {
    variant: {
      success: "border-success/30 bg-success/5",
      warning: "border-warning/30 bg-warning/10",
      error: "border-destructive/50 bg-destructive/5",
      processing: "border-border bg-card",
      secondary: "border-secondary bg-secondary/30",
    },
    dense: {
      true: "px-2 py-1",
      false: "p-3",
    },
  },
  defaultVariants: { variant: "processing", dense: "false" },
});

function StatusBanner({
  variant = "processing",
  dense = false,
  children,
  className,
}: StatusBannerProps) {
  return (
    <div
      className={rootCn({ variant, dense: dense ? "true" : "false" }, className)}
      role="status"
      data-testid="status-banner"
    >
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
      secondary: "bg-secondary-foreground",
    },
  },
  defaultVariants: { variant: "processing" },
});

function StatusBannerProgress({
  value,
  variant = "processing",
  className,
}: StatusBannerProgressProps) {
  const clamped = clamp(value, 0, 100);

  return (
    <div
      role="progressbar"
      data-testid="status-progressbar"
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
