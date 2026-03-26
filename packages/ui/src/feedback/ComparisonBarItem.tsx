import type { CSSProperties } from "react";

import { cn } from "../utils/cn";

interface ComparisonBarItemProps {
  label: string;
  value: number;
  max: number;
  mounted: boolean;
  barClassName?: string;
  primary: boolean;
  subtitle?: string;
  height: string;
  delay: number;
  formatValue: (value: number) => string;
}

function barStyle(widthPercent: number, mounted: boolean, delay: number): CSSProperties {
  return {
    "--bar-width": `${widthPercent}%`,
    transitionDelay: mounted ? `${delay}ms` : "0ms",
  } as CSSProperties;
}

const BAR_CN =
  "h-full w-0 rounded-full motion-safe:transition-[width] motion-safe:duration-1000 motion-safe:ease-out data-[active=true]:w-[var(--bar-width)]";

export function ComparisonBarItem({
  label,
  value,
  max,
  mounted,
  barClassName,
  primary,
  subtitle,
  height,
  delay,
  formatValue,
}: ComparisonBarItemProps) {
  const widthPercent = Math.max((value / max) * 100, 2);

  return (
    <div className="flex flex-col gap-1">
      <div className={cn("bg-muted w-full overflow-hidden rounded-full", height)}>
        <div
          data-active={mounted}
          className={cn(
            BAR_CN,
            barClassName ?? (primary ? "bg-primary" : "bg-muted-foreground/30"),
          )}
          style={barStyle(widthPercent, mounted, delay)}
        />
      </div>
      <ComparisonBarLabel
        label={label}
        value={value}
        primary={primary}
        subtitle={subtitle}
        formatValue={formatValue}
      />
    </div>
  );
}

function ComparisonBarLabel({
  label,
  value,
  primary,
  subtitle,
  formatValue,
}: Pick<ComparisonBarItemProps, "label" | "value" | "primary" | "subtitle" | "formatValue">) {
  return (
    <div className="flex items-baseline justify-between">
      <div className="flex flex-col">
        <span
          className={cn(
            "text-[10px] font-medium",
            primary ? "text-primary" : "text-muted-foreground",
          )}
        >
          {label}
        </span>
        {subtitle && <span className="text-muted-foreground/50 text-[9px]">{subtitle}</span>}
      </div>
      <span className="text-muted-foreground text-[10px]">
        {value > 0 ? formatValue(value) : ""}
      </span>
    </div>
  );
}
