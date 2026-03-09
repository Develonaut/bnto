"use client";

import { useState } from "react";
import {
  Divider,
  PanelLeftIcon,
  Text,
  CopyIcon,
  DownloadIcon,
  FolderOpenIcon,
  PenLineIcon,
  TrashIcon,
} from "@bnto/ui";
import { useEditorExport } from "../../hooks/useEditorExport";
import { useEditorStore } from "../../hooks/useEditorStore";
import { FileMenu, FileMenuItem, FileMenuSeparator } from "./FileMenu";
import { NodeList } from "./NodeList";
import { useAutoSelect } from "../../hooks/useAutoSelect";
import { EditorMenuPanel } from "../EditorMenuPanel";
import { OpenRecipeDialog } from "../OpenRecipeDialog";

/**
 * LayerPanel — Menu-based layer panel.
 *
 * Opens to the right from the left toolbar trigger. Shows recipe name,
 * file menu, and node list.
 */

function LayerPanelRoot() {
  const nodes = useEditorStore((s) => s.nodes);
  const selectedNodeId = useEditorStore((s) => s.selectedNodeId);
  const name = useEditorStore((s) => s.recipeMetadata.name ?? "Untitled");
  const { download, canExport } = useEditorExport();
  const { handleSelectNode } = useAutoSelect({ selectedNodeId });
  const [openDialogOpen, setOpenDialogOpen] = useState(false);

  const canDownload = canExport && nodes.length > 0;
  const handleDownload = () => download();

  return (
    <>
      <EditorMenuPanel
      panelId="layers"
      side="right"
      width="w-56"
      label="Layers"
      icon={<PanelLeftIcon className="size-4" />}
    >
      <div className="flex shrink-0 items-center gap-2 px-3 pt-3 pb-2">
        <Text size="sm" className="min-w-0 flex-1 truncate font-medium">
          {name}
        </Text>
        <FileMenu>
          <FileMenuItem onClick={() => setOpenDialogOpen(true)}>
            <FolderOpenIcon className="size-4" />
            Open
          </FileMenuItem>
          <FileMenuItem onClick={handleDownload} disabled={!canDownload}>
            <DownloadIcon className="size-4" />
            Download
          </FileMenuItem>
          <FileMenuSeparator />
          <FileMenuItem disabled>
            <CopyIcon className="size-4" />
            Duplicate
          </FileMenuItem>
          <FileMenuItem disabled>
            <PenLineIcon className="size-4" />
            Rename
          </FileMenuItem>
          <FileMenuSeparator />
          <FileMenuItem disabled className="text-destructive">
            <TrashIcon className="size-4" />
            Delete
          </FileMenuItem>
        </FileMenu>
      </div>
      <Divider />
      <div className="flex-1 overflow-y-auto">
        <NodeList nodes={nodes} selectedNodeId={selectedNodeId} onSelect={handleSelectNode} />
      </div>
    </EditorMenuPanel>
    <OpenRecipeDialog open={openDialogOpen} onOpenChange={setOpenDialogOpen} />
  </>
  );
}

export { LayerPanelRoot };
