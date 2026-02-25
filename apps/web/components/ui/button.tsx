import * as React from "react";

import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

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

const PRESSABLE_BASE =
  "pressable outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:pointer-events-none disabled:grayscale disabled:contrast-75";

const buttonVariants = cva(
  "depth inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium shrink-0 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground depth-default",
        destructive:
          "bg-destructive text-white focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 depth-destructive",
        success:
          "bg-success text-success-foreground focus-visible:ring-success/20 dark:focus-visible:ring-success/40 depth-success",
        warning:
          "bg-warning text-warning-foreground focus-visible:ring-warning/20 dark:focus-visible:ring-warning/40 depth-warning",
        outline: "bg-card text-card-foreground depth-outline",
        secondary: "bg-secondary text-secondary-foreground depth-secondary",
        muted: "bg-muted text-muted-foreground depth-muted",
      },
      size: {
        default: "h-9 px-4 py-2 has-[>svg]:px-3 depth-md",
        sm: "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5 depth-sm",
        lg: "h-10 rounded-md px-6 has-[>svg]:px-4 depth-lg",
        icon: "size-9 depth-sm",
        "icon-sm": "size-8 depth-sm",
        "icon-lg": "size-10 depth-md",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
);

function Button({
  className,
  variant,
  size,
  depth = true,
  spring = "lg",
  asChild = false,
  style,
  ...props
}: React.ComponentProps<"button"> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean;
    depth?: boolean;
    spring?: SpringMode;
  }) {
  const Comp = asChild ? Slot : "button";

  return (
    <Comp
      data-slot="button"
      className={cn(
        PRESSABLE_BASE,
        !asChild && buttonVariants({ variant, size }),
        !asChild && !depth && "depth-none",
        className,
      )}
      style={{ ...SPRING_STYLES[spring], ...style }}
      {...props}
    />
  );
}

export { Button, buttonVariants };
export type { SpringMode };
