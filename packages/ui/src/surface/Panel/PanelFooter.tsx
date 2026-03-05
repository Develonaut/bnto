"use client";

import { forwardRef } from "react";
import type { HTMLAttributes } from "react";

import { cn } from "../../utils/cn";

import { usePanelContext } from "./context";

export const PanelFooter = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, children, ...props }, ref) => {
    const { collapsed } = usePanelContext();

    if (collapsed) return null;

    return (
      <div ref={ref} className={cn("shrink-0 px-2 py-2", className)} {...props}>
        {children}
      </div>
    );
  },
);
PanelFooter.displayName = "Panel.Footer";
