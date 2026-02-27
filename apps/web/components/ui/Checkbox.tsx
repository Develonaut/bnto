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
        "surface elevation-sm pressable peer size-6 shrink-0 rounded-[5px] outline-none translate-y-px",
        "focus-visible:outline-2 focus-visible:outline-ring/50 focus-visible:outline-offset-2",
        "disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      style={{ "--pressable-ease": "var(--ease-spring-bouncier)", "--pressable-dur": "400ms" } as React.CSSProperties}
      {...props}
    >
      <CheckboxPrimitive.Indicator
        data-slot="checkbox-indicator"
        className="grid place-content-center text-current transition-none"
      >
        <CheckIcon strokeWidth={3} className="size-4" />
      </CheckboxPrimitive.Indicator>
    </CheckboxPrimitive.Root>
  );
}

export { Checkbox };
