import type { ComponentProps, ReactNode, Ref, ElementType } from "react";

import Link from "next/link";
import { Slot } from "@radix-ui/react-slot";

import { cn } from "../utils/cn";
import { createCn } from "../utils/createCn";
import { useButtonProps, VARIANT_CLASSES } from "./useButtonProps";
import type { ButtonVariant } from "./useButtonProps";
import { stripSizeElevation } from "./resolveElevation";
import type { ElevationOverride } from "./resolveElevation";
import type { SpringMode } from "../surface/Pressable";

/* ── Size classes ─────────────────────────────────────────────
 * Button-specific layout: height, padding, font-size, rounded,
 * built-in elevation per size. These are NOT in useButtonProps
 * because they're specific to the Button component shape.
 * ──────────────────────────────────────────────────────────── */

type ButtonSize = "sm" | "md" | "lg" | "icon";

/* ── Text button sizes ─────────────────────────────────────── */
const textCn = createCn({
  base: "inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium shrink-0 [&_svg]:pointer-events-none [&_svg]:shrink-0 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
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
  base: "inline-flex items-center justify-center shrink-0 [&_svg]:pointer-events-none [&_svg]:shrink-0",
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

  // Behavior + surface layer from the shared hook
  const { props: buttonProps } = useButtonProps({
    variant,
    elevation,
    spring,
    pressed,
    hovered,
    muted,
    toggle,
  });

  // Appearance layer — size, layout, typography (Button-specific, skipped with asChild)
  const sizeClasses = !asChild
    ? isIcon
      ? iconCn({ variant, size: resolvedSize })
      : textCn({ variant, size: resolvedSize })
    : "";
  const resolvedSizeClasses = buttonProps.className.includes("elevation-")
    ? stripSizeElevation(sizeClasses)
    : sizeClasses;

  return (
    <Comp
      ref={ref}
      data-slot="button"
      {...buttonProps}
      {...(!!href ? { href } : {})}
      className={cn(buttonProps.className, resolvedSizeClasses, className)}
      style={{ ...buttonProps.style, ...style }}
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
