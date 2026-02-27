"use client";

import * as React from "react";

import * as CheckboxPrimitive from "@radix-ui/react-checkbox";
import { CheckIcon } from "@/components/ui/icons";

import { cn } from "@/lib/cn";

function Checkbox({
  className,
  ...props
}: React.ComponentProps<typeof CheckboxPrimitive.Root>) {
  return (
    <CheckboxPrimitive.Root
      data-slot="checkbox"
      className={cn(
        "surface surface-outline elevation-md peer size-6 shrink-0 rounded-lg",
        "data-[state=checked]:[--variant-bg:var(--primary)] data-[state=checked]:[--variant-fg:var(--primary-foreground)] data-[state=checked]:[--surface-wall:var(--surface-primary-wall)] data-[state=checked]:[--surface-shadow:var(--surface-primary-shadow)]",
        "focus-visible:outline-2 focus-visible:outline-ring/50 focus-visible:outline-offset-2",
        "disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      {...props}
    >
      <CheckboxPrimitive.Indicator
        data-slot="checkbox-indicator"
        className="grid place-content-center text-current transition-none"
      >
        <CheckIcon className="size-4" />
      </CheckboxPrimitive.Indicator>
    </CheckboxPrimitive.Root>
  );
}

export { Checkbox };
