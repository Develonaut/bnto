"use client";

import { forwardRef, Children, isValidElement, cloneElement } from "react";
import type { HTMLAttributes, ReactNode } from "react";

import { Stagger } from "./Stagger";
import { ScaleIn } from "./ScaleIn";
import type { Easing } from "./types";

interface BouncyStaggerProps extends HTMLAttributes<HTMLDivElement> {
  /** Merge stagger + scale-in onto child and its children. */
  asChild?: boolean;
  /** Starting scale (0-1). Default 0.6. */
  from?: number;
  /** Spring easing. Default "spring-bouncy". */
  easing?: Easing;
  /** Gap between each child's animation start. Default 60ms. */
  interval?: number | string;
}

const BouncyStagger = forwardRef<HTMLDivElement, BouncyStaggerProps>(
  ({ from = 0.6, easing = "spring-bouncy", interval, asChild, className, children, ...props }, ref) => {
    if (asChild) {
      const child = Children.only(children);
      if (!isValidElement(child)) return null;
      const grandchildren = (child.props as { children?: ReactNode }).children;
      const animated = Children.map(grandchildren, (gc, i) =>
        gc != null ? (
          <ScaleIn key={i} index={i} from={from} easing={easing} asChild>
            {gc}
          </ScaleIn>
        ) : null,
      );
      return (
        <Stagger ref={ref} interval={interval} className={className} asChild {...props}>
          {cloneElement(child, undefined, animated)}
        </Stagger>
      );
    }

    return (
      <Stagger ref={ref} interval={interval} className={className} {...props}>
        {Children.map(children, (child, i) =>
          child != null ? (
            <ScaleIn key={i} index={i} from={from} easing={easing}>
              {child}
            </ScaleIn>
          ) : null,
        )}
      </Stagger>
    );
  },
);
BouncyStagger.displayName = "Animate.BouncyStagger";

export { BouncyStagger };
