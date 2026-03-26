"use client";

import type { ReactNode } from "react";
import * as SelectPrimitive from "@radix-ui/react-select";

import { cn } from "../../utils/cn";
import { Popup } from "../../overlay/Popup";
import type { SurfaceElevation } from "../../surface/Surface";
import { SelectScrollUpButton, SelectScrollDownButton } from "./SelectScrollButtons";

interface SelectContentInnerProps {
  children: ReactNode;
  position: string;
  elevation: SurfaceElevation;
  className?: string;
}

export function SelectContentInner({
  children,
  position,
  elevation,
  className,
}: SelectContentInnerProps) {
  return (
    <Popup
      originStyle="var(--radix-select-content-transform-origin)"
      elevation={elevation}
      className={className}
    >
      <div className="overflow-x-hidden overflow-y-auto">
        <SelectScrollUpButton />
        <SelectPrimitive.Viewport
          className={cn(
            "p-1",
            position === "popper" && "w-full min-w-[var(--radix-select-trigger-width)] scroll-my-1",
          )}
        >
          {children}
        </SelectPrimitive.Viewport>
        <SelectScrollDownButton />
      </div>
    </Popup>
  );
}
