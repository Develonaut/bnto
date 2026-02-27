import * as React from "react";

import Link from "next/link";
import { Slot } from "@radix-ui/react-slot";

import { cn } from "@/lib/cn";
import { createCn } from "./createCn";

/* ── Spring modes ────────────────────────────────────────────
 * Controls the hover/active/release transition spring.
 * Matches the sm/md/lg pattern used by elevation and size.
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
 * surface, colors. With asChild, the child owns its appearance.
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
type ButtonSize = "md" | "icon";

const buttonCn = createCn({
  base: "surface inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium shrink-0 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  variants: {
    variant: {
      primary: "surface-primary",
      destructive: "surface-destructive",
      success: "surface-success",
      warning: "surface-warning",
      outline: "surface-outline",
      secondary: "surface-secondary",
      muted: "surface-muted",
    },
    size: {
      md: "h-9 px-4 py-2 has-[>svg]:px-3 elevation-md",
      icon: "size-9 elevation-md",
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

type ElevationOverride = boolean | "none" | "sm" | "md" | "lg";

/** Strip the size-variant's built-in elevation-{sm|md|lg} and replace it. */
function resolveElevationClass(elevation: ElevationOverride): string | undefined {
  if (elevation === true) return undefined; // use size variant's built-in elevation
  if (elevation === false || elevation === "none") return "elevation-none";
  return `elevation-${elevation}`;
}

/** Remove elevation-sm / elevation-md / elevation-lg tokens from a class string. */
function stripSizeElevation(classes: string): string {
  return classes.replace(/\belevation-(?:sm|md|lg)\b/g, "").trim();
}

function Button({
  className,
  variant,
  size,
  elevation = true,
  spring = "lg",
  muted = false,
  hovered = false,
  pressed = false,
  toggle = false,
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
    elevation?: ElevationOverride;
    spring?: SpringMode;
    muted?: boolean;
    hovered?: boolean;
    pressed?: boolean;
    toggle?: boolean;
    href?: string;
    ref?: React.Ref<HTMLElement>;
  }) {
  const isLink = !!href;
  const isInternal = isLink && href.startsWith("/") && !props.target;
  const Comp: React.ElementType = asChild
    ? Slot
    : isInternal
      ? Link
      : isLink
        ? "a"
        : "button";
  const dataHover = hovered && !pressed ? "" : undefined;
  const dataActive = pressed ? "" : undefined;
  const dataToggle = toggle ? "" : undefined;
  const elevationClass = resolveElevationClass(elevation);
  // When elevation is overridden (not true), strip the size variant's built-in
  // elevation-{sm|md|lg} so it doesn't compete via CSS source order.
  const sizeClasses = !asChild ? buttonCn({ variant, size }) : "";
  const resolvedSizeClasses = elevationClass
    ? stripSizeElevation(sizeClasses)
    : sizeClasses;

  return (
    <Comp
      ref={ref}
      data-slot="button"
      data-muted={muted || undefined}
      data-hover={dataHover}
      data-active={dataActive}
      data-toggle={dataToggle}
      {...(isLink ? { href } : {})}
      className={cn(PRESSABLE_BASE, resolvedSizeClasses, elevationClass, className)}
      style={{ ...SPRING_STYLES[spring], ...style }}
      {...props}
    />
  );
}

export { Button, buttonCn };
export type { SpringMode };
