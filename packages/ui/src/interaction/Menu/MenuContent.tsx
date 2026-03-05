"use client";

import { forwardRef } from "react";
import type { ComponentPropsWithoutRef, ElementRef } from "react";

import { ScaleIn } from "../../animation/Animate";
import { Card } from "../../surface/Card";
import { PopoverContentUnstyled, PopoverPortal } from "../../overlay/Popover";

type MenuOffset = "sm" | "md" | "lg" | "xl";

const OFFSET_PX: Record<MenuOffset, number> = {
  sm: 8,
  md: 16,
  lg: 24,
  xl: 32,
};

/** Map Radix side -> ScaleIn origin so the menu scales from the edge nearest the trigger. */
const SIDE_ORIGIN = {
  top: "bottom",
  bottom: "top",
  left: "right",
  right: "left",
} as const;

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
        <ScaleIn from={0.6} origin={SIDE_ORIGIN[side]} easing="spring-bouncier">
          <Card className={className} elevation={elevation}>
            {children}
          </Card>
        </ScaleIn>
      </PopoverContentUnstyled>
    </PopoverPortal>
  ),
);
MenuContent.displayName = "MenuContent";
