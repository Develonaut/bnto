import type { RadialSliderProps } from "./RadialSliderRoot";

/** Resolve optional props to their defaults. */
export function resolveRadialDefaults(props: RadialSliderProps) {
  return {
    size: props.size ?? 48,
    strokeWidth: props.strokeWidth ?? 10,
    startAngle: props.startAngle ?? 0,
    endAngle: props.endAngle ?? 360,
    hideProgress: props.hideProgress ?? false,
    hideRing: props.hideRing ?? false,
    trackClassName: props.trackClassName ?? "text-input",
    activeClassName: props.activeClassName ?? "text-primary",
    step: props.step ?? 1,
  };
}
