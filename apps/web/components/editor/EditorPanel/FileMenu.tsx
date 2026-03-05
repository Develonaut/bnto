import type { ReactNode } from "react";
import { ChevronUpIcon, Menu } from "@bnto/ui";

/**
 * FileMenu — composable file operations dropdown.
 *
 * The trigger renders a compact chevron button. Children are Menu.Item
 * elements composed by the consumer.
 *
 * Usage:
 *   <FileMenu>
 *     <Menu.Item onClick={onDownload}>Download</Menu.Item>
 *     <Menu.Separator />
 *     <Menu.Item disabled>Duplicate</Menu.Item>
 *   </FileMenu>
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
        className="size-7"
      >
        <ChevronUpIcon className="size-3.5" />
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
