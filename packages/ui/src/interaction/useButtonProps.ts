import type { CSSProperties } from "react";

import { cn } from "../utils/cn";
import { SPRING_STYLES } from "../surface/Pressable";
import type { SpringMode } from "../surface/Pressable";
import { resolveElevationClass } from "./resolveElevation";
import type { ElevationOverride } from "./resolveElevation";

/**
 * useButtonProps — returns the behavioral + visual slots of a Button.
 *
 * Returns:
 *   props — merged onto a single element (how Button uses it)
 *   slots.root.props — pressable interaction (data attributes, spring styles)
 *   slots.surface.props — visual appearance (surface class, variant, elevation)
 *
 * Usage (single element — like Button):
 *   const { props } = useButtonProps({ ... });
 *   <div {...props} className={cn(props.className, "my-classes")} />
 *
 * Usage (split slots — custom layout):
 *   const { slots } = useButtonProps({ ... });
 *   <div {...slots.root.props} {...slots.surface.props}
 *     className={cn(slots.root.props.className, slots.surface.props.className, "my-classes")}
 *   />
 */

type ButtonVariant =
  | "primary"
  | "destructive"
  | "success"
  | "warning"
  | "outline"
  | "ghost"
  | "secondary"
  | "muted";

const VARIANT_CLASSES: Record<ButtonVariant, string> = {
  primary: "surface-primary",
  destructive: "surface-destructive",
  success: "surface-success",
  warning: "surface-warning",
  outline: "surface-outline",
  ghost: "surface-ghost",
  secondary: "surface-secondary",
  muted: "surface-muted",
};

interface UseButtonPropsInput {
  /** Surface color variant. No default — omit for uncolored surface. */
  variant?: ButtonVariant;
  /** Elevation override. Default: "md". */
  elevation?: ElevationOverride;
  /** Spring transition mode. Default: "bounciest". */
  spring?: SpringMode;
  /** Programmatic pressed state (flush with ground). */
  pressed?: boolean;
  /** Programmatic hover state (partially sunk). */
  hovered?: boolean;
  /** Muted appearance (desaturated). */
  muted?: boolean;
  /** Toggle mode — stays depressed when active. */
  toggle?: boolean;
}

interface SlotProps {
  className: string;
  style: CSSProperties;
  [key: string]: unknown;
}

interface ButtonSlotsResult {
  /** Convenience: root + surface merged onto one element. */
  props: SlotProps;
  /** Individual slots for split composition. */
  slots: {
    /** Pressable interaction — data attributes + spring transition styles. */
    root: { props: SlotProps };
    /** Visual appearance — surface class, variant, elevation. */
    surface: { props: SlotProps };
  };
}

function useButtonProps({
  variant,
  elevation = "md",
  spring = "bounciest",
  pressed = false,
  hovered = false,
  muted = false,
  toggle = false,
}: UseButtonPropsInput): ButtonSlotsResult {
  const elevationClass = resolveElevationClass(elevation);
  const variantClass = variant ? VARIANT_CLASSES[variant] : undefined;

  const rootSlotProps: SlotProps = {
    className: "pressable outline-none",
    style: SPRING_STYLES[spring],
    "data-muted": muted ? "" : undefined,
    "data-hover": hovered && !pressed ? "" : undefined,
    "data-active": pressed ? "" : undefined,
    "data-toggle": toggle ? "" : undefined,
  };

  const surfaceSlotProps: SlotProps = {
    className: cn("surface", variantClass, elevationClass),
    style: {},
  };

  const mergedProps: SlotProps = {
    ...rootSlotProps,
    className: cn(rootSlotProps.className, surfaceSlotProps.className),
    style: rootSlotProps.style,
  };

  return {
    props: mergedProps,
    slots: {
      root: { props: rootSlotProps },
      surface: { props: surfaceSlotProps },
    },
  };
}

export { useButtonProps, VARIANT_CLASSES };
export type { ButtonVariant, UseButtonPropsInput, ButtonSlotsResult };
