"use client";

import { forwardRef, Children, isValidElement, cloneElement } from "react";
import type { HTMLAttributes, CSSProperties, ReactNode } from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "../utils/cn";

/* ── Types ─────────────────────────────────────────────────── */

type Easing = "spring" | "spring-bouncy" | "spring-bouncier" | "out" | "in-out";

interface AnimateBaseProps extends HTMLAttributes<HTMLDivElement> {
  asChild?: boolean;
  /** Position in a `<Animate.Stagger>` cascade. Sets `--stagger-index`. */
  index?: number;
  /** Override the animation easing. Sets `animationTimingFunction`. */
  easing?: Easing;
}

/* ── Helpers ────────────────────────────────────────────────── */

function buildStyle(
  base: CSSProperties | undefined,
  vars: Record<string, string | number | undefined>,
  easing?: Easing,
): CSSProperties {
  const custom: Record<string, string | number> = {};
  for (const [k, v] of Object.entries(vars)) {
    if (v != null) custom[k] = typeof v === "number" ? v : v;
  }
  if (easing) custom.animationTimingFunction = `var(--ease-${easing})`;
  return { ...base, ...custom } as CSSProperties;
}

/* ── Stagger ────────────────────────────────────────────────── */

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

/* ── ScaleIn ────────────────────────────────────────────────── */

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

/* ── FadeIn ─────────────────────────────────────────────────── */

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

/* ── SlideUp ────────────────────────────────────────────────── */

interface SlideProps extends AnimateBaseProps {
  /** Slide distance. Number = px, string = any CSS unit. Sets `--slide-distance`. */
  distance?: number | string;
}

const SlideUp = forwardRef<HTMLDivElement, SlideProps>(
  ({ distance, index, easing, asChild, className, style, ...props }, ref) => {
    const Comp = asChild ? Slot : "div";
    return (
      <Comp
        ref={ref}
        className={cn("motion-safe:animate-slide-up", className)}
        style={buildStyle(style, {
          "--slide-distance": distance != null ? `${distance}${typeof distance === "number" ? "px" : ""}` : undefined,
          "--stagger-index": index,
        }, easing)}
        {...props}
      />
    );
  },
);
SlideUp.displayName = "Animate.SlideUp";

/* ── SlideDown ──────────────────────────────────────────────── */

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

/* ── PulseSoft ──────────────────────────────────────────────── */

const PulseSoft = forwardRef<HTMLDivElement, Omit<AnimateBaseProps, "index" | "easing">>(
  ({ asChild, className, style, ...props }, ref) => {
    const Comp = asChild ? Slot : "div";
    return (
      <Comp
        ref={ref}
        className={cn("motion-safe:animate-pulse-soft", className)}
        style={style}
        {...props}
      />
    );
  },
);
PulseSoft.displayName = "Animate.PulseSoft";

/* ── Breathe ────────────────────────────────────────────────── */

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

/* ── BouncyStagger ─────────────────────────────────────────────
 * Composed Stagger + ScaleIn that auto-wraps each child.
 * Drop it in with a className and children bounce onto the map.
 *
 *   <Animate.BouncyStagger className="grid grid-cols-3 gap-4">
 *     <Card>A</Card>
 *     <Card>B</Card>
 *     <Card>C</Card>
 *   </Animate.BouncyStagger>
 *
 * With `asChild`, merges stagger onto the single child element
 * and ScaleIn onto each of its children. Useful for Grid layouts
 * where items need to stay as direct grid children:
 *
 *   <Animate.BouncyStagger asChild>
 *     <Grid cols={6} rows={4} gap="md">
 *       <Grid.Item colSpan={4} rowSpan={3}>...</Grid.Item>
 *       <Grid.Item colSpan={2} rowSpan={2}>...</Grid.Item>
 *     </Grid>
 *   </Animate.BouncyStagger>
 */

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
      /* asChild mode: merge stagger onto the single child, and
       * ScaleIn asChild onto each of that child's children.
       * This keeps items as direct grid children (no wrapper divs). */
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

/* ── InView (extracted) ─────────────────────────────────────── */

import { InView } from "./InView";

/* ── Namespace ──────────────────────────────────────────────── */

function AnimateRoot({ children }: { children: ReactNode }) {
  return <>{children}</>;
}

export const Animate = Object.assign(AnimateRoot, {
  Root: AnimateRoot,
  Stagger,
  ScaleIn,
  BouncyStagger,
  FadeIn,
  SlideUp,
  SlideDown,
  PulseSoft,
  Breathe,
  InView,
});

/* Re-export for direct import from server components.
 * Server components can't access Object.assign namespace properties
 * on client references — use named imports instead of
 * `Animate.X` when rendering from a server component. */
export { InView, BouncyStagger };
