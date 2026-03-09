import type { ComponentProps, ReactNode } from "react";

import { createCn } from "../utils/createCn";

type IconBadgeVariant =
  | "primary"
  | "secondary"
  | "accent"
  | "muted"
  | "destructive"
  | "success"
  | "warning"
  | "info";
type IconBadgeSize = "sm" | "md" | "lg";
type IconBadgeShape = "circle" | "square";

const iconBadgeCn = createCn({
  base: "flex shrink-0 items-center justify-center",
  variants: {
    variant: {
      primary: "bg-primary/10 text-primary",
      secondary: "bg-secondary/20 text-secondary-foreground",
      accent: "bg-accent/15 text-accent",
      muted: "bg-muted text-muted-foreground",
      destructive: "bg-destructive/10 text-destructive",
      success: "bg-success/10 text-success",
      warning: "bg-warning/10 text-warning",
      info: "bg-chart-2/10 text-chart-2",
    },
    size: {
      sm: "size-8",
      md: "size-9",
      lg: "size-10",
    },
    shape: {
      circle: "rounded-full",
      square: "rounded-lg",
    },
  },
  defaultVariants: {
    variant: "primary",
    size: "lg",
    shape: "circle",
  },
});

type IconBadgeProps = Omit<ComponentProps<"div">, "children"> & {
  variant?: IconBadgeVariant;
  size?: IconBadgeSize;
  shape?: IconBadgeShape;
  children: ReactNode;
};

export function IconBadge({ variant, size, shape, className, ...props }: IconBadgeProps) {
  return <div className={iconBadgeCn({ variant, size, shape }, className)} {...props} />;
}
