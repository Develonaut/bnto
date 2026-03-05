import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";

/**
 * Non-interactive category header inside a menu. Uppercase, muted,
 * small text. Used to group items into sections.
 */
export function MenuLabel({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "px-3 pt-3 pb-1 text-xs font-medium uppercase tracking-wider text-muted-foreground",
        className,
      )}
      {...props}
    />
  );
}
