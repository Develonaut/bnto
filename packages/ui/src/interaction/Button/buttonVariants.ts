/** Button variant maps and createCn definitions for size/variant resolution. */

import { createCn } from "../../utils/createCn";

/* ── Variant classes ────────────────────────────────────────── */

export type ButtonVariant =
  | "primary"
  | "destructive"
  | "success"
  | "warning"
  | "outline"
  | "ghost"
  | "secondary"
  | "muted";

export const VARIANT_CLASSES: Record<ButtonVariant, string> = {
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

export const textCn = createCn({
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

export const iconCn = createCn({
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

export const textFaceCn = createCn({
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

export const iconFaceCn = createCn({
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

export const RADIUS_BY_SIZE: Record<string, string> = {
  sm: "rounded-sm",
  md: "rounded-md",
};

/* ── Elevation for pushable container (must match iconCn / textCn) ── */

export const ICON_ELEVATION_BY_SIZE: Record<string, string> = {
  sm: "elevation-sm",
  md: "elevation-sm",
};

export const TEXT_ELEVATION_BY_SIZE: Record<string, string> = {
  sm: "elevation-sm",
  md: "elevation-md",
};
