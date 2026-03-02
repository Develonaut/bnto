/**
 * Responsive prop utility for UI components.
 *
 * Allows any prop to accept a raw value OR a breakpoint object:
 *   cols={3}                                      // static
 *   cols={{ mobile: 1, tablet: 2, desktop: 3 }}   // responsive
 *
 * Breakpoints map to Tailwind's default responsive prefixes:
 *   mobile  → base (no prefix)
 *   tablet  → sm:
 *   desktop → lg:
 *
 * Usage in a component:
 *   const classes = resolveResponsive(cols, colsMap);
 *   // e.g. "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3"
 */

/** Breakpoint object shape — all keys optional. */
export interface Breakpoints<T> {
  /** Base / mobile (no prefix). */
  mobile?: T;
  /** Tablet breakpoint (`sm:`). */
  tablet?: T;
  /** Desktop breakpoint (`lg:`). */
  desktop?: T;
}

/** A prop value that can be static or responsive. */
export type ResponsiveProp<T> = T | Breakpoints<T>;

/** Tailwind prefixes for each breakpoint. */
const BREAKPOINT_PREFIX: Record<keyof Breakpoints<unknown>, string> = {
  mobile: "",
  tablet: "sm:",
  desktop: "lg:",
};

/**
 * Resolve a responsive prop into Tailwind class string(s).
 *
 * @param value - A static value OR a breakpoint object.
 * @param classMap - Map from value → Tailwind class (e.g. `{ 1: "grid-cols-1", ... }`).
 * @returns An array of class strings (may be empty). Spread into `cn()`.
 *
 * @example
 *   resolveResponsive(3, colsMap)
 *   // → ["grid-cols-3"]
 *
 *   resolveResponsive({ mobile: 1, tablet: 2, desktop: 3 }, colsMap)
 *   // → ["grid-cols-1", "sm:grid-cols-2", "lg:grid-cols-3"]
 */
export function resolveResponsive<T extends string | number>(
  value: ResponsiveProp<T> | undefined,
  classMap: Record<T, string>,
): string[] {
  if (value == null) return [];

  // Static value — return the single class
  if (typeof value !== "object") {
    const cls = classMap[value];
    return cls ? [cls] : [];
  }

  // Breakpoint object — build prefixed classes
  const result: string[] = [];
  const bp = value as Breakpoints<T>;

  for (const [key, prefix] of Object.entries(BREAKPOINT_PREFIX)) {
    const bpValue = bp[key as keyof Breakpoints<T>];
    if (bpValue == null) continue;
    const cls = classMap[bpValue];
    if (cls) result.push(`${prefix}${cls}`);
  }

  return result;
}
