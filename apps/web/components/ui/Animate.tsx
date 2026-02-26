"use client";

import * as React from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "@/lib/utils";

/* ── Types ─────────────────────────────────────────────────── */

type Easing = "spring" | "spring-bouncy" | "spring-bouncier" | "out" | "in-out";

interface AnimateBaseProps extends React.HTMLAttributes<HTMLDivElement> {
  asChild?: boolean;
  /** Position in a `<Animate.Stagger>` cascade. Sets `--stagger-index`. */
  index?: number;
  /** Override the animation easing. Sets `animationTimingFunction`. */
  easing?: Easing;
}

/* ── Helpers ────────────────────────────────────────────────── */

function buildStyle(
  base: React.CSSProperties | undefined,
  vars: Record<string, string | number | undefined>,
  easing?: Easing,
): React.CSSProperties {
  const custom: Record<string, string | number> = {};
  for (const [k, v] of Object.entries(vars)) {
    if (v != null) custom[k] = typeof v === "number" ? v : v;
  }
  if (easing) custom.animationTimingFunction = `var(--ease-${easing})`;
  return { ...base, ...custom } as React.CSSProperties;
}

/* ── Stagger ────────────────────────────────────────────────── */

interface StaggerProps extends React.HTMLAttributes<HTMLDivElement> {
  asChild?: boolean;
  /** Gap between each child's animation start. Sets `--stagger-interval`. */
  interval?: number | string;
}

const Stagger = React.forwardRef<HTMLDivElement, StaggerProps>(
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

const ScaleIn = React.forwardRef<HTMLDivElement, ScaleInProps>(
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

const FadeIn = React.forwardRef<HTMLDivElement, AnimateBaseProps>(
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

const SlideUp = React.forwardRef<HTMLDivElement, SlideProps>(
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

const SlideDown = React.forwardRef<HTMLDivElement, SlideProps>(
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

const PulseSoft = React.forwardRef<HTMLDivElement, Omit<AnimateBaseProps, "index" | "easing">>(
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

const Breathe = React.forwardRef<HTMLDivElement, Omit<AnimateBaseProps, "index" | "easing">>(
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

/* ── InView (extracted) ─────────────────────────────────────── */

import { InView } from "./InView";

/* ── Namespace ──────────────────────────────────────────────── */

function AnimateRoot({ children }: { children: React.ReactNode }) {
  return <>{children}</>;
}

export const Animate = Object.assign(AnimateRoot, {
  Root: AnimateRoot,
  Stagger,
  ScaleIn,
  FadeIn,
  SlideUp,
  SlideDown,
  PulseSoft,
  Breathe,
  InView,
});

/* Re-export InView for direct import from server components.
 * Server components can't access Object.assign namespace properties
 * on client references — use `import { InView }` instead of
 * `Animate.InView` when rendering from a server component. */
export { InView };
