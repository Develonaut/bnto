"use client";

import { forwardRef } from "react";
import type { ComponentProps, ElementRef, ComponentPropsWithoutRef } from "react";

import * as PopoverPrimitive from "@radix-ui/react-popover";

import { Popup } from "./Popup";

export function Popover(props: ComponentProps<typeof PopoverPrimitive.Root>) {
  return <PopoverPrimitive.Root {...props} />;
}

export const PopoverTrigger = PopoverPrimitive.Trigger;

export const PopoverAnchor = PopoverPrimitive.Anchor;

export const PopoverPortal = PopoverPrimitive.Portal;

export const PopoverClose = PopoverPrimitive.Close;

export const PopoverContentUnstyled = PopoverPrimitive.Content;

export const PopoverContent = forwardRef<
  ElementRef<typeof PopoverPrimitive.Content>,
  ComponentPropsWithoutRef<typeof PopoverPrimitive.Content>
>(({ className, align = "center", side = "bottom", sideOffset = 4, children, ...props }, ref) => (
  <PopoverPrimitive.Portal>
    <PopoverPrimitive.Content
      ref={ref}
      align={align}
      side={side}
      sideOffset={sideOffset}
      className="z-dropdown outline-hidden"
      {...props}
    >
      <Popup side={side} className={className}>
        {children}
      </Popup>
    </PopoverPrimitive.Content>
  </PopoverPrimitive.Portal>
));
PopoverContent.displayName = PopoverPrimitive.Content.displayName;
