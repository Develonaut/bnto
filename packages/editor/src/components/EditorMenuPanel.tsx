"use client";

import type { ReactNode } from "react";
import { cn, Menu, MenuContent, MenuTrigger, XIcon } from "@bnto/ui";
import { usePanel } from "../hooks/usePanel";
import type { PanelId } from "../store/types";

/**
 * EditorMenuPanel — reusable menu-as-panel primitive.
 *
 * Wraps Menu + MenuTrigger + MenuContent with consistent sizing,
 * boundary padding, and store-controlled open state. Each panel
 * provides its icon, panel ID, side, width, and children.
 *
 * Panels are toggled via their toolbar buttons only. Opening a panel
 * closes same-side siblings via the store's PANEL_SIDES map.
 * When open, the trigger icon swaps to an X close icon.
 */

interface EditorMenuPanelProps {
  /** Panel ID for store-controlled open/close. */
  panelId: PanelId;
  /** Which side the menu content opens toward. */
  side: "left" | "right";
  /** Trigger icon element (shown when closed). */
  icon: ReactNode;
  /** Accessible label for the trigger button. */
  label: string;
  /** Width class for the content (e.g. "w-56", "w-72"). */
  width?: string;
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
  className,
  children,
}: EditorMenuPanelProps) {
  const { isOpen, toggle } = usePanel(panelId);

  return (
    <Menu
      open={isOpen}
      onOpenChange={(open) => {
        if (open !== isOpen) toggle();
      }}
    >
      <MenuTrigger
        size="icon"
        variant={isOpen ? "muted" : "ghost"}
        elevation="sm"
        aria-label={label}
      >
        {isOpen ? <XIcon className="size-4" /> : icon}
      </MenuTrigger>
      <MenuContent
        side={side}
        offset="lg"
        boundaryPadding={96}
        onPointerDownOutside={(e) => e.preventDefault()}
        onFocusOutside={(e) => e.preventDefault()}
        className={cn(width, "h-[calc(100vh-8rem)] flex flex-col p-0", className)}
      >
        {children}
      </MenuContent>
    </Menu>
  );
}

export { EditorMenuPanel };
export type { EditorMenuPanelProps };
