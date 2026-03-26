"use client";

import { useEffect, useState } from "react";

import { ComparisonBarItem } from "./ComparisonBarItem";

interface ComparisonItem {
  label: string;
  value: number;
  subtitle?: string;
  className?: string;
}

interface ComparisonBarProps {
  /** Items to compare -- first is "ours", rest are competitors */
  items: ComparisonItem[];
  /** Whether the bars are expanded to their target widths */
  active: boolean;
  /** Delay before animation starts in ms */
  delay?: number;
  /** Height class for the bars */
  height?: string;
  /** Custom value formatter -- defaults to ms/s display */
  formatValue?: (value: number) => string;
}

function defaultFormat(value: number): string {
  return value >= 1000 ? `${(value / 1000).toFixed(0)}s` : `${value}ms`;
}

/** Flip to true after one frame so CSS transitions animate from w-0. */
function useMountedToggle(active: boolean) {
  const [mounted, setMounted] = useState(false);
  useEffect(() => {
    const raf = requestAnimationFrame(() => setMounted(active));
    return () => cancelAnimationFrame(raf);
  }, [active]);
  return mounted;
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
  const mounted = useMountedToggle(active);

  return (
    <div className="flex w-full flex-col gap-2">
      {items.map((item, i) => (
        <ComparisonBarItem
          key={item.label}
          label={item.label}
          value={item.value}
          max={max}
          mounted={mounted}
          barClassName={item.className}
          primary={i === 0}
          subtitle={item.subtitle}
          height={height}
          delay={delay}
          formatValue={fmt}
        />
      ))}
    </div>
  );
}
