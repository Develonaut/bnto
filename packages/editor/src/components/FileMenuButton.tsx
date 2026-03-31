"use client";

import { useCallback, useEffect, useRef, useState } from "react";
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
  CloudOffIcon,
  Heading,
  Text,
  Row,
  formatTimeAgo,
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
  statusText: string;
}

/** Track sync status with relative timestamp. */
function useSyncStatus(isDirty: boolean): string {
  const savedAtRef = useRef(Date.now());
  const [now, setNow] = useState(Date.now);

  useEffect(() => {
    if (!isDirty) savedAtRef.current = Date.now();
  }, [isDirty]);

  useEffect(() => {
    const id = setInterval(() => setNow(Date.now()), 1_000);
    return () => clearInterval(id);
  }, []);

  if (isDirty) return "Saving…";
  return `Saved ${formatTimeAgo(savedAtRef.current, now)}`;
}

/** File menu items (rename, new, export, import). */
function FileMenuItems({
  onRename,
  onImport,
  handleNew,
  download,
  canDownload,
  recipeName,
  statusText,
}: FileMenuItemsProps) {
  return (
    <MenuContent side="top" className="w-52 p-1">
      <div className="px-3 py-2">
        <Heading level={3} size="xs" className="truncate">
          {recipeName}
        </Heading>
        <Row className="mt-1 gap-1 items-center">
          <CloudOffIcon className="size-3 text-muted-foreground" />
          <Text size="xs" color="muted">
            {statusText}
          </Text>
        </Row>
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
  const { validationErrors, recipeMetadata, isDirty } = editor.definition.useDefinition();
  const { nodes } = editor.nodes.useNodes();
  const canDownload = validationErrors.length === 0 && nodes.length > 0;
  const handleNew = useCallback(() => editor.definition.createBlank(), [editor]);
  const download = useCallback(() => downloadDefinition(editor.definition), [editor]);
  const statusText = useSyncStatus(isDirty);

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
        statusText={statusText}
      />
    </Menu>
  );
}

export { FileMenuButton };
