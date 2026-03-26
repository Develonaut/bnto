"use client";

import { useCallback } from "react";
import {
  Menu,
  MenuTrigger,
  MenuContent,
  MenuItem,
  FolderOpenIcon,
  PlusIcon,
  DownloadIcon,
  FileUpIcon,
  MenuSeparator,
  PenLineIcon,
  Text,
} from "@bnto/ui";
import { useEditor } from "../context";
import { downloadDefinition } from "../actions/downloadDefinition";
import { ShortcutHint } from "./ShortcutHint";

interface FileMenuButtonProps {
  onRename: () => void;
  onImport: () => void;
}

interface FileMenuItemsProps {
  onRename: () => void;
  onImport: () => void;
  handleNew: () => void;
  download: () => void;
  canDownload: boolean;
  recipeName: string;
}

/** File menu items (rename, new, export, import). */
function FileMenuItems({
  onRename,
  onImport,
  handleNew,
  download,
  canDownload,
  recipeName,
}: FileMenuItemsProps) {
  return (
    <MenuContent side="top" className="w-52 p-1">
      <div className="px-3 py-2">
        <Text weight="medium" size="sm" className="truncate">
          {recipeName}
        </Text>
      </div>
      <MenuSeparator />
      <MenuItem onClick={onRename} data-testid="menu-rename">
        <PenLineIcon /> Rename
      </MenuItem>
      <MenuItem onClick={handleNew} data-testid="menu-new">
        <PlusIcon /> New Recipe
      </MenuItem>
      <MenuSeparator />
      <MenuItem onClick={download} disabled={!canDownload} data-testid="menu-export">
        <DownloadIcon /> Export <ShortcutHint shortcutId="export" />
      </MenuItem>
      <MenuItem onClick={onImport} data-testid="menu-import">
        <FileUpIcon /> Import
      </MenuItem>
    </MenuContent>
  );
}

function FileMenuButton({ onRename, onImport }: FileMenuButtonProps) {
  const editor = useEditor();
  const { validationErrors, recipeMetadata } = editor.definition.useDefinition();
  const { nodes } = editor.nodes.useNodes();
  const canDownload = validationErrors.length === 0 && nodes.length > 0;
  const handleNew = useCallback(() => editor.definition.createBlank(), [editor]);
  const download = useCallback(() => downloadDefinition(editor.definition), [editor]);

  return (
    <Menu>
      <MenuTrigger
        icon={<FolderOpenIcon />}
        variant="ghost"
        elevation="sm"
        aria-label="File menu"
        data-testid="toolbar-file-menu"
      />
      <FileMenuItems
        onRename={onRename}
        onImport={onImport}
        handleNew={handleNew}
        download={download}
        canDownload={canDownload}
        recipeName={recipeMetadata.name}
      />
    </Menu>
  );
}

export { FileMenuButton };
