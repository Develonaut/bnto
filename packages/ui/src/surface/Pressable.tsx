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

/** Entrance-only spring styles for `.springable` (Surface dormant reveals).
 *  Uses `--spring-dur`/`--spring-ease` so the slow spring timing doesn't
 *  cascade into child `.pressable` hover transitions. */
const SPRING_ENTRANCE_STYLES: Record<SpringMode, CSSProperties> = {
  bouncy: {},
  bouncier: {
    "--spring-ease": "var(--ease-spring-bouncier)",
    "--spring-dur": "400ms",
  } as CSSProperties,
  bounciest: {
    "--spring-ease": "var(--ease-spring-pressable)",
    "--spring-dur": "550ms",
  } as CSSProperties,
};

export { SPRING_STYLES, SPRING_ENTRANCE_STYLES };
export type { SpringMode };
