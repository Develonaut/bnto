"use client";

import { forwardRef } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../../utils/cn";
import { buildStyle } from "./buildStyle";
import type { AnimateBaseProps } from "./types";

const FadeIn = forwardRef<HTMLDivElement, AnimateBaseProps>(
  ({ index, easing, asChild, className, style, ...props }, ref) => {
    const Comp = asChild ? Slot : "div";
    return (
      <Comp
        ref={ref}
        className={cn("motion-safe:animate-fade-in", className)}
        style={buildStyle(style, {
          "--stagger-index": index,
        }, easing)}
        {...props}
      />
    );
  },
);
FadeIn.displayName = "Animate.FadeIn";

export { FadeIn };
