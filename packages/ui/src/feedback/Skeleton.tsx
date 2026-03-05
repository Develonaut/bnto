import type { ComponentProps } from "react";

import { cn } from "../utils/cn";

/**
 * Skeleton — placeholder pulse for loading states.
 *
 * Renders a rounded rectangle with a shimmer animation.
 * Animation is gated behind `motion-safe:` so users with
 * `prefers-reduced-motion: reduce` see a static grey box.
 */
export function Skeleton({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "motion-safe:animate-pulse rounded-md bg-muted",
        className,
      )}
      {...props}
    />
  );
}
