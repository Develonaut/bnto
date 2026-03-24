import type { ComponentProps, ReactNode, Ref, ElementType } from "react";

import Link from "next/link";
import { Slot } from "@radix-ui/react-slot";

import { cn } from "../utils/cn";
import { createCn } from "../utils/createCn";
import { SPRING_STYLES } from "../surface/Pressable";
import type { SpringMode } from "../surface/Pressable";
import { resolveElevationClass, stripSizeElevation } from "./resolveElevation";
import type { ElevationOverride } from "./resolveElevation";

/* ── Variant classes ────────────────────────────────────────── */

type ButtonVariant =
  | "primary"
  | "destructive"
  | "success"
  | "warning"
  | "outline"
  | "ghost"
  | "secondary"
  | "muted";

const VARIANT_CLASSES: Record<ButtonVariant, string> = {
  primary: "surface-primary",
  destructive: "surface-destructive",
  success: "surface-success",
  warning: "surface-warning",
  outline: "surface-outline",
  ghost: "surface-ghost",
  secondary: "surface-secondary",
  muted: "surface-muted",
};

/* ── Size classes (asChild path — elevation baked in) ──────── */

const textCn = createCn({
  base: "inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium shrink-0 [&_svg]:pointer-events-none [&_svg]:shrink-0 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  variants: {
    variant: VARIANT_CLASSES,
    size: {
      sm: "h-7 px-3 py-1 text-xs rounded-sm has-[>svg]:px-2 elevation-xs [&_svg:not([class*='size-'])]:size-3.5",
      md: "h-9 px-4 py-2 text-sm rounded-md has-[>svg]:px-3 elevation-md [&_svg:not([class*='size-'])]:size-4",
      icon: "h-9 px-4 py-2 text-sm rounded-md has-[>svg]:px-3 elevation-md [&_svg:not([class*='size-'])]:size-4",
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

const iconCn = createCn({
  base: "inline-flex items-center justify-center shrink-0 [&_svg]:pointer-events-none [&_svg]:shrink-0",
  variants: {
    variant: VARIANT_CLASSES,
    size: {
      sm: "size-7 rounded-sm elevation-xs [&_svg]:size-3.5",
      md: "size-9 rounded-md elevation-sm [&_svg]:size-4",
      icon: "size-9 rounded-md elevation-sm [&_svg]:size-4",
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "md",
  },
});

/* ── Face size classes (pushable path — no elevation) ──────── */

const textFaceCn = createCn({
  base: "inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium shrink-0 [&_svg]:pointer-events-none [&_svg]:shrink-0 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  variants: {
    size: {
      sm: "h-7 px-3 py-1 text-xs has-[>svg]:px-2 rounded-sm [&_svg:not([class*='size-'])]:size-3.5",
      md: "h-9 px-4 py-2 text-sm has-[>svg]:px-3 rounded-md [&_svg:not([class*='size-'])]:size-4",
      icon: "h-9 px-4 py-2 text-sm has-[>svg]:px-3 rounded-md [&_svg:not([class*='size-'])]:size-4",
    },
  },
  defaultVariants: { size: "md" },
});

const iconFaceCn = createCn({
  base: "inline-flex items-center justify-center shrink-0 [&_svg]:pointer-events-none [&_svg]:shrink-0",
  variants: {
    size: {
      sm: "size-7 rounded-sm [&_svg]:size-3.5",
      md: "size-9 rounded-md [&_svg]:size-4",
      icon: "size-9 rounded-md [&_svg]:size-4",
    },
  },
  defaultVariants: { size: "md" },
});

/* ── Radius for pushable container (must match face) ───────── */

const RADIUS_BY_SIZE: Record<string, string> = {
  sm: "rounded-sm",
  md: "rounded-md",
};

/* ── Elevation for pushable container (must match iconCn / textCn) ── */

const ICON_ELEVATION_BY_SIZE: Record<string, string> = {
  sm: "elevation-sm",
  md: "elevation-sm",
};

const TEXT_ELEVATION_BY_SIZE: Record<string, string> = {
  sm: "elevation-sm",
  md: "elevation-md",
};

/* ── Button ─────────────────────────────────────────────────── */

type ButtonSize = "sm" | "icon";

type ButtonProps = Omit<ComponentProps<"button">, "ref"> &
  Omit<ComponentProps<"a">, "ref"> & {
    variant?: ButtonVariant;
    size?: ButtonSize;
    icon?: ReactNode;
    as?: ElementType;
    asChild?: boolean;
    elevation?: ElevationOverride;
    spring?: SpringMode;
    fullWidth?: boolean;
    muted?: boolean;
    hovered?: boolean;
    pressed?: boolean;
    dormant?: boolean;
    toggle?: boolean;
    href?: string;
    ref?: Ref<HTMLElement>;
  };

function Button({
  className,
  variant,
  size,
  icon,
  as,
  elevation = true,
  spring = "bounciest",
  fullWidth = false,
  muted = false,
  hovered = false,
  pressed = false,
  dormant = false,
  toggle = false,
  asChild = false,
  href,
  style,
  ref,
  children,
  disabled,
  ...props
}: ButtonProps) {
  const Comp = resolveComponent(as, asChild, href, props.target);
  const isIcon = icon !== undefined || size === "icon";
  const resolvedSize = size === "icon" ? "md" : (size ?? "md");

  const resolvedVariant = disabled ? "muted" : variant;
  const resolvedElevation = disabled ? false : elevation;

  const elevationClass = resolveElevationClass(resolvedElevation);
  const variantClass = resolvedVariant ? VARIANT_CLASSES[resolvedVariant] : undefined;

  const dataAttrs = {
    "data-slot": "button",
    "data-muted": muted ? "" : undefined,
    "data-hover": hovered && !pressed ? "" : undefined,
    "data-active": pressed ? "" : undefined,
    "data-dormant": dormant ? "" : undefined,
    "data-toggle": toggle ? "" : undefined,
  };

  const sharedProps = {
    ref,
    disabled,
    ...(!!href ? { href } : {}),
    style: { ...SPRING_STYLES[spring], ...style },
    ...dataAttrs,
    ...props,
  };

  const content = isIcon ? (icon ?? children) : children;

  // asChild / as — single-element rendering (Slot requires single child)
  if (asChild || as) {
    const behaviorCn = cn(
      "pressable outline-none surface",
      variantClass,
      elevationClass,
      fullWidth && "flex w-full",
    );
    const applySize = size !== undefined || (!asChild && !as);
    const sizeClasses = applySize
      ? isIcon
        ? iconCn({ variant: resolvedVariant, size: resolvedSize })
        : textCn({ variant: resolvedVariant, size: resolvedSize })
      : "";
    const resolvedSizeClasses = behaviorCn.includes("elevation-")
      ? stripSizeElevation(sizeClasses)
      : sizeClasses;

    return (
      <Comp {...sharedProps} className={cn(behaviorCn, resolvedSizeClasses, className)}>
        {content}
      </Comp>
    );
  }

  // Standard — three-span pushable DOM (blur-free animations)
  const sizeElevation = isIcon
    ? ICON_ELEVATION_BY_SIZE[resolvedSize]
    : TEXT_ELEVATION_BY_SIZE[resolvedSize];
  const containerClasses = cn(
    "pushable",
    fullWidth ? "flex w-full" : "inline-flex",
    variantClass,
    elevationClass ?? sizeElevation,
  );
  const radiusClass = RADIUS_BY_SIZE[resolvedSize] ?? "rounded-md";
  const faceClasses = isIcon
    ? iconFaceCn({ size: resolvedSize })
    : cn("flex-1 min-w-0", textFaceCn({ size: resolvedSize }));

  const button = (
    <Comp {...sharedProps} className={cn(containerClasses, radiusClass, className)}>
      <span className="pushable-shadow" aria-hidden="true" />
      <span className="pushable-edge" aria-hidden="true" />
      <span className={cn("pushable-face", faceClasses, className)}>{content}</span>
    </Comp>
  );

  // Dormant buttons self-manage their group wrapper so consumers
  // don't need to add `group` to an ancestor element.
  // Padding extends the hover zone so the button wakes before the
  // cursor is directly on it; negative margin cancels layout shift.
  // Disabled dormant buttons skip the group wrapper — no wake behavior,
  // just opacity + pointer-events-none to match InputWrapper disabled.
  if (dormant) {
    if (disabled) {
      return <span className="inline-flex opacity-50 pointer-events-none">{button}</span>;
    }
    return <span className="group inline-flex p-4 -m-4">{button}</span>;
  }

  return button;
}

function resolveComponent(
  as: ElementType | undefined,
  asChild: boolean,
  href?: string,
  target?: string,
): ElementType {
  if (asChild) return Slot;
  if (as) return as;
  if (!href) return "button";
  if (href.startsWith("/") && !target) return Link;
  return "a";
}

export { Button };
export type { ButtonVariant, SpringMode };
