import type { ReactNode } from "react";
import { cn } from "../utils/cn";
import { clamp } from "../utils/clamp";
import { LinearProgressHeader } from "./LinearProgressHeader";

interface LinearProgressProps {
  /** Current value (0-100). */
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

export function LinearProgress({
  value,
  icon,
  label,
  valueLabel,
  helperText,
  className,
}: LinearProgressProps) {
  const clamped = clamp(value, 0, 100);
  const showHeader = icon || label || valueLabel !== undefined;

  return (
    <div className={cn("flex w-full flex-col gap-3", className)}>
      {showHeader && (
        <LinearProgressHeader icon={icon} label={label} valueLabel={valueLabel ?? `${clamped}%`} />
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
      <p className="min-h-4 text-xs text-muted-foreground">{helperText ?? "\u00A0"}</p>
    </div>
  );
}
