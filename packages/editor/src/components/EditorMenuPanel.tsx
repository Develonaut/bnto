"use client";

import { type ReactNode, useCallback } from "react";
import { cn, Menu, MenuContent, MenuTrigger, XIcon } from "@bnto/ui";
import { usePanels } from "../hooks/usePanels";
import type { PanelId } from "../store/types";

/** Module-level event preventer -- no hook needed. */
const preventEvent = (e: { preventDefault: () => void }) => e.preventDefault();

/**
 * EditorMenuPanel — reusable menu-as-panel primitive.
 *
 * Wraps Menu + MenuTrigger + MenuContent with consistent sizing,
 * boundary padding, and store-controlled open state. Each panel
 * provides its icon, panel ID, side, width, and children.
 *
 * Panels close via toolbar toggle or store-driven state changes
 * (e.g. node deselection closes config, pane click closes panels).
 * Radix outside-click dismissal is suppressed — the store owns
 * panel lifecycle so clicking between nodes doesn't re-animate.
 * Opening a panel closes same-side siblings via PANEL_SIDES map.
 * When open, the trigger icon swaps to an X close icon.
 */

interface EditorMenuPanelProps {
  /** Panel ID for store-controlled open/close. */
  panelId: PanelId;
  /** Which side the menu content opens toward. */
  side: "top" | "bottom" | "left" | "right";
  /** Trigger icon element (shown when closed). */
  icon: ReactNode;
  /** Accessible label for the trigger button. */
  label: string;
  /** Width class for the content (e.g. "w-56", "w-72"). */
  width?: string;
  /** Pixel distance from viewport edge before content repositions. */
  boundaryPadding?: number;
  /** Allow closing when clicking outside the panel. Default false. */
  dismissOnOutsideClick?: boolean;
  /** Extra classes on MenuContent. */
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
        onPointerDownOutside={dismissOnOutsideClick ? undefined : preventEvent}
        onFocusOutside={dismissOnOutsideClick ? undefined : preventEvent}
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
