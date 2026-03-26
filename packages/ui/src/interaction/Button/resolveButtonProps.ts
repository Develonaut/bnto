import type { ButtonVariant } from "./buttonVariants";
import type { ElevationOverride } from "../resolveElevationClass";

/** Resolve variant based on disabled state. */
export function resolveButtonVariant(
  variant: ButtonVariant | undefined,
  disabled: boolean | undefined,
): ButtonVariant | undefined {
  return disabled ? "muted" : variant;
}

/** Resolve elevation based on disabled state. */
export function resolveButtonElevation(
  elevation: ElevationOverride,
  disabled: boolean | undefined,
): ElevationOverride {
  return disabled ? false : elevation;
}

/** Build data attribute map for button state. */
export function buildDataAttrs(
  muted: boolean,
  hovered: boolean,
  pressed: boolean,
  dormant: boolean,
  toggle: boolean,
) {
  return {
    "data-slot": "button",
    "data-muted": muted ? "" : undefined,
    "data-hover": hovered && !pressed ? "" : undefined,
    "data-active": pressed ? "" : undefined,
    "data-dormant": dormant ? "" : undefined,
    "data-toggle": toggle ? "" : undefined,
  } as const;
}
