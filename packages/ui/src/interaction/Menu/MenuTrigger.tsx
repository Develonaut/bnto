"use client";

import { forwardRef } from "react";
import type { ComponentProps } from "react";

import { Button } from "../Button";
import { Popover } from "../../overlay/Popover";

/* Renders as our Button component inside a Popover.Anchor wrapper.
 *
 * The Button uses .pressable which animates via CSS transform.
 * Without the Anchor, Radix uses the Trigger itself as the
 * Floating UI reference — so getBoundingClientRect() shifts as
 * the Button's spring animation plays, dragging the dropdown
 * with it.
 *
 * Popover.Anchor renders a static <div> that wraps the Trigger.
 * Radix detects the custom anchor and tells Floating UI to read
 * position from the Anchor instead of the Trigger. Since the
 * Anchor has no transform, its bounding rect never changes —
 * the dropdown stays pinned while the Button animates freely. */
export const MenuTrigger = forwardRef<HTMLButtonElement, ComponentProps<typeof Button>>(
  ({ children, ...props }, ref) => {
    return (
      <Popover.Anchor className="inline-flex">
        <Popover.Trigger asChild>
          <Button ref={ref} {...props}>
            {children}
          </Button>
        </Popover.Trigger>
      </Popover.Anchor>
    );
  },
);
MenuTrigger.displayName = "MenuTrigger";
