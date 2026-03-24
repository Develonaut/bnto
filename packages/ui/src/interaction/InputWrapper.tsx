import type { ComponentProps, CSSProperties } from "react";

import { cn } from "../utils/cn";
import { Surface } from "../surface/Surface";

type InputWrapperProps = ComponentProps<"div"> & {
  /** When true, reduces opacity for a disabled appearance. */
  disabled?: boolean;
};

/** Override the muted surface text color so typed input uses the primary foreground. */
const inputFgOverride = { "--variant-fg": "var(--foreground)" } as CSSProperties;

/**
 * Shared surface wrapper for form field elements.
 *
 * Builds on `<Surface>` with muted variant, no elevation, and focus ring /
 * disabled / invalid states. Input, Textarea, and compound inputs (Combobox,
 * KeyValueEditor) inherit a consistent look and feel.
 *
 * Text color is overridden to `--foreground` so typed content is dark;
 * placeholders stay muted via their own `placeholder:text-muted-foreground`.
 */
function InputWrapper({ disabled, className, style, ...props }: InputWrapperProps) {
  return (
    <Surface
      variant="muted"
      elevation="none"
      data-slot="input-wrapper"
      style={{ ...inputFgOverride, ...style }}
      className={cn(
        "has-[:focus-visible]:border-ring has-[:focus-visible]:ring-ring/50 has-[:focus-visible]:ring-[3px]",
        "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
        disabled && "opacity-50",
        className,
      )}
      {...props}
    />
  );
}

export { InputWrapper };
export type { InputWrapperProps };
