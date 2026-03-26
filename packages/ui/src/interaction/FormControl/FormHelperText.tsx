"use client";

import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";
import { useFormControlContext } from "./context";

/**
 * FormHelperText — auto-wires `id` for `aria-describedby` when
 * inside a FormControl.
 *
 * Explicit `id` prop overrides the context value.
 */
function FormHelperText({ id, className, ...props }: ComponentProps<"p">) {
  const ctx = useFormControlContext();
  const resolvedId = id ?? ctx?.helpTextId;

  return (
    <p id={resolvedId} className={cn("text-muted-foreground text-xs", className)} {...props} />
  );
}

export { FormHelperText };
