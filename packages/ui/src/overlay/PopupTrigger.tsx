"use client";

import { forwardRef } from "react";
import type { ComponentProps } from "react";

import * as PopoverPrimitive from "@radix-ui/react-popover";

import { cn } from "../utils/cn";
import { PopupTriggerButton } from "../interaction/PopupTriggerButton";

/**
 * Standard popup trigger composition: PopoverAnchor > PopoverTrigger > PopupTriggerButton.
 *
 * PopoverAnchor renders a static wrapper that Radix reads position from,
 * so the dropdown stays pinned while the Button's spring animation plays.
 */
export const PopupTrigger = forwardRef<
  HTMLButtonElement,
  ComponentProps<typeof PopupTriggerButton> & {
    /** Additional classes for the anchor wrapper. */
    anchorClassName?: string;
  }
>(({ anchorClassName, children, ...props }, ref) => (
  <PopoverPrimitive.Anchor className={cn("inline-flex", anchorClassName)}>
    <PopoverPrimitive.Trigger asChild>
      <PopupTriggerButton ref={ref} {...props}>
        {children}
      </PopupTriggerButton>
    </PopoverPrimitive.Trigger>
  </PopoverPrimitive.Anchor>
));
PopupTrigger.displayName = "PopupTrigger";
