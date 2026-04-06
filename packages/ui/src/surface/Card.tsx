import { forwardRef } from "react";
import type { HTMLAttributes } from "react";

import { cn } from "../utils/cn";

import { Surface } from "./Surface";
import type { SurfaceBorder, SurfaceElevation, SurfaceVariant } from "./Surface";

export const Card = forwardRef<
  HTMLDivElement,
  HTMLAttributes<HTMLDivElement> & {
    elevation?: SurfaceElevation;
    /** Color variant forwarded to Surface. Default uses card surface colors. */
    color?: SurfaceVariant;
    /** Border style. Default `"solid"`. */
    border?: SurfaceBorder;
    /** Flush with ground plane, muted appearance. Springs up when cleared. */
    dormant?: boolean;
    /** Merge onto child element instead of wrapping in a div. */
    asChild?: boolean;
  }
>(({ className, elevation = "md", color, border, dormant, asChild, ...props }, ref) => (
  <Surface
    ref={ref}
    elevation={elevation}
    variant={color}
    dormant={dormant}
    border={border}
    asChild={asChild}
    rounded="xl"
    className={cn(color ? undefined : "bg-card text-card-foreground", className)}
    {...props}
  />
));
Card.displayName = "Card";

export const CardHeader = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={cn("flex flex-col space-y-1.5 p-6", className)} {...props} />
  ),
);
CardHeader.displayName = "CardHeader";

export const CardTitle = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn("leading-none font-semibold tracking-tight", className)}
      {...props}
    />
  ),
);
CardTitle.displayName = "CardTitle";

export const CardDescription = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={cn("text-muted-foreground text-sm", className)} {...props} />
  ),
);
CardDescription.displayName = "CardDescription";

export const CardContent = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={cn("p-6 pt-0", className)} {...props} />
  ),
);
CardContent.displayName = "CardContent";

export const CardFooter = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={cn("flex items-center p-6 pt-0", className)} {...props} />
  ),
);
CardFooter.displayName = "CardFooter";
