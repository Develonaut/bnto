import type { ComponentProps } from "react";

import { cn } from "../utils/cn";
import { InputWrapper } from "./InputWrapper";
import { useFormControlContext } from "./FormControl/context";

const INPUT_CN =
  "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground h-9 w-full min-w-0 rounded-md bg-transparent px-3 py-1 text-base outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed md:text-sm";

/**
 * Text input with Motorway surface treatment.
 *
 * When inside a `<FormControl>`, auto-receives `id` and `aria-describedby`
 * from context. Explicit props override context values.
 */
function Input({
  className,
  wrapperClassName,
  type,
  suffix,
  disabled,
  id,
  "aria-describedby": ariaDescribedBy,
  ...props
}: ComponentProps<"input"> & { wrapperClassName?: string; suffix?: string }) {
  const formCtx = useFormControlContext();

  return (
    <InputWrapper disabled={disabled} className={cn(suffix && "relative", wrapperClassName)}>
      <input
        type={type}
        id={id ?? formCtx?.id}
        aria-describedby={ariaDescribedBy ?? formCtx?.helpTextId}
        data-slot="input"
        className={cn(INPUT_CN, suffix && "pr-8", className)}
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
