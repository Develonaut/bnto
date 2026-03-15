"use client";

import { forwardRef } from "react";
import type { ComponentProps } from "react";

import { Button } from "./Button";

/**
 * Shared trigger button for popup components (Menu, Select, Combobox).
 *
 * Encodes the standard popup trigger pattern:
 * - `variant="outline"` — form-control styling
 * - `open` → `hovered` — sinks the button while the popup is visible
 *
 * Radix primitives also set `data-state="open"` on the trigger, which
 * the `.pressable` CSS already responds to. The explicit `hovered` prop
 * ensures the visual is part of the component contract, not a CSS side effect.
 */
export const PopupTriggerButton = forwardRef<
  HTMLButtonElement,
  ComponentProps<typeof Button> & {
    /** When true, renders the button in its "open" visual state (sunk toward ground). */
    open?: boolean;
  }
>(({ open, hovered, ...props }, ref) => {
  return <Button ref={ref} variant="outline" hovered={open || hovered} {...props} />;
});
PopupTriggerButton.displayName = "PopupTriggerButton";
