import type { ComponentProps } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../utils/cn";

import { SPRING_ENTRANCE_STYLES } from "./Pressable";
import type { SpringMode } from "./Pressable";

type SurfaceVariant =
  | "default"
  | "primary"
  | "secondary"
  | "muted"
  | "destructive"
  | "accent"
  | "success"
  | "warning"
  | "outline"
  | "ghost";

type SurfaceElevation = "none" | "sm" | "md" | "lg";
type SurfaceRounded = "sm" | "md" | "lg" | "xl" | "2xl" | "full";

const roundedMap: Record<SurfaceRounded, string> = {
  sm: "rounded-sm",
  md: "rounded-md",
  lg: "rounded-lg",
  xl: "rounded-xl",
  "2xl": "rounded-2xl",
  full: "rounded-full",
};

type SurfaceBorder = "solid" | "dashed" | "none";

const borderMap: Record<SurfaceBorder, string | undefined> = {
  solid: undefined,
  dashed: "surface-dashed",
  none: "surface-border-none",
};

type SurfaceProps = ComponentProps<"div"> & {
  /** Color variant. `"default"` uses the card surface (no color class). */
  variant?: SurfaceVariant;
  /** Elevation tier. Default `"md"`. */
  elevation?: SurfaceElevation;
  /** Border radius. Default `"lg"`. */
  rounded?: SurfaceRounded;
  /** Spring animation mode. Adds `.springable` and configures the transition. */
  spring?: SpringMode;
  /** Flush with the ground plane, muted appearance. Auto-enables `spring="bounciest"`. */
  dormant?: boolean;
  /** Border style. Default `"solid"`. */
  border?: SurfaceBorder;
  /** Merge onto child element instead of wrapping in a `<div>`. */
  asChild?: boolean;
};

function resolveSurfaceClassName(
  variant: SurfaceVariant,
  elevation: SurfaceElevation,
  rounded: SurfaceRounded,
  spring: SpringMode | undefined,
  border: SurfaceBorder,
  className: string | undefined,
) {
  const resolvedElevation = variant === "ghost" ? "none" : elevation;
  return cn(
    "surface",
    `elevation-${resolvedElevation}`,
    variant !== "default" && `surface-${variant}`,
    spring && "springable",
    borderMap[border],
    roundedMap[rounded],
    className,
  );
}

export function Surface({
  variant = "default",
  elevation = "md",
  rounded = "lg",
  spring,
  dormant,
  border = "solid",
  asChild,
  className,
  style,
  ...props
}: SurfaceProps) {
  const resolvedSpring = spring ?? (dormant !== undefined ? "bounciest" : undefined);
  const Comp = asChild ? Slot : "div";
  return (
    <Comp
      data-dormant={dormant ? "" : undefined}
      className={resolveSurfaceClassName(
        variant,
        elevation,
        rounded,
        resolvedSpring,
        border,
        className,
      )}
      style={resolvedSpring ? { ...SPRING_ENTRANCE_STYLES[resolvedSpring], ...style } : style}
      {...props}
    />
  );
}

export type { SurfaceVariant, SurfaceElevation, SurfaceRounded, SurfaceBorder };
