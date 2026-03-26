"use client";

import type { ComponentProps } from "react";
import * as LabelPrimitive from "@radix-ui/react-label";

import { cn } from "../../utils/cn";
import { useFormControlContext } from "./context";

/**
 * FormLabel — auto-wires `htmlFor` when inside a FormControl.
 *
 * Explicit `htmlFor` prop overrides the context value.
 */
function FormLabel({ htmlFor, className, ...props }: ComponentProps<typeof LabelPrimitive.Root>) {
  const ctx = useFormControlContext();
  const resolvedFor = htmlFor ?? ctx?.id;

  return (
    <LabelPrimitive.Root
      htmlFor={resolvedFor}
      className={cn(
        "text-muted-foreground text-xs leading-none font-medium select-none",
        className,
      )}
      {...props}
    />
  );
}

export { FormLabel };
