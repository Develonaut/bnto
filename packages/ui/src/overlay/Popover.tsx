"use client";

import type { ComponentProps } from "react";

import * as PopoverPrimitive from "@radix-ui/react-popover";

import { PopupContent } from "./PopupContent";

export function Popover(props: ComponentProps<typeof PopoverPrimitive.Root>) {
  return <PopoverPrimitive.Root {...props} />;
}

export const PopoverTrigger = PopoverPrimitive.Trigger;

export const PopoverAnchor = PopoverPrimitive.Anchor;

export const PopoverPortal = PopoverPrimitive.Portal;

export const PopoverClose = PopoverPrimitive.Close;

export const PopoverContentUnstyled = PopoverPrimitive.Content;

/** Backwards-compatible alias — delegates to PopupContent (standardized offset + collision). */
export const PopoverContent = PopupContent;
