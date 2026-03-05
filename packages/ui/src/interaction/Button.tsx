import type { ComponentProps, Ref, ElementType } from "react";

import Link from "next/link";
import { Slot } from "@radix-ui/react-slot";

import { cn } from "../utils/cn";
import { createCn } from "../utils/createCn";
import { SPRING_STYLES } from "../surface/Pressable";
import type { SpringMode } from "../surface/Pressable";
import { resolveElevationClass, stripSizeElevation } from "./resolveElevation";
import type { ElevationOverride } from "./resolveElevation";

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
  | "ghost"
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
      ghost: "surface-ghost",
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

type ButtonProps = Omit<ComponentProps<"button">, "ref"> &
  Omit<ComponentProps<"a">, "ref"> & {
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
    ref?: Ref<HTMLElement>;
  };

function Button({
  className,
  variant,
  size,
  elevation = true,
  spring = "bounciest",
  muted = false,
  hovered = false,
  pressed = false,
  toggle = false,
  asChild = false,
  href,
  style,
  ref,
  ...props
}: ButtonProps) {
  const Comp = resolveComponent(asChild, href, props.target);
  const elevationClass = resolveElevationClass(elevation);
  const sizeClasses = !asChild ? buttonCn({ variant, size }) : "";
  const resolvedSizeClasses = elevationClass
    ? stripSizeElevation(sizeClasses)
    : sizeClasses;

  return (
    <Comp
      ref={ref}
      data-slot="button"
      data-muted={muted || undefined}
      data-hover={hovered && !pressed ? "" : undefined}
      data-active={pressed ? "" : undefined}
      data-toggle={toggle ? "" : undefined}
      {...(!!href ? { href } : {})}
      className={cn(PRESSABLE_BASE, resolvedSizeClasses, elevationClass, className)}
      style={{ ...SPRING_STYLES[spring], ...style }}
      {...props}
    />
  );
}

function resolveComponent(asChild: boolean, href?: string, target?: string): ElementType {
  if (asChild) return Slot;
  if (!href) return "button";
  if (href.startsWith("/") && !target) return Link;
  return "a";
}

export { Button, buttonCn };
export type { SpringMode };
