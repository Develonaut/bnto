"use client";

import type { ReactNode } from "react";
import {
  cn,
  Panel,
  PanelHeader,
  PanelDivider,
  PanelContent,
  PanelFooter,
  Text,
  CopyIcon,
  DownloadIcon,
  PenLineIcon,
  PlusIcon,
  TrashIcon,
} from "@bnto/ui";
import { useEditorExport } from "../../hooks/useEditorExport";
import { useEditorStore } from "../../hooks/useEditorStore";
import { useEditorPanels } from "../../hooks/useEditorPanels";
import { FileMenu, FileMenuItem, FileMenuSeparator } from "./FileMenu";
import { NodeList } from "./NodeList";
import { useAutoSelect } from "../../hooks/useAutoSelect";

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

function LayerPanelRoot({ footer }: LayerPanelProps) {
  const nodes = useEditorStore((s) => s.nodes);
  const selectedNodeId = useEditorStore((s) => s.selectedNodeId);
  const name = useEditorStore((s) => s.recipeMetadata.name ?? "Untitled");
  const { layersOpen } = useEditorPanels();
  const { download, canExport } = useEditorExport();
  const { handleSelectNode } = useAutoSelect({ selectedNodeId });

  const canDownload = canExport && nodes.length > 0;
  const handleDownload = () => download();

  /* CSS transition for panel open/close — Animate.* covers entrance animations,
     not boolean state transitions. See animation.md decision tree. */
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
        <PanelHeader className="gap-2 px-3 pt-3 pb-2">
          <Text size="sm" className="min-w-0 flex-1 truncate font-medium">
            {name}
          </Text>
          <FileMenu>
            <FileMenuItem onClick={handleDownload} disabled={!canDownload}>
              <DownloadIcon className="size-4" />
              Download
            </FileMenuItem>
            <FileMenuSeparator />
            <FileMenuItem disabled>
              <PlusIcon className="size-4" />
              New Recipe
            </FileMenuItem>
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
        </PanelHeader>
        <PanelDivider />
        <PanelContent>
          <NodeList nodes={nodes} selectedNodeId={selectedNodeId} onSelect={handleSelectNode} />
        </PanelContent>
        {footer && (
          <>
            <PanelDivider />
            <PanelFooter>{footer}</PanelFooter>
          </>
        )}
      </Panel>
    </div>
  );
}

export { LayerPanelRoot };
export type { LayerPanelProps };
