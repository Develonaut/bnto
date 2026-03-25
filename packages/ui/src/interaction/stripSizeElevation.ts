/** Remove elevation-xs / elevation-sm / elevation-md / elevation-lg tokens from a class string. */

export function stripSizeElevation(classes: string): string {
  return classes.replace(/\belevation-(?:xs|sm|md|lg)\b/g, "").trim();
}
