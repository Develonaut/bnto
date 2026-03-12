import type { ComponentProps } from "react";

import { cn } from "../utils/cn";

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
  suffix,
  ...props
}: ComponentProps<"input"> & { wrapperClassName?: string; suffix?: string }) {
  return (
    <div
      className={cn(
        "surface surface-outline elevation-sm rounded-md",
        "has-[:focus-visible]:border-ring has-[:focus-visible]:ring-ring/50 has-[:focus-visible]:ring-[3px]",
        "has-[:disabled]:opacity-50",
        suffix && "relative",
        wrapperClassName,
      )}
    >
      <input
        type={type}
        data-slot="input"
        className={cn(
          "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground h-9 w-full min-w-0 rounded-md bg-transparent px-3 py-1 text-base outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed md:text-sm",
          suffix && "pr-8",
          className,
        )}
        {...props}
      />
      {suffix && (
        <span className="text-muted-foreground pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-xs">
          {suffix}
        </span>
      )}
    </div>
  );
}

export { Input };
