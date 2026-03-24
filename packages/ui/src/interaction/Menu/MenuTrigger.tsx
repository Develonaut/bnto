"use client";

import { forwardRef } from "react";
import type { ComponentProps } from "react";

import { PopupTrigger } from "../../overlay/PopupTrigger";

export const MenuTrigger = forwardRef<HTMLButtonElement, ComponentProps<typeof PopupTrigger>>(
  ({ children, ...props }, ref) => (
    <PopupTrigger ref={ref} {...props}>
      {children}
    </PopupTrigger>
  ),
);
MenuTrigger.displayName = "MenuTrigger";
