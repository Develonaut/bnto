import type { ComponentProps } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "@/lib/cn";

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
  /**
   * Loading state — surface sits flat on the ground with muted appearance.
   * When loading ends, walls and shadow spring up to the target elevation.
   */
  loading?: boolean;
  /** Merge onto child element instead of wrapping in a `<div>`. */
  asChild?: boolean;
};

export function Surface({
  variant = "default",
  elevation = "md",
  rounded = "lg",
  loading,
  asChild,
  className,
  ...props
}: SurfaceProps) {
  const Comp = asChild ? Slot : "div";
  return (
    <Comp
      data-loading={loading || undefined}
      className={cn(
        "surface",
        `elevation-${elevation}`,
        variant !== "default" && `surface-${variant}`,
        roundedMap[rounded],
        className,
      )}
      {...props}
    />
  );
}

export type { SurfaceVariant, SurfaceElevation, SurfaceRounded };
