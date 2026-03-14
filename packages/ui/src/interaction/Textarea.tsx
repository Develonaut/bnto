import type { ComponentProps } from "react";

import { cn } from "../utils/cn";
import { InputWrapper } from "./InputWrapper";

type TextareaSize = "sm" | "md";

const SIZE_CLASSES: Record<TextareaSize, string> = {
  sm: "min-h-16 text-xs",
  md: "min-h-24 text-sm",
};

type TextareaProps = ComponentProps<"textarea"> & {
  size?: TextareaSize;
  wrapperClassName?: string;
};

function Textarea({ className, wrapperClassName, size = "md", disabled, ...props }: TextareaProps) {
  return (
    <InputWrapper disabled={disabled} className={wrapperClassName}>
      <textarea
        data-slot="textarea"
        className={cn(
          "placeholder:text-muted-foreground field-sizing-content w-full rounded-md bg-transparent px-3 py-2 outline-none disabled:cursor-not-allowed",
          SIZE_CLASSES[size],
          className,
        )}
        disabled={disabled}
        {...props}
      />
    </InputWrapper>
  );
}

export { Textarea };
export type { TextareaSize };
