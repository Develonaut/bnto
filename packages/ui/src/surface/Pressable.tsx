import type { ComponentProps, CSSProperties } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../utils/cn";

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

type PressableProps = ComponentProps<"button"> & {
  /** Transition spring mode. Default `"bounciest"`. */
  spring?: SpringMode;
  /** Toggle mode — element stays depressed when active. */
  toggle?: boolean;
  /** Programmatic active state (pressed in). */
  active?: boolean;
  /** Programmatic hover state (partially sunk). */
  hovered?: boolean;
  /** Programmatic pressed state (flush with ground). */
  pressed?: boolean;
  /** Muted appearance (desaturated). */
  muted?: boolean;
  /** Merge onto child element instead of wrapping in a `<button>`. */
  asChild?: boolean;
};

export function Pressable({
  spring = "bounciest",
  toggle,
  active,
  hovered,
  pressed,
  muted,
  asChild,
  className,
  style,
  ...props
}: PressableProps) {
  const Comp = asChild ? Slot : "button";
  const dataHover = (hovered && !pressed) ? "" : undefined;
  const dataActive = (pressed || active) ? "" : undefined;
  const dataToggle = toggle ? "" : undefined;

  return (
    <Comp
      data-muted={muted || undefined}
      data-hover={dataHover}
      data-active={dataActive}
      data-toggle={dataToggle}
      className={cn("pressable outline-none", className)}
      style={{ ...SPRING_STYLES[spring], ...style }}
      {...props}
    />
  );
}

export { SPRING_STYLES };
export type { SpringMode };
