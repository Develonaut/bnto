import type { ReactNode } from "react";
import { Menu, ChevronDownIcon } from "@bnto/ui";

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
      <Menu.Trigger
        size="icon"
        variant="ghost"
        elevation="sm"
        aria-label="File options"
        className="group"
      >
        <ChevronDownIcon className="size-4 transition-transform duration-fast group-data-[state=open]:rotate-180" />
      </Menu.Trigger>
      <Menu.Content side="bottom" offset="sm" className="w-52 p-1.5">
        {children}
      </Menu.Content>
    </Menu>
  );
}

export const FileMenu = Object.assign(FileMenuRoot, {
  Root: FileMenuRoot,
  Item: Menu.Item,
  Separator: Menu.Separator,
});
