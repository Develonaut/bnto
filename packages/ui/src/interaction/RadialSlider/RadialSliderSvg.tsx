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

function RingPaths({
  ringPath,
  trackClassName,
  strokeWidth,
}: {
  ringPath: string;
  trackClassName: string;
  strokeWidth: number;
}) {
  return (
    <>
      <path
        d={ringPath}
        fill="none"
        stroke="currentColor"
        className="text-border"
        strokeWidth={strokeWidth + 2}
        strokeLinecap="round"
        opacity={0.3}
      />
      <path
        d={ringPath}
        fill="none"
        stroke="currentColor"
        className={trackClassName}
        strokeWidth={strokeWidth}
        strokeLinecap="round"
        opacity={0.3}
      />
    </>
  );
}

function TrackPaths({
  trackPath,
  trackStroke,
  trackClassName,
  strokeWidth,
}: {
  trackPath: string;
  trackStroke?: string;
  trackClassName: string;
  strokeWidth: number;
}) {
  return (
    <>
      {!trackStroke && (
        <path
          d={trackPath}
          fill="none"
          stroke="currentColor"
          className="text-border"
          strokeWidth={strokeWidth + 2}
          strokeLinecap="round"
        />
      )}
      <path
        d={trackPath}
        fill="none"
        stroke={trackStroke ?? "currentColor"}
        className={trackStroke ? undefined : trackClassName}
        strokeWidth={strokeWidth}
        strokeLinecap="round"
      />
    </>
  );
}

function ActivePath({
  activePath,
  activeStroke,
  activeClassName,
  strokeWidth,
}: {
  activePath: string;
  activeStroke?: string;
  activeClassName: string;
  strokeWidth: number;
}) {
  if (!activePath) return null;
  return (
    <path
      d={activePath}
      fill="none"
      stroke={activeStroke ?? "currentColor"}
      className={activeStroke ? undefined : activeClassName}
      strokeWidth={strokeWidth}
      strokeLinecap="round"
    />
  );
}

function computeArcPaths(startAngle: number, endAngle: number, radius: number, svgCenter: number) {
  let arcSpan = endAngle - startAngle;
  if (arcSpan < 0) arcSpan += 360;
  const ringPath = arcSpan < 360 ? arcPath(0, 360, radius, svgCenter, svgCenter) : "";
  const trackPath = arcPath(startAngle, endAngle, radius, svgCenter, svgCenter);
  return { ringPath, trackPath };
}

export function RadialSliderSvg(props: RadialSliderSvgProps) {
  const {
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
  } = props;
  const { ringPath, trackPath } = computeArcPaths(startAngle, endAngle, radius, svgCenter);
  const showRing = !!ringPath && !trackStroke && !hideRing;

  return (
    <svg
      width={size}
      height={size}
      viewBox={`0 0 ${size} ${size}`}
      className="pointer-events-none absolute inset-0"
    >
      {svgDefs && <defs>{svgDefs}</defs>}
      {showRing && (
        <RingPaths ringPath={ringPath} trackClassName={trackClassName} strokeWidth={strokeWidth} />
      )}
      {trackPath && (
        <TrackPaths
          trackPath={trackPath}
          trackStroke={trackStroke}
          trackClassName={trackClassName}
          strokeWidth={strokeWidth}
        />
      )}
      <ActivePath
        activePath={activePath}
        activeStroke={activeStroke}
        activeClassName={activeClassName}
        strokeWidth={strokeWidth}
      />
    </svg>
  );
}
