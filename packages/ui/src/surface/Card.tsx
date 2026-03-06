"use client";

import { forwardRef } from "react";
import type { HTMLAttributes } from "react";

import { cn } from "../utils/cn";

import type { SpringMode } from "./Pressable";
import { Surface } from "./Surface";
import type { SurfaceElevation, SurfaceVariant } from "./Surface";

export const Card = forwardRef<
  HTMLDivElement,
  HTMLAttributes<HTMLDivElement> & {
    elevation?: SurfaceElevation;
    /** Color variant forwarded to Surface. Default uses card surface colors. */
    color?: SurfaceVariant;
    /** Spring animation mode. Explicit value overrides `loading` default. */
    spring?: SpringMode;
    /** Flush with ground plane. Explicit value overrides `loading` default. */
    grounded?: boolean;
    /** Render with a dashed border instead of a solid one. */
    dashed?: boolean;
    /** Sugar for spring="bounciest" + grounded={loading}. */
    loading?: boolean;
  }
>(({ className, elevation = "md", color, spring, grounded, dashed, loading, ...props }, ref) => (
  <Surface
    ref={ref}
    elevation={elevation}
    variant={color}
    spring={spring ?? (loading !== undefined ? "bounciest" : undefined)}
    grounded={grounded ?? loading}
    dashed={dashed}
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
