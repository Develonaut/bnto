"use client";

import { useEffect, useState } from "react";

import { cn } from "@/lib/utils";

interface ComparisonItem {
  label: string;
  value: number;
  subtitle?: string;
  className?: string;
}

interface ComparisonBarProps {
  /** Items to compare — first is "ours", rest are competitors */
  items: ComparisonItem[];
  /** Whether the animation is active */
  active: boolean;
  /** Delay before animation starts in ms */
  delay?: number;
  /** Height class for the bars */
  height?: string;
  /** Custom value formatter — defaults to ms/s display */
  formatValue?: (value: number) => string;
}

/**
 * Stacked horizontal comparison bars showing relative sizes.
 * Values are normalized to the max. Great for "us vs them" comparisons.
 */
function defaultFormat(value: number): string {
  return value >= 1000 ? `${(value / 1000).toFixed(0)}s` : `${value}ms`;
}

export function ComparisonBar({
  items,
  active,
  delay = 400,
  height = "h-2",
  formatValue,
}: ComparisonBarProps) {
  const fmt = formatValue ?? defaultFormat;
  const [widths, setWidths] = useState<number[]>(items.map(() => 0));
  const max = Math.max(...items.map((i) => i.value));

  useEffect(() => {
    const prefersReduced =
      typeof window !== "undefined" &&
      window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    const timer = setTimeout(
      () => setWidths(
        active
          ? items.map((item) => Math.max((item.value / max) * 100, 2))
          : items.map(() => 0),
      ),
      active && !prefersReduced ? delay : 0,
    );
    return () => clearTimeout(timer);
  }, [active, items, max, delay]);

  return (
    <div className="flex w-full flex-col gap-2">
      {items.map((item, i) => (
        <div key={item.label} className="flex flex-col gap-1">
          <div className={cn("bg-muted w-full overflow-hidden rounded-full", height)}>
            <div
              className={cn(
                "h-full rounded-full transition-all duration-1000 ease-out",
                item.className ?? (i === 0 ? "bg-primary" : "bg-muted-foreground/30"),
              )}
              style={{ width: `${widths[i]}%` }}
            />
          </div>
          <div className="flex items-baseline justify-between">
            <div className="flex flex-col">
              <span className={cn(
                "text-[10px] font-medium",
                i === 0 ? "text-primary" : "text-muted-foreground",
              )}>
                {item.label}
              </span>
              {item.subtitle && (
                <span className="text-muted-foreground/50 text-[9px]">
                  {item.subtitle}
                </span>
              )}
            </div>
            <span className="text-muted-foreground text-[10px]">
              {item.value > 0 ? fmt(item.value) : ""}
            </span>
          </div>
        </div>
      ))}
    </div>
  );
}
