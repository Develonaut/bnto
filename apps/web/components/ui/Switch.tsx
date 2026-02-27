"use client";

import * as React from "react";

import * as SwitchPrimitives from "@radix-ui/react-switch";

import { cn } from "@/lib/cn";
import { CheckIcon, XIcon } from "@/components/ui/icons";

const Switch = React.forwardRef<
  React.ElementRef<typeof SwitchPrimitives.Root>,
  React.ComponentPropsWithoutRef<typeof SwitchPrimitives.Root>
>(({ className, ...props }, ref) => (
  <SwitchPrimitives.Root
    className={cn(
      "peer inline-flex h-10 w-19 shrink-0 cursor-pointer items-center rounded-full bg-input focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50",
      className,
    )}
    {...props}
    ref={ref}
  >
    <SwitchPrimitives.Thumb
      className="group surface surface-primary elevation-sm flex items-center justify-center size-8 rounded-full transition-[translate] duration-[500ms] ease-spring-bouncier data-[state=checked]:translate-x-9 data-[state=unchecked]:translate-x-1 data-[state=unchecked]:[--variant-bg:var(--card)] data-[state=unchecked]:[--variant-fg:var(--card-foreground)] [&_svg]:size-4 [&_svg]:shrink-0"
    >
      <CheckIcon strokeWidth={3} className="hidden group-data-[state=checked]:block" />
      <XIcon strokeWidth={3} className="block group-data-[state=checked]:hidden" />
    </SwitchPrimitives.Thumb>
  </SwitchPrimitives.Root>
));
Switch.displayName = SwitchPrimitives.Root.displayName;

export { Switch };
