"use client";

import { forwardRef } from "react";
import type { ComponentPropsWithoutRef, ElementRef } from "react";

import * as PopoverPrimitive from "@radix-ui/react-popover";

import { Popup } from "./Popup";
import { POPUP_OFFSET_PX, type PopupOffset } from "./popupOffset";
import type { SurfaceElevation } from "../surface/Surface";

export const PopupContent = forwardRef<
  ElementRef<typeof PopoverPrimitive.Content>,
  Omit<ComponentPropsWithoutRef<typeof PopoverPrimitive.Content>, "sideOffset"> & {
    /** Offset from trigger edge. Default "md" (16px). */
    offset?: PopupOffset;
    /** Card elevation. Default "lg". */
    elevation?: SurfaceElevation;
    /** Collision boundary element. */
    boundary?: Element | null;
    /** Padding from collision boundary. Default 16. */
    boundaryPadding?: number;
  }
>(
  (
    {
      className,
      children,
      side = "bottom",
      align = "center",
      offset = "md",
      elevation = "lg",
      boundary,
      boundaryPadding = 16,
      ...props
    },
    ref,
  ) => (
    <PopoverPrimitive.Portal>
      <PopoverPrimitive.Content
        ref={ref}
        side={side}
        align={align}
        sideOffset={POPUP_OFFSET_PX[offset]}
        collisionBoundary={boundary ?? undefined}
        collisionPadding={boundaryPadding}
        className="z-dropdown outline-hidden"
        {...props}
      >
        <Popup side={side} elevation={elevation} className={className}>
          {children}
        </Popup>
      </PopoverPrimitive.Content>
    </PopoverPrimitive.Portal>
  ),
);
PopupContent.displayName = "PopupContent";
