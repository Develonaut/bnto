"use client";

import { forwardRef } from "react";
import type { ComponentProps, ElementRef } from "react";

import { PopupContent } from "../../overlay/PopupContent";
import { cn } from "../../utils/cn";

export const MenuContent = forwardRef<
  ElementRef<typeof PopupContent>,
  ComponentProps<typeof PopupContent>
>(({ className, children, ...props }, ref) => (
  <PopupContent ref={ref} className={cn("p-2", className)} {...props}>
    {children}
  </PopupContent>
));
MenuContent.displayName = "MenuContent";
