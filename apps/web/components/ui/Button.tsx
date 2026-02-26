import * as React from "react";

import { Slot } from "@radix-ui/react-slot";

import { cn } from "@/lib/cn";
import { createCn } from "./createCn";

/* ── Spring modes ────────────────────────────────────────────
 * Controls the hover/active/release transition spring.
 * Matches the sm/md/lg pattern used by depth and size.
 *   sm: 150ms ease-out (firm, no overshoot)
 *   md: 400ms spring-bouncier (gentle single bounce)
 *   lg: 550ms spring-pressable (rubber band, 3 oscillations)
 * ──────────────────────────────────────────────────────────── */

type SpringMode = "sm" | "md" | "lg";

const SPRING_STYLES: Record<SpringMode, React.CSSProperties> = {
  sm: {},
  md: {
    "--pressable-ease": "var(--ease-spring-bouncier)",
    "--pressable-dur": "400ms",
  } as React.CSSProperties,
  lg: {
    "--pressable-ease": "var(--ease-spring-pressable)",
    "--pressable-dur": "550ms",
  } as React.CSSProperties,
};

/* ── Class split ──────────────────────────────────────────────
 * Behavior: always applied (even with asChild). Pressable
 * interaction, focus ring, disabled state.
 * Appearance: only for standalone buttons. Layout, typography,
 * depth, colors. With asChild, the child owns its appearance.
 * ──────────────────────────────────────────────────────────── */

const PRESSABLE_BASE = "pressable outline-none";

type ButtonVariant =
  | "primary"
  | "destructive"
  | "success"
  | "warning"
  | "outline"
  | "secondary"
  | "muted";
type ButtonSize = "md" | "sm" | "lg" | "icon" | "icon-sm" | "icon-lg";

const buttonCn = createCn({
  base: "depth bg-[var(--face-bg)] text-[var(--face-fg)] inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium shrink-0 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  variants: {
    variant: {
      primary: "depth-primary",
      destructive: "depth-destructive",
      success: "depth-success",
      warning: "depth-warning",
      outline: "depth-outline",
      secondary: "depth-secondary",
      muted: "depth-muted",
    },
    size: {
      md: "h-9 px-4 py-2 has-[>svg]:px-3 depth-md",
      sm: "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5 depth-sm",
      lg: "h-10 rounded-md px-6 has-[>svg]:px-4 depth-lg",
      icon: "size-9 depth-md",
      "icon-sm": "size-8 depth-md",
      "icon-lg": "size-10 depth-md",
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

type DepthOverride = boolean | "none" | "sm" | "md" | "lg";

/** Strip the size-variant's built-in depth-{sm|md|lg} and replace it. */
function resolveDepthClass(depth: DepthOverride): string | undefined {
  if (depth === true) return undefined; // use size variant's built-in depth
  if (depth === false || depth === "none") return "depth-none";
  return `depth-${depth}`;
}

/** Remove depth-sm / depth-md / depth-lg tokens from a class string. */
function stripSizeDepth(classes: string): string {
  return classes.replace(/\bdepth-(?:sm|md|lg)\b/g, "").trim();
}

function Button({
  className,
  variant,
  size,
  depth = true,
  spring = "lg",
  muted = false,
  hovered = false,
  pressed = false,
  asChild = false,
  href,
  style,
  ref,
  ...props
}: Omit<React.ComponentProps<"button">, "ref"> &
  Omit<React.ComponentProps<"a">, "ref"> & {
    variant?: ButtonVariant;
    size?: ButtonSize;
    asChild?: boolean;
    depth?: DepthOverride;
    spring?: SpringMode;
    muted?: boolean;
    hovered?: boolean;
    pressed?: boolean;
    href?: string;
    ref?: React.Ref<HTMLElement>;
  }) {
  const isLink = !!href;
  const Comp: React.ElementType = asChild ? Slot : isLink ? "a" : "button";
  const forceState = pressed ? "active" : hovered ? "hover" : undefined;
  const depthClass = resolveDepthClass(depth);
  // When depth is overridden (not true), strip the size variant's built-in
  // depth-{sm|md|lg} so it doesn't compete via CSS source order.
  const sizeClasses = !asChild ? buttonCn({ variant, size }) : "";
  const resolvedSizeClasses = depthClass
    ? stripSizeDepth(sizeClasses)
    : sizeClasses;

  return (
    <Comp
      ref={ref}
      data-slot="button"
      data-muted={muted || undefined}
      data-force-state={forceState}
      {...(isLink ? { href } : {})}
      className={cn(PRESSABLE_BASE, resolvedSizeClasses, depthClass, className)}
      style={{ ...SPRING_STYLES[spring], ...style }}
      {...props}
    />
  );
}

export { Button, buttonCn };
export type { SpringMode };
