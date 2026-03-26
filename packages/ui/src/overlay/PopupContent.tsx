"use client";

import { forwardRef } from "react";
import type { ComponentPropsWithoutRef, ElementRef } from "react";
import * as PopoverPrimitive from "@radix-ui/react-popover";

import type { PopupOffset } from "./popupOffset";
import type { SurfaceElevation } from "../surface/Surface";
import { PopupContentInner } from "./PopupContentInner";

type PopupContentProps = Omit<
  ComponentPropsWithoutRef<typeof PopoverPrimitive.Content>,
  "sideOffset"
> & {
  /** Offset from trigger edge. Default "md" (16px). */
  offset?: PopupOffset;
  /** Card elevation. Default "lg". */
  elevation?: SurfaceElevation;
  /** Collision boundary element. */
  boundary?: Element | null;
  /** Padding from collision boundary. Default 16. */
  boundaryPadding?: number;
};

export const PopupContent = forwardRef<
  ElementRef<typeof PopoverPrimitive.Content>,
  PopupContentProps
>(
  (
    {
      side = "bottom",
      align = "center",
      offset = "md",
      elevation = "lg",
      boundaryPadding = 16,
      ...props
    },
    ref,
  ) => (
    <PopoverPrimitive.Portal>
      <PopupContentInner
        ref={ref}
        side={side}
        align={align}
        offset={offset}
        elevation={elevation}
        boundaryPadding={boundaryPadding}
        {...props}
      />
    </PopoverPrimitive.Portal>
  ),
);
PopupContent.displayName = "PopupContent";
