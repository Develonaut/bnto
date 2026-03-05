"use client";

import { forwardRef } from "react";
import type { ComponentProps } from "react";

import { Button } from "../../interaction/Button";
import { PanelLeftIcon, PanelLeftCloseIcon } from "../../icons";

import { usePanelContext } from "./context";

export const PanelTrigger = forwardRef<
  HTMLButtonElement,
  Omit<ComponentProps<typeof Button>, "onClick" | "children">
>(({ className, ...props }, ref) => {
  const { collapsed, onToggle } = usePanelContext();

  return (
    <Button
      ref={ref}
      size="icon"
      variant="ghost"
      elevation="sm"
      onClick={onToggle}
      aria-label={collapsed ? "Expand panel" : "Collapse panel"}
      className={className}
      {...props}
    >
      {collapsed ? <PanelLeftIcon className="size-4" /> : <PanelLeftCloseIcon className="size-4" />}
    </Button>
  );
});
PanelTrigger.displayName = "Panel.Trigger";
