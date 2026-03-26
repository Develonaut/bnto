import type { CSSProperties } from "react";

import { cn } from "../../utils/cn";

interface ComparisonBarTrackProps {
  widthPercent: number;
  mounted: boolean;
  barClassName?: string;
  primary: boolean;
  height: string;
  delay: number;
}

function barStyle(widthPercent: number, mounted: boolean, delay: number): CSSProperties {
  return {
    "--bar-width": `${widthPercent}%`,
    transitionDelay: mounted ? `${delay}ms` : "0ms",
  } as CSSProperties;
}

const BAR_CN =
  "h-full w-0 rounded-full motion-safe:transition-[width] motion-safe:duration-1000 motion-safe:ease-out data-[active=true]:w-[var(--bar-width)]";

export function ComparisonBarTrack({
  widthPercent,
  mounted,
  barClassName,
  primary,
  height,
  delay,
}: ComparisonBarTrackProps) {
  return (
    <div className={cn("bg-muted w-full overflow-hidden rounded-full", height)}>
      <div
        data-active={mounted}
        className={cn(BAR_CN, barClassName ?? (primary ? "bg-primary" : "bg-muted-foreground/30"))}
        style={barStyle(widthPercent, mounted, delay)}
      />
    </div>
  );
}
