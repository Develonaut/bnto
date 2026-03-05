"use client";

import { forwardRef } from "react";
import type { HTMLAttributes } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../../utils/cn";
import { buildStyle } from "./buildStyle";

interface StaggerProps extends HTMLAttributes<HTMLDivElement> {
  asChild?: boolean;
  /** Gap between each child's animation start. Sets `--stagger-interval`. */
  interval?: number | string;
}

const Stagger = forwardRef<HTMLDivElement, StaggerProps>(
  ({ interval, asChild, className, style, ...props }, ref) => {
    const Comp = asChild ? Slot : "div";
    return (
      <Comp
        ref={ref}
        className={cn("stagger-cascade", className)}
        style={buildStyle(style, {
          "--stagger-interval": interval != null ? `${interval}${typeof interval === "number" ? "ms" : ""}` : undefined,
        })}
        {...props}
      />
    );
  },
);
Stagger.displayName = "Animate.Stagger";

export { Stagger };
export type { StaggerProps };
