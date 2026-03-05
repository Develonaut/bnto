import type { ComponentProps } from "react";

import { Slot } from "@radix-ui/react-slot";

import { createCn } from "../utils/createCn";

type BadgeVariant =
  | "default"
  | "primary"
  | "secondary"
  | "destructive"
  | "outline";
type BadgeSize = "sm" | "md";

const badgeCn = createCn({
  base: "inline-flex items-center rounded-full font-medium",
  variants: {
    variant: {
      default: "bg-muted text-muted-foreground",
      primary: "bg-primary text-primary-foreground",
      secondary: "bg-secondary text-secondary-foreground",
      destructive: "bg-destructive text-destructive-foreground",
      outline: "border border-border text-foreground",
    },
    size: {
      sm: "px-2 py-0.5 text-xs",
      md: "px-2.5 py-0.5 text-sm",
    },
  },
  defaultVariants: {
    variant: "default",
    size: "sm",
  },
});

type BadgeProps = ComponentProps<"span"> & {
  variant?: BadgeVariant;
  size?: BadgeSize;
  asChild?: boolean;
};

export function Badge({
  variant,
  size,
  asChild,
  className,
  ...props
}: BadgeProps) {
  const Comp = asChild ? Slot : "span";
  return <Comp className={badgeCn({ variant, size }, className)} {...props} />;
}
