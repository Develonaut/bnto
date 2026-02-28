import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * Text input with Motorway surface treatment.
 *
 * Because `<input>` is a void element (no ::before/::after pseudo-elements),
 * the surface wrapper lives on a parent `<div>`. The inner `<input>` sits
 * transparently on top so the 3D wall + shadow renders behind it.
 */
function Input({
  className,
  wrapperClassName,
  type,
  ...props
}: ComponentProps<"input"> & { wrapperClassName?: string }) {
  return (
    <div
      className={cn(
        "surface surface-outline elevation-sm rounded-md",
        "has-[:focus-visible]:border-ring has-[:focus-visible]:ring-ring/50 has-[:focus-visible]:ring-[3px]",
        "has-[:disabled]:opacity-50",
        wrapperClassName,
      )}
    >
      <input
        type={type}
        data-slot="input"
        className={cn(
          "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground h-9 w-full min-w-0 rounded-md bg-transparent px-3 py-1 text-base outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed md:text-sm",
          className,
        )}
        {...props}
      />
    </div>
  );
}

export { Input };
