"use client";

import { useId } from "react";
import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";
import { FormControlContext } from "./context";

/**
 * FormControl — layout wrapper that auto-wires `id` and `aria-describedby`
 * between a label, a control (Input/Select/Slider/Switch), and helper text.
 *
 * MUI-inspired composition: wrap any control with FormControl + FormLabel +
 * optional FormHelperText. The control auto-receives `id` and
 * `aria-describedby` via React context — no manual wiring needed.
 */
function FormControlRoot({ className, children, ...props }: ComponentProps<"div">) {
  const generatedId = useId();
  const id = `fc-${generatedId}`;
  const helpTextId = `${id}-help`;

  return (
    <FormControlContext value={{ id, helpTextId }}>
      <div className={cn("flex flex-col gap-1.5", className)} {...props}>
        {children}
      </div>
    </FormControlContext>
  );
}

export { FormControlRoot };
