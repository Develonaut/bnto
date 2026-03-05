"use client";

import type { ReactNode } from "react";
import { Panel } from "@/components/ui/Panel";
import { Text } from "@/components/ui/Text";
import {
  CopyIcon,
  DownloadIcon,
  PenLineIcon,
  PlusIcon,
  TrashIcon,
} from "@/components/ui/icons";
import { useEditorExport } from "@/editor/hooks/useEditorExport";
import { useEditorStore } from "@/editor/hooks/useEditorStore";
import { FileMenu } from "./FileMenu";
import { NodeList } from "./NodeList";
import { useAutoSelect } from "@/editor/hooks/useAutoSelect";

/**
 * EditorPanel — floating Panel with file header + node list.
 *
 * Header shows the recipe/bnto name with a file menu (Download,
 * Duplicate, Rename, Delete) and a collapse toggle. Below is a
 * scrollable list of canvas nodes.
 *
 * Reads nodes directly from the editor store (controlled mode).
 * Clicking a node updates selection via selectNode().
 */

interface EditorPanelProps {
  collapsed: boolean;
  onToggle: () => void;
  selectedNodeId: string | null;
  /** Recipe/bnto name shown in the header. */
  name?: string;
  /** Optional footer content (e.g., recipe selector). */
  footer?: ReactNode;
}

function EditorPanelRoot({
  collapsed,
  onToggle,
  selectedNodeId,
  name = "Untitled",
  footer,
}: EditorPanelProps) {
  const nodes = useEditorStore((s) => s.nodes);
  const { download, canExport } = useEditorExport();
  const { handleSelectNode } = useAutoSelect({ selectedNodeId });

  const canDownload = canExport && nodes.length > 0;
  const handleDownload = () => download();

  return (
    <Panel collapsed={collapsed} onToggle={onToggle} className="h-full w-56">
      <Panel.Header className="gap-2 px-3 pt-3 pb-2">
        <Panel.Trigger />
        <Text size="sm" className="min-w-0 flex-1 truncate font-medium">
          {name}
        </Text>
        <FileMenu>
          <FileMenu.Item onClick={handleDownload} disabled={!canDownload}>
            <DownloadIcon className="size-4" />
            Download .bnto.json
          </FileMenu.Item>
          <FileMenu.Separator />
          <FileMenu.Item disabled>
            <PlusIcon className="size-4" />
            New Recipe
          </FileMenu.Item>
          <FileMenu.Item disabled>
            <CopyIcon className="size-4" />
            Duplicate
          </FileMenu.Item>
          <FileMenu.Item disabled>
            <PenLineIcon className="size-4" />
            Rename
          </FileMenu.Item>
          <FileMenu.Separator />
          <FileMenu.Item disabled className="text-destructive">
            <TrashIcon className="size-4" />
            Delete
          </FileMenu.Item>
        </FileMenu>
      </Panel.Header>
      {!collapsed && <Panel.Divider />}
      <Panel.Content>
        <NodeList
          nodes={nodes}
          selectedNodeId={selectedNodeId}
          onSelect={handleSelectNode}
        />
      </Panel.Content>
      {footer && (
        <>
          <Panel.Divider />
          <Panel.Footer>{footer}</Panel.Footer>
        </>
      )}
    </Panel>
  );
}

export { EditorPanelRoot };
export type { EditorPanelProps };
