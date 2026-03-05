"use client";

import { forwardRef } from "react";
import type { HTMLAttributes } from "react";

import { cn } from "../../utils/cn";

import { usePanelContext } from "./context";

export const PanelContent = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, children, ...props }, ref) => {
    const { collapsed } = usePanelContext();

    if (collapsed) return null;

    return (
      <div ref={ref} className={cn("flex-1 overflow-y-auto", className)} {...props}>
        {children}
      </div>
    );
  },
);
PanelContent.displayName = "Panel.Content";
