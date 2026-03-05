"use client";

import { forwardRef } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../../utils/cn";
import { buildStyle } from "./buildStyle";
import type { AnimateBaseProps } from "./types";

interface SlideProps extends AnimateBaseProps {
  /** Slide distance. Number = px, string = any CSS unit. Sets `--slide-distance`. */
  distance?: number | string;
}

const SlideDown = forwardRef<HTMLDivElement, SlideProps>(
  ({ distance, index, easing, asChild, className, style, ...props }, ref) => {
    const Comp = asChild ? Slot : "div";
    return (
      <Comp
        ref={ref}
        className={cn("motion-safe:animate-slide-down", className)}
        style={buildStyle(style, {
          "--slide-distance": distance != null ? `${distance}${typeof distance === "number" ? "px" : ""}` : undefined,
          "--stagger-index": index,
        }, easing)}
        {...props}
      />
    );
  },
);
SlideDown.displayName = "Animate.SlideDown";

export { SlideDown };
