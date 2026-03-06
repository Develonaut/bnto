import type { ComponentProps, ReactNode, Ref, ElementType } from "react";

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

type ButtonSize = "sm" | "md" | "lg" | "icon";

const VARIANT_CLASSES = {
  primary: "surface-primary",
  destructive: "surface-destructive",
  success: "surface-success",
  warning: "surface-warning",
  outline: "surface-outline",
  ghost: "surface-ghost",
  secondary: "surface-secondary",
  muted: "surface-muted",
} as const;

/* ── Text button sizes ─────────────────────────────────────── */
const textCn = createCn({
  base: "surface inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium shrink-0 [&_svg]:pointer-events-none [&_svg]:shrink-0 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  variants: {
    variant: VARIANT_CLASSES,
    size: {
      sm: "h-7 px-3 text-xs rounded-sm elevation-sm [&_svg:not([class*='size-'])]:size-3",
      md: "h-9 px-4 py-2 text-sm rounded-md has-[>svg]:px-3 elevation-md [&_svg:not([class*='size-'])]:size-4",
      lg: "h-11 px-6 text-base rounded-lg elevation-lg [&_svg:not([class*='size-'])]:size-5",
      icon: "h-9 px-4 py-2 text-sm rounded-md has-[>svg]:px-3 elevation-md [&_svg:not([class*='size-'])]:size-4",
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

/* ── Icon button sizes ─────────────────────────────────────── */
const iconCn = createCn({
  base: "surface inline-flex items-center justify-center shrink-0 [&_svg]:pointer-events-none [&_svg]:shrink-0",
  variants: {
    variant: VARIANT_CLASSES,
    size: {
      sm: "size-6 rounded-sm elevation-sm [&_svg]:size-3",
      md: "size-9 rounded-md elevation-md [&_svg]:size-4",
      lg: "size-11 rounded-lg elevation-lg [&_svg]:size-5",
      icon: "size-9 rounded-md elevation-md [&_svg]:size-4",
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
    /** Pass an icon element to render as a square icon button. */
    icon?: ReactNode;
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
  icon,
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
  children,
  ...props
}: ButtonProps) {
  const Comp = resolveComponent(asChild, href, props.target);
  const isIcon = icon !== undefined || size === "icon";
  const resolvedSize = size === "icon" ? "md" : (size ?? "md");
  const elevationClass = resolveElevationClass(elevation);
  const sizeClasses = !asChild
    ? isIcon
      ? iconCn({ variant, size: resolvedSize })
      : textCn({ variant, size: resolvedSize })
    : "";
  const resolvedSizeClasses = elevationClass ? stripSizeElevation(sizeClasses) : sizeClasses;

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
    >
      {isIcon ? (icon ?? children) : children}
    </Comp>
  );
}

function resolveComponent(asChild: boolean, href?: string, target?: string): ElementType {
  if (asChild) return Slot;
  if (!href) return "button";
  if (href.startsWith("/") && !target) return Link;
  return "a";
}

export { Button, textCn as buttonCn };
export type { SpringMode };
