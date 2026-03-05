type ElevationOverride = boolean | "none" | "sm" | "md" | "lg";

/** Strip the size-variant's built-in elevation-{sm|md|lg} and replace it. */
export function resolveElevationClass(elevation: ElevationOverride): string | undefined {
  if (elevation === true) return undefined; // use size variant's built-in elevation
  if (elevation === false || elevation === "none") return "elevation-none";
  return `elevation-${elevation}`;
}

/** Remove elevation-sm / elevation-md / elevation-lg tokens from a class string. */
export function stripSizeElevation(classes: string): string {
  return classes.replace(/\belevation-(?:sm|md|lg)\b/g, "").trim();
}

export type { ElevationOverride };
