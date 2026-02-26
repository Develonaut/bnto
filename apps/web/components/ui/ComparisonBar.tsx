"use client";

import { useEffect, useState } from "react";

import { cn } from "@/lib/cn";

interface ComparisonItem {
  label: string;
  value: number;
  subtitle?: string;
  className?: string;
}

interface ComparisonBarProps {
  /** Items to compare — first is "ours", rest are competitors */
  items: ComparisonItem[];
  /** Whether the bars are expanded to their target widths */
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
 *
 * Animation is CSS-driven — JS only provides a single boolean toggle
 * after mount so the CSS transition has a state change to animate from.
 * No per-frame JS state updates. The transition timing, easing, and
 * delay are all handled by CSS.
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
  const max = Math.max(...items.map((i) => i.value));

  // Start false so elements mount at w-0, then flip to trigger CSS transition.
  // This is the only JS state — one toggle, not per-frame updates.
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    if (active) {
      // Delay one frame so the browser paints at w-0 first
      const raf = requestAnimationFrame(() => setMounted(true));
      return () => cancelAnimationFrame(raf);
    }
    // Delay the reset to avoid synchronous setState in effect body
    const raf = requestAnimationFrame(() => setMounted(false));
    return () => cancelAnimationFrame(raf);
  }, [active]);

  return (
    <div className="flex w-full flex-col gap-2">
      {items.map((item, i) => {
        const widthPercent = Math.max((item.value / max) * 100, 2);

        return (
          <div key={item.label} className="flex flex-col gap-1">
            <div className={cn("bg-muted w-full overflow-hidden rounded-full", height)}>
              <div
                data-active={mounted}
                className={cn(
                  "h-full w-0 rounded-full",
                  "motion-safe:transition-[width] motion-safe:duration-1000 motion-safe:ease-out",
                  "data-[active=true]:w-[var(--bar-width)]",
                  item.className ?? (i === 0 ? "bg-primary" : "bg-muted-foreground/30"),
                )}
                style={{
                  "--bar-width": `${widthPercent}%`,
                  transitionDelay: mounted ? `${delay}ms` : "0ms",
                } as React.CSSProperties}
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
        );
      })}
    </div>
  );
}
