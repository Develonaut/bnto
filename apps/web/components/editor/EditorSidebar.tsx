"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import { cn } from "@/lib/cn";
import { Panel } from "@/components/ui/Panel";
import { Text } from "@/components/ui/Text";
import { LayersIcon } from "@/components/ui/icons";
import { useCanvasNodes } from "@/editor/hooks/useCanvasNodes";
import type { CompartmentNodeType } from "./canvas/CompartmentNode";
import type { CompartmentVariant } from "@/editor/adapters/types";

/**
 * EditorSidebar — floating Panel listing canvas nodes.
 *
 * Floats on top of the canvas with elevation. Collapses to a single
 * icon button (pill), expands to show header + scrollable node list.
 *
 * Reads nodes directly from the ReactFlow store (not Zustand editor).
 * Clicking a node updates RF selection via setNodes().
 */

/* ── Props ──────────────────────────────────────────────────── */

interface EditorSidebarProps {
  collapsed: boolean;
  onToggle: () => void;
  selectedNodeId: string | null;
}

/* ── Constants ──────────────────────────────────────────────── */

const DOT_COLOR: Record<CompartmentVariant, string> = {
  primary: "bg-primary",
  secondary: "bg-secondary",
  accent: "bg-accent",
  muted: "bg-muted-foreground",
  success: "bg-green-500",
  warning: "bg-amber-500",
};

/* ── Root ────────────────────────────────────────────────────── */

function EditorSidebarRoot({
  collapsed,
  onToggle,
  selectedNodeId,
}: EditorSidebarProps) {
  const nodes = useCanvasNodes();
  const { setNodes } = useReactFlow<CompartmentNodeType>();

  const handleSelectNode = useCallback(
    (nodeId: string) => {
      setNodes((prev) =>
        prev.map((n) => ({ ...n, selected: n.id === nodeId })),
      );
    },
    [setNodes],
  );

  return (
    <Panel collapsed={collapsed} onToggle={onToggle} className="h-full w-56">
      <Panel.Header className="gap-3.5 px-3 pt-4 pb-2">
        <LayersIcon className="size-3.5 text-muted-foreground" />
        <Text size="sm" className="font-medium">
          Nodes
        </Text>
        <Text size="xs" color="muted" className="font-mono">
          {nodes.length}
        </Text>
        <div className="flex-1" />
        <Panel.Trigger />
      </Panel.Header>
      <Panel.Divider />
      <Panel.Content>
        <NodeList
          nodes={nodes}
          selectedNodeId={selectedNodeId}
          onSelect={handleSelectNode}
        />
      </Panel.Content>
    </Panel>
  );
}

/* ── NodeList — scrollable list or empty prompt ──────────────── */

interface NodeListProps {
  nodes: CompartmentNodeType[];
  selectedNodeId: string | null;
  onSelect: (nodeId: string) => void;
}

function NodeList({ nodes, selectedNodeId, onSelect }: NodeListProps) {
  if (nodes.length === 0) {
    return (
      <div className="px-3 py-4">
        <Text size="xs" color="muted" className="text-center">
          No nodes yet. Add one from the toolbar.
        </Text>
      </div>
    );
  }

  return (
    <ul
      className="flex flex-1 flex-col gap-0.5 overflow-y-auto p-1.5"
      role="listbox"
      aria-label="Canvas nodes"
    >
      {nodes.map((node) => (
        <NodeItem
          key={node.id}
          node={node}
          selected={node.id === selectedNodeId}
          onSelect={onSelect}
        />
      ))}
    </ul>
  );
}

/* ── NodeItem — single row in the list ───────────────────────── */

interface NodeItemProps {
  node: CompartmentNodeType;
  selected: boolean;
  onSelect: (nodeId: string) => void;
}

function NodeItem({ node, selected, onSelect }: NodeItemProps) {
  const variant = node.data.variant ?? "muted";

  return (
    <li role="option" aria-selected={selected}>
      <button
        type="button"
        onClick={() => onSelect(node.id)}
        data-selected={selected}
        className="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left transition-colors duration-fast hover:bg-muted data-[selected=true]:bg-muted data-[selected=true]:ring-2 data-[selected=true]:ring-ring"
      >
        <span
          className={cn("size-2 shrink-0 rounded-full", DOT_COLOR[variant])}
        />
        <span className="flex min-w-0 flex-1 flex-col">
          <Text size="sm" className="truncate font-medium leading-tight">
            {node.data.label}
          </Text>
          {node.data.sublabel && (
            <Text size="xs" color="muted" className="truncate leading-tight">
              {node.data.sublabel}
            </Text>
          )}
        </span>
      </button>
    </li>
  );
}

/* ── Public API ──────────────────────────────────────────────── */

export const EditorSidebar = Object.assign(EditorSidebarRoot, {
  Root: EditorSidebarRoot,
});
