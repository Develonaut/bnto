import type { ReactNode } from "react";

import { arcPath } from "./geometry";

interface RadialSliderSvgProps {
  size: number;
  radius: number;
  svgCenter: number;
  strokeWidth: number;
  startAngle: number;
  endAngle: number;
  trackClassName: string;
  activeClassName: string;
  hideRing: boolean;
  svgDefs?: ReactNode;
  trackStroke?: string;
  activeStroke?: string;
  activePath: string;
}

export function RadialSliderSvg({
  size,
  radius,
  svgCenter,
  strokeWidth,
  startAngle,
  endAngle,
  trackClassName,
  activeClassName,
  hideRing,
  svgDefs,
  trackStroke,
  activeStroke,
  activePath,
}: RadialSliderSvgProps) {
  let arcSpan = endAngle - startAngle;
  if (arcSpan < 0) arcSpan += 360;
  const isPartialArc = arcSpan < 360;

  const ringPath = isPartialArc
    ? arcPath(0, 360, radius, svgCenter, svgCenter)
    : "";
  const trackPath = arcPath(startAngle, endAngle, radius, svgCenter, svgCenter);

  return (
    <svg
      width={size}
      height={size}
      viewBox={`0 0 ${size} ${size}`}
      className="pointer-events-none absolute inset-0"
    >
      {svgDefs && <defs>{svgDefs}</defs>}
      {ringPath && !trackStroke && !hideRing && (
        <>
          <path d={ringPath} fill="none" stroke="currentColor" className="text-border" strokeWidth={strokeWidth + 2} strokeLinecap="round" opacity={0.3} />
          <path d={ringPath} fill="none" stroke="currentColor" className={trackClassName} strokeWidth={strokeWidth} strokeLinecap="round" opacity={0.3} />
        </>
      )}
      {trackPath && (
        <>
          {!trackStroke && (
            <path d={trackPath} fill="none" stroke="currentColor" className="text-border" strokeWidth={strokeWidth + 2} strokeLinecap="round" />
          )}
          <path d={trackPath} fill="none" stroke={trackStroke ?? "currentColor"} className={trackStroke ? undefined : trackClassName} strokeWidth={strokeWidth} strokeLinecap="round" />
        </>
      )}
      {activePath && (
        <path
          d={activePath}
          fill="none"
          stroke={activeStroke ?? "currentColor"}
          className={activeStroke ? undefined : activeClassName}
          strokeWidth={strokeWidth}
          strokeLinecap="round"
        />
      )}
    </svg>
  );
}
