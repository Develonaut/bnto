"use client";

import type { ReactNode } from "react";
import { Menu, MenuTrigger, MenuContent, MenuItem, MenuSeparator, ChevronDownIcon } from "@bnto/ui";

/**
 * FileMenu — composable file operations dropdown.
 *
 * Chevron points down when closed, rotates to point up when open.
 * Uses Radix data-state on the trigger + group selector for the icon.
 * Matches toolbar button size/elevation.
 */

interface FileMenuProps {
  children: ReactNode;
}

function FileMenuRoot({ children }: FileMenuProps) {
  return (
    <Menu>
      <MenuTrigger
        size="icon"
        variant="ghost"
        elevation="sm"
        aria-label="File options"
        className="group"
      >
        <ChevronDownIcon className="size-4 transition-transform duration-fast group-data-[state=open]:rotate-180" />
      </MenuTrigger>
      <MenuContent side="bottom" offset="sm" className="w-52 p-1.5">
        {children}
      </MenuContent>
    </Menu>
  );
}

export { FileMenuRoot as FileMenu };
export { MenuItem as FileMenuItem };
export { MenuSeparator as FileMenuSeparator };
