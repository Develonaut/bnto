"use client";

import type { ComponentProps, CSSProperties } from "react";

import * as CheckboxPrimitive from "@radix-ui/react-checkbox";
import { CheckIcon } from "../icons";

import { cn } from "../utils/cn";

function Checkbox({
  className,
  ...props
}: ComponentProps<typeof CheckboxPrimitive.Root>) {
  return (
    <CheckboxPrimitive.Root
      data-slot="checkbox"
      className={cn(
        "surface elevation-sm pressable peer size-6 shrink-0 rounded-[5px] outline-none translate-y-px",
        "focus-visible:outline-2 focus-visible:outline-ring/50 focus-visible:outline-offset-2",
        "disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      style={{ "--pressable-ease": "var(--ease-spring-bouncier)", "--pressable-dur": "400ms" } as CSSProperties}
      {...props}
    >
      <CheckboxPrimitive.Indicator
        data-slot="checkbox-indicator"
        className="grid place-content-center text-primary transition-none"
      >
        <CheckIcon strokeWidth={4} className="size-4" />
      </CheckboxPrimitive.Indicator>
    </CheckboxPrimitive.Root>
  );
}

export { Checkbox };
