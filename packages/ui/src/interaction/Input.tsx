import type { ComponentProps } from "react";

import { cn } from "../utils/cn";
import { InputWrapper } from "./InputWrapper";

/**
 * Text input with flat muted background.
 *
 * Uses InputWrapper for the background/focus/disabled states.
 */
function Input({
  className,
  wrapperClassName,
  type,
  suffix,
  disabled,
  ...props
}: ComponentProps<"input"> & { wrapperClassName?: string; suffix?: string }) {
  return (
    <InputWrapper disabled={disabled} className={cn(suffix && "relative", wrapperClassName)}>
      <input
        type={type}
        data-slot="input"
        className={cn(
          "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground h-9 w-full min-w-0 rounded-md bg-transparent px-3 py-1 text-base outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed md:text-sm",
          suffix && "pr-8",
          className,
        )}
        disabled={disabled}
        {...props}
      />
      {suffix && (
        <span className="text-muted-foreground pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-xs">
          {suffix}
        </span>
      )}
    </InputWrapper>
  );
}

export { Input };
