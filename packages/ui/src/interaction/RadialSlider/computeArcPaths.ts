import { arcPath } from "./geometry";

/** Derive ring + track SVG path strings from start/end angles. */
export function computeArcPaths(
  startAngle: number,
  endAngle: number,
  radius: number,
  svgCenter: number,
) {
  let arcSpan = endAngle - startAngle;
  if (arcSpan < 0) arcSpan += 360;
  const ringPath = arcSpan < 360 ? arcPath(0, 360, radius, svgCenter, svgCenter) : "";
  const trackPath = arcPath(startAngle, endAngle, radius, svgCenter, svgCenter);
  return { ringPath, trackPath };
}
