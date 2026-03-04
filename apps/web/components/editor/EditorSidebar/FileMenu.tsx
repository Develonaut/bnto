"use client";

import { Menu } from "@/components/ui/Menu";
import {
  ChevronUpIcon,
  CopyIcon,
  DownloadIcon,
  PenLineIcon,
  PlusIcon,
  TrashIcon,
} from "@/components/ui/icons";

/**
 * FileMenu — file operations dropdown in the sidebar header.
 *
 * Export, New Recipe, Duplicate, Rename, Delete.
 * Most items are disabled placeholders for future features.
 */

interface FileMenuProps {
  onExport: () => void;
  canExport: boolean;
}

function FileMenu({ onExport, canExport }: FileMenuProps) {
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
        <Menu.Item onClick={onExport} disabled={!canExport}>
          <DownloadIcon className="size-4" />
          Export .bnto.json
        </Menu.Item>
        <Menu.Separator />
        <Menu.Item disabled>
          <PlusIcon className="size-4" />
          New Recipe
        </Menu.Item>
        <Menu.Item disabled>
          <CopyIcon className="size-4" />
          Duplicate
        </Menu.Item>
        <Menu.Item disabled>
          <PenLineIcon className="size-4" />
          Rename
        </Menu.Item>
        <Menu.Separator />
        <Menu.Item disabled className="text-destructive">
          <TrashIcon className="size-4" />
          Delete
        </Menu.Item>
      </Menu.Content>
    </Menu>
  );
}

export { FileMenu };
