"use client";

import type { ReactNode } from "react";
import { cn } from "@/lib/cn";

interface LinearProgressProps {
  /** Current value (0–100). */
  value: number;
  /** Icon shown before the label text. */
  icon?: ReactNode;
  /** Label text shown above-left. */
  label?: string;
  /** Formatted value string shown above-right (e.g. "64%", "4 MB saved"). Defaults to `${value}%`. */
  valueLabel?: string;
  /** Helper text shown below the bar. */
  helperText?: string;
  className?: string;
}

/**
 * Read-only progress bar that mirrors the Slider track style (no thumb).
 *
 * Layout:
 *   [icon] Label              64%  (above: icon + label left, value right)
 *   ████████████░░░░░░░░░░░░░░░░  (bar full-width below)
 *   Helper text                    (below, optional)
 */
export function LinearProgress({
  value,
  icon,
  label,
  valueLabel,
  helperText,
  className,
}: LinearProgressProps) {
  const clamped = Math.min(100, Math.max(0, value));

  return (
    <div className={cn("flex w-full flex-col gap-3", className)}>
      {(icon || label || valueLabel !== undefined) && (
        <div className="flex items-center justify-between gap-2">
          <div className="flex min-w-0 items-center gap-2">
            {icon}
            {label && (
              <span className="truncate text-sm text-muted-foreground">{label}</span>
            )}
          </div>
          <span className="shrink-0 font-mono text-sm tabular-nums text-muted-foreground">
            {valueLabel ?? `${clamped}%`}
          </span>
        </div>
      )}
      <div
        role="progressbar"
        aria-valuenow={clamped}
        aria-valuemin={0}
        aria-valuemax={100}
        className="relative h-4 w-full overflow-hidden rounded-full border border-border bg-input"
      >
        <div
          className="h-full rounded-full bg-primary motion-safe:transition-[width] motion-safe:duration-fast"
          style={{ width: `${clamped}%` }}
        />
      </div>
      <p className="min-h-4 text-xs text-muted-foreground">
        {helperText ?? "\u00A0"}
      </p>
    </div>
  );
}
