"use client";

import { forwardRef } from "react";
import type { HTMLAttributes } from "react";

import { Slot } from "@radix-ui/react-slot";

import { useIntersectionVisible } from "./useIntersectionVisible";

interface InViewProps extends HTMLAttributes<HTMLDivElement> {
  asChild?: boolean;
  /** IntersectionObserver threshold (0-1). Default 0.15. */
  threshold?: number;
  /** Only trigger once -- stays visible after first intersection. Default true. */
  triggerOnce?: boolean;
}

export const InView = forwardRef<HTMLDivElement, InViewProps>(
  (
    { threshold = 0.15, triggerOnce = true, asChild, className, style, children, ...props },
    ref,
  ) => {
    const [inView, setRef] = useIntersectionVisible(threshold, triggerOnce, ref);
    const Comp = asChild ? Slot : "div";

    return (
      <Comp ref={setRef} data-in-view={inView} className={className} style={style} {...props}>
        {children}
      </Comp>
    );
  },
);
InView.displayName = "InView";
