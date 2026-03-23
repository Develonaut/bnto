import type { ComponentProps } from "react";

import { cn } from "../utils/cn";
import { Surface } from "../surface/Surface";

type InputWrapperProps = ComponentProps<"div"> & {
  /** When true, renders with reduced opacity. */
  disabled?: boolean;
};

/**
 * Shared wrapper for form field elements.
 *
 * Muted surface with no elevation — flat background with subtle border.
 * Focus ring and disabled/invalid states built in.
 */
function InputWrapper({ disabled, className, ...props }: InputWrapperProps) {
  return (
    <Surface
      variant="muted"
      elevation="none"
      data-slot="input-wrapper"
      className={cn(
        "has-[:focus-visible]:ring-ring/50 has-[:focus-visible]:ring-[3px]",
        "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
        disabled && "opacity-50 pointer-events-none",
        className,
      )}
      {...props}
    />
  );
}

export { InputWrapper };
export type { InputWrapperProps };
