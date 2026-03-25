/** Button component — three-span pushable with variant and size resolution. */

import type { ComponentProps, ReactNode, Ref, ElementType } from "react";

import Link from "next/link";
import { Slot } from "@radix-ui/react-slot";

import { cn } from "../../utils/cn";
import { SPRING_STYLES } from "../../surface/Pressable";
import type { SpringMode } from "../../surface/Pressable";
import { resolveElevationClass, type ElevationOverride } from "../resolveElevationClass";
import { stripSizeElevation } from "../stripSizeElevation";

import {
  type ButtonVariant,
  VARIANT_CLASSES,
  textCn,
  iconCn,
  textFaceCn,
  iconFaceCn,
  RADIUS_BY_SIZE,
  ICON_ELEVATION_BY_SIZE,
  TEXT_ELEVATION_BY_SIZE,
} from "./buttonVariants";

/* ── Types ─────────────────────────────────────────────────── */

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

/* ── Component ─────────────────────────────────────────────── */

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

/* ── Helpers ────────────────────────────────────────────────── */

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
