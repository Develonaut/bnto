"use client";

import { forwardRef } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../../utils/cn";
import type { AnimateBaseProps } from "./types";

const Breathe = forwardRef<HTMLDivElement, Omit<AnimateBaseProps, "index" | "easing">>(
  ({ asChild, className, style, ...props }, ref) => {
    const Comp = asChild ? Slot : "div";
    return (
      <Comp
        ref={ref}
        className={cn("motion-safe:animate-breathe", className)}
        style={style}
        {...props}
      />
    );
  },
);
Breathe.displayName = "Animate.Breathe";

export { Breathe };
