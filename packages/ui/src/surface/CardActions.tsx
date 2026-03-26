import type { HTMLAttributes, MouseEvent, PointerEvent } from "react";

import { cn } from "../utils/cn";

function stopClickPropagation(e: MouseEvent) {
  e.stopPropagation();
  e.preventDefault();
}

function stopPropagation(e: MouseEvent | PointerEvent) {
  e.stopPropagation();
}

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
      onClick={stopClickPropagation}
      onPointerDown={stopPropagation}
      onMouseDown={stopPropagation}
      {...props}
    />
  );
}
