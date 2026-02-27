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
      "peer inline-flex h-9 w-16 shrink-0 cursor-pointer items-center rounded-full transition-colors data-[state=unchecked]:bg-input data-[state=checked]:bg-primary focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50",
      className,
    )}
    {...props}
    ref={ref}
  >
    <SwitchPrimitives.Thumb
      className="group surface surface-outline elevation-md flex items-center justify-center size-7 rounded-full transition-[translate] duration-[500ms] ease-spring-bouncier data-[state=checked]:translate-x-8 data-[state=unchecked]:translate-x-1 [&_svg]:size-4 [&_svg]:shrink-0"
    >
      <CheckIcon className="hidden group-data-[state=checked]:block" />
      <XIcon className="block group-data-[state=checked]:hidden" />
    </SwitchPrimitives.Thumb>
  </SwitchPrimitives.Root>
));
Switch.displayName = SwitchPrimitives.Root.displayName;

export { Switch };
