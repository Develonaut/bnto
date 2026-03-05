"use client";

import { forwardRef } from "react";
import type { HTMLAttributes } from "react";

import { cn } from "../../utils/cn";

export const PanelDivider = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      role="separator"
      aria-orientation="horizontal"
      className={cn("my-1 h-px w-full shrink-0 bg-border", className)}
      {...props}
    />
  ),
);
PanelDivider.displayName = "Panel.Divider";
