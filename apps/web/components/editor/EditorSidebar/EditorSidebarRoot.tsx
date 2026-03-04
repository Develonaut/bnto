"use client";

import { Panel } from "@/components/ui/Panel";
import { Text } from "@/components/ui/Text";
import { useEditorExport } from "@/editor/hooks/useEditorExport";
import { useCanvasNodes } from "@/editor/hooks/useCanvasNodes";
import { FileMenu } from "./FileMenu";
import { NodeList } from "./NodeList";
import { useAutoSelect } from "./useAutoSelect";

/**
 * EditorSidebar — floating Panel with file header + node list.
 *
 * Header shows the recipe/bnto name with a file menu (Export,
 * Duplicate, Rename, Delete) and a collapse toggle. Below is a
 * scrollable list of canvas nodes.
 *
 * Reads nodes directly from the ReactFlow store (not Zustand editor).
 * Clicking a node updates RF selection via setNodes().
 */

interface EditorSidebarProps {
  collapsed: boolean;
  onToggle: () => void;
  selectedNodeId: string | null;
  /** Recipe/bnto name shown in the header. */
  name?: string;
  /** Optional footer content (e.g., recipe selector). */
  footer?: React.ReactNode;
}

function EditorSidebarRoot({
  collapsed,
  onToggle,
  selectedNodeId,
  name = "Untitled",
  footer,
}: EditorSidebarProps) {
  const nodes = useCanvasNodes();
  const { download, canExport } = useEditorExport();
  const { handleSelectNode } = useAutoSelect({ nodes, selectedNodeId });

  return (
    <Panel collapsed={collapsed} onToggle={onToggle} className="h-full w-56">
      <Panel.Header className="gap-2 px-3 pt-3 pb-2">
        <Panel.Trigger />
        <Text size="sm" className="min-w-0 flex-1 truncate font-medium">
          {name}
        </Text>
        <FileMenu
          onExport={download}
          canExport={canExport && nodes.length > 0}
        />
      </Panel.Header>
      {!collapsed && <Panel.Divider />}
      <Panel.Content>
        <NodeList
          nodes={nodes}
          selectedNodeId={selectedNodeId}
          onSelect={handleSelectNode}
        />
      </Panel.Content>
      {footer && !collapsed && (
        <>
          <Panel.Divider />
          <div className="px-2 py-2">{footer}</div>
        </>
      )}
    </Panel>
  );
}

export { EditorSidebarRoot };
export type { EditorSidebarProps };
