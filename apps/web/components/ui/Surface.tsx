import type { ComponentProps } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "@/lib/cn";

import { SPRING_STYLES } from "./Pressable";
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

type SurfaceProps = ComponentProps<"div"> & {
  /** Color variant. `"default"` uses the card surface (no color class). */
  variant?: SurfaceVariant;
  /** Elevation tier. Default `"md"`. */
  elevation?: SurfaceElevation;
  /** Border radius. Default `"lg"`. */
  rounded?: SurfaceRounded;
  /** Spring animation mode. Adds `.springable` and configures the transition. */
  spring?: SpringMode;
  /** Flush with the ground plane, muted appearance. Requires `spring`. */
  grounded?: boolean;
  /** Merge onto child element instead of wrapping in a `<div>`. */
  asChild?: boolean;
};

export function Surface({
  variant = "default",
  elevation = "md",
  rounded = "lg",
  spring,
  grounded,
  asChild,
  className,
  style,
  ...props
}: SurfaceProps) {
  const Comp = asChild ? Slot : "div";
  return (
    <Comp
      data-grounded={spring && grounded ? "" : undefined}
      className={cn(
        "surface",
        `elevation-${elevation}`,
        variant !== "default" && `surface-${variant}`,
        spring && "springable",
        roundedMap[rounded],
        className,
      )}
      style={spring ? { ...SPRING_STYLES[spring], ...style } : style}
      {...props}
    />
  );
}

export type { SurfaceVariant, SurfaceElevation, SurfaceRounded };
