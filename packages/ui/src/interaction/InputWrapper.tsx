import type { ComponentProps } from "react";

import { cn } from "../utils/cn";
import { Surface } from "../surface/Surface";

type InputWrapperProps = ComponentProps<"div"> & {
  /** When true, renders with muted variant and no elevation. */
  disabled?: boolean;
};

/**
 * Shared surface wrapper for form field elements.
 *
 * Builds on `<Surface>` with outline variant and focus ring / disabled /
 * invalid states. Input, Textarea, and compound inputs (Combobox,
 * KeyValueEditor) inherit a consistent look and feel.
 */
function InputWrapper({ disabled, className, ...props }: InputWrapperProps) {
  return (
    <Surface
      variant={disabled ? "muted" : "outline"}
      elevation={disabled ? "none" : undefined}
      data-slot="input-wrapper"
      className={cn(
        "has-[:focus-visible]:border-ring has-[:focus-visible]:ring-ring/50 has-[:focus-visible]:ring-[3px]",
        "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
        className,
      )}
      {...props}
    />
  );
}

export { InputWrapper };
export type { InputWrapperProps };
