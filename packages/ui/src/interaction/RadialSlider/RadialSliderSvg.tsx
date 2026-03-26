import type { ReactNode } from "react";

import { computeArcPaths } from "./computeArcPaths";
import { RingPaths } from "./RingPaths";
import { TrackPaths } from "./TrackPaths";
import { ActivePath } from "./ActivePath";

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

export function RadialSliderSvg(props: RadialSliderSvgProps) {
  const { size, radius, svgCenter, strokeWidth, startAngle, endAngle } = props;
  const { ringPath, trackPath } = computeArcPaths(startAngle, endAngle, radius, svgCenter);
  const showRing = !!ringPath && !props.trackStroke && !props.hideRing;

  return (
    <svg
      width={size}
      height={size}
      viewBox={`0 0 ${size} ${size}`}
      className="pointer-events-none absolute inset-0"
    >
      {props.svgDefs && <defs>{props.svgDefs}</defs>}
      {showRing && (
        <RingPaths
          ringPath={ringPath}
          trackClassName={props.trackClassName}
          strokeWidth={strokeWidth}
        />
      )}
      {trackPath && (
        <TrackPaths
          trackPath={trackPath}
          trackStroke={props.trackStroke}
          trackClassName={props.trackClassName}
          strokeWidth={strokeWidth}
        />
      )}
      <ActivePath
        activePath={props.activePath}
        activeStroke={props.activeStroke}
        activeClassName={props.activeClassName}
        strokeWidth={strokeWidth}
      />
    </svg>
  );
}
