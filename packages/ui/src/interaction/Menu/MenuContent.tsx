"use client";

import { forwardRef } from "react";
import type { ComponentPropsWithoutRef, ElementRef } from "react";

import { Popup } from "../../overlay/Popup";
import { PopoverContentUnstyled, PopoverPortal } from "../../overlay/Popover";

type MenuOffset = "sm" | "md" | "lg" | "xl";

const OFFSET_PX: Record<MenuOffset, number> = {
  sm: 8,
  md: 16,
  lg: 24,
  xl: 32,
};

export const MenuContent = forwardRef<
  ElementRef<typeof PopoverContentUnstyled>,
  Omit<ComponentPropsWithoutRef<typeof PopoverContentUnstyled>, "sideOffset"> & {
    elevation?: "none" | "sm" | "md" | "lg";
    offset?: MenuOffset;
    boundary?: Element | null;
    boundaryPadding?: number;
  }
>(
  (
    {
      className,
      children,
      side = "bottom",
      offset = "md",
      elevation = "lg",
      boundary,
      boundaryPadding = 16,
      ...props
    },
    ref,
  ) => (
    <PopoverPortal>
      <PopoverContentUnstyled
        ref={ref}
        side={side}
        sideOffset={OFFSET_PX[offset]}
        collisionBoundary={boundary ?? undefined}
        collisionPadding={boundaryPadding}
        className="z-50 outline-hidden"
        {...props}
      >
        <Popup side={side} elevation={elevation} className={className}>
          {children}
        </Popup>
      </PopoverContentUnstyled>
    </PopoverPortal>
  ),
);
MenuContent.displayName = "MenuContent";
