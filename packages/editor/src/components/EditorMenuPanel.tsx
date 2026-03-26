"use client";

import { type ReactNode, useCallback } from "react";
import { cn, Menu, MenuContent, MenuTrigger, XIcon } from "@bnto/ui";
import { usePanels } from "../hooks/usePanels";
import type { PanelId } from "../store/types";

/** Module-level event preventer — no hook needed. */
const preventEvent = (e: { preventDefault: () => void }) => e.preventDefault();

interface EditorMenuPanelProps {
  panelId: PanelId;
  side: "top" | "bottom" | "left" | "right";
  icon: ReactNode;
  label: string;
  width?: string;
  boundaryPadding?: number;
  dismissOnOutsideClick?: boolean;
  className?: string;
  children: ReactNode;
}

function EditorMenuPanel({
  panelId,
  side,
  icon,
  label,
  width = "w-72",
  boundaryPadding = 96,
  dismissOnOutsideClick = false,
  className,
  children,
}: EditorMenuPanelProps) {
  const { isOpen, toggle } = usePanels(panelId);

  const handleOpenChange = useCallback(
    (open: boolean) => {
      if (open !== isOpen) toggle();
    },
    [isOpen, toggle],
  );

  const outsideHandler = dismissOnOutsideClick ? undefined : preventEvent;

  return (
    <Menu open={isOpen} onOpenChange={handleOpenChange}>
      <MenuTrigger
        size="icon"
        variant={isOpen ? "muted" : "ghost"}
        elevation="sm"
        aria-label={label}
        data-testid={`toolbar-${panelId}`}
      >
        {isOpen ? <XIcon className="size-4" /> : icon}
      </MenuTrigger>
      <MenuContent
        side={side}
        offset="lg"
        boundaryPadding={boundaryPadding}
        onPointerDownOutside={outsideHandler}
        onFocusOutside={outsideHandler}
        className={cn(width, "min-w-[290px] h-[calc(100vh-8rem)] flex flex-col p-0", className)}
        data-testid={`panel-${panelId}`}
      >
        {children}
      </MenuContent>
    </Menu>
  );
}

export { EditorMenuPanel };
export type { EditorMenuPanelProps };
