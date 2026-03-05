"use client";

import { Children, forwardRef, isValidElement } from "react";
import type { HTMLAttributes } from "react";

import { cn } from "../../utils/cn";

import { usePanelContext } from "./context";
import { PanelTrigger } from "./PanelTrigger";

export const PanelHeader = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, children, ...props }, ref) => {
    const { collapsed } = usePanelContext();

    /* When collapsed, only render PanelTrigger children — everything
       else (title, count, icons) is hidden so the pill shows just the button.
       Note: Panel.Trigger must be a direct child (not wrapped in Fragment
       or another component) for the type check to work. */
    const visibleChildren = collapsed
      ? Children.toArray(children).filter(
          (child) => isValidElement(child) && child.type === PanelTrigger,
        )
      : children;

    return (
      <div
        ref={ref}
        className={cn(
          "flex shrink-0 items-center",
          className,
          /* Collapsed overrides come LAST — strip horizontal padding/border.
             Header keeps its own vertical padding for consistent trigger position.
             overflow-hidden only when expanded (clips during width transition).
             Collapsed needs visible overflow so button shadow isn't clipped. */
          collapsed ? "justify-center overflow-visible border-0 px-0" : "overflow-hidden",
        )}
        {...props}
      >
        {visibleChildren}
      </div>
    );
  },
);
PanelHeader.displayName = "Panel.Header";
