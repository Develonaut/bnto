"use client";

import type { ReactNode } from "react";
import {
  cn,
  Panel,
  Text,
  CopyIcon,
  DownloadIcon,
  PenLineIcon,
  PlusIcon,
  TrashIcon,
} from "@bnto/ui";
import { useEditorExport } from "@/editor/hooks/useEditorExport";
import { useEditorStore } from "@/editor/hooks/useEditorStore";
import { useEditorPanels } from "@/editor/hooks/useEditorPanels";
import { FileMenu } from "./FileMenu";
import { NodeList } from "./NodeList";
import { useAutoSelect } from "@/editor/hooks/useAutoSelect";

/**
 * LayerPanel — self-contained left-side panel.
 *
 * Reads selection from the editor store. Slides in from the left
 * with spring-bouncy easing, mirrors the ConfigPanel's right-side
 * slide-in pattern.
 */

interface LayerPanelProps {
  /** Optional footer content (e.g., recipe selector). */
  footer?: ReactNode;
}

function LayerPanelRoot({
  footer,
}: LayerPanelProps) {
  const nodes = useEditorStore((s) => s.nodes);
  const selectedNodeId = useEditorStore((s) => s.selectedNodeId);
  const name = useEditorStore((s) => s.recipeMetadata.name ?? "Untitled");
  const { layersOpen } = useEditorPanels();
  const { download, canExport } = useEditorExport();
  const { handleSelectNode } = useAutoSelect({ selectedNodeId });

  const canDownload = canExport && nodes.length > 0;
  const handleDownload = () => download();

  return (
    <div
      onPointerDownCapture={(e) => e.stopPropagation()}
      className={cn(
        "pointer-events-auto absolute bottom-0 left-0 top-0 w-56",
        "motion-safe:transition-[translate,opacity]",
        layersOpen
          ? "translate-x-0 opacity-100 motion-safe:duration-slow motion-safe:ease-spring-bouncy"
          : "pointer-events-none -translate-x-[110%] opacity-0 motion-safe:duration-fast motion-safe:ease-out",
      )}
    >
      <Panel className="h-full w-full">
        <Panel.Header className="gap-2 px-3 pt-3 pb-2">
          <Text size="sm" className="min-w-0 flex-1 truncate font-medium">
            {name}
          </Text>
          <FileMenu>
            <FileMenu.Item onClick={handleDownload} disabled={!canDownload}>
              <DownloadIcon className="size-4" />
              Download
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
        <Panel.Divider />
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
    </div>
  );
}

export { LayerPanelRoot };
export type { LayerPanelProps };
