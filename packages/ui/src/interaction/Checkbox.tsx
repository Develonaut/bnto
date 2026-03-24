"use client";

import type { ComponentProps } from "react";

import * as CheckboxPrimitive from "@radix-ui/react-checkbox";
import { CheckIcon } from "../icons";
import { Button } from "./Button";

import { cn } from "../utils/cn";

function Checkbox({
  className,
  disabled,
  ...props
}: ComponentProps<typeof CheckboxPrimitive.Root>) {
  return (
    <CheckboxPrimitive.Root asChild data-slot="checkbox" disabled={disabled} {...props}>
      <Button
        variant="outline"
        size="icon"
        disabled={disabled}
        spring="bounciest"
        className={cn("peer size-6 shrink-0 rounded-[5px] translate-y-px", "focus-ring", className)}
      >
        <CheckboxPrimitive.Indicator
          data-slot="checkbox-indicator"
          className="grid place-content-center text-primary transition-none"
        >
          <CheckIcon strokeWidth={4} className="size-4" />
        </CheckboxPrimitive.Indicator>
      </Button>
    </CheckboxPrimitive.Root>
  );
}

export { Checkbox };
