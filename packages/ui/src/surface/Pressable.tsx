import type { CSSProperties } from "react";

/* ── Spring modes ────────────────────────────────────────────
 * Controls the hover/active/release transition spring.
 * Shared with Button and Surface — import from here to deduplicate.
 *
 *   bouncy:    150ms ease-out (snappy, minimal overshoot — controls)
 *   bouncier:  400ms spring-bouncier (pronounced single bounce)
 *   bounciest: 550ms spring-pressable (rubber band, 3 oscillations)
 * ──────────────────────────────────────────────────────────── */

type SpringMode = "bouncy" | "bouncier" | "bounciest";

const SPRING_STYLES: Record<SpringMode, CSSProperties> = {
  bouncy: {},
  bouncier: {
    "--pressable-ease": "var(--ease-spring-bouncier)",
    "--pressable-dur": "400ms",
  } as CSSProperties,
  bounciest: {
    "--pressable-ease": "var(--ease-spring-pressable)",
    "--pressable-dur": "550ms",
  } as CSSProperties,
};

export { SPRING_STYLES };
export type { SpringMode };
