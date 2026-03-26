import type { CSSProperties } from "react";

import { SPRING_STYLES } from "../../surface/Pressable";
import type { SpringMode } from "../../surface/Pressable";
import { buildDataAttrs } from "./resolveButtonProps";

interface SharedPropsInput {
  ref: unknown;
  disabled: boolean | undefined;
  href: string | undefined;
  spring: SpringMode;
  style: CSSProperties | undefined;
  muted: boolean;
  hovered: boolean;
  pressed: boolean;
  dormant: boolean;
  toggle: boolean;
  rest: Record<string, unknown>;
}

/** Assemble the prop bag shared by both rendering paths. */
export function buildSharedProps({
  ref,
  disabled,
  href,
  spring,
  style,
  muted,
  hovered,
  pressed,
  dormant,
  toggle,
  rest,
}: SharedPropsInput): Record<string, unknown> {
  return {
    ref,
    disabled,
    ...(href ? { href } : {}),
    style: { ...SPRING_STYLES[spring], ...style },
    ...buildDataAttrs(muted, hovered, pressed, dormant, toggle),
    ...rest,
  };
}
