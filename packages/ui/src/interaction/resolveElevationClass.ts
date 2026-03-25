/** Resolve an elevation override to the appropriate CSS class. */

export type ElevationOverride = boolean | "none" | "xs" | "sm" | "md" | "lg";

/** Strip the size-variant's built-in elevation-{xs|sm|md|lg} and replace it. */
export function resolveElevationClass(elevation: ElevationOverride): string | undefined {
  if (elevation === true) return undefined; // use size variant's built-in elevation
  if (elevation === false || elevation === "none") return "elevation-none";
  return `elevation-${elevation}`;
}
