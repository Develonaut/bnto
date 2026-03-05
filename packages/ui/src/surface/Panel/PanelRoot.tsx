"use client";

import { forwardRef } from "react";
import type { ComponentProps, ReactNode } from "react";

import { cn } from "../../utils/cn";
import { Card } from "../Card";

import { PanelContext } from "./context";

interface PanelRootProps extends Omit<ComponentProps<typeof Card>, "children"> {
  collapsed?: boolean;
  onToggle?: () => void;
  children: ReactNode;
}

/* CSS property transition (not an entrance animation) — Animate.* components
   handle keyframe entrances; property transitions on width/padding are the
   correct CSS-first approach per the animation decision tree. */
const PANEL_BASE =
  "flex h-full flex-col motion-safe:transition-[width,padding] motion-safe:duration-slow motion-safe:ease-spring-bouncy";

export const PanelRoot = forwardRef<HTMLDivElement, PanelRootProps>(
  ({ collapsed = false, onToggle, elevation = "md", className, children, ...props }, ref) => (
    <PanelContext.Provider value={{ collapsed, onToggle }}>
      <Card
        ref={ref}
        elevation={elevation}
        data-collapsed={collapsed}
        className={cn(
          PANEL_BASE,
          className,
          /* Collapsed overrides come LAST so twMerge picks w-[62px] over
             any width in className (e.g. w-56). */
          collapsed && "w-[62px] rounded-full px-3 py-0",
        )}
        {...props}
      >
        {children}
      </Card>
    </PanelContext.Provider>
  ),
);
PanelRoot.displayName = "Panel";
