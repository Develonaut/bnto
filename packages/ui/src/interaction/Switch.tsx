"use client";

import { forwardRef } from "react";
import type { ElementRef, ComponentPropsWithoutRef } from "react";

import * as SwitchPrimitives from "@radix-ui/react-switch";

import { cn } from "../utils/cn";
import { CheckIcon, XIcon } from "../icons";
import { Button } from "./Button";

const Switch = forwardRef<
  ElementRef<typeof SwitchPrimitives.Root>,
  ComponentPropsWithoutRef<typeof SwitchPrimitives.Root>
>(({ className, ...props }, ref) => (
  <SwitchPrimitives.Root
    className={cn(
      "peer inline-flex h-10 w-19 shrink-0 cursor-pointer items-center rounded-full bg-input border border-border focus-ring focus-visible:outline-offset-[-2px] disabled:cursor-not-allowed disabled:opacity-50",
      className,
    )}
    {...props}
    ref={ref}
  >
    <Button asChild variant="primary" elevation="sm" spring="bouncy" className="rounded-full">
      <SwitchPrimitives.Thumb
        className="group flex items-center justify-center size-8 data-[state=checked]:translate-x-[39px] data-[state=unchecked]:translate-x-[7px] translate-y-px data-[state=unchecked]:[--variant-bg:var(--card)] data-[state=unchecked]:[--variant-fg:var(--card-foreground)] [&_svg]:size-4 [&_svg]:shrink-0"
        style={{
          transition:
            "transform var(--pressable-dur, 150ms) var(--pressable-ease, cubic-bezier(0, 0, 0.58, 1)), translate 500ms var(--ease-spring-bouncier)",
        }}
      >
        <CheckIcon strokeWidth={4} className="hidden group-data-[state=checked]:block" />
        <XIcon strokeWidth={4} className="block group-data-[state=checked]:hidden" />
      </SwitchPrimitives.Thumb>
    </Button>
  </SwitchPrimitives.Root>
));
Switch.displayName = SwitchPrimitives.Root.displayName;

export { Switch };
