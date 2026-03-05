"use client";

import { forwardRef } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../../utils/cn";
import { buildStyle } from "./buildStyle";
import type { AnimateBaseProps } from "./types";

type Origin =
  | "center"
  | "top"
  | "top-left"
  | "top-right"
  | "bottom"
  | "bottom-left"
  | "bottom-right"
  | "left"
  | "right";

const ORIGIN_MAP: Record<Origin, string> = {
  center: "center center",
  top: "top center",
  "top-left": "top left",
  "top-right": "top right",
  bottom: "bottom center",
  "bottom-left": "bottom left",
  "bottom-right": "bottom right",
  left: "center left",
  right: "center right",
};

interface ScaleInProps extends AnimateBaseProps {
  /** Starting scale (0-1). Sets `--scale-from`. Default 0.9. */
  from?: number;
  /** Transform origin direction. Default "center". */
  origin?: Origin;
}

const ScaleIn = forwardRef<HTMLDivElement, ScaleInProps>(
  ({ from, origin, index, easing, asChild, className, style, ...props }, ref) => {
    const Comp = asChild ? Slot : "div";
    return (
      <Comp
        ref={ref}
        className={cn("motion-safe:animate-scale-in", className)}
        style={buildStyle(style, {
          "--scale-from": from != null ? String(from) : undefined,
          "--stagger-index": index,
          transformOrigin: origin ? ORIGIN_MAP[origin] : undefined,
        }, easing)}
        {...props}
      />
    );
  },
);
ScaleIn.displayName = "Animate.ScaleIn";

export { ScaleIn };
