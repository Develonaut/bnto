"use client";

import type { ComponentProps } from "react";

import * as RadioGroupPrimitive from "@radix-ui/react-radio-group";
import { Button } from "./Button";

import { cn } from "../utils/cn";

export function RadioGroup({
  className,
  ...props
}: ComponentProps<typeof RadioGroupPrimitive.Root>) {
  return (
    <RadioGroupPrimitive.Root
      data-slot="radio-group"
      className={cn("grid gap-3", className)}
      {...props}
    />
  );
}

export function RadioGroupItem({
  className,
  disabled,
  ...props
}: ComponentProps<typeof RadioGroupPrimitive.Item>) {
  return (
    <RadioGroupPrimitive.Item asChild data-slot="radio-group-item" disabled={disabled} {...props}>
      <Button
        variant="outline"
        size="icon"
        disabled={disabled}
        spring="bounciest"
        className={cn(
          "peer size-6 shrink-0 rounded-full",
          "focus-visible:outline-2 focus-visible:outline-ring/50 focus-visible:outline-offset-2",
          className,
        )}
      >
        <RadioGroupPrimitive.Indicator
          data-slot="radio-group-indicator"
          className="grid place-content-center"
        >
          <span className="size-3 rounded-full bg-primary" />
        </RadioGroupPrimitive.Indicator>
      </Button>
    </RadioGroupPrimitive.Item>
  );
}
