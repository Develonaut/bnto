import { arcPath, polarToOffset } from "./geometry";
import { valueToAngle } from "./valueMapping";

interface RadialLayoutInput {
  size: number;
  strokeWidth: number;
  min: number;
  max: number;
  value: number;
  startAngle: number;
  endAngle: number;
  hideProgress: boolean;
}

/** Compute derived layout values for the radial slider SVG. */
export function computeRadialLayout({
  size,
  strokeWidth,
  min,
  max,
  value,
  startAngle,
  endAngle,
  hideProgress,
}: RadialLayoutInput) {
  const trackInset = strokeWidth * 2 + 4;
  const radius = (size - trackInset) / 2;
  const svgCenter = size / 2;
  const currentAngle = valueToAngle(value, min, max, startAngle, endAngle);
  const thumbOffset = polarToOffset(currentAngle, radius);
  const activePath =
    !hideProgress && value > min
      ? arcPath(startAngle, currentAngle, radius, svgCenter, svgCenter)
      : "";

  return { radius, svgCenter, currentAngle, thumbOffset, activePath };
}
