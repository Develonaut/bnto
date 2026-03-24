type ElevationOverride = boolean | "none" | "xs" | "sm" | "md" | "lg";

/** Strip the size-variant's built-in elevation-{xs|sm|md|lg} and replace it. */
export function resolveElevationClass(elevation: ElevationOverride): string | undefined {
  if (elevation === true) return undefined; // use size variant's built-in elevation
  if (elevation === false || elevation === "none") return "elevation-none";
  return `elevation-${elevation}`;
}

/** Remove elevation-xs / elevation-sm / elevation-md / elevation-lg tokens from a class string. */
export function stripSizeElevation(classes: string): string {
  return classes.replace(/\belevation-(?:xs|sm|md|lg)\b/g, "").trim();
}

export type { ElevationOverride };
