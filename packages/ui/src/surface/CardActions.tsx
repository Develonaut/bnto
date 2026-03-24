import type { HTMLAttributes } from "react";

import { cn } from "../utils/cn";

/**
 * Blocks click/pointer propagation from nested controls inside clickable cards.
 *
 * Wrap interactive elements (menus, buttons) that sit inside a card-as-link
 * so clicks don't bubble up and trigger card navigation.
 */
export function CardActions({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      data-card-actions=""
      className={cn("relative z-10", className)}
      onClick={(e) => {
        e.stopPropagation();
        e.preventDefault();
      }}
      onPointerDown={(e) => e.stopPropagation()}
      onMouseDown={(e) => e.stopPropagation()}
      {...props}
    />
  );
}
