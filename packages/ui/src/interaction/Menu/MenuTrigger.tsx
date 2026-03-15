"use client";

import { forwardRef } from "react";
import type { ComponentProps } from "react";

import { PopupTriggerButton } from "../PopupTriggerButton";
import { PopoverAnchor, PopoverTrigger } from "../../overlay/Popover";

/* Renders as a PopupTriggerButton inside a PopoverAnchor wrapper.
 *
 * PopoverAnchor renders a static <div> that wraps the Trigger.
 * Radix tells Floating UI to read position from the Anchor instead
 * of the animating Button — so the dropdown stays pinned while the
 * Button's spring animation plays. */
export const MenuTrigger = forwardRef<HTMLButtonElement, ComponentProps<typeof PopupTriggerButton>>(
  ({ children, ...props }, ref) => {
    return (
      <PopoverAnchor className="inline-flex">
        <PopoverTrigger asChild>
          <PopupTriggerButton ref={ref} {...props}>
            {children}
          </PopupTriggerButton>
        </PopoverTrigger>
      </PopoverAnchor>
    );
  },
);
MenuTrigger.displayName = "MenuTrigger";
