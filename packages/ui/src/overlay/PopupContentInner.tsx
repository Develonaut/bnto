"use client";

import { forwardRef } from "react";
import type { ComponentPropsWithoutRef, ElementRef } from "react";
import * as PopoverPrimitive from "@radix-ui/react-popover";

import { Popup } from "./Popup";
import { POPUP_OFFSET_PX, type PopupOffset } from "./popupOffset";
import type { SurfaceElevation } from "../surface/Surface";

type PopupContentInnerProps = ComponentPropsWithoutRef<typeof PopoverPrimitive.Content> & {
  offset: PopupOffset;
  elevation: SurfaceElevation;
  boundary?: Element | null;
  boundaryPadding: number;
};

export const PopupContentInner = forwardRef<
  ElementRef<typeof PopoverPrimitive.Content>,
  PopupContentInnerProps
>(
  (
    { className, children, side, align, offset, elevation, boundary, boundaryPadding, ...props },
    ref,
  ) => (
    <PopoverPrimitive.Content
      ref={ref}
      side={side}
      align={align}
      sideOffset={POPUP_OFFSET_PX[offset]}
      collisionBoundary={boundary ?? undefined}
      collisionPadding={boundaryPadding}
      className="z-modal outline-hidden"
      {...props}
    >
      <Popup side={side} elevation={elevation} className={className}>
        {children}
      </Popup>
    </PopoverPrimitive.Content>
  ),
);
PopupContentInner.displayName = "PopupContentInner";
