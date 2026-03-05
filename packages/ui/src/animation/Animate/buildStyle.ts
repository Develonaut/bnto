import type { CSSProperties } from "react";

export type Easing = "spring" | "spring-bouncy" | "spring-bouncier" | "out" | "in-out";

export function buildStyle(
  base: CSSProperties | undefined,
  vars: Record<string, string | number | undefined>,
  easing?: Easing,
): CSSProperties {
  const custom: Record<string, string | number> = {};
  for (const [k, v] of Object.entries(vars)) {
    if (v != null) custom[k] = typeof v === "number" ? v : v;
  }
  if (easing) custom.animationTimingFunction = `var(--ease-${easing})`;
  return { ...base, ...custom } as CSSProperties;
}
