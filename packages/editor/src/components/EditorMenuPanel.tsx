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

/** Sync Radix open state with store panel state. */
function useMenuOpenChange(panelId: PanelId) {
  const { isOpen, toggle } = usePanels(panelId);
  const handleOpenChange = useCallback(
    (open: boolean) => {
      if (open !== isOpen) toggle();
    },
    [isOpen, toggle],
  );
  return { isOpen, handleOpenChange };
}

/** Panel trigger — shows X when open, icon when closed. */
function PanelTrigger({
  isOpen,
  icon,
  label,
  panelId,
}: {
  isOpen: boolean;
  icon: ReactNode;
  label: string;
  panelId: PanelId;
}) {
  return (
    <MenuTrigger
      size="icon"
      variant={isOpen ? "muted" : "ghost"}
      elevation="sm"
      aria-label={label}
      data-testid={`toolbar-${panelId}`}
    >
      {isOpen ? <XIcon className="size-4" /> : icon}
    </MenuTrigger>
  );
}

function EditorMenuPanel(props: EditorMenuPanelProps) {
  const { panelId, side, icon, label, className, children } = props;
  const { width = "w-72", boundaryPadding = 96, dismissOnOutsideClick = false } = props;
  const { isOpen, handleOpenChange } = useMenuOpenChange(panelId);
  const outsideHandler = dismissOnOutsideClick ? undefined : preventEvent;

  return (
    <Menu open={isOpen} onOpenChange={handleOpenChange}>
      <PanelTrigger isOpen={isOpen} icon={icon} label={label} panelId={panelId} />
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
