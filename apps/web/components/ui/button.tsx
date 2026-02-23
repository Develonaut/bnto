import * as React from "react";

import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

const buttonVariants = cva(
  "depth pressable inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium disabled:pointer-events-none disabled:grayscale disabled:contrast-75 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground depth-default",
        destructive:
          "bg-destructive text-white focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:brightness-75 depth-destructive",
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
  asChild = false,
  ...props
}: React.ComponentProps<"button"> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean;
    depth?: boolean;
  }) {
  const Comp = asChild ? Slot : "button";

  return (
    <Comp
      data-slot="button"
      className={cn(
        buttonVariants({ variant, size, className }),
        !depth && "depth-none",
      )}
      {...props}
    />
  );
}

export { Button, buttonVariants };
