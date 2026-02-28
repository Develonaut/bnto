"use client";

import { forwardRef } from "react";
import type { ElementRef, ComponentPropsWithoutRef } from "react";

import * as SwitchPrimitives from "@radix-ui/react-switch";

import { cn } from "@/lib/cn";
import { CheckIcon, XIcon } from "@/components/ui/icons";

const Switch = forwardRef<
  ElementRef<typeof SwitchPrimitives.Root>,
  ComponentPropsWithoutRef<typeof SwitchPrimitives.Root>
>(({ className, ...props }, ref) => (
  <SwitchPrimitives.Root
    className={cn(
      "peer inline-flex h-10 w-19 shrink-0 cursor-pointer items-center rounded-full bg-input border border-border focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50",
      className,
    )}
    {...props}
    ref={ref}
  >
    <SwitchPrimitives.Thumb
      className="group surface surface-primary elevation-sm pressable outline-none flex items-center justify-center size-8 rounded-full data-[state=checked]:translate-x-[39px] data-[state=unchecked]:translate-x-[7px] translate-y-px data-[state=unchecked]:[--variant-bg:var(--card)] data-[state=unchecked]:[--variant-fg:var(--card-foreground)] [&_svg]:size-4 [&_svg]:shrink-0"
      style={{ transition: "transform var(--pressable-dur, 150ms) var(--pressable-ease, cubic-bezier(0, 0, 0.58, 1)), translate 500ms var(--ease-spring-bouncier)" }}
    >
      <CheckIcon strokeWidth={4} className="hidden group-data-[state=checked]:block" />
      <XIcon strokeWidth={4} className="block group-data-[state=checked]:hidden" />
    </SwitchPrimitives.Thumb>
  </SwitchPrimitives.Root>
));
Switch.displayName = SwitchPrimitives.Root.displayName;

export { Switch };
